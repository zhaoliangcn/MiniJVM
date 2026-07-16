use std::sync::Arc;
use crate::error::{JvmError, RuntimeError, Result};
use crate::runtime::{JVM, Frame, Value, HeapObject, method_area::{Method, NativeImplementation}};

// ========== java.math.BigInteger ==========

pub struct BigInteger;

impl BigInteger {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Store the integer value
                let int_val = val.as_long();
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Long(int_val));
                }
            }
            Ok(())
        });
        Method::new_native("java.math.BigInteger".to_string(), "<init>".to_string(), "(J)V".to_string(), false, Some(native_impl))
    }

    pub fn init_string() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let int_val = if let Value::ObjectRef(str_id) = str_ref {
                    if let Some(str_obj) = jvm.heap.get(str_id) {
                        if let Some(s) = &str_obj.string_value {
                            s.parse::<i64>().unwrap_or(0)
                        } else { 0 }
                    } else { 0 }
                } else { 0 };
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Long(int_val));
                }
            }
            Ok(())
        });
        Method::new_native("java.math.BigInteger".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn add() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let a = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let b = if let Value::ObjectRef(other_id) = other {
                if let Some(obj) = jvm.heap.get(other_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let result = a + b;
            let obj = HeapObject::new("java.math.BigInteger".to_string());
            let obj_ref = jvm.allocate(obj)?;
            if let Some(o) = jvm.heap.get_mut(obj_ref) {
                o.fields.insert("value".to_string(), Value::Long(result));
            }
            frame.push(Value::ObjectRef(obj_ref))?;
            Ok(())
        });
        Method::new_native("java.math.BigInteger".to_string(), "add".to_string(), "(Ljava/math/BigInteger;)Ljava/math/BigInteger;".to_string(), false, Some(native_impl))
    }

    pub fn subtract() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let a = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let b = if let Value::ObjectRef(other_id) = other {
                if let Some(obj) = jvm.heap.get(other_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let result = a - b;
            let obj = HeapObject::new("java.math.BigInteger".to_string());
            let obj_ref = jvm.allocate(obj)?;
            if let Some(o) = jvm.heap.get_mut(obj_ref) {
                o.fields.insert("value".to_string(), Value::Long(result));
            }
            frame.push(Value::ObjectRef(obj_ref))?;
            Ok(())
        });
        Method::new_native("java.math.BigInteger".to_string(), "subtract".to_string(), "(Ljava/math/BigInteger;)Ljava/math/BigInteger;".to_string(), false, Some(native_impl))
    }

    pub fn multiply() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let a = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let b = if let Value::ObjectRef(other_id) = other {
                if let Some(obj) = jvm.heap.get(other_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let result = a * b;
            let obj = HeapObject::new("java.math.BigInteger".to_string());
            let obj_ref = jvm.allocate(obj)?;
            if let Some(o) = jvm.heap.get_mut(obj_ref) {
                o.fields.insert("value".to_string(), Value::Long(result));
            }
            frame.push(Value::ObjectRef(obj_ref))?;
            Ok(())
        });
        Method::new_native("java.math.BigInteger".to_string(), "multiply".to_string(), "(Ljava/math/BigInteger;)Ljava/math/BigInteger;".to_string(), false, Some(native_impl))
    }

    pub fn divide() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let a = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let b = if let Value::ObjectRef(other_id) = other {
                if let Some(obj) = jvm.heap.get(other_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            if b == 0 { return Err(JvmError::RuntimeError(RuntimeError::ArithmeticException)); }
            let result = a / b;
            let obj = HeapObject::new("java.math.BigInteger".to_string());
            let obj_ref = jvm.allocate(obj)?;
            if let Some(o) = jvm.heap.get_mut(obj_ref) {
                o.fields.insert("value".to_string(), Value::Long(result));
            }
            frame.push(Value::ObjectRef(obj_ref))?;
            Ok(())
        });
        Method::new_native("java.math.BigInteger".to_string(), "divide".to_string(), "(Ljava/math/BigInteger;)Ljava/math/BigInteger;".to_string(), false, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Long(v) = v { Some(*v) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            let s = val.to_string();
            let obj = HeapObject::new_string("java.lang.String".to_string(), s);
            let ref_id = jvm.allocate(obj)?;
            frame.push(Value::ObjectRef(ref_id))?;
            Ok(())
        });
        Method::new_native("java.math.BigInteger".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn longValue() -> Method {
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
        Method::new_native("java.math.BigInteger".to_string(), "longValue".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::init());
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::init_string());
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::add());
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::subtract());
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::multiply());
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::divide());
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::toString());
        jvm.method_area.add_native_method("java.math.BigInteger", BigInteger::longValue());
    }
}

