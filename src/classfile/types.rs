use std::collections::HashMap;
use super::constant_pool::ConstantPool;
use super::attributes::Attribute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessFlags(u16);

impl AccessFlags {
    pub const PUBLIC: u16 = 0x0001;
    pub const PRIVATE: u16 = 0x0002;
    pub const PROTECTED: u16 = 0x0004;
    pub const STATIC: u16 = 0x0008;
    pub const FINAL: u16 = 0x0010;
    pub const SUPER: u16 = 0x0020;
    pub const SYNCHRONIZED: u16 = 0x0020;
    pub const VOLATILE: u16 = 0x0040;
    pub const BRIDGE: u16 = 0x0040;
    pub const TRANSIENT: u16 = 0x0080;
    pub const VARARGS: u16 = 0x0080;
    pub const NATIVE: u16 = 0x0100;
    pub const INTERFACE: u16 = 0x0200;
    pub const ABSTRACT: u16 = 0x0400;
    pub const STRICT: u16 = 0x0800;
    pub const SYNTHETIC: u16 = 0x1000;
    pub const ANNOTATION: u16 = 0x2000;
    pub const ENUM: u16 = 0x4000;
    pub const MODULE: u16 = 0x8000;

    pub fn new(flags: u16) -> Self {
        AccessFlags(flags)
    }

    pub fn contains(&self, flag: u16) -> bool {
        (self.0 & flag) != 0
    }
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub access_flags: AccessFlags,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes: Vec<Attribute>,
}

impl FieldInfo {
    pub fn new(access_flags: u16, name_index: usize, descriptor_index: usize, attributes: Vec<Attribute>) -> Self {
        FieldInfo {
            access_flags: AccessFlags::new(access_flags),
            name_index,
            descriptor_index,
            attributes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: AccessFlags,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes: Vec<Attribute>,
}

impl MethodInfo {
    pub fn new(access_flags: u16, name_index: usize, descriptor_index: usize, attributes: Vec<Attribute>) -> Self {
        MethodInfo {
            access_flags: AccessFlags::new(access_flags),
            name_index,
            descriptor_index,
            attributes,
        }
    }

    pub fn get_code_attribute(&self) -> Option<&super::attributes::CodeAttribute> {
        for attr in &self.attributes {
            if let Attribute::Code(code) = attr {
                return Some(code);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    pub this_class: usize,
    pub super_class: usize,
    pub interfaces: Vec<usize>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn new(
        magic: u32,
        minor_version: u16,
        major_version: u16,
        constant_pool: ConstantPool,
        access_flags: u16,
        this_class: usize,
        super_class: usize,
        interfaces: Vec<usize>,
        fields: Vec<FieldInfo>,
        methods: Vec<MethodInfo>,
        attributes: Vec<Attribute>,
    ) -> Self {
        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags: AccessFlags::new(access_flags),
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }

    pub fn get_class_name(&self) -> Option<String> {
        self.constant_pool.get_class_name(self.this_class)
    }

    pub fn get_super_class_name(&self) -> Option<String> {
        if self.super_class == 0 {
            None
        } else {
            self.constant_pool.get_class_name(self.super_class)
        }
    }

    pub fn get_method(&self, name: &str, descriptor: &str) -> Option<&MethodInfo> {
        self.methods.iter().find(|m| {
            let m_name = self.constant_pool.get_utf8(m.name_index);
            let m_desc = self.constant_pool.get_utf8(m.descriptor_index);
            m_name == Some(name.to_string()) && m_desc == Some(descriptor.to_string())
        })
    }

    pub fn get_field(&self, name: &str, descriptor: &str) -> Option<&FieldInfo> {
        self.fields.iter().find(|f| {
            let f_name = self.constant_pool.get_utf8(f.name_index);
            let f_desc = self.constant_pool.get_utf8(f.descriptor_index);
            f_name == Some(name.to_string()) && f_desc == Some(descriptor.to_string())
        })
    }
}
