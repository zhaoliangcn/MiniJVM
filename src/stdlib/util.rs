use std::sync::Arc;
use crate::error::{JvmError, RuntimeError, Result};
use crate::runtime::{JVM, Frame, Value, HeapObject, method_area::{Method, NativeImplementation}};

// ========== java.util.ArrayList ==========

pub struct ArrayList;

impl ArrayList {
    /// ArrayList() void constructor
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Allocate first, then update the object
                let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
                let arr_ref = jvm.allocate(arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    /// ArrayList(int) constructor with initial capacity
    pub fn init_capacity() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let capacity = frame.get_local(1)?.as_int();
            if let Value::ObjectRef(this_id) = this_ref {
                // Allocate first, then update the object
                let initial_capacity = capacity.max(0) as usize;
                let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), initial_capacity);
                let arr_ref = jvm.allocate(arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "<init>".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn size() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let size = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(size))?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn add() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract fields first, then drop the borrow
                let (current_size, arr_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let a_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (size, a_ref)
                };
                
                // Grow the array if needed
                if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                    if let Some(elements) = &mut arr.array_elements {
                        if current_size >= elements.len() {
                            let new_capacity = (elements.len() * 3 / 2 + 1).max(10);
                            let mut new_elements = vec![Value::Null; new_capacity];
                            for (i, e) in elements.iter().enumerate() {
                                new_elements[i] = e.clone();
                            }
                            *elements = new_elements;
                        }
                        if current_size < elements.len() {
                            elements[current_size] = elem;
                        }
                    }
                }
                // Update size
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int((current_size + 1) as i32));
                }
            }
            frame.push(Value::Boolean(true))?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "add".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn get() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let index = frame.pop()?.as_int() as usize;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let arr_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            if index < elements.len() {
                                frame.push(elements[index].clone())?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "get".to_string(), "(I)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn isEmpty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let size = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Boolean(size == 0))?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "isEmpty".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::init());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::init_capacity());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::size());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::add());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::get());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::isEmpty());
    }
}

// ========== java.util.HashMap ==========

pub struct HashMap;

impl HashMap {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Allocate arrays first, then update the object
                let keys_arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
                let vals_arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
                let keys_ref = jvm.allocate(keys_arr)?;
                let vals_ref = jvm.allocate(vals_arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("keys".to_string(), Value::ArrayRef(keys_ref));
                    obj.fields.insert("values".to_string(), Value::ArrayRef(vals_ref));
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn init_capacity() -> Method {
        // Simplified: ignore initial capacity, just use default
        Self::init()
    }

    pub fn size() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let size = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(size))?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn put() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let value = frame.pop()?;
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract fields first, then drop the borrow
                let (current_size, keys_ref, vals_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (size, k_ref, v_ref)
                };
                
                // Check if key already exists
                let mut found_index = None;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(elements) = &keys_arr.array_elements {
                        for (i, e) in elements.iter().enumerate() {
                            if *e == key {
                                found_index = Some(i);
                                break;
                            }
                        }
                    }
                }
                
                if let Some(idx) = found_index {
                    // Update existing entry
                    let this_id_val = *this_id; // Copy the id before mutable borrow
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(elements) = &mut vals_arr.array_elements {
                            if idx < elements.len() {
                                let old_val = std::mem::replace(&mut elements[idx], value);
                                frame.push(old_val)?;
                                // Update the map's fields
                                if let Some(obj) = jvm.heap.get_mut(this_id_val) {
                                    obj.fields.insert("size".to_string(), Value::Int(current_size as i32));
                                }
                                return Ok(());
                            }
                        }
                    }
                } else {
                    // Add new entry
                    let new_size = current_size + 1;
                    let grow_to = (new_size * 3 / 2 + 1).max(10);
                    
                    // Grow keys array if needed
                    if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                        if let Some(elements) = &mut keys_arr.array_elements {
                            if current_size >= elements.len() {
                                let mut new_elems = vec![Value::Null; grow_to];
                                for (i, e) in elements.iter().enumerate() {
                                    new_elems[i] = e.clone();
                                }
                                *elements = new_elems;
                            }
                            if current_size < elements.len() {
                                elements[current_size] = key;
                            }
                        }
                    }
                    // Grow values array if needed
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(elements) = &mut vals_arr.array_elements {
                            if current_size >= elements.len() {
                                let mut new_elems = vec![Value::Null; grow_to];
                                for (i, e) in elements.iter().enumerate() {
                                    new_elems[i] = e.clone();
                                }
                                *elements = new_elems;
                            }
                            if current_size < elements.len() {
                                elements[current_size] = value;
                            }
                        }
                    }
                    // Update the map's fields
                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                        obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "put".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn get() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let keys_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let vals_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    
                    if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                        if let Some(elements) = &keys_arr.array_elements {
                            for (i, e) in elements.iter().enumerate() {
                                if *e == key {
                                    if let Some(vals_arr) = jvm.heap.get(vals_ref) {
                                        if let Some(vals) = &vals_arr.array_elements {
                                            if i < vals.len() {
                                                frame.push(vals[i].clone())?;
                                                return Ok(());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "get".to_string(), "(Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn containsKey() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let mut found = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let keys_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                        if let Some(elements) = &keys_arr.array_elements {
                            found = elements.iter().any(|e| *e == key);
                        }
                    }
                }
            }
            frame.push(Value::Boolean(found))?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "containsKey".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn isEmpty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let size = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Boolean(size == 0))?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "isEmpty".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::init());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::init_capacity());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::size());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::put());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::get());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::containsKey());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::isEmpty());
    }
}

/// Register all java.util classes with the JVM.
pub fn register_util_classes(jvm: &mut JVM) {
    ArrayList::register(jvm);
    HashMap::register(jvm);
}