// ========== java.math.BigDecimal ==========

pub struct BigDecimal;

impl BigDecimal {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let double_val = val.as_double();
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Double(double_val));
                }
            }
            Ok(())
        });
        Method::new_native("java.math.BigDecimal".to_string(), "<init>".to_string(), "(D)V".to_string(), false, Some(native_impl))
    }

    pub fn init_string() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let double_val = if let Value::ObjectRef(str_id) = str_ref {
                    if let Some(str_obj) = jvm.heap.get(str_id) {
                        if let Some(s) = &str_obj.string_value {
                            s.parse::<f64>().unwrap_or(0.0)
                        } else { 0.0 }
                    } else { 0.0 }
                } else { 0.0 };
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("value".to_string(), Value::Double(double_val));
                }
            }
            Ok(())
        });
        Method::new_native("java.math.BigDecimal".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn add() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let a = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Double(v) = v { Some(*v) } else { None })
                        .unwrap_or(0.0)
                } else { 0.0 }
            } else { 0.0 };
            let b = if let Value::ObjectRef(other_id) = other {
                if let Some(obj) = jvm.heap.get(other_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Double(v) = v { Some(*v) } else { None })
                        .unwrap_or(0.0)
                } else { 0.0 }
            } else { 0.0 };
            let result = a + b;
            let obj = HeapObject::new("java.math.BigDecimal".to_string());
            let obj_ref = jvm.allocate(obj)?;
            if let Some(o) = jvm.heap.get_mut(obj_ref) {
                o.fields.insert("value".to_string(), Value::Double(result));
            }
            frame.push(Value::ObjectRef(obj_ref))?;
            Ok(())
        });
        Method::new_native("java.math.BigDecimal".to_string(), "add".to_string(), "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;".to_string(), false, Some(native_impl))
    }

    pub fn multiply() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let a = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Double(v) = v { Some(*v) } else { None })
                        .unwrap_or(0.0)
                } else { 0.0 }
            } else { 0.0 };
            let b = if let Value::ObjectRef(other_id) = other {
                if let Some(obj) = jvm.heap.get(other_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Double(v) = v { Some(*v) } else { None })
                        .unwrap_or(0.0)
                } else { 0.0 }
            } else { 0.0 };
            let result = a * b;
            let obj = HeapObject::new("java.math.BigDecimal".to_string());
            let obj_ref = jvm.allocate(obj)?;
            if let Some(o) = jvm.heap.get_mut(obj_ref) {
                o.fields.insert("value".to_string(), Value::Double(result));
            }
            frame.push(Value::ObjectRef(obj_ref))?;
            Ok(())
        });
        Method::new_native("java.math.BigDecimal".to_string(), "multiply".to_string(), "(Ljava/math/BigDecimal;)Ljava/math/BigDecimal;".to_string(), false, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let val = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("value")
                        .and_then(|v| if let Value::Double(v) = v { Some(*v) } else { None })
                        .unwrap_or(0.0)
                } else { 0.0 }
            } else { 0.0 };
            let s = format!("{:.2}", val);
            let obj = HeapObject::new_string("java.lang.String".to_string(), s);
            let ref_id = jvm.allocate(obj)?;
            frame.push(Value::ObjectRef(ref_id))?;
            Ok(())
        });
        Method::new_native("java.math.BigDecimal".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn doubleValue() -> Method {
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
        Method::new_native("java.math.BigDecimal".to_string(), "doubleValue".to_string(), "()D".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.math.BigDecimal", BigDecimal::init());
        jvm.method_area.add_native_method("java.math.BigDecimal", BigDecimal::init_string());
        jvm.method_area.add_native_method("java.math.BigDecimal", BigDecimal::add());
        jvm.method_area.add_native_method("java.math.BigDecimal", BigDecimal::multiply());
        jvm.method_area.add_native_method("java.math.BigDecimal", BigDecimal::toString());
        jvm.method_area.add_native_method("java.math.BigDecimal", BigDecimal::doubleValue());
    }
}

/// Register all java.math classes with the JVM.
pub fn register_math_classes(jvm: &mut JVM) {
    BigInteger::register(jvm);
    BigDecimal::register(jvm);
}