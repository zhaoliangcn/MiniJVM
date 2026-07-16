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
    pub monitor_owner: Option<usize>,
    pub monitor_count: usize,
    /// GC generation: 0 = young, 1 = old
    pub generation: u8,
    /// Age counter for young generation objects (survived collections)
    pub age: u8,
}

impl HeapObject {
    pub fn new(class_name: String) -> Self {
        HeapObject {
            class_name,
            fields: HashMap::new(),
            string_value: None,
            array_elements: None,
            array_length: 0,
            monitor_owner: None,
            monitor_count: 0,
            generation: 0,
            age: 0,
        }
    }

    pub fn new_string(class_name: String, value: String) -> Self {
        HeapObject {
            class_name,
            fields: HashMap::new(),
            string_value: Some(value),
            array_elements: None,
            array_length: 0,
            monitor_owner: None,
            monitor_count: 0,
            generation: 0,
            age: 0,
        }
    }

    pub fn new_array(class_name: String, length: usize) -> Self {
        HeapObject {
            class_name,
            fields: HashMap::new(),
            string_value: None,
            array_elements: Some(vec![Value::Null; length]),
            array_length: length,
            monitor_owner: None,
            monitor_count: 0,
            generation: 0,
            age: 0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_allocate() {
        let mut heap = Heap::new();
        let obj = HeapObject::new("java.lang.Object".to_string());
        let id = heap.allocate(obj).unwrap();
        assert!(id > 0);
        assert!(heap.get(id).is_some());
    }

    #[test]
    fn test_heap_get_mut() {
        let mut heap = Heap::new();
        let obj = HeapObject::new("java.lang.Object".to_string());
        let id = heap.allocate(obj).unwrap();
        let obj_mut = heap.get_mut(id).unwrap();
        obj_mut.class_name = "Modified".to_string();
        assert_eq!(heap.get(id).unwrap().class_name, "Modified");
    }

    #[test]
    fn test_heap_deallocate() {
        let mut heap = Heap::new();
        let obj = HeapObject::new("java.lang.Object".to_string());
        let id = heap.allocate(obj).unwrap();
        heap.deallocate(id).unwrap();
        assert!(heap.get(id).is_none());
    }

    #[test]
    fn test_heap_get_null() {
        let heap = Heap::new();
        assert!(heap.get(0).is_none());
    }

    #[test]
    fn test_heap_allocated_count() {
        let mut heap = Heap::new();
        assert_eq!(heap.allocated_count(), 0);
        heap.allocate(HeapObject::new("A".to_string())).unwrap();
        heap.allocate(HeapObject::new("B".to_string())).unwrap();
        assert_eq!(heap.allocated_count(), 2);
    }

    #[test]
    fn test_heap_object_new_string() {
        let obj = HeapObject::new_string("java.lang.String".to_string(), "hello".to_string());
        assert_eq!(obj.string_value, Some("hello".to_string()));
        assert!(!obj.is_array());
        assert!(obj.is_string());
    }

    #[test]
    fn test_heap_object_new_array() {
        let mut obj = HeapObject::new_array("java.lang.Object".to_string(), 5);
        assert_eq!(obj.array_length, 5);
        assert!(obj.is_array());
        assert!(!obj.is_string());
        obj.set_array_element(0, Value::Int(42)).unwrap();
        assert_eq!(*obj.get_array_element(0).unwrap(), Value::Int(42));
    }

    #[test]
    fn test_heap_object_fields() {
        let mut obj = HeapObject::new("java.lang.Point".to_string());
        obj.set_field("x", Value::Int(10));
        obj.set_field("y", Value::Int(20));
        assert_eq!(*obj.get_field("x").unwrap(), Value::Int(10));
        assert_eq!(*obj.get_field("y").unwrap(), Value::Int(20));
    }

    #[test]
    fn test_array_out_of_bounds() {
        let mut obj = HeapObject::new_array("[I".to_string(), 3);
        assert!(obj.set_array_element(10, Value::Int(0)).is_err());
        assert!(obj.get_array_element(10).is_err());
    }

    #[test]
    fn test_deallocate_invalid_id() {
        let mut heap = Heap::new();
        assert!(heap.deallocate(0).is_err());
        assert!(heap.deallocate(999).is_err());
    }
}
