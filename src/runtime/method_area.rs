use std::collections::HashMap;
use std::sync::Arc;
use crate::error::{RuntimeError, JvmError, Result};
use crate::classfile::{ClassFile, MethodInfo, FieldInfo};
use crate::classfile::attributes::CodeAttribute;
use crate::runtime::{JVM, Frame, Value};

pub type NativeImplementation = Arc<dyn Fn(&mut Frame, &mut JVM) -> Result<()> + Send + Sync>;

#[derive(Clone)]
pub struct Method {
    pub class_name: String,
    pub name: String,
    pub descriptor: String,
    pub code: Vec<u8>,
    pub max_stack: usize,
    pub max_locals: usize,
    pub is_native: bool,
    pub is_static: bool,
    pub native_impl: Option<NativeImplementation>,
}

impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method")
            .field("class_name", &self.class_name)
            .field("name", &self.name)
            .field("descriptor", &self.descriptor)
            .field("code_len", &self.code.len())
            .field("max_stack", &self.max_stack)
            .field("max_locals", &self.max_locals)
            .field("is_native", &self.is_native)
            .field("is_static", &self.is_static)
            .field("native_impl", &self.native_impl.is_some())
            .finish()
    }
}

impl Method {
    pub fn new(class_name: String, method_info: &MethodInfo, constant_pool: &crate::classfile::constant_pool::ConstantPool) -> Result<Self> {
        let name = constant_pool.get_utf8(method_info.name_index)
            .ok_or(JvmError::RuntimeError(RuntimeError::UnsupportedOperationException))?;
        let descriptor = constant_pool.get_utf8(method_info.descriptor_index)
            .ok_or(JvmError::RuntimeError(RuntimeError::UnsupportedOperationException))?;
        
        let (code, max_stack, max_locals) = if let Some(code_attr) = method_info.get_code_attribute() {
            (code_attr.code.clone(), code_attr.max_stack, code_attr.max_locals)
        } else {
            (Vec::new(), 0, 0)
        };

        Ok(Method {
            class_name,
            name,
            descriptor,
            code,
            max_stack,
            max_locals,
            is_native: method_info.access_flags.contains(crate::classfile::types::AccessFlags::NATIVE),
            is_static: method_info.access_flags.contains(crate::classfile::types::AccessFlags::STATIC),
            native_impl: None,
        })
    }

