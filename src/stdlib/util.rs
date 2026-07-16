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

// ========== java.util.HashSet ==========

pub struct HashSet;

impl HashSet {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // HashSet uses HashMap internally
                let map = HeapObject::new("java.util.HashMap".to_string());
                let map_ref = jvm.allocate(map)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("map".to_string(), Value::ObjectRef(map_ref));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.HashSet".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn add() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Get the map reference from the HashSet object
                let map_ref = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    obj.fields.get("map")
                        .and_then(|v| if let Value::ObjectRef(m) = v { Some(*m) } else { None })
                        .unwrap_or(0)
                };
                if map_ref == 0 { frame.push(Value::Boolean(true))?; return Ok(()); }
                
                // Extract current state from the map
                let (keys_ref, vals_ref, current_size, found) = {
                    let map_obj = jvm.heap.get(map_ref)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let k_ref = map_obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = map_obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = map_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let mut f = false;
                    if let Some(keys_arr) = jvm.heap.get(k_ref) {
                        if let Some(keys) = &keys_arr.array_elements {
                            f = keys.iter().any(|k| *k == elem);
                        }
                    }
                    (k_ref, v_ref, size, f)
                };
                
                if !found {
                    // Add to the map's keys array
                    let new_size = current_size + 1;
                    if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                        if let Some(keys) = &mut keys_arr.array_elements {
                            if current_size >= keys.len() {
                                let mut new_keys = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, k) in keys.iter().enumerate() {
                                    new_keys[i] = k.clone();
                                }
                                *keys = new_keys;
                            }
                            if current_size < keys.len() {
                                keys[current_size] = elem;
                            }
                        }
                    }
                    // Add to values array
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if current_size >= vals.len() {
                                let mut new_vals = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, v) in vals.iter().enumerate() {
                                    new_vals[i] = v.clone();
                                }
                                *vals = new_vals;
                            }
                            if current_size < vals.len() {
                                vals[current_size] = Value::Null;
                            }
                        }
                    }
                    // Update size
                    if let Some(map_obj) = jvm.heap.get_mut(map_ref) {
                        map_obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                    }
                }
            }
            frame.push(Value::Boolean(true))?;
            Ok(())
        });
        Method::new_native("java.util.HashSet".to_string(), "add".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn size() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let size = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(map_ref)) = obj.fields.get("map") {
                        if let Some(map_obj) = jvm.heap.get(*map_ref) {
                            map_obj.fields.get("size")
                                .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                                .unwrap_or(0)
                        } else { 0 }
                    } else { 0 }
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(size))?;
            Ok(())
        });
        Method::new_native("java.util.HashSet".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn isEmpty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let size = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(map_ref)) = obj.fields.get("map") {
                        if let Some(map_obj) = jvm.heap.get(*map_ref) {
                            map_obj.fields.get("size")
                                .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                                .unwrap_or(0)
                        } else { 0 }
                    } else { 0 }
                } else { 0 }
            } else { 0 };
            frame.push(Value::Boolean(size == 0))?;
            Ok(())
        });
        Method::new_native("java.util.HashSet".to_string(), "isEmpty".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn contains() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let mut found = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(map_ref)) = obj.fields.get("map") {
                        if let Some(map_obj) = jvm.heap.get(*map_ref) {
                            let keys_ref = map_obj.fields.get("keys")
                                .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                                .unwrap_or(0);
                            if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                                if let Some(keys) = &keys_arr.array_elements {
                                    found = keys.iter().any(|k| *k == elem);
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(found))?;
            Ok(())
        });
        Method::new_native("java.util.HashSet".to_string(), "contains".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.HashSet", HashSet::init());
        jvm.method_area.add_native_method("java.util.HashSet", HashSet::add());
        jvm.method_area.add_native_method("java.util.HashSet", HashSet::size());
        jvm.method_area.add_native_method("java.util.HashSet", HashSet::isEmpty());
        jvm.method_area.add_native_method("java.util.HashSet", HashSet::contains());
    }
}

// ========== java.util.Arrays ==========

pub struct Arrays;

