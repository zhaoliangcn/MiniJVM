use std::sync::Arc;
use crate::runtime::{JVM, Frame, Value, method_area::{Method, NativeImplementation}};

pub struct Object;

impl Object {
    pub fn get_class() -> Method {
        Method::new_native("java.lang.Object".to_string(), "getClass".to_string(), "()Ljava/lang/Class;".to_string(), false, None)
    }

    pub fn hashCode() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(ref ref_id) = this_ref {
                frame.push(Value::Int(*ref_id as i32))?;
            } else {
                frame.push(Value::Int(0))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "hashCode".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn equals() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let result = matches!((&this_ref, &other), (Value::ObjectRef(a), Value::ObjectRef(b)) if a == b);
            frame.push(Value::Boolean(result))?;
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "equals".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(ref ref_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*ref_id) {
                    let s = format!("{}@{}", obj.class_name, ref_id);
                    let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                    let str_ref = jvm.heap.allocate(str_obj)?;
                    frame.push(Value::ObjectRef(str_ref))?;
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn clone() -> Method {
        Method::new_native("java.lang.Object".to_string(), "clone".to_string(), "()Ljava/lang/Object;".to_string(), false, None)
    }

    pub fn notify() -> Method {
        Method::new_native("java.lang.Object".to_string(), "notify".to_string(), "()V".to_string(), false, None)
    }

    pub fn notifyAll() -> Method {
        Method::new_native("java.lang.Object".to_string(), "notifyAll".to_string(), "()V".to_string(), false, None)
    }

    pub fn wait() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "()V".to_string(), false, None)
    }

    pub fn wait_timeout() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(J)V".to_string(), false, None)
    }

    pub fn wait_nanos() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(JI)V".to_string(), false, None)
    }

    pub fn finalize() -> Method {
        Method::new_native("java.lang.Object".to_string(), "finalize".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Object", Object::get_class());
        jvm.method_area.add_native_method("java.lang.Object", Object::hashCode());
        jvm.method_area.add_native_method("java.lang.Object", Object::equals());
        jvm.method_area.add_native_method("java.lang.Object", Object::toString());
        jvm.method_area.add_native_method("java.lang.Object", Object::clone());
        jvm.method_area.add_native_method("java.lang.Object", Object::notify());
        jvm.method_area.add_native_method("java.lang.Object", Object::notifyAll());
        jvm.method_area.add_native_method("java.lang.Object", Object::wait());
        jvm.method_area.add_native_method("java.lang.Object", Object::wait_timeout());
        jvm.method_area.add_native_method("java.lang.Object", Object::wait_nanos());
        jvm.method_area.add_native_method("java.lang.Object", Object::finalize());
    }
}

pub struct String;

impl String {
    pub fn length() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(ref ref_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*ref_id) {
                    if let Some(s) = &obj.string_value {
                        frame.push(Value::Int(s.len() as i32))?;
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "length".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn charAt() -> Method {
        Method::new_native("java.lang.String".to_string(), "charAt".to_string(), "(I)C".to_string(), false, None)
    }

    pub fn getBytes() -> Method {
        Method::new_native("java.lang.String".to_string(), "getBytes".to_string(), "()[B".to_string(), false, None)
    }

    pub fn getBytes_charset() -> Method {
        Method::new_native("java.lang.String".to_string(), "getBytes".to_string(), "(Ljava/lang/String;)[B".to_string(), false, None)
    }

    pub fn equals() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let other = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            if let (Value::ObjectRef(ref this_id), Value::ObjectRef(ref other_id)) = (this_ref, other) {
                if let (Some(this_obj), Some(other_obj)) = (jvm.heap.get(*this_id), jvm.heap.get(*other_id)) {
                    if let (Some(this_str), Some(other_str)) = (&this_obj.string_value, &other_obj.string_value) {
                        frame.push(Value::Boolean(this_str == other_str))?;
                        return Ok(());
                    }
                }
            }
            frame.push(Value::Boolean(false))?;
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "equals".to_string(), "(Ljava/lang/Object;)Z".to_string(), false, Some(native_impl))
    }

    pub fn compareTo() -> Method {
        Method::new_native("java.lang.String".to_string(), "compareTo".to_string(), "(Ljava/lang/String;)I".to_string(), false, None)
    }

    pub fn indexOf() -> Method {
        Method::new_native("java.lang.String".to_string(), "indexOf".to_string(), "(Ljava/lang/String;)I".to_string(), false, None)
    }

