use std::fmt;

#[derive(Debug, Clone)]
pub enum Attribute {
    Code(CodeAttribute),
    StackMapTable(StackMapTable),
    LineNumberTable(LineNumberTable),
    LocalVariableTable(LocalVariableTable),
    SourceFile(SourceFile),
    InnerClasses(InnerClasses),
    EnclosingMethod(EnclosingMethod),
    Synthetic,
    Signature(String),
    BootstrapMethods(BootstrapMethods),
    NestHost(NestHost),
    NestMembers(NestMembers),
    Record(RecordAttribute),
    PermittedSubclasses(PermittedSubclasses),
    Unparsed(String, Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Attribute>,
}

impl CodeAttribute {
    pub fn new(
        max_stack: usize,
        max_locals: usize,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute>,
    ) -> Self {
        CodeAttribute {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    pub start_pc: usize,
    pub end_pc: usize,
    pub handler_pc: usize,
    pub catch_type: usize,
}

impl ExceptionTableEntry {
    pub fn new(start_pc: usize, end_pc: usize, handler_pc: usize, catch_type: usize) -> Self {
        ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapTable {
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    SameFrame(u8),
    SameLocals1StackItemFrame(u8, VerificationType),
    SameLocals1StackItemFrameExtended(u16, VerificationType),
    ChopFrame(u8, u16),
    SameFrameExtended(u16),
    AppendFrame(u8, u16, Vec<VerificationType>),
    FullFrame(u16, Vec<VerificationType>, Vec<VerificationType>),
}

#[derive(Debug, Clone)]
pub enum VerificationType {
    Top,
    Integer,
    Float,
    Double,
    Long,
    Null,
    UninitializedThis,
    Object(usize),
    Uninitialized(usize),
}

#[derive(Debug, Clone)]
pub struct LineNumberTable {
    pub entries: Vec<LineNumberEntry>,
}

#[derive(Debug, Clone)]
pub struct LineNumberEntry {
    pub start_pc: usize,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct LocalVariableTable {
    pub entries: Vec<LocalVariableEntry>,
}

#[derive(Debug, Clone)]
pub struct LocalVariableEntry {
    pub start_pc: usize,
    pub length: usize,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub source_file_index: usize,
}

#[derive(Debug, Clone)]
pub struct InnerClasses {
    pub classes: Vec<InnerClassEntry>,
}

#[derive(Debug, Clone)]
pub struct InnerClassEntry {
    pub inner_class_info_index: usize,
    pub outer_class_info_index: usize,
    pub inner_name_index: usize,
    pub inner_class_access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct EnclosingMethod {
    pub class_index: usize,
    pub method_index: usize,
}

#[derive(Debug, Clone)]
pub struct BootstrapMethods {
    pub methods: Vec<BootstrapMethod>,
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: usize,
    pub bootstrap_arguments: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct NestHost {
    pub host_class_index: usize,
}

#[derive(Debug, Clone)]
pub struct NestMembers {
    pub member_classes: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct RecordAttribute {
    pub components: Vec<RecordComponentInfo>,
}

#[derive(Debug, Clone)]
pub struct RecordComponentInfo {
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct PermittedSubclasses {
    pub permitted_subclass_indices: Vec<usize>,
}