impl Arrays {
    pub fn asList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            // Return the array itself wrapped in a simple list view
            let arr = frame.pop()?;
            // Create a simple list wrapper
            let list = HeapObject::new("java.util.Arrays$ArrayList".to_string());
            let list_ref = jvm.allocate(list)?;
            let arr_copy = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
            let arr_copy_ref = jvm.allocate(arr_copy)?;
            if let Some(obj) = jvm.heap.get_mut(list_ref) {
                obj.fields.insert("elementData".to_string(), arr);
                obj.fields.insert("size".to_string(), Value::Int(0));
            }
            frame.push(Value::ObjectRef(list_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Arrays".to_string(), "asList".to_string(), "([Ljava/lang/Object;)Ljava/util/List;".to_string(), true, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let arr = frame.pop()?;
            let mut result = "[".to_string();
            if let Value::ArrayRef(arr_id) = arr {
                if let Some(arr_obj) = jvm.heap.get(arr_id) {
                    if let Some(elements) = &arr_obj.array_elements {
                        for (i, elem) in elements.iter().enumerate() {
                            if i > 0 { result.push_str(", "); }
                            match elem {
                                Value::Int(v) => result.push_str(&v.to_string()),
                                Value::Long(v) => result.push_str(&v.to_string()),
                                Value::Float(v) => result.push_str(&v.to_string()),
                                Value::Double(v) => result.push_str(&v.to_string()),
                                Value::Boolean(v) => result.push_str(&v.to_string()),
                                Value::Null => result.push_str("null"),
                                Value::ObjectRef(id) => {
                                    if let Some(s) = jvm.heap.get(*id).and_then(|o| o.string_value.clone()) {
                                        result.push_str(&s);
                                    } else {
                                        result.push_str(&format!("@{}", id));
                                    }
                                }
                                _ => result.push_str("?"),
                            }
                        }
                    }
                }
            }
            result.push(']');
            let str_obj = HeapObject::new_string("java.lang.String".to_string(), result);
            let str_ref = jvm.allocate(str_obj)?;
            frame.push(Value::ObjectRef(str_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Arrays".to_string(), "toString".to_string(), "([Ljava/lang/Object;)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn sort() -> Method {
        // Simplified: no-op for now
        let native_impl: NativeImplementation = Arc::new(|_frame, _jvm| Ok(()));
        Method::new_native("java.util.Arrays".to_string(), "sort".to_string(), "([Ljava/lang/Object;)V".to_string(), true, Some(native_impl))
    }

    pub fn binarySearch() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Int(-1))?;
            Ok(())
        });
        Method::new_native("java.util.Arrays".to_string(), "binarySearch".to_string(), "([Ljava/lang/Object;Ljava/lang/Object;)I".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Arrays", Arrays::asList());
        jvm.method_area.add_native_method("java.util.Arrays", Arrays::toString());
        jvm.method_area.add_native_method("java.util.Arrays", Arrays::sort());
        jvm.method_area.add_native_method("java.util.Arrays", Arrays::binarySearch());
    }
}

// ========== java.util.Collections ==========

pub struct Collections;

impl Collections {
    pub fn singletonList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let list = HeapObject::new("java.util.Collections$SingletonList".to_string());
            let list_ref = jvm.allocate(list)?;
            let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 1);
            let arr_ref = jvm.allocate(arr)?;
            if let Some(arr_obj) = jvm.heap.get_mut(arr_ref) {
                if let Some(elements) = &mut arr_obj.array_elements {
                    if !elements.is_empty() {
                        elements[0] = elem;
                    }
                }
            }
            if let Some(obj) = jvm.heap.get_mut(list_ref) {
                obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                obj.fields.insert("size".to_string(), Value::Int(1));
            }
            frame.push(Value::ObjectRef(list_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "singletonList".to_string(), "(Ljava/lang/Object;)Ljava/util/List;".to_string(), true, Some(native_impl))
    }

    pub fn emptyList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let list = HeapObject::new("java.util.Collections$EmptyList".to_string());
            let list_ref = jvm.allocate(list)?;
            let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
            let arr_ref = jvm.allocate(arr)?;
            if let Some(obj) = jvm.heap.get_mut(list_ref) {
                obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                obj.fields.insert("size".to_string(), Value::Int(0));
            }
            frame.push(Value::ObjectRef(list_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "emptyList".to_string(), "()Ljava/util/List;".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Collections", Collections::singletonList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::emptyList());
    }
}

// ========== java.util.Iterator ==========

pub struct Iterator;

impl Iterator {
    pub fn hasNext() -> Method {
        Method::new_native("java.util.Iterator".to_string(), "hasNext".to_string(), "()Z".to_string(), false, None)
    }

    pub fn next() -> Method {
        Method::new_native("java.util.Iterator".to_string(), "next".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Iterator", Iterator::hasNext());
        jvm.method_area.add_native_method("java.util.Iterator", Iterator::next());
    }
}

// ========== java.lang.Iterable ==========

pub struct Iterable;

impl Iterable {
    pub fn iterator() -> Method {
        Method::new_native("java.lang.Iterable".to_string(), "iterator".to_string(), "()Ljava/util/Iterator;".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Iterable", Iterable::iterator());
    }
}

// ========== java.util.Objects ==========

pub struct Objects;

impl Objects {
    pub fn equals() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let b = frame.pop()?;
            let a = frame.pop()?;
            let result = match (&a, &b) {
                (Value::Null, Value::Null) => true,
                (Value::Null, _) | (_, Value::Null) => false,
                _ => a == b,
            };
            frame.push(Value::Boolean(result))?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "equals".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Z".to_string(), true, Some(native_impl))
    }

    pub fn hashCode() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let obj = frame.pop()?;
            let hash = match obj {
                Value::Null => 0,
                Value::ObjectRef(id) => id as i32,
                Value::Int(v) => v,
                _ => 0,
            };
            frame.push(Value::Int(hash))?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "hashCode".to_string(), "(Ljava/lang/Object;)I".to_string(), true, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj = frame.pop()?;
            match obj {
                Value::Null => {
                    let s = HeapObject::new_string("java.lang.String".to_string(), "null".to_string());
                    let r = jvm.allocate(s)?;
                    frame.push(Value::ObjectRef(r))?;
                }
                Value::ObjectRef(id) => {
                    if let Some(o) = jvm.heap.get(id) {
                        if let Some(s) = &o.string_value {
                            let str_obj = HeapObject::new_string("java.lang.String".to_string(), s.clone());
                            let r = jvm.allocate(str_obj)?;
                            frame.push(Value::ObjectRef(r))?;
                        } else {
                            let s = format!("{}@{}", o.class_name, id);
                            let str_obj = HeapObject::new_string("java.lang.String".to_string(), s);
                            let r = jvm.allocate(str_obj)?;
                            frame.push(Value::ObjectRef(r))?;
                        }
                    }
                }
                _ => {
                    let s = format!("{}", obj);
                    let str_obj = HeapObject::new_string("java.lang.String".to_string(), s);
                    let r = jvm.allocate(str_obj)?;
                    frame.push(Value::ObjectRef(r))?;
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "toString".to_string(), "(Ljava/lang/Object;)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn requireNonNull() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let obj = frame.pop()?;
            if obj.is_null() {
                return Err(JvmError::RuntimeError(RuntimeError::NullPointerException));
            }
            frame.push(obj)?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "requireNonNull".to_string(), "(Ljava/lang/Object;)Ljava/lang/Object;".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Objects", Objects::equals());
        jvm.method_area.add_native_method("java.util.Objects", Objects::hashCode());
        jvm.method_area.add_native_method("java.util.Objects", Objects::toString());
        jvm.method_area.add_native_method("java.util.Objects", Objects::requireNonNull());
    }
}

// ========== java.util.Optional ==========

pub struct Optional;

impl Optional {
    pub fn empty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let opt = HeapObject::new("java.util.Optional".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), Value::Null);
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Optional".to_string(), "empty".to_string(), "()Ljava/util/Optional;".to_string(), true, Some(native_impl))
    }

    pub fn of() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let value = frame.pop()?;
            let opt = HeapObject::new("java.util.Optional".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), value);
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Optional".to_string(), "of".to_string(), "(Ljava/lang/Object;)Ljava/util/Optional;".to_string(), true, Some(native_impl))
    }

    pub fn isPresent() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let present = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    !obj.fields.get("value").map(|v| v.is_null()).unwrap_or(true)
                } else { false }
            } else { false };
            frame.push(Value::Boolean(present))?;
            Ok(())
        });
        Method::new_native("java.util.Optional".to_string(), "isPresent".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn get() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(val) = obj.fields.get("value") {
                        frame.push(val.clone())?;
                        return Ok(());
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Optional".to_string(), "get".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn orElse() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(val) = obj.fields.get("value") {
                        if !val.is_null() {
                            frame.push(val.clone())?;
                            return Ok(());
                        }
                    }
                }
            }
            frame.push(other)?;
            Ok(())
        });
        Method::new_native("java.util.Optional".to_string(), "orElse".to_string(), "(Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Optional", Optional::empty());
        jvm.method_area.add_native_method("java.util.Optional", Optional::of());
        jvm.method_area.add_native_method("java.util.Optional", Optional::isPresent());
        jvm.method_area.add_native_method("java.util.Optional", Optional::get());
        jvm.method_area.add_native_method("java.util.Optional", Optional::orElse());
    }
}

