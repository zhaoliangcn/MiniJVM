use std::sync::Arc;
use std::string::String as StdString;
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
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(obj_id) = &this_ref {
                if let Some(obj) = jvm.heap.get_mut(*obj_id) {
                    let current_thread_id = 1;
                    if obj.monitor_owner != Some(current_thread_id) {
                        return Err(crate::error::JvmError::ThreadingError(
                            crate::error::ThreadingError::IllegalMonitorState
                        ));
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "notify".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn notifyAll() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(obj_id) = &this_ref {
                if let Some(obj) = jvm.heap.get_mut(*obj_id) {
                    let current_thread_id = 1;
                    if obj.monitor_owner != Some(current_thread_id) {
                        return Err(crate::error::JvmError::ThreadingError(
                            crate::error::ThreadingError::IllegalMonitorState
                        ));
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "notifyAll".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn wait() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(obj_id) = &this_ref {
                if let Some(obj) = jvm.heap.get_mut(*obj_id) {
                    let current_thread_id = 1;
                    if obj.monitor_owner != Some(current_thread_id) {
                        return Err(crate::error::JvmError::ThreadingError(
                            crate::error::ThreadingError::IllegalMonitorState
                        ));
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn wait_timeout() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(obj_id) = &this_ref {
                if let Some(obj) = jvm.heap.get_mut(*obj_id) {
                    let current_thread_id = 1;
                    if obj.monitor_owner != Some(current_thread_id) {
                        return Err(crate::error::JvmError::ThreadingError(
                            crate::error::ThreadingError::IllegalMonitorState
                        ));
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(J)V".to_string(), false, Some(native_impl))
    }

    pub fn wait_nanos() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(obj_id) = &this_ref {
                if let Some(obj) = jvm.heap.get_mut(*obj_id) {
                    let current_thread_id = 1;
                    if obj.monitor_owner != Some(current_thread_id) {
                        return Err(crate::error::JvmError::ThreadingError(
                            crate::error::ThreadingError::IllegalMonitorState
                        ));
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(JI)V".to_string(), false, Some(native_impl))
    }

    pub fn finalize() -> Method {
        Method::new_native("java.lang.Object".to_string(), "finalize".to_string(), "()V".to_string(), false, None)
    }

    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|_frame, _jvm| {
            Ok(())
        });
        Method::new_native("java.lang.Object".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Object", Object::init());
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
            let other = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            if let (Value::ObjectRef(this_id), Value::ObjectRef(other_id)) = (this_ref, other) {
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
        jvm.method_area.add_native_method("java.lang.String", String::equals());
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

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Thread", Thread::start());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::run());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::sleep());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::join());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::r#yield());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::currentThread());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::getName());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::setName());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::getPriority());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::setPriority());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::getId());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::getState());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::interrupt());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::isInterrupted());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::interrupted());
        jvm.method_area.add_native_method("java.lang.Thread", Thread::isAlive());
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

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Throwable", Throwable::getMessage());
        jvm.method_area.add_native_method("java.lang.Throwable", Throwable::getLocalizedMessage());
        jvm.method_area.add_native_method("java.lang.Throwable", Throwable::toString());
        jvm.method_area.add_native_method("java.lang.Throwable", Throwable::printStackTrace());
        jvm.method_area.add_native_method("java.lang.Throwable", Throwable::fillInStackTrace());
        jvm.method_area.add_native_method("java.lang.Throwable", Throwable::getCause());
        jvm.method_area.add_native_method("java.lang.Throwable", Throwable::initCause());
    }
}

pub struct StringBuilder;

impl StringBuilder {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value = Some(StdString::new());
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn init_with_capacity() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value = Some(StdString::new());
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "<init>".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn init_with_string() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            let s = if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    str_obj.string_value.clone().unwrap_or_default()
                } else {
                    StdString::new()
                }
            } else {
                StdString::new()
            };
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value = Some(s);
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn append_string() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            let s = if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    str_obj.string_value.clone().unwrap_or_default()
                } else {
                    StdString::new()
                }
            } else {
                StdString::new()
            };
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value.get_or_insert_with(StdString::new).push_str(&s);
                }
            }
            
            frame.push(this_ref.clone())?;
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "append".to_string(), "(Ljava/lang/String;)Ljava/lang/StringBuilder;".to_string(), false, Some(native_impl))
    }

    pub fn append_int() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            let s = if let Value::Int(i) = val {
                i.to_string()
            } else {
                StdString::new()
            };
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value.get_or_insert_with(StdString::new).push_str(&s);
                }
            }
            
            frame.push(this_ref.clone())?;
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "append".to_string(), "(I)Ljava/lang/StringBuilder;".to_string(), false, Some(native_impl))
    }

    pub fn append_long() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            let s = if let Value::Long(l) = val {
                l.to_string()
            } else {
                StdString::new()
            };
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value.get_or_insert_with(StdString::new).push_str(&s);
                }
            }
            
            frame.push(this_ref.clone())?;
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "append".to_string(), "(J)Ljava/lang/StringBuilder;".to_string(), false, Some(native_impl))
    }

    pub fn append_double() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            let s = if let Value::Double(d) = val {
                d.to_string()
            } else {
                StdString::new()
            };
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value.get_or_insert_with(StdString::new).push_str(&s);
                }
            }
            
            frame.push(this_ref.clone())?;
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "append".to_string(), "(D)Ljava/lang/StringBuilder;".to_string(), false, Some(native_impl))
    }

    pub fn append_bool() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            let s = if let Value::Boolean(b) = val {
                b.to_string()
            } else {
                StdString::new()
            };
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value.get_or_insert_with(StdString::new).push_str(&s);
                }
            }
            
            frame.push(this_ref.clone())?;
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "append".to_string(), "(Z)Ljava/lang/StringBuilder;".to_string(), false, Some(native_impl))
    }

    pub fn append_char() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            if let Value::Char(c) = val {
                if let Value::ObjectRef(this_id) = this_ref {
                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                        if let Some(ch) = char::from_u32(*c as u32) {
                            obj.string_value.get_or_insert_with(StdString::new).push(ch);
                        }
                    }
                }
            }
            
            frame.push(this_ref.clone())?;
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "append".to_string(), "(C)Ljava/lang/StringBuilder;".to_string(), false, Some(native_impl))
    }

    pub fn append_obj() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let obj_ref = frame.get_local(1)?;
            let this_ref = frame.get_local(0)?;
            
            let s = if let Value::ObjectRef(obj_id) = obj_ref {
                if let Some(obj) = jvm.heap.get(*obj_id) {
                    obj.string_value.clone().unwrap_or_else(|| format!("{}@{}", obj.class_name, obj_id))
                } else {
                    StdString::new()
                }
            } else {
                "null".to_string()
            };
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value.get_or_insert_with(StdString::new).push_str(&s);
                }
            }
            
            frame.push(this_ref.clone())?;
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "append".to_string(), "(Ljava/lang/Object;)Ljava/lang/StringBuilder;".to_string(), false, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let s = obj.string_value.clone().unwrap_or_default();
                    let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                    let str_ref = jvm.heap.allocate(str_obj)?;
                    frame.push(Value::ObjectRef(str_ref))?;
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn length() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let len = obj.string_value.as_ref().map(|s| s.len() as i32).unwrap_or(0);
                    frame.push(Value::Int(len))?;
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.StringBuilder".to_string(), "length".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::init());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::init_with_capacity());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::init_with_string());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::append_string());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::append_int());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::append_long());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::append_double());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::append_bool());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::append_char());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::append_obj());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::toString());
        jvm.method_area.add_native_method("java.lang.StringBuilder", StringBuilder::length());
    }
}

