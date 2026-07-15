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

fn load_all_classes_in_directory(jvm: &mut JVM) -> Result<(), JvmError> {
    let current_dir = env::current_dir()?;
    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "class" {
                    let class_data = fs::read(&path)?;
                    let mut parser = ClassFileParser::new(&class_data);
                    match parser.parse() {
                        Ok(class_file) => {
                            let class_name = class_file.get_class_name().unwrap_or_default();
                            if !jvm.method_area.has_class(&class_name) {
                                match minijvm_lib::runtime::method_area::Class::new(class_file) {
                                    Ok(class) => {
                                        jvm.method_area.add_class(class);
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn run_jvm(classfile_path: &str) -> Result<(), JvmError> {
    let class_data = fs::read(classfile_path)?;
    
    let mut parser = ClassFileParser::new(&class_data);
    let class_file = parser.parse()?;
    
    let class_name = class_file.get_class_name()
        .ok_or(JvmError::ClassFileError(minijvm_lib::error::ClassFileError::ClassNotFound("unknown".to_string())))?;
    
    let mut jvm = JVM::new();
    
    stdlib::lang::register_standard_classes(&mut jvm);
    stdlib::io::PrintStream::register(&mut jvm);
    
    // Register native Thread methods
    stdlib::lang::register_thread_natives(&mut jvm);
    
    let print_stream_obj = minijvm_lib::runtime::heap::HeapObject::new("java.io.PrintStream".to_string());
    let print_stream_ref = jvm.allocate(print_stream_obj)?;
    
    jvm.method_area.set_static_field("java.lang.System", "out", "Ljava/io/PrintStream;", Value::ObjectRef(print_stream_ref));
    
    let class = minijvm_lib::runtime::method_area::Class::new(class_file)?;
    jvm.method_area.add_class(class);
    
    load_all_classes_in_directory(&mut jvm)?;
    
    // Run <clinit> on the main thread
    let main_thread_id = jvm.current_thread_id;
    if let Some(clinit_method) = jvm.method_area.get_method(&class_name, "<clinit>", "()V") {
        let clinit_frame = Frame::new(clinit_method.clone());
        jvm.stack.push(clinit_frame)?;
        // Save the stack to the main thread's scheduler slot before running
        jvm.save_current_stack();
        let interpreter = Interpreter::new();
        interpreter.run(&mut jvm, main_thread_id)?;
    }
    
    // Create and run the main method on the main thread
    let main_method = jvm.method_area.get_method(&class_name, "main", "([Ljava/lang/String;)V")
        .ok_or(JvmError::RuntimeError(minijvm_lib::error::RuntimeError::MethodNotFound(class_name.clone(), "main".to_string())))?;
    
    let main_frame = Frame::new(main_method.clone());
    jvm.stack.push(main_frame)?;
    // Save the stack to the main thread's scheduler slot before running
    jvm.save_current_stack();
    
    let interpreter = Interpreter::new();
    // Run as a single-threaded program (the main thread)
    interpreter.run(&mut jvm, main_thread_id)?;
    
    Ok(())
}