// ========== java.util.LinkedList ==========

pub struct LinkedList;

impl LinkedList {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
                let arr_ref = jvm.allocate(arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.LinkedList".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn add() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
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
                let new_size = current_size + 1;
                if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                    if let Some(elements) = &mut arr.array_elements {
                        if current_size >= elements.len() {
                            let mut new_elems = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                            for (i, e) in elements.iter().enumerate() {
                                new_elems[i] = e.clone();
                            }
                            *elements = new_elems;
                        }
                        if current_size < elements.len() {
                            elements[current_size] = elem;
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                }
            }
            frame.push(Value::Boolean(true))?;
            Ok(())
        });
        Method::new_native("java.util.LinkedList".to_string(), "add".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn addFirst() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
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
                let new_size = current_size + 1;
                if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                    if let Some(elements) = &mut arr.array_elements {
                        if current_size >= elements.len() {
                            let mut new_elems = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                            for (i, e) in elements.iter().enumerate() {
                                new_elems[i] = e.clone();
                            }
                            *elements = new_elems;
                        }
                        if current_size < elements.len() {
                            // Shift elements right
                            for i in (0..current_size).rev() {
                                elements[i + 1] = elements[i].clone();
                            }
                            elements[0] = elem;
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.LinkedList".to_string(), "addFirst".to_string(), "(Ljava/lang/Object;)V".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.LinkedList".to_string(), "get".to_string(), "(I)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn getFirst() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let arr_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            if !elements.is_empty() {
                                frame.push(elements[0].clone())?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.LinkedList".to_string(), "getFirst".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn getLast() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let arr_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            if !elements.is_empty() {
                                frame.push(elements[elements.len() - 1].clone())?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.LinkedList".to_string(), "getLast".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.LinkedList".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.LinkedList".to_string(), "isEmpty".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn remove() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let index = frame.pop()?.as_int() as usize;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (arr_ref, current_size) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let a_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    (a_ref, size)
                };
                let mut removed = Value::Null;
                if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                    if let Some(elements) = &mut arr.array_elements {
                        if index < current_size {
                            removed = elements[index].clone();
                            for i in index..current_size - 1 {
                                elements[i] = elements[i + 1].clone();
                            }
                            if current_size > 0 {
                                elements[current_size - 1] = Value::Null;
                            }
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int((current_size - 1) as i32));
                }
                frame.push(removed)?;
            }
            Ok(())
        });
        Method::new_native("java.util.LinkedList".to_string(), "remove".to_string(), "(I)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::init());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::add());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::addFirst());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::get());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::getFirst());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::getLast());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::size());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::isEmpty());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::remove());
    }
}

