use std::io::{Read, Cursor};
use super::types::{ClassFile, FieldInfo, MethodInfo};
use super::constant_pool::{ConstantPool, CpInfo};
use super::attributes::{Attribute, CodeAttribute, ExceptionTableEntry, StackMapTable, LineNumberTable, SourceFile};
use crate::error::{ClassFileError, Result};

pub struct ClassFileParser<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> ClassFileParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ClassFileParser {
            cursor: Cursor::new(data),
        }
    }

    pub fn parse(&mut self) -> Result<ClassFile> {
        self.parse_magic()?;
        let (minor_version, major_version) = self.parse_version()?;
        let constant_pool = self.parse_constant_pool()?;
        let access_flags = self.read_u16()?;
        let this_class = self.read_u16()? as usize;
        let super_class = self.read_u16()? as usize;
        let interfaces = self.parse_interfaces()?;
        let fields = self.parse_fields(&constant_pool)?;
        let methods = self.parse_methods(&constant_pool)?;
        let attributes = self.parse_attributes(&constant_pool)?;

        Ok(ClassFile::new(
            0xCAFEBABE,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        ))
    }

    fn parse_magic(&mut self) -> Result<()> {
        let magic = self.read_u32()?;
        if magic != 0xCAFEBABE {
            return Err(ClassFileError::InvalidMagic(magic));
        }
        Ok(())
    }

    fn parse_version(&mut self) -> Result<(u16, u16)> {
        let minor_version = self.read_u16()?;
        let major_version = self.read_u16()?;
        
        if major_version < 45 || major_version > 61 {
            return Err(ClassFileError::UnsupportedVersion(major_version, minor_version));
        }
        
        Ok((minor_version, major_version))
    }

    fn parse_constant_pool(&mut self) -> Result<ConstantPool> {
        let cp_count = self.read_u16()? as usize;
        let mut entries = Vec::with_capacity(cp_count);
        entries.push(None);

        for i in 1..cp_count {
            let tag = self.read_u8()?;
            let entry = self.parse_constant_pool_entry(tag, i)?;
            entries.push(Some(entry));
            
            if tag == 5 || tag == 6 {
                entries.push(None);
            }
        }

        Ok(ConstantPool::new(entries))
    }

    fn parse_constant_pool_entry(&mut self, tag: u8, _index: usize) -> Result<CpInfo> {
        match tag {
            1 => {
                let length = self.read_u16()? as usize;
                let mut bytes = vec![0u8; length];
                self.cursor.read_exact(&mut bytes)?;
                let string = String::from_utf8(bytes)?;
                Ok(CpInfo::Utf8(string))
            }
            3 => {
                let value = self.read_u32()? as i32;
                Ok(CpInfo::Integer(value))
            }
            4 => {
                let value = self.read_u32()?;
                Ok(CpInfo::Float(f32::from_bits(value)))
            }
            5 => {
                let high = self.read_u32()?;
                let low = self.read_u32()?;
                let value = ((high as i64) << 32) | (low as i64);
                Ok(CpInfo::Long(value))
            }
            6 => {
                let high = self.read_u32()?;
                let low = self.read_u32()?;
                let value = f64::from_bits(((high as u64) << 32) | (low as u64));
                Ok(CpInfo::Double(value))
            }
            7 => {
                let name_index = self.read_u16()? as usize;
                Ok(CpInfo::Class(name_index))
            }
            8 => {
                let string_index = self.read_u16()? as usize;
                Ok(CpInfo::String(string_index))
            }
            9 => {
                let class_index = self.read_u16()? as usize;
                let name_and_type_index = self.read_u16()? as usize;
                Ok(CpInfo::FieldRef { class_index, name_and_type_index })
            }
            10 => {
                let class_index = self.read_u16()? as usize;
                let name_and_type_index = self.read_u16()? as usize;
                Ok(CpInfo::MethodRef { class_index, name_and_type_index })
            }
            11 => {
                let class_index = self.read_u16()? as usize;
                let name_and_type_index = self.read_u16()? as usize;
                Ok(CpInfo::InterfaceMethodRef { class_index, name_and_type_index })
            }
            12 => {
                let name_index = self.read_u16()? as usize;
                let descriptor_index = self.read_u16()? as usize;
                Ok(CpInfo::NameAndType { name_index, descriptor_index })
            }
            15 => {
                let reference_kind = self.read_u8()?;
                let reference_index = self.read_u16()? as usize;
                Ok(CpInfo::MethodHandle { reference_kind, reference_index })
            }
            16 => {
                let descriptor_index = self.read_u16()? as usize;
                Ok(CpInfo::MethodType(descriptor_index))
            }
            17 => {
                let bootstrap_method_attr_index = self.read_u16()?;
                let name_and_type_index = self.read_u16()? as usize;
                Ok(CpInfo::Dynamic { bootstrap_method_attr_index, name_and_type_index })
            }
            18 => {
                let bootstrap_method_attr_index = self.read_u16()?;
                let name_and_type_index = self.read_u16()? as usize;
                Ok(CpInfo::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index })
            }
            19 => {
                let name_index = self.read_u16()? as usize;
                Ok(CpInfo::Module(name_index))
            }
            20 => {
                let name_index = self.read_u16()? as usize;
                Ok(CpInfo::Package(name_index))
            }
            _ => Err(ClassFileError::InvalidConstantPoolTag(tag)),
        }
    }

    fn parse_interfaces(&mut self) -> Result<Vec<usize>> {
        let interface_count = self.read_u16()? as usize;
        let mut interfaces = Vec::with_capacity(interface_count);
        for _ in 0..interface_count {
            interfaces.push(self.read_u16()? as usize);
        }
        Ok(interfaces)
    }

    fn parse_fields(&mut self, constant_pool: &ConstantPool) -> Result<Vec<FieldInfo>> {
        let field_count = self.read_u16()? as usize;
        let mut fields = Vec::with_capacity(field_count);
        for _ in 0..field_count {
            fields.push(self.parse_field(constant_pool)?);
        }
        Ok(fields)
    }

    fn parse_field(&mut self, constant_pool: &ConstantPool) -> Result<FieldInfo> {
        let access_flags = self.read_u16()?;
        let name_index = self.read_u16()? as usize;
        let descriptor_index = self.read_u16()? as usize;
        let attributes = self.parse_attributes(constant_pool)?;
        Ok(FieldInfo::new(access_flags, name_index, descriptor_index, attributes))
    }

    fn parse_methods(&mut self, constant_pool: &ConstantPool) -> Result<Vec<MethodInfo>> {
        let method_count = self.read_u16()? as usize;
        let mut methods = Vec::with_capacity(method_count);
        for _ in 0..method_count {
            methods.push(self.parse_method(constant_pool)?);
        }
        Ok(methods)
    }

    fn parse_method(&mut self, constant_pool: &ConstantPool) -> Result<MethodInfo> {
        let access_flags = self.read_u16()?;
        let name_index = self.read_u16()? as usize;
        let descriptor_index = self.read_u16()? as usize;
        let attributes = self.parse_attributes(constant_pool)?;
        Ok(MethodInfo::new(access_flags, name_index, descriptor_index, attributes))
    }

    fn parse_attributes(&mut self, constant_pool: &ConstantPool) -> Result<Vec<Attribute>> {
        let attribute_count = self.read_u16()? as usize;
        let mut attributes = Vec::with_capacity(attribute_count);
        for _ in 0..attribute_count {
            attributes.push(self.parse_attribute(constant_pool)?);
        }
        Ok(attributes)
    }

    fn parse_attribute(&mut self, constant_pool: &ConstantPool) -> Result<Attribute> {
        let name_index = self.read_u16()? as usize;
        let length = self.read_u32()? as usize;
        
        let name = constant_pool.get_utf8(name_index)
            .ok_or(ClassFileError::ConstantPoolIndexOutOfBounds(name_index))?;
        
        let mut data = vec![0u8; length];
        self.cursor.read_exact(&mut data)?;

        match name.as_str() {
            "Code" => self.parse_code_attribute(&mut data.as_slice(), constant_pool),
            "StackMapTable" => Ok(Attribute::StackMapTable(StackMapTable { entries: Vec::new() })),
            "LineNumberTable" => self.parse_line_number_table(&mut data.as_slice()),
            "SourceFile" => Ok(Attribute::SourceFile(SourceFile { source_file_index: name_index })),
            "InnerClasses" => Ok(Attribute::InnerClasses(super::attributes::InnerClasses { classes: Vec::new() })),
            "EnclosingMethod" => Ok(Attribute::EnclosingMethod(super::attributes::EnclosingMethod { class_index: 0, method_index: 0 })),
            "Synthetic" => Ok(Attribute::Synthetic),
            "Signature" => Ok(Attribute::Signature(name)),
            "BootstrapMethods" => Ok(Attribute::BootstrapMethods(super::attributes::BootstrapMethods { methods: Vec::new() })),
            "NestHost" => Ok(Attribute::NestHost(super::attributes::NestHost { host_class_index: 0 })),
            "NestMembers" => Ok(Attribute::NestMembers(super::attributes::NestMembers { member_classes: Vec::new() })),
            _ => Ok(Attribute::Unparsed(name, data)),
        }
    }

    fn parse_code_attribute(&self, data: &mut &[u8], constant_pool: &ConstantPool) -> Result<Attribute> {
        let mut cursor = Cursor::new(*data);
        let max_stack = cursor.read_u16()? as usize;
        let max_locals = cursor.read_u16()? as usize;
        let code_length = cursor.read_u32()? as usize;
        let mut code = vec![0u8; code_length];
        cursor.read_exact(&mut code)?;
        
        let exception_table_length = cursor.read_u16()? as usize;
        let mut exception_table = Vec::with_capacity(exception_table_length);
        for _ in 0..exception_table_length {
            let start_pc = cursor.read_u16()? as usize;
            let end_pc = cursor.read_u16()? as usize;
            let handler_pc = cursor.read_u16()? as usize;
            let catch_type = cursor.read_u16()? as usize;
            exception_table.push(ExceptionTableEntry::new(start_pc, end_pc, handler_pc, catch_type));
        }
        
        let mut remaining_data = vec![0u8; cursor.get_ref().len() - cursor.position() as usize];
        cursor.read_exact(&mut remaining_data)?;
        let mut remaining_cursor = Cursor::new(&remaining_data);
        let mut attr_parser = ClassFileParser { cursor: remaining_cursor };
        let attributes = attr_parser.parse_attributes(constant_pool)?;

        Ok(Attribute::Code(CodeAttribute::new(
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        )))
    }

    fn parse_line_number_table(&self, data: &mut &[u8]) -> Result<Attribute> {
        let mut cursor = Cursor::new(*data);
        let table_length = cursor.read_u16()? as usize;
        let mut entries = Vec::with_capacity(table_length);
        for _ in 0..table_length {
            let start_pc = cursor.read_u16()? as usize;
            let line_number = cursor.read_u16()? as usize;
            entries.push(super::attributes::LineNumberEntry { start_pc, line_number });
        }
        Ok(Attribute::LineNumberTable(LineNumberTable { entries }))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.cursor.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.cursor.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.cursor.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }
}
