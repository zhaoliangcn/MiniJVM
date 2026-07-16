use std::sync::Arc;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::path::Path;
use crate::error::{JvmError, RuntimeError, Result};
use crate::runtime::{JVM, Frame, Value, HeapObject, method_area::{Method, NativeImplementation}};

pub struct InputStream;

impl InputStream {
    pub fn read() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn read_bytes() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "read".to_string(), "([B)I".to_string(), false, None)
    }

    pub fn read_bytes_offset() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "read".to_string(), "([BII)I".to_string(), false, None)
    }

    pub fn skip() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "skip".to_string(), "(J)J".to_string(), false, None)
    }

    pub fn available() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "available".to_string(), "()I".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn mark() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "mark".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn reset() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "reset".to_string(), "()V".to_string(), false, None)
    }

    pub fn markSupported() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "markSupported".to_string(), "()Z".to_string(), false, None)
    }
}

pub struct OutputStream;

impl OutputStream {
    pub fn write() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "write".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn write_bytes() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "write".to_string(), "([B)V".to_string(), false, None)
    }

    pub fn write_bytes_offset() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "write".to_string(), "([BII)V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }
}

pub struct PrintStream;

impl PrintStream {
    fn flush_stdout() {
        std::io::stdout().flush().ok();
    }

    pub fn print_bool() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(Z)V".to_string(), false, None)
    }

    pub fn print_char() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(C)V".to_string(), false, None)
    }

    pub fn print_int() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn print_long() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(J)V".to_string(), false, None)
    }

    pub fn print_float() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(F)V".to_string(), false, None)
    }

    pub fn print_double() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(D)V".to_string(), false, None)
    }

    pub fn print_char_array() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "([C)V".to_string(), false, None)
    }

    pub fn print_string() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let s = frame.get_local(1)?;
            if let Value::ObjectRef(ref_id) = s {
                if let Some(obj) = _jvm.heap.get(*ref_id) {
                    if let Some(str_val) = &obj.string_value {
                        print!("{}", str_val);
                        Self::flush_stdout();
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn print_object() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(Ljava/lang/Object;)V".to_string(), false, None)
    }

    pub fn println_bool() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(Z)V".to_string(), false, None)
    }

    pub fn println_char() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(C)V".to_string(), false, None)
    }

    pub fn println_int() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn println_long() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(J)V".to_string(), false, None)
    }

    pub fn println_float() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(F)V".to_string(), false, None)
    }

    pub fn println_double() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(D)V".to_string(), false, None)
    }

    pub fn println_char_array() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "([C)V".to_string(), false, None)
    }

    pub fn println_string() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            let s = frame.get_local(1)?;
            if let Value::ObjectRef(ref_id) = s {
                if let Some(obj) = _jvm.heap.get(*ref_id) {
                    if let Some(str_val) = &obj.string_value {
                        print!("{}\n", str_val);
                        Self::flush_stdout();
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn println_object() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(Ljava/lang/Object;)V".to_string(), false, None)
    }

    pub fn println() -> Method {
        let native_impl: NativeImplementation = Arc::new(|_frame, _jvm| {
            print!("\n");
            Self::flush_stdout();
            Ok(())
        });
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn printf() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "printf".to_string(), "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;".to_string(), false, None)
    }

    pub fn format() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "format".to_string(), "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_bool());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_char());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_int());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_long());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_float());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_double());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_char_array());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_string());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::print_object());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_bool());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_char());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_int());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_long());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_float());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_double());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_char_array());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_string());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println_object());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::println());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::printf());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::format());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::flush());
        jvm.method_area.add_native_method("java.io.PrintStream", PrintStream::close());
    }
}

// ========== java.io.FileInputStream ==========

pub struct FileInputStream;

