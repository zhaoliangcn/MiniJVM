use std::sync::Arc;
use crate::error::{JvmError, RuntimeError, Result};
use crate::runtime::{JVM, Frame, Value, HeapObject, method_area::{Method, NativeImplementation}};

/// Compare two Values for equality, with special handling for String objects.
/// In Java, HashMap uses equals() for key comparison, so String keys should be
/// compared by their content rather than by object reference identity.
fn values_equal(jvm: &JVM, a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::ObjectRef(id_a), Value::ObjectRef(id_b)) => {
            if id_a == id_b { return true; }
            // Check if both are String objects — compare by value
            let str_a = jvm.heap.get(*id_a).and_then(|o| o.string_value.clone());
            let str_b = jvm.heap.get(*id_b).and_then(|o| o.string_value.clone());
            match (str_a, str_b) {
                (Some(sa), Some(sb)) => sa == sb,
                _ => false,
            }
        }
        _ => a == b,
    }
}

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
                if index < current_size {
                    let removed = if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            if index < elements.len() {
                                elements[index].clone()
                            } else { Value::Null }
                        } else { Value::Null }
                    } else { Value::Null };
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            for i in index..current_size - 1 {
                                elements[i] = elements[i + 1].clone();
                            }
                            if current_size > 0 { elements[current_size - 1] = Value::Null; }
                        }
                    }
                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                        obj.fields.insert("size".to_string(), Value::Int((current_size - 1) as i32));
                    }
                    frame.push(removed)?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "remove".to_string(), "(I)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn indexOf() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let mut idx = -1;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let arr_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            for i in 0..size {
                                if i < elements.len() && elements[i] == elem {
                                    idx = i as i32;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(idx))?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "indexOf".to_string(), "(Ljava/lang/Object;)I".to_string(), false, Some(native_impl))
    }

    pub fn contains() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let mut found = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let arr_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            for i in 0..size {
                                if i < elements.len() && elements[i] == elem {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(found))?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "contains".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::init());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::init_capacity());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::size());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::add());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::addAll());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::get());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::isEmpty());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::remove());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::indexOf());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::contains());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::clear());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::set());
        jvm.method_area.add_native_method("java.util.ArrayList", ArrayList::forEach());
    }
}

impl ArrayList {
    pub fn clear() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract arr_ref first, then drop the borrow
                let arr_ref = if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0)
                } else { 0 };
                // Set size to 0
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
                // Clear the array
                if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                    if let Some(elements) = &mut arr.array_elements {
                        for e in elements.iter_mut() { *e = Value::Null; }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "clear".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn set() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
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
                if index < current_size {
                    let old = if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            if index < elements.len() { elements[index].clone() } else { Value::Null }
                        } else { Value::Null }
                    } else { Value::Null };
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            if index < elements.len() { elements[index] = elem; }
                        }
                    }
                    frame.push(old)?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "set".to_string(), "(ILjava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn forEach() -> Method {
        // ArrayList.forEach - simplified: iterate without Consumer
        let native_impl: NativeImplementation = Arc::new(|_frame, _jvm| Ok(()));
        Method::new_native("java.util.ArrayList".to_string(), "forEach".to_string(), "(Ljava/util/function/Consumer;)V".to_string(), false, Some(native_impl))
    }
}

impl ArrayList {
    pub fn addAll() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let coll_ref = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let mut elems = Vec::new();
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract elements from the collection
                if let Value::ObjectRef(coll_id) = coll_ref {
                    if let Some(coll_obj) = jvm.heap.get(coll_id) {
                        let arr_ref = coll_obj.fields.get("elementData")
                            .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                            .unwrap_or(0);
                        let size = coll_obj.fields.get("size")
                            .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                            .unwrap_or(0);
                        if let Some(arr) = jvm.heap.get(arr_ref) {
                            if let Some(elements) = &arr.array_elements {
                                for i in 0..size.min(elements.len()) {
                                    elems.push(elements[i].clone());
                                }
                            }
                        }
                    }
                }
                // Add all elements to this ArrayList
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
                let new_size = current_size + elems.len();
                if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                    if let Some(elements) = &mut arr.array_elements {
                        if new_size > elements.len() {
                            let mut new_elems = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                            for (i, e) in elements.iter().enumerate() {
                                new_elems[i] = e.clone();
                            }
                            *elements = new_elems;
                        }
                        for i in 0..elems.len() {
                            if current_size + i < elements.len() {
                                elements[current_size + i] = elems[i].clone();
                            }
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                }
            }
            frame.push(Value::Boolean(!elems.is_empty()))?;
            Ok(())
        });
        Method::new_native("java.util.ArrayList".to_string(), "addAll".to_string(), "(Ljava/util/Collection;)Z".to_string(), false, Some(native_impl))
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
                            if values_equal(&*jvm, e, &key) {
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
                                if values_equal(&*jvm, e, &key) {
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
                            found = elements.iter().any(|e| values_equal(&*jvm, e, &key));
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

    pub fn keySet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let keys_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    // Copy key data first
                    let mut key_data = Vec::new();
                    if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                        if let Some(keys) = &keys_arr.array_elements {
                            for i in 0..size.min(keys.len()) {
                                key_data.push(keys[i].clone());
                            }
                        }
                    }
                    // Create a new HashSet with the keys
                    let set = HeapObject::new("java.util.HashSet".to_string());
                    let set_ref = jvm.allocate(set)?;
                    let set_keys = HeapObject::new_array("[Ljava/lang/Object;".to_string(), key_data.len());
                    let set_keys_ref = jvm.allocate(set_keys)?;
                    if let Some(set_arr) = jvm.heap.get_mut(set_keys_ref) {
                        if let Some(set_elems) = &mut set_arr.array_elements {
                            for i in 0..key_data.len().min(set_elems.len()) {
                                set_elems[i] = key_data[i].clone();
                            }
                        }
                    }
                    if let Some(set_obj) = jvm.heap.get_mut(set_ref) {
                        set_obj.fields.insert("keys".to_string(), Value::ArrayRef(set_keys_ref));
                        set_obj.fields.insert("size".to_string(), Value::Int(size as i32));
                    }
                    frame.push(Value::ObjectRef(set_ref))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "keySet".to_string(), "()Ljava/util/Set;".to_string(), false, Some(native_impl))
    }

    pub fn clear() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract refs first, then drop the borrow
                let (keys_ref, vals_ref) = if let Some(obj) = jvm.heap.get(*this_id) {
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (k_ref, v_ref)
                } else { (0, 0) };
                // Clear the size
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
                // Clear the keys and values arrays
                if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                    if let Some(elements) = &mut keys_arr.array_elements {
                        for e in elements.iter_mut() {
                            *e = Value::Null;
                        }
                    }
                }
                if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                    if let Some(elements) = &mut vals_arr.array_elements {
                        for e in elements.iter_mut() {
                            *e = Value::Null;
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "clear".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::init());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::init_capacity());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::size());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::put());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::get());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::containsKey());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::isEmpty());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::keySet());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::values());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::entrySet());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::clear());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::computeIfAbsent());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::merge());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::forEach());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::getOrDefault());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::putIfAbsent());
        jvm.method_area.add_native_method("java.util.HashMap", HashMap::replace());
    }
}

impl HashMap {
    pub fn computeIfAbsent() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let _mapping_fn = frame.pop()?;
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (mut size, keys_ref, vals_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let sz = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (sz, k_ref, v_ref)
                };
                // Check if key exists
                let mut found = false;
                let mut found_idx = 0;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(keys) = &keys_arr.array_elements {
                        for (i, k) in keys.iter().enumerate() {
                            if i >= size { break; }
                            if values_equal(&*jvm, k, &key) { found = true; found_idx = i; break; }
                        }
                    }
                }
                if found {
                    if let Some(vals_arr) = jvm.heap.get(vals_ref) {
                        if let Some(vals) = &vals_arr.array_elements {
                            if found_idx < vals.len() { frame.push(vals[found_idx].clone())?; return Ok(()); }
                        }
                    }
                }
                // Key not found, return null (simplified - no mapping function)
                frame.push(Value::Null)?;
                return Ok(());
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "computeIfAbsent".to_string(), "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn merge() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let _remapping_fn = frame.pop()?;
            let value = frame.pop()?;
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (mut size, keys_ref, vals_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let sz = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (sz, k_ref, v_ref)
                };
                let mut found = false;
                let mut found_idx = 0;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(keys) = &keys_arr.array_elements {
                        for (i, k) in keys.iter().enumerate() {
                            if i >= size { break; }
                            if values_equal(&*jvm, k, &key) { found = true; found_idx = i; break; }
                        }
                    }
                }
                if found {
                    // Update existing
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if found_idx < vals.len() {
                                vals[found_idx] = value.clone();
                            }
                        }
                    }
                } else {
                    // Add new entry
                    let new_size = size + 1;
                    if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                        if let Some(keys) = &mut keys_arr.array_elements {
                            if size >= keys.len() {
                                let mut new_keys = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, k) in keys.iter().enumerate() { new_keys[i] = k.clone(); }
                                *keys = new_keys;
                            }
                            if size < keys.len() { keys[size] = key; }
                        }
                    }
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if size >= vals.len() {
                                let mut new_vals = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, v) in vals.iter().enumerate() { new_vals[i] = v.clone(); }
                                *vals = new_vals;
                            }
                            if size < vals.len() { vals[size] = value.clone(); }
                        }
                    }
                    size = new_size;
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(size as i32));
                }
                frame.push(value)?;
                return Ok(());
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "merge".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn forEach() -> Method {
        // Simplified: no-op for HashMap forEach (requires BiConsumer)
        let native_impl: NativeImplementation = Arc::new(|_frame, _jvm| Ok(()));
        Method::new_native("java.util.HashMap".to_string(), "forEach".to_string(), "(Ljava/util/function/BiConsumer;)V".to_string(), false, Some(native_impl))
    }

    pub fn getOrDefault() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let default_val = frame.pop()?;
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
                                if i < keys.len() && values_equal(&*jvm, &keys[i], &key) {
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
            frame.push(default_val)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "getOrDefault".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn putIfAbsent() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let value = frame.pop()?;
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (mut size, keys_ref, vals_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let sz = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (sz, k_ref, v_ref)
                };
                // Check if key exists
                let mut found = false;
                let mut found_idx = 0;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(keys) = &keys_arr.array_elements {
                        for (i, k) in keys.iter().enumerate() {
                            if i >= size { break; }
                            if values_equal(&*jvm, k, &key) { found = true; found_idx = i; break; }
                        }
                    }
                }
                if found {
                    // Key exists, return existing value
                    if let Some(vals_arr) = jvm.heap.get(vals_ref) {
                        if let Some(vals) = &vals_arr.array_elements {
                            if found_idx < vals.len() {
                                frame.push(vals[found_idx].clone())?;
                                return Ok(());
                            }
                        }
                    }
                } else {
                    // Key not found, insert
                    let new_size = size + 1;
                    if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                        if let Some(keys) = &mut keys_arr.array_elements {
                            if size >= keys.len() {
                                let mut new_keys = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, k) in keys.iter().enumerate() { new_keys[i] = k.clone(); }
                                *keys = new_keys;
                            }
                            if size < keys.len() { keys[size] = key; }
                        }
                    }
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if size >= vals.len() {
                                let mut new_vals = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, v) in vals.iter().enumerate() { new_vals[i] = v.clone(); }
                                *vals = new_vals;
                            }
                            if size < vals.len() { vals[size] = value; }
                        }
                    }
                    size = new_size;
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(size as i32));
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "putIfAbsent".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn replace() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let new_value = frame.pop()?;
            let key = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract all data first, then update
                let (keys_ref, vals_ref, size) = if let Some(obj) = jvm.heap.get(*this_id) {
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let sz = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    (k_ref, v_ref, sz)
                } else { (0, 0, 0) };
                // Find matching index
                let mut found_idx = None;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(keys) = &keys_arr.array_elements {
                        for i in 0..size {
                            if i < keys.len() && values_equal(&*jvm, &keys[i], &key) {
                                found_idx = Some(i);
                                break;
                            }
                        }
                    }
                }
                // Update the value if found
                if let Some(idx) = found_idx {
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if idx < vals.len() {
                                let old = vals[idx].clone();
                                vals[idx] = new_value;
                                frame.push(old)?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "replace".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }
}

