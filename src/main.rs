use std::env;
use std::fs;
use minijvm_lib::{
    classfile::{ClassFileParser, ClassFile},
    runtime::{JVM, Frame},
    interpreter::Interpreter,
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
    
    println!("Loaded class: {}", class_name);
    
    let mut jvm = JVM::new();
    
    let class = minijvm_lib::runtime::method_area::Class::new(class_file)?;
    jvm.method_area.add_class(class);
    
    let main_method = jvm.method_area.get_method(&class_name, "main", "([Ljava/lang/String;)V")
        .ok_or(JvmError::RuntimeError(minijvm_lib::error::RuntimeError::MethodNotFound(class_name.clone(), "main".to_string())))?;
    
    let mut main_frame = Frame::new(main_method.clone());
    jvm.stack.push(main_frame)?;
    
    let interpreter = Interpreter::new();
    interpreter.run(&mut jvm)?;
    
    Ok(())
}
