use std::env;
use std::fs;
use minijvm_lib::{
    classfile::ClassFileParser,
    runtime::{JVM, Frame, Value},
    interpreter::Interpreter,
    stdlib,
    JvmError,
};

fn main() {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: minijvm <classfile>");
        return;
    }
    
    let classfile_path = &args[1];
    
    match run_jvm(classfile_path) {
        Ok(_) => println!("Execution completed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

fn run_jvm(classfile_path: &str) -> Result<(), JvmError> {
    let class_data = fs::read(classfile_path)?;
    
    let mut parser = ClassFileParser::new(&class_data);
    let class_file = parser.parse()?;
    
    let class_name = class_file.get_class_name()
        .ok_or(JvmError::ClassFileError(minijvm_lib::error::ClassFileError::ClassNotFound("unknown".to_string())))?;
    
    let mut jvm = JVM::new();
    
    // Register native/standard library classes (Bootstrap class loader)
    stdlib::lang::register_standard_classes(&mut jvm);
    stdlib::io::PrintStream::register(&mut jvm);
    stdlib::io::register_io_classes(&mut jvm);
    stdlib::util::register_util_classes(&mut jvm);
    stdlib::math::register_math_classes(&mut jvm);
    stdlib::regex::register_regex_classes(&mut jvm);
    stdlib::lang::register_thread_natives(&mut jvm);
    
    // Set up System.out
    let print_stream_obj = minijvm_lib::runtime::heap::HeapObject::new("java.io.PrintStream".to_string());
    let print_stream_ref = jvm.allocate(print_stream_obj)?;
    jvm.method_area.set_static_field("java.lang.System", "out", "Ljava/io/PrintStream;", Value::ObjectRef(print_stream_ref));
    
    // Add the main class to the method area and run <clinit>
    let main_class = minijvm_lib::runtime::method_area::Class::new(class_file)?;
    let has_clinit = main_class.get_method("<clinit>", "()V").is_some();
    jvm.method_area.add_class(main_class);
    
    let main_thread_id = jvm.current_thread_id;
    if has_clinit {
        if let Some(clinit_method) = jvm.method_area.get_method(&class_name, "<clinit>", "()V") {
            let clinit_frame = Frame::new(clinit_method.clone());
            jvm.stack.push(clinit_frame)?;
            jvm.save_current_stack();
            let interpreter = Interpreter::new();
            interpreter.run(&mut jvm, main_thread_id)?;
        }
    }
    
    // Create and run the main method
    let main_method = jvm.method_area.get_method(&class_name, "main", "([Ljava/lang/String;)V")
        .ok_or(JvmError::RuntimeError(minijvm_lib::error::RuntimeError::MethodNotFound(class_name.clone(), "main".to_string())))?;
    
    let main_frame = Frame::new(main_method.clone());
    jvm.stack.push(main_frame)?;
    jvm.save_current_stack();
    
    let interpreter = Interpreter::new();
    interpreter.run(&mut jvm, main_thread_id)?;
    
    Ok(())
}