impl HashMap {
    pub fn values() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let vals_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    // Copy value data
                    let mut val_data = Vec::new();
                    if let Some(vals_arr) = jvm.heap.get(vals_ref) {
                        if let Some(vals) = &vals_arr.array_elements {
                            for i in 0..size.min(vals.len()) {
                                val_data.push(vals[i].clone());
                            }
                        }
                    }
                    // Create a new ArrayList with the values
                    let list = HeapObject::new("java.util.ArrayList".to_string());
                    let list_ref = jvm.allocate(list)?;
                    let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), val_data.len());
                    let arr_ref = jvm.allocate(arr)?;
                    if let Some(arr_obj) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elems) = &mut arr_obj.array_elements {
                            for i in 0..val_data.len().min(elems.len()) {
                                elems[i] = val_data[i].clone();
                            }
                        }
                    }
                    if let Some(list_obj) = jvm.heap.get_mut(list_ref) {
                        list_obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                        list_obj.fields.insert("size".to_string(), Value::Int(size as i32));
                    }
                    frame.push(Value::ObjectRef(list_ref))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "values".to_string(), "()Ljava/util/Collection;".to_string(), false, Some(native_impl))
    }

    pub fn entrySet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract key/value data first
                let (keys_data, vals_data) = if let Some(obj) = jvm.heap.get(*this_id) {
                    let keys_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let vals_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let mut kd = Vec::new();
                    let mut vd = Vec::new();
                    if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                        if let Some(keys) = &keys_arr.array_elements {
                            for i in 0..size.min(keys.len()) { kd.push(keys[i].clone()); }
                        }
                    }
                    if let Some(vals_arr) = jvm.heap.get(vals_ref) {
                        if let Some(vals) = &vals_arr.array_elements {
                            for i in 0..size.min(vals.len()) { vd.push(vals[i].clone()); }
                        }
                    }
                    (kd, vd)
                } else { (Vec::new(), Vec::new()) };
                
                // Create entry objects and the set
                let mut entry_refs = Vec::new();
                for i in 0..keys_data.len().min(vals_data.len()) {
                    let entry = HeapObject::new("java.util.AbstractMap$SimpleEntry".to_string());
                    let entry_ref = jvm.allocate(entry)?;
                    if let Some(entry_obj) = jvm.heap.get_mut(entry_ref) {
                        entry_obj.fields.insert("key".to_string(), keys_data[i].clone());
                        entry_obj.fields.insert("value".to_string(), vals_data[i].clone());
                    }
                    entry_refs.push(Value::ObjectRef(entry_ref));
                }
                
                // Create a HashSet with the entries
                let set = HeapObject::new("java.util.HashSet".to_string());
                let set_ref = jvm.allocate(set)?;
                let set_keys = HeapObject::new_array("[Ljava/lang/Object;".to_string(), entry_refs.len());
                let set_keys_ref = jvm.allocate(set_keys)?;
                if let Some(set_arr) = jvm.heap.get_mut(set_keys_ref) {
                    if let Some(set_elems) = &mut set_arr.array_elements {
                        for i in 0..entry_refs.len().min(set_elems.len()) {
                            set_elems[i] = entry_refs[i].clone();
                        }
                    }
                }
                if let Some(set_obj) = jvm.heap.get_mut(set_ref) {
                    set_obj.fields.insert("keys".to_string(), Value::ArrayRef(set_keys_ref));
                    set_obj.fields.insert("size".to_string(), Value::Int(entry_refs.len() as i32));
                }
                frame.push(Value::ObjectRef(set_ref))?;
                return Ok(());
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.HashMap".to_string(), "entrySet".to_string(), "()Ljava/util/Set;".to_string(), false, Some(native_impl))
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
        jvm.method_area.add_native_method("java.util.Arrays", Arrays::copyOf());
        jvm.method_area.add_native_method("java.util.Arrays", Arrays::fill());
    }
}

