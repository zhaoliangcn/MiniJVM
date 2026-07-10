use thiserror::Error;

#[derive(Error, Debug)]
pub enum JvmError {
    #[error("Class file parsing error: {0}")]
    ClassFileError(#[from] ClassFileError),
    
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] RuntimeError),
    
    #[error("Interpreter error: {0}")]
    InterpreterError(#[from] InterpreterError),
    
    #[error("GC error: {0}")]
    GcError(#[from] GcError),
    
    #[error("Threading error: {0}")]
    ThreadingError(#[from] ThreadingError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Error, Debug)]
pub enum ClassFileError {
    #[error("Invalid magic number: expected 0xCAFEBABE, got 0x{0:08X}")]
    InvalidMagic(u32),
    
    #[error("Unsupported class file version: {0}.{1}")]
    UnsupportedVersion(u16, u16),
    
    #[error("Invalid constant pool tag at index: {0}")]
    InvalidConstantPoolTag(usize),
    
    #[error("Constant pool index out of bounds: {0}")]
    ConstantPoolIndexOutOfBounds(usize),
    
    #[error("Invalid attribute name: {0}")]
    InvalidAttributeName(String),
    
    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),
    
    #[error("Invalid method descriptor: {0}")]
    InvalidMethodDescriptor(String),
    
    #[error("Invalid field descriptor: {0}")]
    InvalidFieldDescriptor(String),
    
    #[error("Unexpected end of class file")]
    UnexpectedEndOfFile,
    
    #[error("Invalid access flags: {0}")]
    InvalidAccessFlags(u16),
    
    #[error("Class not found: {0}")]
    ClassNotFound(String),
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Stack underflow")]
    StackUnderflow,
    
    #[error("Stack overflow")]
    StackOverflow,
    
    #[error("Local variable index out of bounds: {0}")]
    LocalVariableIndexOutOfBounds(usize),
    
    #[error("Heap allocation failed")]
    HeapAllocationFailed,
    
    #[error("Null pointer exception")]
    NullPointerException,
    
    #[error("Array index out of bounds: {0}")]
    ArrayIndexOutOfBounds(usize),
    
    #[error("Negative array size")]
    NegativeArraySize,
    
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },
    
    #[error("Method not found: {0}.{1}")]
    MethodNotFound(String, String),
    
    #[error("Field not found: {0}.{1}")]
    FieldNotFound(String, String),
    
    #[error("No such class: {0}")]
    NoSuchClass(String),
    
    #[error("Instantiation error for class: {0}")]
    InstantiationError(String),
    
    #[error("Illegal access error")]
    IllegalAccessError,
    
    #[error("Class cast error")]
    ClassCastError,
    
    #[error("Arithmetic exception")]
    ArithmeticException,
    
    #[error("Unsupported operation")]
    UnsupportedOperationException,
}

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Unknown opcode: 0x{0:02X}")]
    UnknownOpcode(u8),
    
    #[error("Invalid instruction format at PC {0}")]
    InvalidInstructionFormat(usize),
    
    #[error("Invalid branch offset")]
    InvalidBranchOffset,
    
    #[error("Method invocation error: {0}")]
    InvocationError(String),
    
    #[error("Return type mismatch")]
    ReturnTypeMismatch,
    
    #[error("Invalid method signature")]
    InvalidMethodSignature,
    
    #[error("Invalid stack map frame")]
    InvalidStackMapFrame,
}

#[derive(Error, Debug)]
pub enum GcError {
    #[error("GC thread interrupted")]
    Interrupted,
    
    #[error("Memory corruption detected")]
    MemoryCorruption,
    
    #[error("Reference processing error")]
    ReferenceProcessingError,
}

#[derive(Error, Debug)]
pub enum ThreadingError {
    #[error("Thread creation failed")]
    ThreadCreationFailed,
    
    #[error("Thread interrupted")]
    ThreadInterrupted,
    
    #[error("Deadlock detected")]
    DeadlockDetected,
    
    #[error("Illegal monitor state")]
    IllegalMonitorState,
}

pub type Result<T> = std::result::Result<T, JvmError>;