pub struct Integer;

impl Integer {
    pub fn parseInt() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    if let Some(s) = &str_obj.string_value {
                        if let Ok(v) = s.parse::<i32>() {
                            frame.push(Value::Int(v))?;
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "parseInt".to_string(), "(Ljava/lang/String;)I".to_string(), true, Some(native_impl))
    }

    pub fn parseInt_radix() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let radix = frame.get_local(1)?;
            let str_ref = frame.get_local(0)?;
            
            let radix_val = if let Value::Int(r) = radix { *r } else { 10 };
            
            if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    if let Some(s) = &str_obj.string_value {
                        if let Ok(v) = i32::from_str_radix(s, radix_val as u32) {
                            frame.push(Value::Int(v))?;
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "parseInt".to_string(), "(Ljava/lang/String;I)I".to_string(), true, Some(native_impl))
    }

    pub fn valueOf_int() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(0)?;
            if let Value::Int(i) = val {
                let s = i.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "valueOf".to_string(), "(I)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }
    
    pub fn valueOf_int_obj() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(0)?;
            if let Value::Int(ref i) = val {
                let mut obj = crate::runtime::heap::HeapObject::new("java.lang.Integer".to_string());
                obj.fields.insert("value:I".to_string(), Value::Int(*i));
                let obj_ref = jvm.heap.allocate(obj)?;
                frame.push(Value::ObjectRef(obj_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "valueOf".to_string(), "(I)Ljava/lang/Integer;".to_string(), true, Some(native_impl))
    }

    pub fn valueOf_string() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    if let Some(s) = &str_obj.string_value {
                        if let Ok(v) = s.parse::<i32>() {
                            let result = Method::new_native(
                                "java.lang.Integer".to_string(),
                                "valueOf".to_string(),
                                "(I)Ljava/lang/Integer;".to_string(),
                                true,
                                None
                            );
                            let obj = crate::runtime::heap::HeapObject::new("java.lang.Integer".to_string());
                            let obj_ref = jvm.heap.allocate(obj)?;
                            frame.push(Value::ObjectRef(obj_ref))?;
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "valueOf".to_string(), "(Ljava/lang/String;)Ljava/lang/Integer;".to_string(), true, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(0)?;
            if let Value::Int(i) = val {
                let s = i.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "toString".to_string(), "(I)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn toHexString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(0)?;
            if let Value::Int(i) = val {
                let s = format!("{:x}", i);
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "toHexString".to_string(), "(I)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn MAX_VALUE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Int(i32::MAX))?;
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "MAX_VALUE".to_string(), "()I".to_string(), true, Some(native_impl))
    }

    pub fn MIN_VALUE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Int(i32::MIN))?;
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "MIN_VALUE".to_string(), "()I".to_string(), true, Some(native_impl))
    }

    pub fn SIZE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Int(32))?;
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "SIZE".to_string(), "()I".to_string(), true, Some(native_impl))
    }
    
    pub fn intValue() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(obj_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*obj_id) {
                    if let Some(val) = obj.get_field("value:I") {
                        if let Value::Int(i) = val {
                            frame.push(Value::Int(*i))?;
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Integer".to_string(), "intValue".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Integer", Integer::parseInt());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::parseInt_radix());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::valueOf_int());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::valueOf_int_obj());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::valueOf_string());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::toString());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::toHexString());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::MAX_VALUE());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::MIN_VALUE());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::SIZE());
        jvm.method_area.add_native_method("java.lang.Integer", Integer::intValue());
    }
}