impl Arrays {
    pub fn copyOf() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let new_len = frame.pop()?.as_int().max(0) as usize;
            let original = frame.pop()?;
            let result = if let Value::ArrayRef(arr_id) = original {
                if let Some(arr_obj) = jvm.heap.get(arr_id) {
                    let mut new_elems = vec![Value::Null; new_len.max(1)];
                    if let Some(elements) = &arr_obj.array_elements {
                        for i in 0..new_len.min(elements.len()).min(new_elems.len()) {
                            new_elems[i] = elements[i].clone();
                        }
                    }
                    let class_name = arr_obj.class_name.clone();
                    let result = HeapObject {
                        class_name,
                        fields: std::collections::HashMap::new(),
                        string_value: None,
                        array_elements: Some(new_elems),
                        array_length: new_len,
                        monitor_owner: None,
                        monitor_count: 0,
                        generation: 0,
                        age: 0,
                    };
                    let result_ref = jvm.allocate(result)?;
                    Value::ArrayRef(result_ref)
                } else { Value::Null }
            } else { Value::Null };
            frame.push(result)?;
            Ok(())
        });
        Method::new_native("java.util.Arrays".to_string(), "copyOf".to_string(), "([JI)[J".to_string(), true, Some(native_impl))
    }

    pub fn fill() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?;
            let arr_ref = frame.pop()?;
            if let Value::ArrayRef(arr_id) = arr_ref {
                if let Some(arr_obj) = jvm.heap.get_mut(arr_id) {
                    if let Some(elements) = &mut arr_obj.array_elements {
                        for e in elements.iter_mut() {
                            *e = val.clone();
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Arrays".to_string(), "fill".to_string(), "([IJ)V".to_string(), true, Some(native_impl))
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

    pub fn sort() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let comp = frame.pop()?; // Comparator (optional)
            let list_ref = frame.pop()?; // List
            if let Value::ObjectRef(list_id) = list_ref {
                // Get the backing array from the list
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if arr_ref == 0 { return Ok(()); }
                    
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    
                    // Simple bubble sort on the backing array
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            for i in 0..size {
                                for j in 0..size - i - 1 {
                                    // Compare elements using the comparator if available
                                    let should_swap = if !comp.is_null() {
                                        // Call comparator.compare(a, b) - simplified
                                        // For now, just compare by hash code
                                        let a = &elements[j];
                                        let b = &elements[j + 1];
                                        hash_for_sort(a) > hash_for_sort(b)
                                    } else {
                                        // Natural ordering: compare by hash code
                                        let a = &elements[j];
                                        let b = &elements[j + 1];
                                        hash_for_sort(a) > hash_for_sort(b)
                                    };
                                    if should_swap {
                                        elements.swap(j, j + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "sort".to_string(), "(Ljava/util/List;)V".to_string(), true, Some(native_impl))
    }

    pub fn sort_with_comparator() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let comp = frame.pop()?;
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if arr_ref == 0 { return Ok(()); }
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            for i in 0..size {
                                for j in 0..size - i - 1 {
                                    let a = &elements[j];
                                    let b = &elements[j + 1];
                                    if hash_for_sort(a) > hash_for_sort(b) {
                                        elements.swap(j, j + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "sort".to_string(), "(Ljava/util/List;Ljava/util/Comparator;)V".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Collections", Collections::singletonList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::emptyList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::sort());
        jvm.method_area.add_native_method("java.util.Collections", Collections::sort_with_comparator());
        jvm.method_area.add_native_method("java.util.Collections", Collections::reverse());
        jvm.method_area.add_native_method("java.util.Collections", Collections::shuffle());
        jvm.method_area.add_native_method("java.util.Collections", Collections::fill());
        jvm.method_area.add_native_method("java.util.Collections", Collections::frequency());
        jvm.method_area.add_native_method("java.util.Collections", Collections::max());
        jvm.method_area.add_native_method("java.util.Collections", Collections::min());
        jvm.method_area.add_native_method("java.util.Collections", Collections::disjoint());
        jvm.method_area.add_native_method("java.util.Collections", Collections::replaceAll());
        jvm.method_area.add_native_method("java.util.Collections", Collections::nCopies());
        jvm.method_area.add_native_method("java.util.Collections", Collections::singleton());
        jvm.method_area.add_native_method("java.util.Collections", Collections::singletonMap());
        jvm.method_area.add_native_method("java.util.Collections", Collections::unmodifiableList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::unmodifiableMap());
        jvm.method_area.add_native_method("java.util.Collections", Collections::unmodifiableSet());
        jvm.method_area.add_native_method("java.util.Collections", Collections::checkedList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::checkedMap());
        jvm.method_area.add_native_method("java.util.Collections", Collections::checkedSet());
        jvm.method_area.add_native_method("java.util.Collections", Collections::synchronizedList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::synchronizedMap());
        jvm.method_area.add_native_method("java.util.Collections", Collections::synchronizedSet());
        jvm.method_area.add_native_method("java.util.Collections", Collections::newSetFromMap());
        jvm.method_area.add_native_method("java.util.Collections", Collections::reverseOrder());
        jvm.method_area.add_native_method("java.util.Collections", Collections::list());
        jvm.method_area.add_native_method("java.util.Collections", Collections::enumeration());
        jvm.method_area.add_native_method("java.util.Collections", Collections::emptyIterator());
        jvm.method_area.add_native_method("java.util.Collections", Collections::emptyListIterator());
        jvm.method_area.add_native_method("java.util.Collections", Collections::emptyEnumeration());
        jvm.method_area.add_native_method("java.util.Collections", Collections::addAll());
        jvm.method_area.add_native_method("java.util.Collections", Collections::indexOfSubList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::lastIndexOfSubList());
        jvm.method_area.add_native_method("java.util.Collections", Collections::rotate());
        jvm.method_area.add_native_method("java.util.Collections", Collections::swap());
        jvm.method_area.add_native_method("java.util.Collections", Collections::asLifoQueue());
        jvm.method_area.add_native_method("java.util.Collections", Collections::copy());
    }
}

// ========== More Collections methods ==========

impl Collections {
    pub fn disjoint() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let b = frame.pop()?;
            let a = frame.pop()?;
            let mut disjoint = true;
            // Extract elements from first collection
            let mut elems_a = Vec::new();
            if let Value::ObjectRef(coll_id) = a {
                if let Some(coll_obj) = jvm.heap.get(coll_id) {
                    let arr_ref = coll_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = coll_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            for i in 0..size.min(elements.len()) {
                                elems_a.push(elements[i].clone());
                            }
                        }
                    }
                }
            }
            // Check against second collection
            if let Value::ObjectRef(coll_id) = b {
                if let Some(coll_obj) = jvm.heap.get(coll_id) {
                    let arr_ref = coll_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = coll_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            for i in 0..size.min(elements.len()) {
                                if elems_a.contains(&elements[i]) {
                                    disjoint = false;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(disjoint))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "disjoint".to_string(), "(Ljava/util/Collection;Ljava/util/Collection;)Z".to_string(), true, Some(native_impl))
    }

    pub fn replaceAll() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let new_val = frame.pop()?;
            let old_val = frame.pop()?;
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            for i in 0..size.min(elements.len()) {
                                if elements[i] == old_val {
                                    elements[i] = new_val.clone();
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "replaceAll".to_string(), "(Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z".to_string(), true, Some(native_impl))
    }
}

// ========== More Collections methods ==========

impl Collections {
    pub fn reverse() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            for i in 0..size / 2 {
                                elements.swap(i, size - 1 - i);
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "reverse".to_string(), "(Ljava/util/List;)V".to_string(), true, Some(native_impl))
    }

    pub fn shuffle() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            use std::time::{SystemTime, UNIX_EPOCH};
                            for i in (1..size).rev() {
                                let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
                                let j = ((ts.as_nanos() + i as u128) % (i + 1) as u128) as usize;
                                elements.swap(i, j);
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "shuffle".to_string(), "(Ljava/util/List;)V".to_string(), true, Some(native_impl))
    }

    pub fn fill() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj = frame.pop()?;
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            for i in 0..size {
                                if i < elements.len() {
                                    elements[i] = obj.clone();
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "fill".to_string(), "(Ljava/util/List;Ljava/lang/Object;)V".to_string(), true, Some(native_impl))
    }

    pub fn frequency() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj = frame.pop()?;
            let coll_ref = frame.pop()?;
            let mut count = 0;
            if let Value::ObjectRef(coll_id) = coll_ref {
                if let Some(coll_obj) = jvm.heap.get(coll_id) {
                    let arr_ref = coll_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = coll_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            for i in 0..size {
                                if i < elements.len() && elements[i] == obj {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(count))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "frequency".to_string(), "(Ljava/util/Collection;Ljava/lang/Object;)I".to_string(), true, Some(native_impl))
    }

    pub fn max() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let coll_ref = frame.pop()?;
            if let Value::ObjectRef(coll_id) = coll_ref {
                if let Some(coll_obj) = jvm.heap.get(coll_id) {
                    let arr_ref = coll_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = coll_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if size > 0 {
                        if let Some(arr) = jvm.heap.get(arr_ref) {
                            if let Some(elements) = &arr.array_elements {
                                let mut max_idx = 0;
                                for i in 1..size {
                                    if i < elements.len() && hash_for_sort(&elements[i]) > hash_for_sort(&elements[max_idx]) {
                                        max_idx = i;
                                    }
                                }
                                if max_idx < elements.len() {
                                    frame.push(elements[max_idx].clone())?;
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "max".to_string(), "(Ljava/util/Collection;)Ljava/lang/Object;".to_string(), true, Some(native_impl))
    }

    pub fn min() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let coll_ref = frame.pop()?;
            if let Value::ObjectRef(coll_id) = coll_ref {
                if let Some(coll_obj) = jvm.heap.get(coll_id) {
                    let arr_ref = coll_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = coll_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if size > 0 {
                        if let Some(arr) = jvm.heap.get(arr_ref) {
                            if let Some(elements) = &arr.array_elements {
                                let mut min_idx = 0;
                                for i in 1..size {
                                    if i < elements.len() && hash_for_sort(&elements[i]) < hash_for_sort(&elements[min_idx]) {
                                        min_idx = i;
                                    }
                                }
                                if min_idx < elements.len() {
                                    frame.push(elements[min_idx].clone())?;
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "min".to_string(), "(Ljava/util/Collection;)Ljava/lang/Object;".to_string(), true, Some(native_impl))
    }

    pub fn nCopies() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj = frame.pop()?;
            let n = frame.pop()?.as_int().max(0) as usize;
            let list = HeapObject::new("java.util.ArrayList".to_string());
            let list_ref = jvm.allocate(list)?;
            let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), n);
            let arr_ref = jvm.allocate(arr)?;
            if let Some(arr_obj) = jvm.heap.get_mut(arr_ref) {
                if let Some(elements) = &mut arr_obj.array_elements {
                    for i in 0..n.min(elements.len()) {
                        elements[i] = obj.clone();
                    }
                }
            }
            if let Some(list_obj) = jvm.heap.get_mut(list_ref) {
                list_obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                list_obj.fields.insert("size".to_string(), Value::Int(n as i32));
            }
            frame.push(Value::ObjectRef(list_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "nCopies".to_string(), "(ILjava/lang/Object;)Ljava/util/List;".to_string(), true, Some(native_impl))
    }

    pub fn singleton() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj = frame.pop()?;
            let set = HeapObject::new("java.util.HashSet".to_string());
            let set_ref = jvm.allocate(set)?;
            let keys = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 1);
            let keys_ref = jvm.allocate(keys)?;
            if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                if let Some(elements) = &mut keys_arr.array_elements {
                    if !elements.is_empty() { elements[0] = obj; }
                }
            }
            if let Some(set_obj) = jvm.heap.get_mut(set_ref) {
                set_obj.fields.insert("keys".to_string(), Value::ArrayRef(keys_ref));
                set_obj.fields.insert("size".to_string(), Value::Int(1));
            }
            frame.push(Value::ObjectRef(set_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "singleton".to_string(), "(Ljava/lang/Object;)Ljava/util/Set;".to_string(), true, Some(native_impl))
    }

    pub fn singletonMap() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let value = frame.pop()?;
            let key = frame.pop()?;
            let map = HeapObject::new("java.util.HashMap".to_string());
            let map_ref = jvm.allocate(map)?;
            let keys = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 1);
            let vals = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 1);
            let keys_ref = jvm.allocate(keys)?;
            let vals_ref = jvm.allocate(vals)?;
            if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                if let Some(elements) = &mut keys_arr.array_elements {
                    if !elements.is_empty() { elements[0] = key; }
                }
            }
            if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                if let Some(elements) = &mut vals_arr.array_elements {
                    if !elements.is_empty() { elements[0] = value; }
                }
            }
            if let Some(map_obj) = jvm.heap.get_mut(map_ref) {
                map_obj.fields.insert("keys".to_string(), Value::ArrayRef(keys_ref));
                map_obj.fields.insert("values".to_string(), Value::ArrayRef(vals_ref));
                map_obj.fields.insert("size".to_string(), Value::Int(1));
            }
            frame.push(Value::ObjectRef(map_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "singletonMap".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;".to_string(), true, Some(native_impl))
    }

    pub fn unmodifiableList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0);
                    // Create a wrapper list
                    let wrapper = HeapObject::new("java.util.Collections$UnmodifiableList".to_string());
                    let wrapper_ref = jvm.allocate(wrapper)?;
                    if let Some(wrapper_obj) = jvm.heap.get_mut(wrapper_ref) {
                        wrapper_obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                        wrapper_obj.fields.insert("size".to_string(), Value::Int(size));
                    }
                    frame.push(Value::ObjectRef(wrapper_ref))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "unmodifiableList".to_string(), "(Ljava/util/List;)Ljava/util/List;".to_string(), true, Some(native_impl))
    }

    pub fn unmodifiableMap() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let map_ref = frame.pop()?;
            if let Value::ObjectRef(map_id) = map_ref {
                if let Some(map_obj) = jvm.heap.get(map_id) {
                    let keys_ref = map_obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let vals_ref = map_obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = map_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0);
                    let wrapper = HeapObject::new("java.util.Collections$UnmodifiableMap".to_string());
                    let wrapper_ref = jvm.allocate(wrapper)?;
                    if let Some(wrapper_obj) = jvm.heap.get_mut(wrapper_ref) {
                        wrapper_obj.fields.insert("keys".to_string(), Value::ArrayRef(keys_ref));
                        wrapper_obj.fields.insert("values".to_string(), Value::ArrayRef(vals_ref));
                        wrapper_obj.fields.insert("size".to_string(), Value::Int(size));
                    }
                    frame.push(Value::ObjectRef(wrapper_ref))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "unmodifiableMap".to_string(), "(Ljava/util/Map;)Ljava/util/Map;".to_string(), true, Some(native_impl))
    }

    pub fn unmodifiableSet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let set_ref = frame.pop()?;
            frame.push(set_ref)?; // Return the same set (simplified)
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "unmodifiableSet".to_string(), "(Ljava/util/Set;)Ljava/util/Set;".to_string(), true, Some(native_impl))
    }

    pub fn checkedList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let _type_ref = frame.pop()?; // type token (ignored)
            let list_ref = frame.pop()?;
            // Return the same list (type checking is simplified)
            frame.push(list_ref)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "checkedList".to_string(), "(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;".to_string(), true, Some(native_impl))
    }

    pub fn checkedMap() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let _type_ref = frame.pop()?; // value type (ignored)
            let _key_type_ref = frame.pop()?; // key type (ignored)
            let map_ref = frame.pop()?;
            frame.push(map_ref)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "checkedMap".to_string(), "(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;".to_string(), true, Some(native_impl))
    }

    pub fn checkedSet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let _type_ref = frame.pop()?;
            let set_ref = frame.pop()?;
            frame.push(set_ref)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "checkedSet".to_string(), "(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;".to_string(), true, Some(native_impl))
    }

    pub fn synchronizedList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let list_ref = frame.pop()?;
            // Return the same list wrapped in a SynchronizedList marker
            if let Value::ObjectRef(list_id) = &list_ref {
                if let Some(list_obj) = jvm.heap.get(*list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0);
                    let wrapper = HeapObject::new("java.util.Collections$SynchronizedList".to_string());
                    let wrapper_ref = jvm.allocate(wrapper)?;
                    if let Some(w_obj) = jvm.heap.get_mut(wrapper_ref) {
                        w_obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                        w_obj.fields.insert("size".to_string(), Value::Int(size));
                    }
                    frame.push(Value::ObjectRef(wrapper_ref))?;
                    return Ok(());
                }
            }
            frame.push(list_ref)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "synchronizedList".to_string(), "(Ljava/util/List;)Ljava/util/List;".to_string(), true, Some(native_impl))
    }

    pub fn synchronizedMap() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let map_ref = frame.pop()?;
            if let Value::ObjectRef(map_id) = &map_ref {
                if let Some(map_obj) = jvm.heap.get(*map_id) {
                    let keys_ref = map_obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let vals_ref = map_obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = map_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0);
                    let wrapper = HeapObject::new("java.util.Collections$SynchronizedMap".to_string());
                    let wrapper_ref = jvm.allocate(wrapper)?;
                    if let Some(w_obj) = jvm.heap.get_mut(wrapper_ref) {
                        w_obj.fields.insert("keys".to_string(), Value::ArrayRef(keys_ref));
                        w_obj.fields.insert("values".to_string(), Value::ArrayRef(vals_ref));
                        w_obj.fields.insert("size".to_string(), Value::Int(size));
                    }
                    frame.push(Value::ObjectRef(wrapper_ref))?;
                    return Ok(());
                }
            }
            frame.push(map_ref)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "synchronizedMap".to_string(), "(Ljava/util/Map;)Ljava/util/Map;".to_string(), true, Some(native_impl))
    }

    pub fn synchronizedSet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let set_ref = frame.pop()?;
            frame.push(set_ref)?; // Return the same set (simplified)
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "synchronizedSet".to_string(), "(Ljava/util/Set;)Ljava/util/Set;".to_string(), true, Some(native_impl))
    }

    pub fn newSetFromMap() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let map_ref = frame.pop()?;
            if let Value::ObjectRef(map_id) = &map_ref {
                if let Some(map_obj) = jvm.heap.get(*map_id) {
                    let keys_ref = map_obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = map_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0);
                    let set = HeapObject::new("java.util.HashSet".to_string());
                    let set_ref = jvm.allocate(set)?;
                    // Copy key data first
                    let mut key_data = Vec::new();
                    if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                        if let Some(keys) = &keys_arr.array_elements {
                            for i in 0..(size as usize).min(keys.len()) {
                                key_data.push(keys[i].clone());
                            }
                        }
                    }
                    let set_keys = HeapObject::new_array("[Ljava/lang/Object;".to_string(), key_data.len());
                    let set_keys_ref = jvm.allocate(set_keys)?;
                    if let Some(set_arr) = jvm.heap.get_mut(set_keys_ref) {
                        if let Some(set_elems) = &mut set_arr.array_elements {
                            for i in 0..key_data.len().min(set_elems.len()) {
                                set_elems[i] = key_data[i].clone();
                            }
                        }
                    }
                    if let Some(set_obj) = jvm.heap.get_mut(set_ref) {
                        set_obj.fields.insert("keys".to_string(), Value::ArrayRef(set_keys_ref));
                        set_obj.fields.insert("size".to_string(), Value::Int(size));
                    }
                    frame.push(Value::ObjectRef(set_ref))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "newSetFromMap".to_string(), "(Ljava/util/Map;)Ljava/util/Set;".to_string(), true, Some(native_impl))
    }

    pub fn reverseOrder() -> Method {
        // Returns a comparator that reverses natural ordering
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            // Return a new Comparator object
            let comp = HeapObject::new("java.util.Comparator".to_string());
            comp; // Placeholder - just return null
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "reverseOrder".to_string(), "()Ljava/util/Comparator;".to_string(), true, Some(native_impl))
    }

    pub fn list() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let enum_ref = frame.pop()?;
            // Simplified: create an empty ArrayList
            let list = HeapObject::new("java.util.ArrayList".to_string());
            let list_ref = jvm.allocate(list)?;
            let arr = HeapObject::new_array("[Ljava/lang/Object;".to_string(), 0);
            let arr_ref = jvm.allocate(arr)?;
            if let Some(list_obj) = jvm.heap.get_mut(list_ref) {
                list_obj.fields.insert("elementData".to_string(), Value::ArrayRef(arr_ref));
                list_obj.fields.insert("size".to_string(), Value::Int(0));
            }
            frame.push(Value::ObjectRef(list_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "list".to_string(), "(Ljava/util/Enumeration;)Ljava/util/ArrayList;".to_string(), true, Some(native_impl))
    }

    pub fn enumeration() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let coll_ref = frame.pop()?;
            // Return the collection wrapped as an Enumeration
            let enum_obj = HeapObject::new("java.util.Collections$Enumeration".to_string());
            let enum_ref = jvm.allocate(enum_obj)?;
            if let Value::ObjectRef(coll_id) = &coll_ref {
                if let Some(enum_o) = jvm.heap.get_mut(enum_ref) {
                    enum_o.fields.insert("collection".to_string(), coll_ref.clone());
                    enum_o.fields.insert("position".to_string(), Value::Int(0));
                }
            }
            frame.push(Value::ObjectRef(enum_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "enumeration".to_string(), "(Ljava/util/Collection;)Ljava/util/Enumeration;".to_string(), true, Some(native_impl))
    }

    pub fn emptyIterator() -> Method {
        Method::new_native("java.util.Collections".to_string(), "emptyIterator".to_string(), "()Ljava/util/Iterator;".to_string(), true, None)
    }

    pub fn emptyListIterator() -> Method {
        Method::new_native("java.util.Collections".to_string(), "emptyListIterator".to_string(), "()Ljava/util/ListIterator;".to_string(), true, None)
    }

    pub fn emptyEnumeration() -> Method {
        Method::new_native("java.util.Collections".to_string(), "emptyEnumeration".to_string(), "()Ljava/util/Enumeration;".to_string(), true, None)
    }

    pub fn addAll() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let mut added = false;
            // Pop all elements from the stack (varargs) - we pop the last arg which is the array
            let elements_arr = frame.pop()?; // Object[] elements
            let coll_ref = frame.pop()?;
            let mut elements = Vec::new();
            if let Value::ArrayRef(arr_id) = elements_arr {
                if let Some(arr_obj) = jvm.heap.get(arr_id) {
                    if let Some(arr_elems) = &arr_obj.array_elements {
                        for elem in arr_elems {
                            elements.push(elem.clone());
                        }
                    }
                }
            }
            if let Value::ObjectRef(coll_id) = coll_ref {
                if let Some(coll_obj) = jvm.heap.get(coll_id) {
                    let arr_ref = coll_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let mut size = coll_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elems) = &mut arr.array_elements {
                            let new_size = size + elements.len();
                            if new_size > elems.len() {
                                let mut new_elems = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                for (i, e) in elems.iter().enumerate() { new_elems[i] = e.clone(); }
                                *elems = new_elems;
                            }
                            for i in 0..elements.len() {
                                if size + i < elems.len() { elems[size + i] = elements[i].clone(); }
                            }
                            size = new_size;
                            added = !elements.is_empty();
                        }
                    }
                    if let Some(coll_obj) = jvm.heap.get_mut(coll_id) {
                        coll_obj.fields.insert("size".to_string(), Value::Int(size as i32));
                    }
                }
            }
            frame.push(Value::Boolean(added))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "addAll".to_string(), "(Ljava/util/Collection;[Ljava/lang/Object;)Z".to_string(), true, Some(native_impl))
    }

    pub fn indexOfSubList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let target_ref = frame.pop()?;
            let source_ref = frame.pop()?;
            frame.push(Value::Int(-1))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "indexOfSubList".to_string(), "(Ljava/util/List;Ljava/util/List;)I".to_string(), true, Some(native_impl))
    }

    pub fn lastIndexOfSubList() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let target_ref = frame.pop()?;
            let source_ref = frame.pop()?;
            frame.push(Value::Int(-1))?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "lastIndexOfSubList".to_string(), "(Ljava/util/List;Ljava/util/List;)I".to_string(), true, Some(native_impl))
    }

    pub fn rotate() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let distance = frame.pop()?.as_int();
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = list_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if size > 0 {
                        if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                            if let Some(elements) = &mut arr.array_elements {
                                let d = ((distance % size as i32) + size as i32) % size as i32;
                                let d = d as usize;
                                // Rotate by reversing three segments
                                for i in 0..size / 2 { elements.swap(i, size - 1 - i); }
                                for i in 0..(size - d) / 2 { elements.swap(i, size - d - 1 - i); }
                                for i in 0..d / 2 { elements.swap(size - d + i, size - 1 - i); }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "rotate".to_string(), "(Ljava/util/List;I)V".to_string(), true, Some(native_impl))
    }

    pub fn swap() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let j = frame.pop()?.as_int() as usize;
            let i = frame.pop()?.as_int() as usize;
            let list_ref = frame.pop()?;
            if let Value::ObjectRef(list_id) = list_ref {
                if let Some(list_obj) = jvm.heap.get(list_id) {
                    let arr_ref = list_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            if i < elements.len() && j < elements.len() {
                                elements.swap(i, j);
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "swap".to_string(), "(Ljava/util/List;II)V".to_string(), true, Some(native_impl))
    }

    pub fn asLifoQueue() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let deque_ref = frame.pop()?;
            // Return the deque as a LIFO Queue (simplified: return the same deque)
            if let Value::ObjectRef(deque_id) = &deque_ref {
                // Extract field data first
                let (arr_ref, size) = if let Some(deque_obj) = jvm.heap.get(*deque_id) {
                    let a = deque_obj.fields.get("elementData").cloned();
                    let s = deque_obj.fields.get("size").cloned();
                    (a, s)
                } else { (None, None) };
                let wrapper = HeapObject::new("java.util.Collections$AsLIFOQueue".to_string());
                let wrapper_ref = jvm.allocate(wrapper)?;
                if let Some(w_obj) = jvm.heap.get_mut(wrapper_ref) {
                    if let Some(a) = arr_ref { w_obj.fields.insert("elementData".to_string(), a); }
                    if let Some(s) = size { w_obj.fields.insert("size".to_string(), s); }
                }
                frame.push(Value::ObjectRef(wrapper_ref))?;
                return Ok(());
            }
            frame.push(deque_ref)?;
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "asLifoQueue".to_string(), "(Ljava/util/Deque;)Ljava/util/Queue;".to_string(), true, Some(native_impl))
    }

    pub fn copy() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let dest_ref = frame.pop()?;
            let src_ref = frame.pop()?;
            // Extract source data first
            let (src_arr, src_size) = if let Value::ObjectRef(src_id) = &src_ref {
                if let Some(src_obj) = jvm.heap.get(*src_id) {
                    let a = src_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let s = src_obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    (a, s)
                } else { (0, 0) }
            } else { (0, 0) };
            // Copy source elements to a Vec first
            let mut src_elems = Vec::new();
            if let Some(src_arr_obj) = jvm.heap.get(src_arr) {
                if let Some(elems) = &src_arr_obj.array_elements {
                    for i in 0..src_size.min(elems.len()) {
                        src_elems.push(elems[i].clone());
                    }
                }
            }
            // Now update the destination
            if let Value::ObjectRef(dest_id) = &dest_ref {
                // Extract dest_arr first
                let dest_arr = if let Some(dest_obj) = jvm.heap.get(*dest_id) {
                    dest_obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0)
                } else { 0 };
                if let Some(dest_arr_obj) = jvm.heap.get_mut(dest_arr) {
                    if let Some(dest_elems) = &mut dest_arr_obj.array_elements {
                        for i in 0..src_elems.len().min(dest_elems.len()) {
                            dest_elems[i] = src_elems[i].clone();
                        }
                    }
                }
                if let Some(dest_obj) = jvm.heap.get_mut(*dest_id) {
                    dest_obj.fields.insert("size".to_string(), Value::Int(src_size as i32));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Collections".to_string(), "copy".to_string(), "(Ljava/util/List;Ljava/util/List;)V".to_string(), true, Some(native_impl))
    }
}