    pub fn new_native(class_name: String, name: String, descriptor: String, is_static: bool, native_impl: Option<NativeImplementation>) -> Self {
        let (param_count, _) = crate::interpreter::instruction_set::parse_method_descriptor(&descriptor);
        let max_locals = if is_static { param_count } else { param_count + 1 };
        Method {
            class_name,
            name,
            descriptor,
            code: Vec::new(),
            max_stack: 0,
            max_locals,
            is_native: true,
            is_static,
            native_impl,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub class_name: String,
    pub name: String,
    pub descriptor: String,
    pub is_static: bool,
    pub is_final: bool,
    pub initial_value: Option<crate::runtime::value::Value>,
}

impl Field {
    pub fn new(class_name: String, field_info: &FieldInfo, constant_pool: &crate::classfile::constant_pool::ConstantPool) -> Result<Self> {
        let name = constant_pool.get_utf8(field_info.name_index)
            .ok_or(JvmError::RuntimeError(RuntimeError::UnsupportedOperationException))?;
        let descriptor = constant_pool.get_utf8(field_info.descriptor_index)
            .ok_or(JvmError::RuntimeError(RuntimeError::UnsupportedOperationException))?;

        Ok(Field {
            class_name,
            name,
            descriptor,
            is_static: field_info.access_flags.contains(crate::classfile::types::AccessFlags::STATIC),
            is_final: field_info.access_flags.contains(crate::classfile::types::AccessFlags::FINAL),
            initial_value: None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub class_file: ClassFile,
    pub methods: HashMap<String, Method>,
    pub fields: HashMap<String, Field>,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,
    pub instance_fields: Vec<String>,
    pub static_fields: HashMap<String, crate::runtime::value::Value>,
}

impl Class {
    pub fn new(class_file: ClassFile) -> Result<Self> {
        let mut methods = HashMap::new();
        let mut fields = HashMap::new();
        let mut instance_fields = Vec::new();
        let mut static_fields = HashMap::new();

        let class_name = class_file.get_class_name().unwrap_or_default();

        for method_info in &class_file.methods {
            let method = Method::new(class_name.clone(), method_info, &class_file.constant_pool)?;
            let key = format!("{}:{}", method.name, method.descriptor);
            methods.insert(key, method);
        }

        for field_info in &class_file.fields {
            let field = Field::new(class_name.clone(), field_info, &class_file.constant_pool)?;
            let key = format!("{}:{}", field.name, field.descriptor);
            fields.insert(key.clone(), field.clone());
            
            if field.is_static {
                static_fields.insert(key, crate::runtime::value::Value::Null);
            } else {
                instance_fields.push(key);
            }
        }

        let super_class = class_file.get_super_class_name();

        let interfaces = class_file.interfaces.iter()
            .filter_map(|&idx| class_file.constant_pool.get_class_name(idx))
            .collect();

        Ok(Class {
            class_file,
            methods,
            fields,
            super_class,
            interfaces,
            instance_fields,
            static_fields,
        })
    }

    pub fn get_method(&self, name: &str, descriptor: &str) -> Option<&Method> {
        self.methods.get(&format!("{}:{}", name, descriptor))
    }

    pub fn get_field(&self, name: &str, descriptor: &str) -> Option<&Field> {
        self.fields.get(&format!("{}:{}", name, descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct MethodArea {
    classes: HashMap<String, Class>,
}

impl MethodArea {
    pub fn new() -> Self {
        MethodArea {
            classes: HashMap::new(),
        }
    }

    pub fn add_class(&mut self, class: Class) {
        let class_name = class.class_file.get_class_name().unwrap_or_default();
        self.classes.insert(class_name, class);
    }

    pub fn add_native_method(&mut self, class_name: &str, method: Method) {
        if let Some(class) = self.classes.get_mut(class_name) {
            let key = format!("{}:{}", method.name, method.descriptor);
            class.methods.insert(key, method);
        } else {
            let mut methods = HashMap::new();
            let key = format!("{}:{}", method.name, method.descriptor);
            methods.insert(key, method);
            let class_file = crate::classfile::types::ClassFile::new(
                0xCAFEBABE, 0, 0,
                crate::classfile::constant_pool::ConstantPool::new(vec![None]),
                0, 1, 0, vec![], vec![], vec![], vec![],
            );
            let class = Class {
                class_file,
                methods,
                fields: HashMap::new(),
                super_class: None,
                interfaces: vec![],
                instance_fields: vec![],
                static_fields: HashMap::new(),
            };
            self.classes.insert(class_name.to_string(), class);
        }
    }

    pub fn get_class(&self, class_name: &str) -> Option<&Class> {
        self.classes.get(class_name)
    }

    pub fn get_class_mut(&mut self, class_name: &str) -> Option<&mut Class> {
        self.classes.get_mut(class_name)
    }

    pub fn set_static_field(&mut self, class_name: &str, field_name: &str, descriptor: &str, value: crate::runtime::value::Value) {
        if let Some(class) = self.classes.get_mut(class_name) {
            let key = format!("{}:{}", field_name, descriptor);
            class.static_fields.insert(key, value);
        }
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        self.classes.contains_key(class_name)
    }

    pub fn get_method(&self, class_name: &str, method_name: &str, descriptor: &str) -> Option<&Method> {
        self.get_class(class_name)?.get_method(method_name, descriptor)
    }

    pub fn get_field(&self, class_name: &str, field_name: &str, descriptor: &str) -> Option<&Field> {
        self.get_class(class_name)?.get_field(field_name, descriptor)
    }

    pub fn classes_count(&self) -> usize {
        self.classes.len()
    }
}
