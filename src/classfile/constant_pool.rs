use std::fmt;
use crate::error::{ClassFileError, JvmError, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum CpInfo {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(usize),
    String(usize),
    FieldRef { class_index: usize, name_and_type_index: usize },
    MethodRef { class_index: usize, name_and_type_index: usize },
    InterfaceMethodRef { class_index: usize, name_and_type_index: usize },
    NameAndType { name_index: usize, descriptor_index: usize },
    MethodHandle { reference_kind: u8, reference_index: usize },
    MethodType(usize),
    Dynamic { bootstrap_method_attr_index: u16, name_and_type_index: usize },
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: usize },
    Module(usize),
    Package(usize),
    RecordComponent { name_index: usize, descriptor_index: usize },
}

impl fmt::Display for CpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpInfo::Utf8(s) => write!(f, "Utf8({})", s),
            CpInfo::Integer(v) => write!(f, "Integer({})", v),
            CpInfo::Float(v) => write!(f, "Float({})", v),
            CpInfo::Long(v) => write!(f, "Long({})", v),
            CpInfo::Double(v) => write!(f, "Double({})", v),
            CpInfo::Class(idx) => write!(f, "Class(#{})", idx),
            CpInfo::String(idx) => write!(f, "String(#{})", idx),
            CpInfo::FieldRef { class_index, name_and_type_index } => 
                write!(f, "FieldRef(#{}, #{})", class_index, name_and_type_index),
            CpInfo::MethodRef { class_index, name_and_type_index } => 
                write!(f, "MethodRef(#{}, #{})", class_index, name_and_type_index),
            CpInfo::InterfaceMethodRef { class_index, name_and_type_index } => 
                write!(f, "InterfaceMethodRef(#{}, #{})", class_index, name_and_type_index),
            CpInfo::NameAndType { name_index, descriptor_index } => 
                write!(f, "NameAndType(#{}, #{})", name_index, descriptor_index),
            CpInfo::MethodHandle { reference_kind, reference_index } => 
                write!(f, "MethodHandle({} #{})", reference_kind, reference_index),
            CpInfo::MethodType(descriptor_index) => 
                write!(f, "MethodType(#{})", descriptor_index),
            CpInfo::Dynamic { bootstrap_method_attr_index, name_and_type_index } => 
                write!(f, "Dynamic(BSM#{}, #{})", bootstrap_method_attr_index, name_and_type_index),
            CpInfo::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index } => 
                write!(f, "InvokeDynamic(BSM#{}, #{})", bootstrap_method_attr_index, name_and_type_index),
            CpInfo::Module(name_index) => write!(f, "Module(#{})", name_index),
            CpInfo::Package(name_index) => write!(f, "Package(#{})", name_index),
            CpInfo::RecordComponent { name_index, descriptor_index } => 
                write!(f, "RecordComponent(#{}, #{})", name_index, descriptor_index),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    entries: Vec<Option<CpInfo>>,
}

impl ConstantPool {
    pub fn new(entries: Vec<Option<CpInfo>>) -> Self {
        ConstantPool { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&CpInfo> {
        if index == 0 {
            None
        } else {
            self.entries.get(index).and_then(|e| e.as_ref())
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut CpInfo> {
        if index == 0 {
            None
        } else {
            self.entries.get_mut(index).and_then(|e| e.as_mut())
        }
    }

    pub fn get_utf8(&self, index: usize) -> Option<String> {
        match self.get(index) {
            Some(CpInfo::Utf8(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_class_name(&self, class_index: usize) -> Option<String> {
        match self.get(class_index) {
            Some(CpInfo::Class(name_index)) => self.get_utf8(*name_index),
            _ => None,
        }
    }

    pub fn resolve_method_ref(&self, method_ref_index: usize) -> Result<(String, String, String)> {
        let cp_info = self.get(method_ref_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(method_ref_index)))?;
        
        let (class_index, name_and_type_index) = match cp_info {
            CpInfo::MethodRef { class_index, name_and_type_index } => 
                (*class_index, *name_and_type_index),
            CpInfo::InterfaceMethodRef { class_index, name_and_type_index } => 
                (*class_index, *name_and_type_index),
            _ => return Err(JvmError::ClassFileError(ClassFileError::InvalidConstantPoolTag(method_ref_index))),
        };

        let class_name = self.get_class_name(class_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(class_index)))?;
        
        let name_and_type = self.get(name_and_type_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(name_and_type_index)))?;
        
        let (name_index, descriptor_index) = match name_and_type {
            CpInfo::NameAndType { name_index, descriptor_index } => 
                (*name_index, *descriptor_index),
            _ => return Err(JvmError::ClassFileError(ClassFileError::InvalidConstantPoolTag(name_and_type_index))),
        };

        let method_name = self.get_utf8(name_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(name_index)))?;
        
        let descriptor = self.get_utf8(descriptor_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(descriptor_index)))?;

        Ok((class_name.replace('/', "."), method_name, descriptor))
    }

    pub fn resolve_field_ref(&self, field_ref_index: usize) -> Result<(String, String, String)> {
        let cp_info = self.get(field_ref_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(field_ref_index)))?;
        
        let (class_index, name_and_type_index) = match cp_info {
            CpInfo::FieldRef { class_index, name_and_type_index } => 
                (*class_index, *name_and_type_index),
            _ => return Err(JvmError::ClassFileError(ClassFileError::InvalidConstantPoolTag(field_ref_index))),
        };

        let class_name = self.get_class_name(class_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(class_index)))?;
        
        let name_and_type = self.get(name_and_type_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(name_and_type_index)))?;
        
        let (name_index, descriptor_index) = match name_and_type {
            CpInfo::NameAndType { name_index, descriptor_index } => 
                (*name_index, *descriptor_index),
            _ => return Err(JvmError::ClassFileError(ClassFileError::InvalidConstantPoolTag(name_and_type_index))),
        };

        let field_name = self.get_utf8(name_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(name_index)))?;
        
        let descriptor = self.get_utf8(descriptor_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(descriptor_index)))?;

        Ok((class_name.replace('/', "."), field_name, descriptor))
    }

    pub fn resolve_string(&self, string_index: usize) -> Result<String> {
        let cp_info = self.get(string_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(string_index)))?;
        
        let utf8_index = match cp_info {
            CpInfo::String(index) => *index,
            _ => return Err(JvmError::ClassFileError(ClassFileError::InvalidConstantPoolTag(string_index))),
        };

        Ok(self.get_utf8(utf8_index)
            .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(utf8_index)))?)
    }
}