pub struct Long;

impl Long {
    pub fn parseLong() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    if let Some(s) = &str_obj.string_value {
                        if let Ok(v) = s.parse::<i64>() {
                            frame.push(Value::Long(v))?;
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Long".to_string(), "parseLong".to_string(), "(Ljava/lang/String;)J".to_string(), true, Some(native_impl))
    }

    pub fn valueOf() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    if let Some(s) = &str_obj.string_value {
                        if let Ok(v) = s.parse::<i64>() {
                            let obj = crate::runtime::heap::HeapObject::new("java.lang.Long".to_string());
                            let obj_ref = jvm.heap.allocate(obj)?;
                            frame.push(Value::ObjectRef(obj_ref))?;
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Long".to_string(), "valueOf".to_string(), "(Ljava/lang/String;)Ljava/lang/Long;".to_string(), true, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(0)?;
            if let Value::Long(l) = val {
                let s = l.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Long".to_string(), "toString".to_string(), "(J)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn MAX_VALUE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Long(i64::MAX))?;
            Ok(())
        });
        Method::new_native("java.lang.Long".to_string(), "MAX_VALUE".to_string(), "()J".to_string(), true, Some(native_impl))
    }

    pub fn MIN_VALUE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Long(i64::MIN))?;
            Ok(())
        });
        Method::new_native("java.lang.Long".to_string(), "MIN_VALUE".to_string(), "()J".to_string(), true, Some(native_impl))
    }

    pub fn SIZE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Int(64))?;
            Ok(())
        });
        Method::new_native("java.lang.Long".to_string(), "SIZE".to_string(), "()I".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Long", Long::parseLong());
        jvm.method_area.add_native_method("java.lang.Long", Long::valueOf());
        jvm.method_area.add_native_method("java.lang.Long", Long::toString());
        jvm.method_area.add_native_method("java.lang.Long", Long::MAX_VALUE());
        jvm.method_area.add_native_method("java.lang.Long", Long::MIN_VALUE());
        jvm.method_area.add_native_method("java.lang.Long", Long::SIZE());
    }
}