/// Helper: get a sort-friendly hash from a Value
fn hash_for_sort(val: &Value) -> i64 {
    match val {
        Value::Int(v) => *v as i64,
        Value::Long(v) => *v,
        Value::Float(v) => *v as i64,
        Value::Double(v) => *v as i64,
        Value::Byte(v) => *v as i64,
        Value::Short(v) => *v as i64,
        Value::Char(v) => *v as i64,
        Value::Boolean(v) => if *v { 1 } else { 0 },
        Value::ObjectRef(id) => *id as i64,
        Value::ArrayRef(id) => *id as i64,
        Value::Null => 0,
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
        jvm.method_area.add_native_method("java.util.Objects", Objects::deepEquals());
        jvm.method_area.add_native_method("java.util.Objects", Objects::hashCode());
        jvm.method_area.add_native_method("java.util.Objects", Objects::hash());
        jvm.method_area.add_native_method("java.util.Objects", Objects::toString());
        jvm.method_area.add_native_method("java.util.Objects", Objects::requireNonNull());
        jvm.method_area.add_native_method("java.util.Objects", Objects::compare());
        jvm.method_area.add_native_method("java.util.Objects", Objects::isNull());
        jvm.method_area.add_native_method("java.util.Objects", Objects::nonNull());
    }
}

impl Objects {
    pub fn deepEquals() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let b = frame.pop()?;
            let a = frame.pop()?;
            let result = match (&a, &b) {
                (Value::Null, Value::Null) => true,
                (Value::Null, _) | (_, Value::Null) => false,
                (Value::ArrayRef(a_id), Value::ArrayRef(b_id)) => {
                    if a_id == b_id { true }
                    else {
                        let arr_a = jvm.heap.get(*a_id);
                        let arr_b = jvm.heap.get(*b_id);
                        match (arr_a, arr_b) {
                            (Some(aa), Some(bb)) => aa.array_elements == bb.array_elements,
                            _ => false,
                        }
                    }
                }
                _ => a == b,
            };
            frame.push(Value::Boolean(result))?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "deepEquals".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Z".to_string(), true, Some(native_impl))
    }

    pub fn hash() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let values = frame.pop()?; // Object[]
            let mut hash = 0;
            if let Value::ArrayRef(arr_id) = values {
                // Simplified: use length as hash
                hash = arr_id as i32;
            }
            frame.push(Value::Int(hash))?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "hash".to_string(), "([Ljava/lang/Object;)I".to_string(), true, Some(native_impl))
    }

    pub fn compare() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let c = frame.pop()?; // Comparator
            let b = frame.pop()?;
            let a = frame.pop()?;
            // Simple comparison by hash value
            let result = if a == b { 0 } else { hash_for_sort(&a).cmp(&hash_for_sort(&b)) as i32 };
            frame.push(Value::Int(result))?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "compare".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I".to_string(), true, Some(native_impl))
    }

    pub fn isNull() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let obj = frame.pop()?;
            frame.push(Value::Boolean(obj.is_null()))?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "isNull".to_string(), "(Ljava/lang/Object;)Z".to_string(), true, Some(native_impl))
    }

    pub fn nonNull() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let obj = frame.pop()?;
            frame.push(Value::Boolean(!obj.is_null()))?;
            Ok(())
        });
        Method::new_native("java.util.Objects".to_string(), "nonNull".to_string(), "(Ljava/lang/Object;)Z".to_string(), true, Some(native_impl))
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
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::addLast());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::get());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::getFirst());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::getLast());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::size());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::isEmpty());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::remove());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::removeFirst());
        jvm.method_area.add_native_method("java.util.LinkedList", LinkedList::removeLast());
    }
}

impl LinkedList {
    pub fn addLast() -> Method {
        // Same as add()
        LinkedList::add()
    }

    pub fn removeFirst() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
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
                if current_size > 0 {
                    let result = if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            if !elements.is_empty() { elements[0].clone() } else { Value::Null }
                        } else { Value::Null }
                    } else { Value::Null };
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            for i in 0..current_size - 1 {
                                elements[i] = elements[i + 1].clone();
                            }
                            if current_size > 0 { elements[current_size - 1] = Value::Null; }
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
        Method::new_native("java.util.LinkedList".to_string(), "removeFirst".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn removeLast() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
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
                if current_size > 0 {
                    let result = if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            if current_size - 1 < elements.len() { elements[current_size - 1].clone() } else { Value::Null }
                        } else { Value::Null }
                    } else { Value::Null };
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            elements[current_size - 1] = Value::Null;
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
        Method::new_native("java.util.LinkedList".to_string(), "removeLast".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
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
                            if values_equal(&*jvm, k, &key) {
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
                                if i < keys.len() && values_equal(&*jvm, &keys[i], &key) {
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
        jvm.method_area.add_native_method("java.util.UUID", UUID::fromString());
        jvm.method_area.add_native_method("java.util.UUID", UUID::toString());
    }
}

impl UUID {
    pub fn fromString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.pop()?;
            let uuid_str = if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(str_id) {
                    str_obj.string_value.clone().unwrap_or_default()
                } else { String::new() }
            } else { String::new() };
            // Store the UUID string (simplified: just store the input string)
            let obj = HeapObject::new_string("java.lang.String".to_string(), uuid_str);
            let ref_id = jvm.allocate(obj)?;
            let uuid_obj = HeapObject::new("java.util.UUID".to_string());
            let uuid_ref = jvm.allocate(uuid_obj)?;
            if let Some(u) = jvm.heap.get_mut(uuid_ref) {
                u.fields.insert("value".to_string(), Value::ObjectRef(ref_id));
            }
            frame.push(Value::ObjectRef(uuid_ref))?;
            Ok(())
        });
        Method::new_native("java.util.UUID".to_string(), "fromString".to_string(), "(Ljava/lang/String;)Ljava/util/UUID;".to_string(), true, Some(native_impl))
    }
}