    pub fn substring() -> Method {
        Method::new_native("java.lang.String".to_string(), "substring".to_string(), "(II)Ljava/lang/String;".to_string(), false, None)
    }

    pub fn concat() -> Method {
        Method::new_native("java.lang.String".to_string(), "concat".to_string(), "(Ljava/lang/String;)Ljava/lang/String;".to_string(), false, None)
    }

    pub fn replace() -> Method {
        Method::new_native("java.lang.String".to_string(), "replace".to_string(), "(CC)Ljava/lang/String;".to_string(), false, None)
    }

    pub fn trim() -> Method {
        Method::new_native("java.lang.String".to_string(), "trim".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn toLowerCase() -> Method {
        Method::new_native("java.lang.String".to_string(), "toLowerCase".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn toUpperCase() -> Method {
        Method::new_native("java.lang.String".to_string(), "toUpperCase".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn valueOf_int() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let v = frame.pop()?;
            if let Value::Int(i) = v {
                let s = i.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(I)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn valueOf_long() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let v = frame.pop()?;
            if let Value::Long(l) = v {
                let s = l.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(J)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn valueOf_float() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let v = frame.pop()?;
            if let Value::Float(f) = v {
                let s = f.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(F)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn valueOf_double() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let v = frame.pop()?;
            if let Value::Double(d) = v {
                let s = d.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(D)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn valueOf_bool() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let v = frame.pop()?;
            if let Value::Boolean(b) = v {
                let s = b.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(Z)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn valueOf_obj() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(Ljava/lang/Object;)Ljava/lang/String;".to_string(), true, None)
    }

    pub fn format() -> Method {
        Method::new_native("java.lang.String".to_string(), "format".to_string(), "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;".to_string(), true, None)
    }

    pub fn startsWith() -> Method {
        Method::new_native("java.lang.String".to_string(), "startsWith".to_string(), "(Ljava/lang/String;)Z".to_string(), false, None)
    }

    pub fn endsWith() -> Method {
        Method::new_native("java.lang.String".to_string(), "endsWith".to_string(), "(Ljava/lang/String;)Z".to_string(), false, None)
    }

    pub fn contains() -> Method {
        Method::new_native("java.lang.String".to_string(), "contains".to_string(), "(Ljava/lang/CharSequence;)Z".to_string(), false, None)
    }

    pub fn isEmpty() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(ref ref_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*ref_id) {
                    if let Some(s) = &obj.string_value {
                        frame.push(Value::Boolean(s.is_empty()))?;
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.String".to_string(), "isEmpty".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn split() -> Method {
        Method::new_native("java.lang.String".to_string(), "split".to_string(), "(Ljava/lang/String;)[Ljava/lang/String;".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.String", String::length());
        jvm.method_area.add_native_method("java.lang.String", String::charAt());
        jvm.method_area.add_native_method("java.lang.String", String::getBytes());
        jvm.method_area.add_native_method("java.lang.String", String::getBytes_charset());
        jvm.method_area.add_native_method("java.lang.String", String::compareTo());
        jvm.method_area.add_native_method("java.lang.String", String::indexOf());
        jvm.method_area.add_native_method("java.lang.String", String::substring());
        jvm.method_area.add_native_method("java.lang.String", String::concat());
        jvm.method_area.add_native_method("java.lang.String", String::replace());
        jvm.method_area.add_native_method("java.lang.String", String::toLowerCase());
        jvm.method_area.add_native_method("java.lang.String", String::toUpperCase());
        jvm.method_area.add_native_method("java.lang.String", String::trim());
        jvm.method_area.add_native_method("java.lang.String", String::valueOf_int());
        jvm.method_area.add_native_method("java.lang.String", String::valueOf_long());
        jvm.method_area.add_native_method("java.lang.String", String::valueOf_float());
        jvm.method_area.add_native_method("java.lang.String", String::valueOf_double());
        jvm.method_area.add_native_method("java.lang.String", String::valueOf_bool());
        jvm.method_area.add_native_method("java.lang.String", String::valueOf_obj());
        jvm.method_area.add_native_method("java.lang.String", String::format());
        jvm.method_area.add_native_method("java.lang.String", String::startsWith());
        jvm.method_area.add_native_method("java.lang.String", String::endsWith());
        jvm.method_area.add_native_method("java.lang.String", String::contains());
        jvm.method_area.add_native_method("java.lang.String", String::isEmpty());
        jvm.method_area.add_native_method("java.lang.String", String::split());
    }
}

pub struct System;

impl System {
    pub fn arraycopy() -> Method {
        Method::new_native("java.lang.System".to_string(), "arraycopy".to_string(), "(Ljava/lang/Object;ILjava/lang/Object;II)V".to_string(), true, None)
    }

    pub fn currentTimeMillis() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as i64;
            frame.push(Value::Long(now))?;
            Ok(())
        });
        Method::new_native("java.lang.System".to_string(), "currentTimeMillis".to_string(), "()J".to_string(), true, Some(native_impl))
    }

    pub fn nanoTime() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as i64;
            frame.push(Value::Long(now))?;
            Ok(())
        });
        Method::new_native("java.lang.System".to_string(), "nanoTime".to_string(), "()J".to_string(), true, Some(native_impl))
    }

    pub fn identityHashCode() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let obj = frame.pop()?;
            if let Value::ObjectRef(ref_id) = obj {
                frame.push(Value::Int(ref_id as i32))?;
            } else {
                frame.push(Value::Int(0))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.System".to_string(), "identityHashCode".to_string(), "(Ljava/lang/Object;)I".to_string(), true, Some(native_impl))
    }

    pub fn setErr() -> Method {
        Method::new_native("java.lang.System".to_string(), "setErr".to_string(), "(Ljava/io/PrintStream;)V".to_string(), true, None)
    }

    pub fn setIn() -> Method {
        Method::new_native("java.lang.System".to_string(), "setIn".to_string(), "(Ljava/io/InputStream;)V".to_string(), true, None)
    }

    pub fn setOut() -> Method {
        Method::new_native("java.lang.System".to_string(), "setOut".to_string(), "(Ljava/io/PrintStream;)V".to_string(), true, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.System", System::arraycopy());
        jvm.method_area.add_native_method("java.lang.System", System::currentTimeMillis());
        jvm.method_area.add_native_method("java.lang.System", System::nanoTime());
        jvm.method_area.add_native_method("java.lang.System", System::identityHashCode());
        jvm.method_area.add_native_method("java.lang.System", System::setErr());
        jvm.method_area.add_native_method("java.lang.System", System::setIn());
        jvm.method_area.add_native_method("java.lang.System", System::setOut());
    }
}

pub struct Thread;

impl Thread {
    pub fn start() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "start".to_string(), "()V".to_string(), false, None)
    }

    pub fn run() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "run".to_string(), "()V".to_string(), false, None)
    }

    pub fn sleep() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "sleep".to_string(), "(J)V".to_string(), true, None)
    }

    pub fn join() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "join".to_string(), "()V".to_string(), false, None)
    }

    pub fn r#yield() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "yield".to_string(), "()V".to_string(), true, None)
    }

    pub fn currentThread() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "currentThread".to_string(), "()Ljava/lang/Thread;".to_string(), true, None)
    }

    pub fn getName() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getName".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn setName() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "setName".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn getPriority() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getPriority".to_string(), "()I".to_string(), false, None)
    }

    pub fn setPriority() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "setPriority".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn getId() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getId".to_string(), "()J".to_string(), false, None)
    }

    pub fn getState() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getState".to_string(), "()Ljava/lang/Thread$State;".to_string(), false, None)
    }

    pub fn interrupt() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "interrupt".to_string(), "()V".to_string(), false, None)
    }

    pub fn isInterrupted() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "isInterrupted".to_string(), "()Z".to_string(), false, None)
    }

    pub fn interrupted() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "interrupted".to_string(), "()Z".to_string(), true, None)
    }

    pub fn isAlive() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "isAlive".to_string(), "()Z".to_string(), false, None)
    }
}

pub struct Throwable;

impl Throwable {
    pub fn getMessage() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getMessage".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn getLocalizedMessage() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getLocalizedMessage".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn toString() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn printStackTrace() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            println!("java.lang.Throwable.printStackTrace()");
            Ok(())
        });
        Method::new_native("java.lang.Throwable".to_string(), "printStackTrace".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn fillInStackTrace() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "fillInStackTrace".to_string(), "()Ljava/lang/Throwable;".to_string(), false, None)
    }

    pub fn getCause() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getCause".to_string(), "()Ljava/lang/Throwable;".to_string(), false, None)
    }

    pub fn initCause() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "initCause".to_string(), "(Ljava/lang/Throwable;)Ljava/lang/Throwable;".to_string(), false, None)
    }
}