pub struct Double;

impl Double {
    pub fn parseDouble() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    if let Some(s) = &str_obj.string_value {
                        if let Ok(v) = s.parse::<f64>() {
                            frame.push(Value::Double(v))?;
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Double".to_string(), "parseDouble".to_string(), "(Ljava/lang/String;)D".to_string(), true, Some(native_impl))
    }

    pub fn valueOf() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let str_ref = frame.get_local(0)?;
            
            if let Value::ObjectRef(str_id) = str_ref {
                if let Some(str_obj) = jvm.heap.get(*str_id) {
                    if let Some(s) = &str_obj.string_value {
                        if let Ok(v) = s.parse::<f64>() {
                            let obj = crate::runtime::heap::HeapObject::new("java.lang.Double".to_string());
                            let obj_ref = jvm.heap.allocate(obj)?;
                            frame.push(Value::ObjectRef(obj_ref))?;
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.lang.Double".to_string(), "valueOf".to_string(), "(Ljava/lang/String;)Ljava/lang/Double;".to_string(), true, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let val = frame.get_local(0)?;
            if let Value::Double(d) = val {
                let s = d.to_string();
                let str_obj = crate::runtime::heap::HeapObject::new_string("java.lang.String".to_string(), s);
                let str_ref = jvm.heap.allocate(str_obj)?;
                frame.push(Value::ObjectRef(str_ref))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Double".to_string(), "toString".to_string(), "(D)Ljava/lang/String;".to_string(), true, Some(native_impl))
    }

    pub fn MAX_VALUE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Double(f64::MAX))?;
            Ok(())
        });
        Method::new_native("java.lang.Double".to_string(), "MAX_VALUE".to_string(), "()D".to_string(), true, Some(native_impl))
    }

    pub fn MIN_VALUE() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            frame.push(Value::Double(f64::MIN))?;
            Ok(())
        });
        Method::new_native("java.lang.Double".to_string(), "MIN_VALUE".to_string(), "()D".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Double", Double::parseDouble());
        jvm.method_area.add_native_method("java.lang.Double", Double::valueOf());
        jvm.method_area.add_native_method("java.lang.Double", Double::toString());
        jvm.method_area.add_native_method("java.lang.Double", Double::MAX_VALUE());
        jvm.method_area.add_native_method("java.lang.Double", Double::MIN_VALUE());
    }
}

pub struct Math;