// ========== java.util.Random ==========

pub struct Random;

impl Random {
    pub fn init() -> Method {
        Method::new_native("java.util.Random".to_string(), "<init>".to_string(), "()V".to_string(), false, None)
    }

    pub fn nextInt() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
            let r = (ts.as_nanos() & 0x7FFFFFFF) as i32;
            frame.push(Value::Int(r))?;
            Ok(())
        });
        Method::new_native("java.util.Random".to_string(), "nextInt".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn nextInt_bound() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let bound = frame.pop()?.as_int();
            if bound <= 0 { return Err(JvmError::RuntimeError(RuntimeError::ArithmeticException)); }
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
            let r = ((ts.as_nanos() & 0x7FFFFFFF) as i32).abs() % bound;
            frame.push(Value::Int(r))?;
            Ok(())
        });
        Method::new_native("java.util.Random".to_string(), "nextInt".to_string(), "(I)I".to_string(), false, Some(native_impl))
    }

    pub fn nextLong() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
            let r = (ts.as_nanos() & 0x7FFFFFFFFFFFFFFF) as i64;
            frame.push(Value::Long(r))?;
            Ok(())
        });
        Method::new_native("java.util.Random".to_string(), "nextLong".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn nextDouble() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
            let r = ((ts.as_nanos() & 0x7FFFFFFF) as f64) / 2147483648.0;
            frame.push(Value::Double(r))?;
            Ok(())
        });
        Method::new_native("java.util.Random".to_string(), "nextDouble".to_string(), "()D".to_string(), false, Some(native_impl))
    }

    pub fn nextBoolean() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
            let r = (ts.as_nanos() & 1) == 1;
            frame.push(Value::Boolean(r))?;
            Ok(())
        });
        Method::new_native("java.util.Random".to_string(), "nextBoolean".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Random", Random::init());
        jvm.method_area.add_native_method("java.util.Random", Random::nextInt());
        jvm.method_area.add_native_method("java.util.Random", Random::nextInt_bound());
        jvm.method_area.add_native_method("java.util.Random", Random::nextLong());
        jvm.method_area.add_native_method("java.util.Random", Random::nextDouble());
        jvm.method_area.add_native_method("java.util.Random", Random::nextBoolean());
    }
}

// ========== java.util.Stack ==========

pub struct Stack;

impl Stack {
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
        Method::new_native("java.util.Stack".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn push() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let elem_clone = elem.clone();
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
                            elements[current_size] = elem_clone;
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(new_size as i32));
                }
            }
            frame.push(elem)?; // push returns the element
            Ok(())
        });
        Method::new_native("java.util.Stack".to_string(), "push".to_string(), "(Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn pop() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
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
                if current_size > 0 {
                    let result = if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            elements[current_size - 1].clone()
                        } else { Value::Null }
                    } else { Value::Null };
                    if let Some(arr) = jvm.heap.get_mut(arr_ref) {
                        if let Some(elements) = &mut arr.array_elements {
                            elements[current_size - 1] = Value::Null;
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
        Method::new_native("java.util.Stack".to_string(), "pop".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn peek() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let arr_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if size > 0 {
                        if let Some(arr) = jvm.heap.get(arr_ref) {
                            if let Some(elements) = &arr.array_elements {
                                if size - 1 < elements.len() {
                                    frame.push(elements[size - 1].clone())?;
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Stack".to_string(), "peek".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn empty() -> Method {
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
        Method::new_native("java.util.Stack".to_string(), "empty".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn search() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let elem = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let mut pos = -1;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let arr_ref = obj.fields.get("elementData")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(arr) = jvm.heap.get(arr_ref) {
                        if let Some(elements) = &arr.array_elements {
                            for i in (0..size).rev() {
                                if i < elements.len() && elements[i] == elem {
                                    pos = (size - i) as i32;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(pos))?;
            Ok(())
        });
        Method::new_native("java.util.Stack".to_string(), "search".to_string(), "(Ljava/lang/Object;)I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Stack", Stack::init());
        jvm.method_area.add_native_method("java.util.Stack", Stack::push());
        jvm.method_area.add_native_method("java.util.Stack", Stack::pop());
        jvm.method_area.add_native_method("java.util.Stack", Stack::peek());
        jvm.method_area.add_native_method("java.util.Stack", Stack::empty());
        jvm.method_area.add_native_method("java.util.Stack", Stack::search());
    }
}

// ========== java.util.Base64 ==========

pub struct Base64;

impl Base64 {
    pub fn getEncoder() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj = HeapObject::new("java.util.Base64$Encoder".to_string());
            let ref_id = jvm.allocate(obj)?;
            frame.push(Value::ObjectRef(ref_id))?;
            Ok(())
        });
        Method::new_native("java.util.Base64".to_string(), "getEncoder".to_string(), "()Ljava/util/Base64$Encoder;".to_string(), true, Some(native_impl))
    }

    pub fn getDecoder() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj = HeapObject::new("java.util.Base64$Decoder".to_string());
            let ref_id = jvm.allocate(obj)?;
            frame.push(Value::ObjectRef(ref_id))?;
            Ok(())
        });
        Method::new_native("java.util.Base64".to_string(), "getDecoder".to_string(), "()Ljava/util/Base64$Decoder;".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Base64", Base64::getEncoder());
        jvm.method_area.add_native_method("java.util.Base64", Base64::getDecoder());
        // Register the inner Encoder/Decoder classes
        let encoder_encode = Method::new_native(
            "java.util.Base64$Encoder".to_string(),
            "encodeToString".to_string(),
            "([B)Ljava/lang/String;".to_string(),
            false,
            Some(Arc::new(|frame, jvm| {
                let arr_ref = frame.get_local(1)?.clone();
                let mut bytes = Vec::new();
                if let Value::ArrayRef(arr_id) = arr_ref {
                    if let Some(arr_obj) = jvm.heap.get(arr_id) {
                        if let Some(elements) = &arr_obj.array_elements {
                            for elem in elements {
                                match elem {
                                    Value::Byte(b) => bytes.push(*b as u8),
                                    Value::Int(i) => bytes.push(*i as u8),
                                    _ => bytes.push(0),
                                }
                            }
                        }
                    }
                }
                let encoded = base64_encode(&bytes);
                let s = HeapObject::new_string("java.lang.String".to_string(), encoded);
                let r = jvm.allocate(s)?;
                frame.push(Value::ObjectRef(r))?;
                Ok(())
            }))
        );
        jvm.method_area.add_native_method("java.util.Base64$Encoder", encoder_encode);

        let decoder_decode = Method::new_native(
            "java.util.Base64$Decoder".to_string(),
            "decode".to_string(),
            "(Ljava/lang/String;)[B".to_string(),
            false,
            Some(Arc::new(|frame, jvm| {
                let str_ref = frame.get_local(1)?.clone();
                let s = if let Value::ObjectRef(str_id) = str_ref {
                    if let Some(str_obj) = jvm.heap.get(str_id) {
                        str_obj.string_value.clone().unwrap_or_default()
                    } else { String::new() }
                } else { String::new() };
                let decoded = base64_decode(&s);
                let arr = HeapObject::new_array("[B".to_string(), decoded.len());
                let arr_ref = jvm.allocate(arr)?;
                if let Some(arr_obj) = jvm.heap.get_mut(arr_ref) {
                    if let Some(elements) = &mut arr_obj.array_elements {
                        for (i, &b) in decoded.iter().enumerate() {
                            if i < elements.len() {
                                elements[i] = Value::Byte(b as i8);
                            }
                        }
                    }
                }
                frame.push(Value::ArrayRef(arr_ref))?;
                Ok(())
            }))
        );
        jvm.method_area.add_native_method("java.util.Base64$Decoder", decoder_decode);
    }
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else { result.push('='); }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else { result.push('='); }
    }
    result
}

fn base64_decode(data: &str) -> Vec<u8> {
    fn char_val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some((c - b'A') as u32),
            b'a'..=b'z' => Some((c - b'a' + 26) as u32),
            b'0'..=b'9' => Some((c - b'0' + 52) as u32),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let mut result = Vec::new();
    let bytes: Vec<u8> = data.bytes().filter(|&c| c != b'=' && char_val(c).is_some()).collect();
    for chunk in bytes.chunks(4) {
        if chunk.len() < 4 { break; }
        let vals: Vec<u32> = chunk.iter().filter_map(|&c| char_val(c)).collect();
        if vals.len() < 4 { continue; }
        let triple = (vals[0] << 18) | (vals[1] << 12) | (vals[2] << 6) | vals[3];
        result.push((triple >> 16) as u8);
        result.push((triple >> 8) as u8);
        result.push(triple as u8);
    }
    result
}

// ========== java.util.Properties ==========

pub struct Properties;

impl Properties {
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
        Method::new_native("java.util.Properties".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn setProperty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?;
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
                // Check if key exists
                let mut found = false;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(keys) = &keys_arr.array_elements {
                        found = keys.iter().any(|k| values_equal(&*jvm, k, &key));
                    }
                }
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
                                vals[current_size] = val;
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
        Method::new_native("java.util.Properties".to_string(), "setProperty".to_string(), "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn getProperty() -> Method {
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
                                if i < keys.len() && values_equal(&*jvm, &keys[i], &key) {
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
        Method::new_native("java.util.Properties".to_string(), "getProperty".to_string(), "(Ljava/lang/String;)Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Properties", Properties::init());
        jvm.method_area.add_native_method("java.util.Properties", Properties::setProperty());
        jvm.method_area.add_native_method("java.util.Properties", Properties::getProperty());
        jvm.method_area.add_native_method("java.util.Properties", Properties::load());
    }
}

impl Properties {
    pub fn load() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let in_ref = frame.pop()?; // InputStream
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Try to read from the InputStream by reading its string content
                let mut content = String::new();
                if let Value::ObjectRef(in_id) = &in_ref {
                    if let Some(in_obj) = jvm.heap.get(*in_id) {
                        if let Some(Value::ObjectRef(str_id)) = in_obj.fields.get("path") {
                            if let Some(str_obj) = jvm.heap.get(*str_id) {
                                if let Some(path) = &str_obj.string_value {
                                    if let Ok(data) = std::fs::read_to_string(path) {
                                        content = data;
                                    }
                                }
                            }
                        }
                    }
                }
                // Parse the properties file content (key=value format)
                let (keys_ref, vals_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let k_ref = obj.fields.get("keys")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let v_ref = obj.fields.get("values")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (k_ref, v_ref)
                };
                let mut size = 0usize;
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') || line.starts_with('!') { continue; }
                    if let Some(eq_pos) = line.find('=') {
                        let key = line[..eq_pos].trim();
                        let val = line[eq_pos+1..].trim();
                        if !key.is_empty() {
                            // Add to properties
                            let key_obj = HeapObject::new_string("java.lang.String".to_string(), key.to_string());
                            let val_obj = HeapObject::new_string("java.lang.String".to_string(), val.to_string());
                            let key_ref = jvm.allocate(key_obj)?;
                            let val_ref = jvm.allocate(val_obj)?;
                            let new_size = size + 1;
                            if let Some(keys_arr) = jvm.heap.get_mut(keys_ref) {
                                if let Some(keys) = &mut keys_arr.array_elements {
                                    if size >= keys.len() {
                                        let mut new_keys = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                        for (i, k) in keys.iter().enumerate() { new_keys[i] = k.clone(); }
                                        *keys = new_keys;
                                    }
                                    if size < keys.len() { keys[size] = Value::ObjectRef(key_ref); }
                                }
                            }
                            if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                                if let Some(vals) = &mut vals_arr.array_elements {
                                    if size >= vals.len() {
                                        let mut new_vals = vec![Value::Null; (new_size * 3 / 2 + 1).max(10)];
                                        for (i, v) in vals.iter().enumerate() { new_vals[i] = v.clone(); }
                                        *vals = new_vals;
                                    }
                                    if size < vals.len() { vals[size] = Value::ObjectRef(val_ref); }
                                }
                            }
                            size = new_size;
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("size".to_string(), Value::Int(size as i32));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Properties".to_string(), "load".to_string(), "(Ljava/io/InputStream;)V".to_string(), false, Some(native_impl))
    }
}

// ========== java.util.Comparator ==========

pub struct Comparator;

impl Comparator {
    pub fn compare() -> Method {
        // Abstract method - registered for interface completeness
        Method::new_native("java.util.Comparator".to_string(), "compare".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)I".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Comparator", Comparator::compare());
        jvm.method_area.add_native_method("java.util.Comparator", Comparator::naturalOrder());
        jvm.method_area.add_native_method("java.util.Comparator", Comparator::reverseOrder());
        jvm.method_area.add_native_method("java.util.Comparator", Comparator::thenComparing());
        jvm.method_area.add_native_method("java.util.Comparator", Comparator::nullsFirst());
        jvm.method_area.add_native_method("java.util.Comparator", Comparator::nullsLast());
    }
}

impl Comparator {
    pub fn naturalOrder() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            // Return a comparator object that uses natural ordering
            let comp = HeapObject::new("java.util.Comparator".to_string());
            // We push the comparator but it's a placeholder - native methods handle comparison
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Comparator".to_string(), "naturalOrder".to_string(), "()Ljava/util/Comparator;".to_string(), true, Some(native_impl))
    }

    pub fn reverseOrder() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Comparator".to_string(), "reverseOrder".to_string(), "()Ljava/util/Comparator;".to_string(), true, Some(native_impl))
    }

    pub fn thenComparing() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Comparator".to_string(), "thenComparing".to_string(), "(Ljava/util/Comparator;)Ljava/util/Comparator;".to_string(), false, Some(native_impl))
    }

    pub fn nullsFirst() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Comparator".to_string(), "nullsFirst".to_string(), "()Ljava/util/Comparator;".to_string(), true, Some(native_impl))
    }

    pub fn nullsLast() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Comparator".to_string(), "nullsLast".to_string(), "()Ljava/util/Comparator;".to_string(), true, Some(native_impl))
    }

    pub fn reversed() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Comparator".to_string(), "reversed".to_string(), "()Ljava/util/Comparator;".to_string(), false, Some(native_impl))
    }
}