impl FileInputStream {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let name_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Value::ObjectRef(str_id) = name_ref {
                    if let Some(str_obj) = jvm.heap.get(str_id) {
                        if let Some(path) = &str_obj.string_value {
                            match File::open(path) {
                                Ok(file) => {
                                    // Store the file descriptor
                                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                                        obj.fields.insert("path".to_string(), Value::ObjectRef(str_id));
                                        // Store fd as a placeholder
                                        obj.fields.insert("fd".to_string(), Value::Int(0));
                                    }
                                }
                                Err(_) => {
                                    return Err(JvmError::RuntimeError(RuntimeError::NoSuchClass(
                                        format!("File not found: {}", path)
                                    )));
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.io.FileInputStream".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn read() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                // Read a single byte from the file
                                match fs::read(path) {
                                    Ok(bytes) => {
                                        // Track position via an external counter
                                        // For simplicity, read the first byte
                                        let pos = obj.fields.get("pos")
                                            .and_then(|v| if let Value::Int(p) = v { Some(*p as usize) } else { None })
                                            .unwrap_or(0);
                                        if pos < bytes.len() {
                                            let byte = bytes[pos] as i32;
                                            if let Some(obj) = jvm.heap.get_mut(*this_id) {
                                                obj.fields.insert("pos".to_string(), Value::Int((pos + 1) as i32));
                                            }
                                            frame.push(Value::Int(byte))?;
                                        } else {
                                            frame.push(Value::Int(-1))?; // EOF
                                        }
                                        return Ok(());
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(-1))?;
            Ok(())
        });
        Method::new_native("java.io.FileInputStream".to_string(), "read".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn read_bytes() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let arr_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                match fs::read(path) {
                                    Ok(bytes) => {
                                        let pos = obj.fields.get("pos")
                                            .and_then(|v| if let Value::Int(p) = v { Some(*p as usize) } else { None })
                                            .unwrap_or(0);
                                        if let Value::ArrayRef(arr_id) = arr_ref {
                                            if let Some(arr_obj) = jvm.heap.get_mut(arr_id) {
                                                if let Some(elements) = &mut arr_obj.array_elements {
                                                    let mut count = 0;
                                                    for i in 0..elements.len() {
                                                        if pos + i < bytes.len() {
                                                            elements[i] = Value::Byte(bytes[pos + i] as i8);
                                                            count += 1;
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                                                        obj.fields.insert("pos".to_string(), Value::Int((pos + count) as i32));
                                                    }
                                                    frame.push(Value::Int(count as i32))?;
                                                    return Ok(());
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(-1))?;
            Ok(())
        });
        Method::new_native("java.io.FileInputStream".to_string(), "read".to_string(), "([B)I".to_string(), false, Some(native_impl))
    }

    pub fn close() -> Method {
        let native_impl: NativeImplementation = Arc::new(|_frame, _jvm| Ok(()));
        Method::new_native("java.io.FileInputStream".to_string(), "close".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.FileInputStream", FileInputStream::init());
        jvm.method_area.add_native_method("java.io.FileInputStream", FileInputStream::read());
        jvm.method_area.add_native_method("java.io.FileInputStream", FileInputStream::read_bytes());
        jvm.method_area.add_native_method("java.io.FileInputStream", FileInputStream::close());
    }
}

// ========== java.io.FileOutputStream ==========

pub struct FileOutputStream;

impl FileOutputStream {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let name_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Value::ObjectRef(str_id) = name_ref {
                    if let Some(str_obj) = jvm.heap.get(str_id) {
                        if let Some(path) = &str_obj.string_value {
                            // Create the file (truncate if exists)
                            if let Ok(_) = File::create(path) {
                                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                                    obj.fields.insert("path".to_string(), Value::ObjectRef(str_id));
                                    obj.fields.insert("fd".to_string(), Value::Int(0));
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.io.FileOutputStream".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn write() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let byte_val = frame.get_local(1)?.as_int() as u8;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                // Append byte to file
                                if let Ok(mut file) = fs::OpenOptions::new().append(true).create(true).open(path) {
                                    use std::io::Write;
                                    let _ = file.write(&[byte_val]);
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.io.FileOutputStream".to_string(), "write".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn write_bytes() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let arr_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                if let Value::ArrayRef(arr_id) = arr_ref {
                                    if let Some(arr_obj) = jvm.heap.get(arr_id) {
                                        if let Some(elements) = &arr_obj.array_elements {
                                            let mut bytes = Vec::new();
                                            for elem in elements {
                                                match elem {
                                                    Value::Byte(b) => bytes.push(*b as u8),
                                                    Value::Int(i) => bytes.push(*i as u8),
                                                    _ => bytes.push(0),
                                                }
                                            }
                                            if let Ok(mut file) = fs::OpenOptions::new().append(true).create(true).open(path) {
                                                use std::io::Write;
                                                let _ = file.write(&bytes);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.io.FileOutputStream".to_string(), "write".to_string(), "([B)V".to_string(), false, Some(native_impl))
    }

    pub fn close() -> Method {
        let native_impl: NativeImplementation = Arc::new(|_frame, _jvm| Ok(()));
        Method::new_native("java.io.FileOutputStream".to_string(), "close".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.FileOutputStream", FileOutputStream::init());
        jvm.method_area.add_native_method("java.io.FileOutputStream", FileOutputStream::write());
        jvm.method_area.add_native_method("java.io.FileOutputStream", FileOutputStream::write_bytes());
        jvm.method_area.add_native_method("java.io.FileOutputStream", FileOutputStream::close());
    }
}

// ========== java.io.File ==========

pub struct File_;

impl File_ {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let pathname_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Value::ObjectRef(str_id) = pathname_ref {
                    if let Some(str_obj) = jvm.heap.get(str_id) {
                        if let Some(path) = &str_obj.string_value {
                            if let Some(obj) = jvm.heap.get_mut(*this_id) {
                                obj.fields.insert("path".to_string(), Value::ObjectRef(str_id));
                            }
                        }
                    }
                }
            }
            Ok(())
        });
        Method::new_native("java.io.File".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, Some(native_impl))
    }

    pub fn getPath() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                let s = HeapObject::new_string("java.lang.String".to_string(), path.clone());
                                let r = jvm.allocate(s)?;
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
        Method::new_native("java.io.File".to_string(), "getPath".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn exists() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut exists = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                exists = Path::new(path).exists();
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(exists))?;
            Ok(())
        });
        Method::new_native("java.io.File".to_string(), "exists".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn isFile() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut is_file = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                is_file = Path::new(path).is_file();
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(is_file))?;
            Ok(())
        });
        Method::new_native("java.io.File".to_string(), "isFile".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn isDirectory() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut is_dir = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                is_dir = Path::new(path).is_dir();
                            }
                        }
                    }
                }
            }
            frame.push(Value::Boolean(is_dir))?;
            Ok(())
        });
        Method::new_native("java.io.File".to_string(), "isDirectory".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn length() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut len: i64 = 0;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                if let Ok(meta) = fs::metadata(path) {
                                    len = meta.len() as i64;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Long(len))?;
            Ok(())
        });
        Method::new_native("java.io.File".to_string(), "length".to_string(), "()J".to_string(), false, Some(native_impl))
    }

    pub fn getName() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(str_id)) = obj.fields.get("path") {
                        if let Some(str_obj) = jvm.heap.get(*str_id) {
                            if let Some(path) = &str_obj.string_value {
                                let name = Path::new(path).file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(path);
                                let s = HeapObject::new_string("java.lang.String".to_string(), name.to_string());
                                let r = jvm.allocate(s)?;
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
        Method::new_native("java.io.File".to_string(), "getName".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.File", File_::init());
        jvm.method_area.add_native_method("java.io.File", File_::getPath());
        jvm.method_area.add_native_method("java.io.File", File_::exists());
        jvm.method_area.add_native_method("java.io.File", File_::isFile());
        jvm.method_area.add_native_method("java.io.File", File_::isDirectory());
        jvm.method_area.add_native_method("java.io.File", File_::length());
        jvm.method_area.add_native_method("java.io.File", File_::getName());
    }
}

/// Register all IO classes with the JVM.
pub fn register_io_classes(jvm: &mut JVM) {
    FileInputStream::register(jvm);
    FileOutputStream::register(jvm);
    File_::register(jvm);
}