impl Math {
    pub fn abs_int() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let val = frame.pop()?;
            if let Value::Int(i) = val {
                frame.push(Value::Int(i.abs()))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "abs".to_string(), "(I)I".to_string(), true, Some(native_impl))
    }

    pub fn abs_long() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let val = frame.pop()?;
            if let Value::Long(l) = val {
                frame.push(Value::Long(l.abs()))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "abs".to_string(), "(J)J".to_string(), true, Some(native_impl))
    }

    pub fn abs_float() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let val = frame.pop()?;
            if let Value::Float(f) = val {
                frame.push(Value::Float(f.abs()))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "abs".to_string(), "(F)F".to_string(), true, Some(native_impl))
    }

    pub fn abs_double() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let val = frame.pop()?;
            if let Value::Double(d) = val {
                frame.push(Value::Double(d.abs()))?;
            }
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "abs".to_string(), "(D)D".to_string(), true, Some(native_impl))
    }

    pub fn max_int() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let b = frame.pop()?.as_int();
            let a = frame.pop()?.as_int();
            frame.push(Value::Int(a.max(b)))?;
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "max".to_string(), "(II)I".to_string(), true, Some(native_impl))
    }

    pub fn max_long() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let b = frame.pop()?.as_long();
            let a = frame.pop()?.as_long();
            frame.push(Value::Long(a.max(b)))?;
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "max".to_string(), "(JJ)J".to_string(), true, Some(native_impl))
    }

    pub fn min_int() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let b = frame.pop()?.as_int();
            let a = frame.pop()?.as_int();
            frame.push(Value::Int(a.min(b)))?;
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "min".to_string(), "(II)I".to_string(), true, Some(native_impl))
    }

    pub fn min_long() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let b = frame.pop()?.as_long();
            let a = frame.pop()?.as_long();
            frame.push(Value::Long(a.min(b)))?;
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "min".to_string(), "(JJ)J".to_string(), true, Some(native_impl))
    }

    pub fn sqrt() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let val = frame.pop()?.as_double();
            frame.push(Value::Double(val.sqrt()))?;
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "sqrt".to_string(), "(D)D".to_string(), true, Some(native_impl))
    }

    pub fn pow() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let exp = frame.pop()?.as_double();
            let base = frame.pop()?.as_double();
            frame.push(Value::Double(base.powf(exp)))?;
            Ok(())
        });
        Method::new_native("java.lang.Math".to_string(), "pow".to_string(), "(DD)D".to_string(), true, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.lang.Math", Math::abs_int());
        jvm.method_area.add_native_method("java.lang.Math", Math::abs_long());
        jvm.method_area.add_native_method("java.lang.Math", Math::abs_float());
        jvm.method_area.add_native_method("java.lang.Math", Math::abs_double());
        jvm.method_area.add_native_method("java.lang.Math", Math::max_int());
        jvm.method_area.add_native_method("java.lang.Math", Math::max_long());
        jvm.method_area.add_native_method("java.lang.Math", Math::min_int());
        jvm.method_area.add_native_method("java.lang.Math", Math::min_long());
        jvm.method_area.add_native_method("java.lang.Math", Math::sqrt());
        jvm.method_area.add_native_method("java.lang.Math", Math::pow());
        
        jvm.method_area.set_static_field("java.lang.Math", "PI", "D", Value::Double(std::f64::consts::PI));
        jvm.method_area.set_static_field("java.lang.Math", "E", "D", Value::Double(std::f64::consts::E));
    }
}

pub struct Record;

impl Record {
    pub fn register(jvm: &mut JVM) {
        let init_method = Method::new_native(
            "java.lang.Record".to_string(),
            "<init>".to_string(),
            "()V".to_string(),
            false,
            None
        );
        jvm.method_area.add_native_method("java.lang.Record", init_method);
    }
}

pub fn register_standard_classes(jvm: &mut JVM) {
    Object::register(jvm);
    Record::register(jvm);
    String::register(jvm);
    StringBuilder::register(jvm);
    Integer::register(jvm);
    Long::register(jvm);
    Double::register(jvm);
    Math::register(jvm);
    System::register(jvm);
    Thread::register(jvm);
    Throwable::register(jvm);
    
    let exception_classes = vec![
        "java.lang.Exception",
        "java.lang.RuntimeException",
        "java.lang.NullPointerException",
        "java.lang.ArrayIndexOutOfBoundsException",
        "java.lang.ClassCastException",
        "java.lang.IllegalArgumentException",
        "java.lang.NegativeArraySizeException",
    ];
    
    for class_name in exception_classes {
        let class_name_clone = class_name.to_string();
        let init_method = Method::new_native(
            class_name_clone.clone(), 
            "<init>".to_string(), 
            "()V".to_string(), 
            false, 
            None
        );
        jvm.method_area.add_native_method(class_name, init_method);
        
        let init_with_message_impl: NativeImplementation = Arc::new(move |frame, jvm| {
            let msg_ref = frame.get_local(1)?;
            let msg_str = if let Value::ObjectRef(msg_id) = msg_ref {
                if let Some(msg_obj) = jvm.heap.get(*msg_id) {
                    msg_obj.string_value.clone()
                } else {
                    None
                }
            } else {
                None
            };
            
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.string_value = msg_str;
                }
            }
            Ok(())
        });
        
        let init_with_message = Method::new_native(
            class_name.to_string(), 
            "<init>".to_string(), 
            "(Ljava/lang/String;)V".to_string(), 
            false, 
            Some(init_with_message_impl)
        );
        jvm.method_area.add_native_method(class_name, init_with_message);
    }
}