// ========== java.util.concurrent.atomic.AtomicInteger ==========

pub struct AtomicInteger;

impl AtomicInteger {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn init_value() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?.as_int();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Int(val));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "<init>".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn get() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Int(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "get".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn set() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?.as_int();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Int(val));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "set".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn incrementAndGet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Int(v) = v { Some(*v) } else { None })
                        .unwrap_or(0);
                    let new_val = old + 1;
                    obj.fields.insert("value".to_string(), Value::Int(new_val));
                    new_val
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "incrementAndGet".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn decrementAndGet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Int(v) = v { Some(*v) } else { None })
                        .unwrap_or(0);
                    let new_val = old - 1;
                    obj.fields.insert("value".to_string(), Value::Int(new_val));
                    new_val
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "decrementAndGet".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn addAndGet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let delta = frame.get_local(1)?.as_int();
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Int(v) = v { Some(*v) } else { None })
                        .unwrap_or(0);
                    let new_val = old + delta;
                    obj.fields.insert("value".to_string(), Value::Int(new_val));
                    new_val
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "addAndGet".to_string(), "(I)I".to_string(), false, Some(native_impl))
    }

    pub fn compareAndSet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let update = frame.get_local(2)?.as_int();
            let expect = frame.get_local(1)?.as_int();
            let this_ref = frame.get_local(0)?;
            let mut success = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Int(v) = v { Some(*v) } else { None })
                        .unwrap_or(0);
                    if old == expect {
                        obj.fields.insert("value".to_string(), Value::Int(update));
                        success = true;
                    }
                }
            }
            frame.push(Value::Boolean(success))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicInteger".to_string(), "compareAndSet".to_string(), "(II)Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::init());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::init_value());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::get());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::set());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::incrementAndGet());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::decrementAndGet());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::addAndGet());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicInteger", AtomicInteger::compareAndSet());
    }
}

// ========== java.util.concurrent.atomic.AtomicLong ==========

pub struct AtomicLong;

impl AtomicLong {
    pub fn init() -> Method {
        Method::new_native("java.util.concurrent.atomic.AtomicLong".to_string(), "<init>".to_string(), "()V".to_string(), false, None)
    }

    pub fn init_value() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?.as_long();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Long(val));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicLong".to_string(), "<init>".to_string(), "(J)V".to_string(), false, Some(native_impl))
    }

    pub fn get() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Long(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicLong".to_string(), "get".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn set() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?.as_long();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Long(val));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicLong".to_string(), "set".to_string(), "(J)V".to_string(), false, Some(native_impl))
    }

    pub fn incrementAndGet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0);
                    let new_val = old + 1;
                    obj.fields.insert("value".to_string(), Value::Long(new_val));
                    new_val
                } else { 0 }
            } else { 0 };
            frame.push(Value::Long(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicLong".to_string(), "incrementAndGet".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn addAndGet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let delta = frame.get_local(1)?.as_long();
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0);
                    let new_val = old + delta;
                    obj.fields.insert("value".to_string(), Value::Long(new_val));
                    new_val
                } else { 0 }
            } else { 0 };
            frame.push(Value::Long(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicLong".to_string(), "addAndGet".to_string(), "(J)J".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicLong", AtomicLong::init());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicLong", AtomicLong::init_value());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicLong", AtomicLong::get());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicLong", AtomicLong::set());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicLong", AtomicLong::incrementAndGet());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicLong", AtomicLong::addAndGet());
    }
}

// ========== java.util.concurrent.atomic.AtomicReference ==========

pub struct AtomicReference;

impl AtomicReference {
    pub fn init() -> Method {
        Method::new_native("java.util.concurrent.atomic.AtomicReference".to_string(), "<init>".to_string(), "()V".to_string(), false, None)
    }

    pub fn init_value() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), val);
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicReference".to_string(), "<init>".to_string(), "(Ljava/lang/Object;)V".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.concurrent.atomic.AtomicReference".to_string(), "get".to_string(), "()Ljava/lang/Object;".to_string(), false, Some(native_impl))
    }

    pub fn set() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), val);
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicReference".to_string(), "set".to_string(), "(Ljava/lang/Object;)V".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicReference", AtomicReference::init());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicReference", AtomicReference::init_value());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicReference", AtomicReference::get());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicReference", AtomicReference::set());
    }
}

// ========== java.util.concurrent.atomic.AtomicBoolean ==========

pub struct AtomicBoolean;

impl AtomicBoolean {
    pub fn init() -> Method {
        Method::new_native("java.util.concurrent.atomic.AtomicBoolean".to_string(), "<init>".to_string(), "()V".to_string(), false, None)
    }

    pub fn init_value() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?.as_bool();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Boolean(val));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicBoolean".to_string(), "<init>".to_string(), "(Z)V".to_string(), false, Some(native_impl))
    }

    pub fn get() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Boolean(v) = v { Some(*v) } else { None })
                        .unwrap_or(false)
                } else { false }
            } else { false };
            frame.push(Value::Boolean(val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicBoolean".to_string(), "get".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn set() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?.as_bool();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Boolean(val));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicBoolean".to_string(), "set".to_string(), "(Z)V".to_string(), false, Some(native_impl))
    }

    pub fn getAndSet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let new_val = frame.get_local(1)?.as_bool();
            let this_ref = frame.get_local(0)?;
            let old_val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Boolean(v) = v { Some(*v) } else { None })
                        .unwrap_or(false);
                    obj.fields.insert("value".to_string(), Value::Boolean(new_val));
                    old
                } else { false }
            } else { false };
            frame.push(Value::Boolean(old_val))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicBoolean".to_string(), "getAndSet".to_string(), "(Z)Z".to_string(), false, Some(native_impl))
    }

    pub fn compareAndSet() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let update = frame.get_local(2)?.as_bool();
            let expect = frame.get_local(1)?.as_bool();
            let this_ref = frame.get_local(0)?;
            let mut success = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let old = obj.fields.get("value")
                        .and_then(|v| if let Value::Boolean(v) = v { Some(*v) } else { None })
                        .unwrap_or(false);
                    if old == expect {
                        obj.fields.insert("value".to_string(), Value::Boolean(update));
                        success = true;
                    }
                }
            }
            frame.push(Value::Boolean(success))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.atomic.AtomicBoolean".to_string(), "compareAndSet".to_string(), "(ZZ)Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicBoolean", AtomicBoolean::init());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicBoolean", AtomicBoolean::init_value());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicBoolean", AtomicBoolean::get());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicBoolean", AtomicBoolean::set());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicBoolean", AtomicBoolean::getAndSet());
        jvm.method_area.add_native_method("java.util.concurrent.atomic.AtomicBoolean", AtomicBoolean::compareAndSet());
    }
}

// ========== java.util.LinkedHashMap ==========

pub struct LinkedHashMap;