// ========== java.util.TreeMap ==========

pub struct TreeMap;

impl TreeMap {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
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
        Method::new_native("java.util.TreeMap".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn put() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let value = frame.pop()?;
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (mut current_size, keys_ref, vals_ref) = {
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
                // Check if key exists (sorted insertion)
                let mut found_index = None;
                let mut insert_index = current_size;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(keys) = &keys_arr.array_elements {
                        for (i, k) in keys.iter().enumerate() {
                            if i >= current_size { break; }
                            if *k == key {
                                found_index = Some(i);
                                break;
                            }
                        }
                    }
                }
                if let Some(idx) = found_index {
                    // Update existing
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if idx < vals.len() {
                                vals[idx] = value;
                            }
                        }
                    }
                } else {
                    // Insert new
                    let new_size = current_size + 1;
                    if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                        if let Some(keys) = &mut keys_arr.array_elements {
                            if current_size >= keys.len() {
                                let mut new_keys = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, k) in keys.iter().enumerate() {
                                    new_keys[i] = k.clone();
                                }
                                *keys = new_keys;
                            }
                            if current_size < keys.len() {
                                keys[current_size] = key;
                            }
                        }
                    }
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if current_size >= vals.len() {
                                let mut new_vals = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, v) in vals.iter().enumerate() {
                                    new_vals[i] = v.clone();
                                }
                                *vals = new_vals;
                            }
                            if current_size < vals.len() {
                                vals[current_size] = value;
                            }
                        }
                    }
                    current_size = new_size;
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(current_size as i32));
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.TreeMap".to_string(), "put".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
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
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                        if let Some(keys) = &keys_arr.array_elements {
                            for i in 0..size {
                                if i < keys.len() && keys[i] == key {
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
        Method::new_native("java.util.TreeMap".to_string(), "get".to_string(), "(Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.TreeMap".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.TreeMap", TreeMap::init());
        jvm.method_area.add_native_method("java.util.TreeMap", TreeMap::put());
        jvm.method_area.add_native_method("java.util.TreeMap", TreeMap::get());
        jvm.method_area.add_native_method("java.util.TreeMap", TreeMap::size());
    }
}

// ========== java.util.LinkedHashSet ==========

pub struct LinkedHashSet;

impl LinkedHashSet {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let keys_arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
                let keys_ref = jvm.allocate(keys_arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("keys".to_string(), Value::ArrayRef(keys_ref));
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.LinkedHashSet".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn add() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (current_size, keys_ref, found) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let mut f = false;
                    if let Some(keys_arr) = jvm.heap.get(k_ref) {
                        if let Some(keys) = &keys_arr.array_elements {
                            f = keys.iter().any(|k| *k == elem);
                        }
                    }
                    (size, k_ref, f)
                };
                if !found {
                    let new_size = current_size + 1;
                    if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                        if let Some(keys) = &mut keys_arr.array_elements {
                            if current_size >= keys.len() {
                                let mut new_keys = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, k) in keys.iter().enumerate() {
                                    new_keys[i] = k.clone();
                                }
                                *keys = new_keys;
                            }
                            if current_size < keys.len() {
                                keys[current_size] = elem;
                            }
                        }
                    }
                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                        obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                    }
                }
            }
            frame.push(Value::Boolean(true))?;
            Ok(())
        });
        Method::new_native("java.util.LinkedHashSet".to_string(), "add".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.LinkedHashSet".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.LinkedHashSet", LinkedHashSet::init());
        jvm.method_area.add_native_method("java.util.LinkedHashSet", LinkedHashSet::add());
        jvm.method_area.add_native_method("java.util.LinkedHashSet", LinkedHashSet::size());
    }
}

