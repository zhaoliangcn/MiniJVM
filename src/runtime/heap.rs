use std::collections::HashMap;
use crate::error::{RuntimeError, JvmError, Result};
use super::value::Value;

#[derive(Debug, Clone)]
pub struct HeapObject {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
    pub string_value: Option<String>,
    pub array_elements: Option<Vec<Value>>,
    pub array_length: usize,
}

impl HeapObject {
    pub fn new(class_name: String) -> Self {
        HeapObject {
            class_name,
            fields: HashMap::new(),
            string_value: None,
            array_elements: None,
            array_length: 0,
        }
    }

    pub fn new_string(class_name: String, value: String) -> Self {
        HeapObject {
            class_name,
            fields: HashMap::new(),
            string_value: Some(value),
            array_elements: None,
            array_length: 0,
        }
    }

    pub fn new_array(class_name: String, length: usize) -> Self {
        HeapObject {
            class_name,
            fields: HashMap::new(),
            string_value: None,
            array_elements: Some(vec![Value::Null; length]),
            array_length: length,
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&Value> {
        self.fields.get(name)
    }

    pub fn set_field(&mut self, name: &str, value: Value) {
        self.fields.insert(name.to_string(), value);
    }

    pub fn get_array_element(&self, index: usize) -> Result<&Value> {
        match &self.array_elements {
            Some(elements) => elements.get(index)
                .ok_or(JvmError::RuntimeError(RuntimeError::ArrayIndexOutOfBounds(index))),
            None => Err(JvmError::RuntimeError(RuntimeError::UnsupportedOperationException)),
        }
    }

    pub fn set_array_element(&mut self, index: usize, value: Value) -> Result<()> {
        match &mut self.array_elements {
            Some(elements) => {
                if index >= elements.len() {
                    return Err(JvmError::RuntimeError(RuntimeError::ArrayIndexOutOfBounds(index)));
                }
                elements[index] = value;
                Ok(())
            }
            None => Err(JvmError::RuntimeError(RuntimeError::UnsupportedOperationException)),
        }
    }

    pub fn is_array(&self) -> bool {
        self.array_elements.is_some()
    }

    pub fn is_string(&self) -> bool {
        self.string_value.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct Heap {
    objects: Vec<Option<HeapObject>>,
    next_id: usize,
    max_size: usize,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            objects: Vec::new(),
            next_id: 1,
            max_size: 1024 * 1024,
        }
    }

    pub fn allocate(&mut self, object: HeapObject) -> Result<usize> {
        if self.objects.len() >= self.max_size {
            return Err(JvmError::RuntimeError(RuntimeError::HeapAllocationFailed));
        }
        
        let id = self.next_id;
        
        if id >= self.objects.len() {
            self.objects.resize(id + 1, None);
        }
        
        self.objects[id] = Some(object);
        self.next_id += 1;
        
        Ok(id)
    }

    pub fn get(&self, id: usize) -> Option<&HeapObject> {
        if id == 0 {
            None
        } else {
            self.objects.get(id).and_then(|e| e.as_ref())
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut HeapObject> {
        if id == 0 {
            None
        } else {
            self.objects.get_mut(id).and_then(|e| e.as_mut())
        }
    }

    pub fn deallocate(&mut self, id: usize) -> Result<()> {
        if id == 0 || id >= self.objects.len() {
            return Err(JvmError::RuntimeError(RuntimeError::NullPointerException));
        }
        
        self.objects[id] = None;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn allocated_count(&self) -> usize {
        self.objects.iter().filter(|e| e.is_some()).count()
    }
}