impl LinkedHashMap {
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
        Method::new_native("java.util.LinkedHashMap".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
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
                // Check if key exists
                let mut found = false;
                let mut found_idx = 0;
                if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                    if let Some(keys) = &keys_arr.array_elements {
                        for (i, k) in keys.iter().enumerate() {
                            if i >= current_size { break; }
                            if values_equal(&*jvm, k, &key) {
                                found = true;
                                found_idx = i;
                                break;
                            }
                        }
                    }
                }
                if found {
                    // Update existing entry
                    if let Some(vals_arr) = jvm.heap.get_mut(vals_ref) {
                        if let Some(vals) = &mut vals_arr.array_elements {
                            if found_idx < vals.len() {
                                vals[found_idx] = value;
                            }
                        }
                    }
                } else {
                    // Add new entry
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
        Method::new_native("java.util.LinkedHashMap".to_string(), "put".to_string(), "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
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
                                if i < keys.len() && values_equal(&*jvm, &keys[i], &key) {
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
        Method::new_native("java.util.LinkedHashMap".to_string(), "get".to_string(), "(Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.LinkedHashMap".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
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
        Method::new_native("java.util.LinkedHashMap".to_string(), "isEmpty".to_string(), "()Z".to_string(), false, Some(native_impl))
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
                    let size = obj.fields.get("size")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(keys_arr) = jvm.heap.get(keys_ref) {
                        if let Some(keys) = &keys_arr.array_elements {
                            for i in 0..size {
                                if i < keys.len() && values_equal(&*jvm, &keys[i], &key) {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(found))?;
            Ok(())
        });
        Method::new_native("java.util.LinkedHashMap".to_string(), "containsKey".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.LinkedHashMap", LinkedHashMap::init());
        jvm.method_area.add_native_method("java.util.LinkedHashMap", LinkedHashMap::put());
        jvm.method_area.add_native_method("java.util.LinkedHashMap", LinkedHashMap::get());
        jvm.method_area.add_native_method("java.util.LinkedHashMap", LinkedHashMap::size());
        jvm.method_area.add_native_method("java.util.LinkedHashMap", LinkedHashMap::isEmpty());
        jvm.method_area.add_native_method("java.util.LinkedHashMap", LinkedHashMap::containsKey());
    }
}

// ========== java.util.Scanner ==========

pub struct Scanner;

impl Scanner {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let source = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("source".to_string(), source);
                    obj.fields.insert("position".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Scanner".to_string(), "<init>".to_string(), "(Ljava/io/InputStream;)V".to_string(), false, Some(native_impl))
    }

    fn get_input_string(jvm: &JVM, this_id: usize) -> String {
        if let Some(obj) = jvm.heap.get(this_id) {
            if let Some(Value::ObjectRef(source_id)) = obj.fields.get("source") {
                if let Some(source_obj) = jvm.heap.get(*source_id) {
                    if let Some(s) = &source_obj.string_value {
                        return s.clone();
                    }
                }
            }
        }
        String::new()
    }

    fn get_position(jvm: &JVM, this_id: usize) -> usize {
        if let Some(obj) = jvm.heap.get(this_id) {
            if let Some(Value::Int(pos)) = obj.fields.get("position") {
                return *pos as usize;
            }
        }
        0
    }

    fn set_position(jvm: &mut JVM, this_id: usize, pos: usize) {
        if let Some(obj) = jvm.heap.get_mut(this_id) {
            obj.fields.insert("position".to_string(), Value::Int(pos as i32));
        }
    }

    pub fn nextInt() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let input = Self::get_input_string(jvm, *this_id);
                let pos = Self::get_position(jvm, *this_id);
                let chars: Vec<char> = input.chars().collect();
                // Skip whitespace
                let mut p = pos;
                while p < chars.len() && chars[p].is_whitespace() { p += 1; }
                // Parse integer
                let start = p;
                if p < chars.len() && (chars[p] == '-' || chars[p] == '+') { p += 1; }
                while p < chars.len() && chars[p].is_ascii_digit() { p += 1; }
                if p > start {
                    let num_str: String = chars[start..p].iter().collect();
                    if let Ok(val) = num_str.parse::<i32>() {
                        Self::set_position(jvm, *this_id, p);
                        frame.push(Value::Int(val))?;
                        return Ok(());
                    }
                }
            }
            frame.push(Value::Int(0))?;
            Ok(())
        });
        Method::new_native("java.util.Scanner".to_string(), "nextInt".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn nextLong() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let input = Self::get_input_string(jvm, *this_id);
                let pos = Self::get_position(jvm, *this_id);
                let chars: Vec<char> = input.chars().collect();
                let mut p = pos;
                while p < chars.len() && chars[p].is_whitespace() { p += 1; }
                let start = p;
                if p < chars.len() && (chars[p] == '-' || chars[p] == '+') { p += 1; }
                while p < chars.len() && chars[p].is_ascii_digit() { p += 1; }
                if p > start {
                    let num_str: String = chars[start..p].iter().collect();
                    if let Ok(val) = num_str.parse::<i64>() {
                        Self::set_position(jvm, *this_id, p);
                        frame.push(Value::Long(val))?;
                        return Ok(());
                    }
                }
            }
            frame.push(Value::Long(0))?;
            Ok(())
        });
        Method::new_native("java.util.Scanner".to_string(), "nextLong".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn nextDouble() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let input = Self::get_input_string(jvm, *this_id);
                let pos = Self::get_position(jvm, *this_id);
                let chars: Vec<char> = input.chars().collect();
                let mut p = pos;
                while p < chars.len() && chars[p].is_whitespace() { p += 1; }
                let start = p;
                if p < chars.len() && (chars[p] == '-' || chars[p] == '+') { p += 1; }
                while p < chars.len() && (chars[p].is_ascii_digit() || chars[p] == '.') { p += 1; }
                if p > start {
                    let num_str: String = chars[start..p].iter().collect();
                    if let Ok(val) = num_str.parse::<f64>() {
                        Self::set_position(jvm, *this_id, p);
                        frame.push(Value::Double(val))?;
                        return Ok(());
                    }
                }
            }
            frame.push(Value::Double(0.0))?;
            Ok(())
        });
        Method::new_native("java.util.Scanner".to_string(), "nextDouble".to_string(), "()D".to_string(), false, Some(native_impl))
    }

    pub fn next() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let input = Self::get_input_string(jvm, *this_id);
                let pos = Self::get_position(jvm, *this_id);
                let chars: Vec<char> = input.chars().collect();
                let mut p = pos;
                while p < chars.len() && chars[p].is_whitespace() { p += 1; }
                let start = p;
                while p < chars.len() && !chars[p].is_whitespace() { p += 1; }
                if p > start {
                    let token: String = chars[start..p].iter().collect();
                    let s = HeapObject::new_string("java.lang.String".to_string(), token);
                    let ref_id = jvm.allocate(s)?;
                    Self::set_position(jvm, *this_id, p);
                    frame.push(Value::ObjectRef(ref_id))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Scanner".to_string(), "next".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn nextLine() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let input = Self::get_input_string(jvm, *this_id);
                let pos = Self::get_position(jvm, *this_id);
                let chars: Vec<char> = input.chars().collect();
                if pos < chars.len() {
                    // Find end of line
                    let mut p = pos;
                    let mut skip_newline = false;
                    if p < chars.len() && chars[p] == '\n' { skip_newline = true; p += 1; }
                    if skip_newline || (p < chars.len() && chars[p] == '\r') { p += 1; }
                    
                    let start = p;
                    while p < chars.len() && chars[p] != '\n' { p += 1; }
                    let line: String = chars[start..p].iter().collect();
                    // Skip past the newline
                    if p < chars.len() && chars[p] == '\n' { p += 1; }
                    let s = HeapObject::new_string("java.lang.String".to_string(), line);
                    let ref_id = jvm.allocate(s)?;
                    Self::set_position(jvm, *this_id, p);
                    frame.push(Value::ObjectRef(ref_id))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.Scanner".to_string(), "nextLine".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn hasNext() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut has = false;
            if let Value::ObjectRef(this_id) = this_ref {
                let input = Self::get_input_string(jvm, *this_id);
                let pos = Self::get_position(jvm, *this_id);
                let chars: Vec<char> = input.chars().collect();
                let mut p = pos;
                while p < chars.len() && chars[p].is_whitespace() { p += 1; }
                has = p < chars.len();
            }
            frame.push(Value::Boolean(has))?;
            Ok(())
        });
        Method::new_native("java.util.Scanner".to_string(), "hasNext".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn close() -> Method {
        Method::new_native("java.util.Scanner".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::init());
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::nextInt());
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::nextLong());
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::nextDouble());
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::next());
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::nextLine());
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::hasNext());
        jvm.method_area.add_native_method("java.util.Scanner", Scanner::close());
    }
}

// ========== java.util.concurrent.locks.ReentrantLock ==========

pub struct ReentrantLock;

impl ReentrantLock {
    pub fn init() -> Method {
        Method::new_native("java.util.concurrent.locks.ReentrantLock".to_string(), "<init>".to_string(), "()V".to_string(), false, None)
    }

    pub fn lock() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    // Track the owning thread
                    obj.fields.insert("owner".to_string(), Value::Int(jvm.current_thread_id as i32));
                    let count = obj.fields.get("holdCount")
                        .and_then(|v| if let Value::Int(c) = v { Some(*c) } else { None })
                        .unwrap_or(0);
                    obj.fields.insert("holdCount".to_string(), Value::Int(count + 1));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.locks.ReentrantLock".to_string(), "lock".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn unlock() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let count = obj.fields.get("holdCount")
                        .and_then(|v| if let Value::Int(c) = v { Some(*c) } else { None })
                        .unwrap_or(0);
                    if count > 1 {
                        obj.fields.insert("holdCount".to_string(), Value::Int(count - 1));
                    } else {
                        obj.fields.insert("owner".to_string(), Value::Int(-1));
                        obj.fields.insert("holdCount".to_string(), Value::Int(0));
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.concurrent.locks.ReentrantLock".to_string(), "unlock".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn tryLock() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut acquired = true;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let owner = obj.fields.get("owner")
                        .and_then(|v| if let Value::Int(o) = v { Some(*o) } else { None })
                        .unwrap_or(-1);
                    if owner == -1 || owner == jvm.current_thread_id as i32 {
                        obj.fields.insert("owner".to_string(), Value::Int(jvm.current_thread_id as i32));
                        let count = obj.fields.get("holdCount")
                            .and_then(|v| if let Value::Int(c) = v { Some(*c) } else { None })
                            .unwrap_or(0);
                        obj.fields.insert("holdCount".to_string(), Value::Int(count + 1));
                    } else {
                        acquired = false;
                    }
                }
            }
            frame.push(Value::Boolean(acquired))?;
            Ok(())
        });
        Method::new_native("java.util.concurrent.locks.ReentrantLock".to_string(), "tryLock".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.concurrent.locks.ReentrantLock", ReentrantLock::init());
        jvm.method_area.add_native_method("java.util.concurrent.locks.ReentrantLock", ReentrantLock::lock());
        jvm.method_area.add_native_method("java.util.concurrent.locks.ReentrantLock", ReentrantLock::unlock());
        jvm.method_area.add_native_method("java.util.concurrent.locks.ReentrantLock", ReentrantLock::tryLock());
    }
}

// ========== java.util.Date ==========

pub struct Date;

impl Date {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                use std::time::{SystemTime, UNIX_EPOCH};
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
                let millis = now.as_millis() as i64;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("fastTime".to_string(), Value::Long(millis));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Date".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn init_millis() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let millis = frame.get_local(1)?.as_long();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("fastTime".to_string(), Value::Long(millis));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Date".to_string(), "<init>".to_string(), "(J)V".to_string(), false, Some(native_impl))
    }

    pub fn getTime() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let millis = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("fastTime")
                        .and_then(|v| if let Value::Long(t) = v { Some(*t) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Long(millis))?;
            Ok(())
        });
        Method::new_native("java.util.Date".to_string(), "getTime".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn setTime() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let millis = frame.get_local(1)?.as_long();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("fastTime".to_string(), Value::Long(millis));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.Date".to_string(), "setTime".to_string(), "(J)V".to_string(), false, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let millis = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("fastTime")
                        .and_then(|v| if let Value::Long(t) = v { Some(*t) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            // Simple date format: "EEE MMM dd HH:mm:ss zzz yyyy"
            let secs = millis / 1000;
            let days = secs / 86400;
            let time_secs = secs % 86400;
            let hours = time_secs / 3600;
            let minutes = (time_secs % 3600) / 60;
            let seconds = time_secs % 60;
            // Approximate date from days since epoch
            let year = 1970 + (days / 365) as i32;
            let day_of_year = (days % 365) as i32;
            let month = (day_of_year / 30).min(11);
            let day = (day_of_year % 30 + 1).min(31);
            let s = format!("{:02} {:02} {:02}:{:02}:{:02} UTC {}", 
                month + 1, day, hours, minutes, seconds, year);
            let str_obj = HeapObject::new_string("java.lang.String".to_string(), s);
            let ref_id = jvm.allocate(str_obj)?;
            frame.push(Value::ObjectRef(ref_id))?;
            Ok(())
        });
        Method::new_native("java.util.Date".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Date", Date::init());
        jvm.method_area.add_native_method("java.util.Date", Date::init_millis());
        jvm.method_area.add_native_method("java.util.Date", Date::getTime());
        jvm.method_area.add_native_method("java.util.Date", Date::setTime());
        jvm.method_area.add_native_method("java.util.Date", Date::toString());
    }
}

// ========== java.util.Queue ==========

pub struct Queue;

impl Queue {
    pub fn offer() -> Method {
        Method::new_native("java.util.Queue".to_string(), "offer".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, None)
    }
    pub fn poll() -> Method {
        Method::new_native("java.util.Queue".to_string(), "poll".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }
    pub fn peek() -> Method {
        Method::new_native("java.util.Queue".to_string(), "peek".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }
    pub fn element() -> Method {
        Method::new_native("java.util.Queue".to_string(), "element".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }
    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Queue", Queue::offer());
        jvm.method_area.add_native_method("java.util.Queue", Queue::poll());
        jvm.method_area.add_native_method("java.util.Queue", Queue::peek());
        jvm.method_area.add_native_method("java.util.Queue", Queue::element());
    }
}

// ========== java.util.Deque ==========

pub struct Deque;

impl Deque {
    pub fn offerFirst() -> Method {
        Method::new_native("java.util.Deque".to_string(), "offerFirst".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, None)
    }
    pub fn offerLast() -> Method {
        Method::new_native("java.util.Deque".to_string(), "offerLast".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, None)
    }
    pub fn pollFirst() -> Method {
        Method::new_native("java.util.Deque".to_string(), "pollFirst".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }
    pub fn pollLast() -> Method {
        Method::new_native("java.util.Deque".to_string(), "pollLast".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }
    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Deque", Deque::offerFirst());
        jvm.method_area.add_native_method("java.util.Deque", Deque::offerLast());
        jvm.method_area.add_native_method("java.util.Deque", Deque::pollFirst());
        jvm.method_area.add_native_method("java.util.Deque", Deque::pollLast());
    }
}

// ========== java.util.Locale ==========

pub struct Locale;

impl Locale {
    pub fn getDefault() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let locale = HeapObject::new("java.util.Locale".to_string());
            let locale_ref = jvm.allocate(locale)?;
            let lang = HeapObject::new_string("java.lang.String".to_string(), "en".to_string());
            let lang_ref = jvm.allocate(lang)?;
            let country = HeapObject::new_string("java.lang.String".to_string(), "US".to_string());
            let country_ref = jvm.allocate(country)?;
            if let Some(obj) = jvm.heap.get_mut(locale_ref) {
                obj.fields.insert("language".to_string(), Value::ObjectRef(lang_ref));
                obj.fields.insert("country".to_string(), Value::ObjectRef(country_ref));
            }
            frame.push(Value::ObjectRef(locale_ref))?;
            Ok(())
        });
        Method::new_native("java.util.Locale".to_string(), "getDefault".to_string(), "()Ljava/util/Locale;".to_string(), true, Some(native_impl))
    }