// ========== java.util.PriorityQueue ==========

pub struct PriorityQueue;

impl PriorityQueue {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
                let arr_ref = jvm.allocate(arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("queue".to_string(), Value::ArrayRef(arr_ref));
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.PriorityQueue".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn add() -> Method {
        // Simplified: just append to the array (no heap ordering)
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (current_size, queue_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let q_ref = obj.fields.get("queue")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (size, q_ref)
                };
                let new_size = current_size + 1;
                if let Some(queue) = jvm.heap.get_mut(queue_ref) {
                    if let Some(elements) = &mut queue.array_elements {
                        if current_size >= elements.len() {
                            let mut new_elems = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                            for (i, e) in elements.iter().enumerate() {
                                new_elems[i] = e.clone();
                            }
                            *elements = new_elems;
                        }
                        if current_size < elements.len() {
                            elements[current_size] = elem;
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                }
            }
            frame.push(Value::Boolean(true))?;
            Ok(())
        });
        Method::new_native("java.util.PriorityQueue".to_string(), "add".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn peek() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let queue_ref = obj.fields.get("queue")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if let Some(queue) = jvm.heap.get(queue_ref) {
                        if let Some(elements) = &queue.array_elements {
                            if !elements.is_empty() {
                                frame.push(elements[0].clone())?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.PriorityQueue".to_string(), "peek".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn poll() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (queue_ref, current_size) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let q_ref = obj.fields.get("queue")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    (q_ref, size)
                };
                if current_size > 0 {
                    let result = if let Some(queue) = jvm.heap.get(queue_ref) {
                        if let Some(elements) = &queue.array_elements {
                            elements[0].clone()
                        } else { Value::Null }
                    } else { Value::Null };
                    if let Some(queue) = jvm.heap.get_mut(queue_ref) {
                        if let Some(elements) = &mut queue.array_elements {
                            for i in 0..current_size - 1 {
                                elements[i] = elements[i + 1].clone();
                            }
                            if current_size > 0 {
                                elements[current_size - 1] = Value::Null;
                            }
                        }
                    }
                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                        obj.fields.insert("size".to_string(), Value::Int((current_size - 1) as i32));
                    }
                    frame.push(result)?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.PriorityQueue".to_string(), "poll".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.PriorityQueue".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.PriorityQueue", PriorityQueue::init());
        jvm.method_area.add_native_method("java.util.PriorityQueue", PriorityQueue::add());
        jvm.method_area.add_native_method("java.util.PriorityQueue", PriorityQueue::peek());
        jvm.method_area.add_native_method("java.util.PriorityQueue", PriorityQueue::poll());
        jvm.method_area.add_native_method("java.util.PriorityQueue", PriorityQueue::size());
    }
}

// ========== java.util.UUID ==========

pub struct UUID;

impl UUID {
    pub fn randomUUID() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
            let most_sig = (ts.as_nanos() & 0xFFFF_FFFF_FFFF_0FFF) | 0x0000_0000_0000_4000;
            let least_sig = (ts.as_nanos() & 0x3FFF_FFFF_FFFF_FFFF) | 0x8000_0000_0000_0000;
            let uuid_str = format!("{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
                (most_sig >> 32) as u32,
                (most_sig >> 16) as u16 & 0xFFFF,
                most_sig as u16 & 0xFFFF,
                (least_sig >> 48) as u16,
                least_sig & 0xFFFF_FFFF_FFFF);
            let obj = HeapObject::new_string("java.lang.String".to_string(), uuid_str);
            let ref_id = jvm.allocate(obj)?;
            if let Value::ObjectRef(this_id) = frame.get_local(0)? {
                if let Some(uuid_obj) = jvm.heap.get_mut(*this_id) {
                    uuid_obj.fields.insert("value".to_string(), Value::ObjectRef(ref_id));
                }
            }
            frame.push(Value::ObjectRef(ref_id))?;
            Ok(())
        });
        Method::new_native("java.util.UUID".to_string(), "randomUUID".to_string(), "()Ljava/util/UUID;".to_string(), true, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("value") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(s) = &str_obj.string_value {
                                let result = HeapObject::new_string("java.lang.String".to_string(), s.clone());
                                let r = jvm.allocate(result)?;
                                frame.push(Value::ObjectRef(r))?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.UUID".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.UUID", UUID::randomUUID());
        jvm.method_area.add_native_method("java.util.UUID", UUID::toString());
    }
}

/// Register all java.util classes with the JVM.
pub fn register_util_classes(jvm: &mut JVM) {
    ArrayList::register(jvm);
    LinkedList::register(jvm);
    HashMap::register(jvm);
    TreeMap::register(jvm);
    HashSet::register(jvm);
    LinkedHashSet::register(jvm);
    PriorityQueue::register(jvm);
    UUID::register(jvm);
    Arrays::register(jvm);
    Collections::register(jvm);
    Iterator::register(jvm);
    Iterable::register(jvm);
    Objects::register(jvm);
    Optional::register(jvm);
}