    pub fn getLanguage() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(lang_id)) = obj.fields.get("language") {
                        if let Some(lang_obj) = jvm.heap.get(*lang_id) {
                            if let Some(s) = &lang_obj.string_value {
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
        Method::new_native("java.util.Locale".to_string(), "getLanguage".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn getCountry() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(country_id)) = obj.fields.get("country") {
                        if let Some(country_obj) = jvm.heap.get(*country_id) {
                            if let Some(s) = &country_obj.string_value {
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
        Method::new_native("java.util.Locale".to_string(), "getCountry".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.Locale", Locale::getDefault());
        jvm.method_area.add_native_method("java.util.Locale", Locale::getLanguage());
        jvm.method_area.add_native_method("java.util.Locale", Locale::getCountry());
    }
}

// ========== java.util.BitSet ==========

pub struct BitSet;

impl BitSet {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let arr = HeapObject::new_array("[J".to_string(), 0);
                let arr_ref = jvm.allocate(arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("words".to_string(), Value::ArrayRef(arr_ref));
                    obj.fields.insert("size".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.BitSet".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn get() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let bit = frame.pop()?.as_int() as usize;
            let this_ref = frame.get_local(0)?;
            let mut result = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ArrayRef(words_ref)) = obj.fields.get("words") {
                        let word_idx = bit / 64;
                        let bit_idx = bit % 64;
                        if let Some(words) = jvm.heap.get(*words_ref) {
                            if let Some(elements) = &words.array_elements {
                                if word_idx < elements.len() {
                                    if let Value::Long(w) = &elements[word_idx] {
                                        result = (*w >> bit_idx) & 1 == 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(result))?;
            Ok(())
        });
        Method::new_native("java.util.BitSet".to_string(), "get".to_string(), "(I)Z".to_string(), false, Some(native_impl))
    }

    pub fn set() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let bit = frame.pop()?.as_int() as usize;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract words_ref first, then drop the borrow
                let (word_idx, bit_idx, words_ref) = if let Some(obj) = jvm.heap.get(*this_id) {
                    let wi = bit / 64;
                    let bi = bit % 64;
                    let wr = obj.fields.get("words")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (wi, bi, wr)
                } else { (0, 0, 0) };
                // Ensure the array is large enough
                if let Some(words) = jvm.heap.get_mut(words_ref) {
                    if let Some(elements) = &mut words.array_elements {
                        if word_idx >= elements.len() {
                            let new_len = word_idx + 1;
                            let mut new_elems = vec![Value::Long(0); new_len.max(10)];
                            for (i, e) in elements.iter().enumerate() {
                                if let Value::Long(v) = e { new_elems[i] = Value::Long(*v); }
                            }
                            *elements = new_elems;
                        }
                        if word_idx < elements.len() {
                            let mut val = 0i64;
                            if let Value::Long(v) = &elements[word_idx] { val = *v; }
                            val |= 1i64 << bit_idx;
                            elements[word_idx] = Value::Long(val);
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let size = word_idx * 64 + bit_idx + 1;
                    obj.fields.insert("size".to_string(), Value::Int(size as i32));
                }
            }
            Ok(())
        });
        Method::new_native("java.util.BitSet".to_string(), "set".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn clear() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let bit = frame.pop()?.as_int() as usize;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    let word_idx = bit / 64;
                    let bit_idx = bit % 64;
                    let words_ref = obj.fields.get("words")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    if let Some(words) = jvm.heap.get_mut(words_ref) {
                        if let Some(elements) = &mut words.array_elements {
                            if word_idx < elements.len() {
                                let mut val = 0i64;
                                if let Value::Long(v) = &elements[word_idx] { val = *v; }
                                val &= !(1i64 << bit_idx);
                                elements[word_idx] = Value::Long(val);
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.util.BitSet".to_string(), "clear".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn cardinality() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut count = 0;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ArrayRef(words_ref)) = obj.fields.get("words") {
                        if let Some(words) = jvm.heap.get(*words_ref) {
                            if let Some(elements) = &words.array_elements {
                                for elem in elements {
                                    if let Value::Long(v) = elem {
                                        count += v.count_ones() as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(count))?;
            Ok(())
        });
        Method::new_native("java.util.BitSet".to_string(), "cardinality".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.BitSet", BitSet::init());
        jvm.method_area.add_native_method("java.util.BitSet", BitSet::get());
        jvm.method_area.add_native_method("java.util.BitSet", BitSet::set());
        jvm.method_area.add_native_method("java.util.BitSet", BitSet::clear());
        jvm.method_area.add_native_method("java.util.BitSet", BitSet::cardinality());
    }
}

// ========== java.util.function interfaces ==========

pub struct Consumer;
pub struct Function;
pub struct Supplier;
pub struct Predicate;

impl Consumer {
    pub fn accept() -> Method {
        Method::new_native("java.util.function.Consumer".to_string(), "accept".to_string(), "(Ljava/lang/Object;)V".to_string(), false, None)
    }
    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.function.Consumer", Consumer::accept());
    }
}

impl Function {
    pub fn apply() -> Method {
        Method::new_native("java.util.function.Function".to_string(), "apply".to_string(), "(Ljava/lang/Object;)Ljava/lang/Object;".to_string(), false, None)
    }
    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.function.Function", Function::apply());
    }
}

impl Supplier {
    pub fn get() -> Method {
        Method::new_native("java.util.function.Supplier".to_string(), "get".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }
    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.function.Supplier", Supplier::get());
    }
}

impl Predicate {
    pub fn test() -> Method {
        Method::new_native("java.util.function.Predicate".to_string(), "test".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, None)
    }
    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.function.Predicate", Predicate::test());
    }
}

// ========== java.util.OptionalInt ==========

pub struct OptionalInt;

impl OptionalInt {
    pub fn empty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let opt = HeapObject::new("java.util.OptionalInt".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), Value::Int(0));
                obj.fields.insert("isPresent".to_string(), Value::Boolean(false));
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalInt".to_string(), "empty".to_string(), "()Ljava/util/OptionalInt;".to_string(), true, Some(native_impl))
    }

    pub fn of() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?.as_int();
            let opt = HeapObject::new("java.util.OptionalInt".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), Value::Int(val));
                obj.fields.insert("isPresent".to_string(), Value::Boolean(true));
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalInt".to_string(), "of".to_string(), "(I)Ljava/util/OptionalInt;".to_string(), true, Some(native_impl))
    }

    pub fn isPresent() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let present = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("isPresent")
                        .and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None })
                        .unwrap_or(false)
                } else { false }
            } else { false };
            frame.push(Value::Boolean(present))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalInt".to_string(), "isPresent".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn getAsInt() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Int(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(val))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalInt".to_string(), "getAsInt".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn orElse() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?.as_int();
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let present = obj.fields.get("isPresent")
                        .and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None })
                        .unwrap_or(false);
                    if present {
                        obj.fields.get("value")
                            .and_then(|v| if let Value::Int(v) = v { Some(*v) } else { None })
                            .unwrap_or(other)
                    } else { other }
                } else { other }
            } else { other };
            frame.push(Value::Int(val))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalInt".to_string(), "orElse".to_string(), "(I)I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.OptionalInt", OptionalInt::empty());
        jvm.method_area.add_native_method("java.util.OptionalInt", OptionalInt::of());
        jvm.method_area.add_native_method("java.util.OptionalInt", OptionalInt::isPresent());
        jvm.method_area.add_native_method("java.util.OptionalInt", OptionalInt::getAsInt());
        jvm.method_area.add_native_method("java.util.OptionalInt", OptionalInt::orElse());
    }
}

// ========== java.util.OptionalLong ==========

pub struct OptionalLong;

impl OptionalLong {
    pub fn empty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let opt = HeapObject::new("java.util.OptionalLong".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), Value::Long(0));
                obj.fields.insert("isPresent".to_string(), Value::Boolean(false));
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalLong".to_string(), "empty".to_string(), "()Ljava/util/OptionalLong;".to_string(), true, Some(native_impl))
    }

    pub fn of() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?.as_long();
            let opt = HeapObject::new("java.util.OptionalLong".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), Value::Long(val));
                obj.fields.insert("isPresent".to_string(), Value::Boolean(true));
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalLong".to_string(), "of".to_string(), "(J)Ljava/util/OptionalLong;".to_string(), true, Some(native_impl))
    }

    pub fn isPresent() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let present = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("isPresent")
                        .and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None })
                        .unwrap_or(false)
                } else { false }
            } else { false };
            frame.push(Value::Boolean(present))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalLong".to_string(), "isPresent".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn getAsLong() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Long(val))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalLong".to_string(), "getAsLong".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.OptionalLong", OptionalLong::empty());
        jvm.method_area.add_native_method("java.util.OptionalLong", OptionalLong::of());
        jvm.method_area.add_native_method("java.util.OptionalLong", OptionalLong::isPresent());
        jvm.method_area.add_native_method("java.util.OptionalLong", OptionalLong::getAsLong());
    }
}

// ========== java.util.OptionalDouble ==========

pub struct OptionalDouble;

impl OptionalDouble {
    pub fn empty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let opt = HeapObject::new("java.util.OptionalDouble".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), Value::Double(0.0));
                obj.fields.insert("isPresent".to_string(), Value::Boolean(false));
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalDouble".to_string(), "empty".to_string(), "()Ljava/util/OptionalDouble;".to_string(), true, Some(native_impl))
    }

    pub fn of() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?.as_double();
            let opt = HeapObject::new("java.util.OptionalDouble".to_string());
            let opt_ref = jvm.allocate(opt)?;
            if let Some(obj) = jvm.heap.get_mut(opt_ref) {
                obj.fields.insert("value".to_string(), Value::Double(val));
                obj.fields.insert("isPresent".to_string(), Value::Boolean(true));
            }
            frame.push(Value::ObjectRef(opt_ref))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalDouble".to_string(), "of".to_string(), "(D)Ljava/util/OptionalDouble;".to_string(), true, Some(native_impl))
    }

    pub fn isPresent() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let present = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("isPresent")
                        .and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None })
                        .unwrap_or(false)
                } else { false }
            } else { false };
            frame.push(Value::Boolean(present))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalDouble".to_string(), "isPresent".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn getAsDouble() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Double(v) = v { Some(*v) } else { None })
                        .unwrap_or(0.0)
                } else { 0.0 }
            } else { 0.0 };
            frame.push(Value::Double(val))?;
            Ok(())
        });
        Method::new_native("java.util.OptionalDouble".to_string(), "getAsDouble".to_string(), "()D".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.OptionalDouble", OptionalDouble::empty());
        jvm.method_area.add_native_method("java.util.OptionalDouble", OptionalDouble::of());
        jvm.method_area.add_native_method("java.util.OptionalDouble", OptionalDouble::isPresent());
        jvm.method_area.add_native_method("java.util.OptionalDouble", OptionalDouble::getAsDouble());
    }
}

/// Register all java.util classes with the JVM.
pub fn register_util_classes(jvm: &mut JVM) {
    ArrayList::register(jvm);
    LinkedList::register(jvm);
    Stack::register(jvm);
    HashMap::register(jvm);
    LinkedHashMap::register(jvm);
    TreeMap::register(jvm);
    HashSet::register(jvm);
    LinkedHashSet::register(jvm);
    PriorityQueue::register(jvm);
    Random::register(jvm);
    UUID::register(jvm);
    BitSet::register(jvm);
    Consumer::register(jvm);
    Function::register(jvm);
    Supplier::register(jvm);
    Predicate::register(jvm);
    Base64::register(jvm);
    Arrays::register(jvm);
    Collections::register(jvm);
    Comparator::register(jvm);
    Locale::register(jvm);
    Queue::register(jvm);
    Deque::register(jvm);
    Iterator::register(jvm);
    Iterable::register(jvm);
    Objects::register(jvm);
    Optional::register(jvm);
    OptionalInt::register(jvm);
    OptionalLong::register(jvm);
    OptionalDouble::register(jvm);
    AtomicInteger::register(jvm);
    AtomicLong::register(jvm);
    AtomicReference::register(jvm);
    AtomicBoolean::register(jvm);
    ReentrantLock::register(jvm);
    Date::register(jvm);
    Scanner::register(jvm);
}