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

// ========== java.io.ByteArrayInputStream ==========

pub struct ByteArrayInputStream;

impl ByteArrayInputStream {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let buf_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("buf".to_string(), buf_ref);
                    obj.fields.insert("pos".to_string(), Value::Int(0));
                    obj.fields.insert("count".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.io.ByteArrayInputStream".to_string(), "<init>".to_string(), "([B)V".to_string(), false, Some(native_impl))
    }

    pub fn read() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let pos = obj.fields.get("pos")
                        .and_then(|v| if let Value::Int(p) = v { Some(*p as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(Value::ArrayRef(buf_id)) = obj.fields.get("buf") {
                        if let Some(buf) = jvm.heap.get(*buf_id) {
                            if let Some(elements) = &buf.array_elements {
                                if pos < elements.len() {
                                    let byte = match &elements[pos] {
                                        Value::Byte(b) => *b as i32,
                                        Value::Int(i) => *i,
                                        _ => 0,
                                    };
                                    if let Some(obj) = jvm.heap.get_mut(*this_id) {
                                        obj.fields.insert("pos".to_string(), Value::Int((pos + 1) as i32));
                                    }
                                    frame.push(Value::Int(byte))?;
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(-1))?;
            Ok(())
        });
        Method::new_native("java.io.ByteArrayInputStream".to_string(), "read".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn available() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut avail = 0;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let pos = obj.fields.get("pos")
                        .and_then(|v| if let Value::Int(p) = v { Some(*p as usize) } else { None })
                        .unwrap_or(0);
                    if let Some(Value::ArrayRef(buf_id)) = obj.fields.get("buf") {
                        if let Some(buf) = jvm.heap.get(*buf_id) {
                            if let Some(elements) = &buf.array_elements {
                                if pos < elements.len() {
                                    avail = (elements.len() - pos) as i32;
                                }
                            }
                        }
                    }
                }
            }
            frame.push(Value::Int(avail))?;
            Ok(())
        });
        Method::new_native("java.io.ByteArrayInputStream".to_string(), "available".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.ByteArrayInputStream", ByteArrayInputStream::init());
        jvm.method_area.add_native_method("java.io.ByteArrayInputStream", ByteArrayInputStream::read());
        jvm.method_area.add_native_method("java.io.ByteArrayInputStream", ByteArrayInputStream::available());
    }
}

// ========== java.io.ByteArrayOutputStream ==========

pub struct ByteArrayOutputStream;

impl ByteArrayOutputStream {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let arr = HeapObject::new_array("[B".to_string(), 0);
                let arr_ref = jvm.allocate(arr)?;
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("buf".to_string(), Value::ArrayRef(arr_ref));
                    obj.fields.insert("count".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.io.ByteArrayOutputStream".to_string(), "<init>".to_string(), "()V".to_string(), false, Some(native_impl))
    }

    pub fn write() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let byte = frame.get_local(1)?.as_int() as u8;
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                let (current_size, buf_ref) = {
                    let obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    let size = obj.fields.get("count")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let b_ref = obj.fields.get("buf")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    (size, b_ref)
                };
                let new_size = current_size + 1;
                if let Some(buf) = jvm.heap.get_mut(buf_ref) {
                    if let Some(elements) = &mut buf.array_elements {
                        if current_size >= elements.len() {
                            let mut new_elems = vec![Value::Null; (new_size * 3 / 2 + 1).max(32)];
                            for (i, e) in elements.iter().enumerate() {
                                new_elems[i] = e.clone();
                            }
                            *elements = new_elems;
                        }
                        if current_size < elements.len() {
                            elements[current_size] = Value::Byte(byte as i8);
                        }
                    }
                }
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("count".to_string(), Value::Int(new_size as i32));
                }
            }
            Ok(())
        });
        Method::new_native("java.io.ByteArrayOutputStream".to_string(), "write".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn toByteArray() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let buf_ref = obj.fields.get("buf")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let count = obj.fields.get("count")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    // Copy data from buf first
                    let mut buf_data = Vec::new();
                    if let Some(buf) = jvm.heap.get(buf_ref) {
                        if let Some(buf_elems) = &buf.array_elements {
                            for i in 0..count.min(buf_elems.len()) {
                                buf_data.push(buf_elems[i].clone());
                            }
                        }
                    }
                    // Create a new array with exact size
                    let result = HeapObject::new_array("[B".to_string(), count);
                    let result_ref = jvm.allocate(result)?;
                    if let Some(result_arr) = jvm.heap.get_mut(result_ref) {
                        if let Some(result_elems) = &mut result_arr.array_elements {
                            for i in 0..buf_data.len().min(result_elems.len()) {
                                result_elems[i] = buf_data[i].clone();
                            }
                        }
                    }
                    frame.push(Value::ArrayRef(result_ref))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.io.ByteArrayOutputStream".to_string(), "toByteArray".to_string(), "()[B".to_string(), false, Some(native_impl))
    }

    pub fn size() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let size = if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    obj.fields.get("count")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s) } else { None })
                        .unwrap_or(0)
                } else { 0 }
            } else { 0 };
            frame.push(Value::Int(size))?;
            Ok(())
        });
        Method::new_native("java.io.ByteArrayOutputStream".to_string(), "size".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn toString() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let buf_ref = obj.fields.get("buf")
                        .and_then(|v| if let Value::ArrayRef(a) = v { Some(*a) } else { None })
                        .unwrap_or(0);
                    let count = obj.fields.get("count")
                        .and_then(|v| if let Value::Int(s) = v { Some(*s as usize) } else { None })
                        .unwrap_or(0);
                    let mut bytes = Vec::new();
                    if let Some(buf) = jvm.heap.get(buf_ref) {
                        if let Some(elements) = &buf.array_elements {
                            for i in 0..count.min(elements.len()) {
                                match &elements[i] {
                                    Value::Byte(b) => bytes.push(*b as u8),
                                    Value::Int(i) => bytes.push(*i as u8),
                                    _ => bytes.push(0),
                                }
                            }
                        }
                    }
                    let s = String::from_utf8_lossy(&bytes).to_string();
                    let str_obj = HeapObject::new_string("java.lang.String".to_string(), s);
                    let str_ref = jvm.allocate(str_obj)?;
                    frame.push(Value::ObjectRef(str_ref))?;
                    return Ok(());
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.io.ByteArrayOutputStream".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.ByteArrayOutputStream", ByteArrayOutputStream::init());
        jvm.method_area.add_native_method("java.io.ByteArrayOutputStream", ByteArrayOutputStream::write());
        jvm.method_area.add_native_method("java.io.ByteArrayOutputStream", ByteArrayOutputStream::toByteArray());
        jvm.method_area.add_native_method("java.io.ByteArrayOutputStream", ByteArrayOutputStream::size());
        jvm.method_area.add_native_method("java.io.ByteArrayOutputStream", ByteArrayOutputStream::toString());
    }
}

/// Register all IO classes with the JVM.
pub fn register_io_classes(jvm: &mut JVM) {
    FileInputStream::register(jvm);
    FileOutputStream::register(jvm);
    File_::register(jvm);
    ByteArrayInputStream::register(jvm);
    ByteArrayOutputStream::register(jvm);
    BufferedInputStream::register(jvm);
    BufferedOutputStream::register(jvm);
    PrintWriter::register(jvm);
    BufferedReader::register(jvm);
    BufferedWriter::register(jvm);
    InputStreamReader::register(jvm);
    OutputStreamWriter::register(jvm);
    Reader::register(jvm);
    Writer::register(jvm);
    StringReader::register(jvm);
    StringWriter::register(jvm);
    FilterInputStream::register(jvm);
    FilterOutputStream::register(jvm);
    DataInputStream::register(jvm);
    DataOutputStream::register(jvm);
    PushbackInputStream::register(jvm);
    PushbackReader::register(jvm);
    FileReader::register(jvm);
    FileWriter::register(jvm);
}

// ========== java.io.BufferedInputStream ==========

pub struct BufferedInputStream;

impl BufferedInputStream {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let in_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("in".to_string(), in_ref);
                    obj.fields.insert("buf".to_string(), Value::Null);
                    obj.fields.insert("pos".to_string(), Value::Int(0));
                    obj.fields.insert("count".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.io.BufferedInputStream".to_string(), "<init>".to_string(), "(Ljava/io/InputStream;)V".to_string(), false, Some(native_impl))
    }

    pub fn read() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get(*this_id) {
                    let pos = obj.fields.get("pos")
                        .and_then(|v| if let Value::Int(p) = v { Some(*p as usize) } else { None })
                        .unwrap_or(0);
                    let count = obj.fields.get("count")
                        .and_then(|v| if let Value::Int(c) = v { Some(*c as usize) } else { None })
                        .unwrap_or(0);
                    if pos < count {
                        // Read from buffer
                        if let Some(Value::ArrayRef(buf_id)) = obj.fields.get("buf") {
                            if let Some(buf) = jvm.heap.get(*buf_id) {
                                if let Some(elements) = &buf.array_elements {
                                    if pos < elements.len() {
                                        let byte = match &elements[pos] {
                                            Value::Byte(b) => *b as i32,
                                            Value::Int(i) => *i,
                                            _ => 0,
                                        };
                                        if let Some(obj) = jvm.heap.get_mut(*this_id) {
                                            obj.fields.insert("pos".to_string(), Value::Int((pos + 1) as i32));
                                        }
                                        frame.push(Value::Int(byte))?;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                        // Buffer exhausted, refill
                        if let Some(obj) = jvm.heap.get_mut(*this_id) {
                            obj.fields.insert("pos".to_string(), Value::Int(0));
                            obj.fields.insert("count".to_string(), Value::Int(0));
                        }
                    }
                }
            }
            frame.push(Value::Int(-1))?;
            Ok(())
        });
        Method::new_native("java.io.BufferedInputStream".to_string(), "read".to_string(), "()I".to_string(), false, Some(native_impl))
    }

    pub fn close() -> Method {
        Method::new_native("java.io.BufferedInputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.BufferedInputStream", BufferedInputStream::init());
        jvm.method_area.add_native_method("java.io.BufferedInputStream", BufferedInputStream::read());
        jvm.method_area.add_native_method("java.io.BufferedInputStream", BufferedInputStream::close());
    }
}

// ========== java.io.BufferedOutputStream ==========

pub struct BufferedOutputStream;

impl BufferedOutputStream {
    pub fn init() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let out_ref = frame.get_local(1)?.clone();
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(obj) = jvm.heap.get_mut(*this_id) {
                    obj.fields.insert("out".to_string(), out_ref);
                    obj.fields.insert("buf".to_string(), Value::Null);
                    obj.fields.insert("count".to_string(), Value::Int(0));
                }
            }
            Ok(())
        });
        Method::new_native("java.io.BufferedOutputStream".to_string(), "<init>".to_string(), "(Ljava/io/OutputStream;)V".to_string(), false, Some(native_impl))
    }

    pub fn write() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, _jvm| {
            // Simplified: delegate to underlying stream
            Ok(())
        });
        Method::new_native("java.io.BufferedOutputStream".to_string(), "write".to_string(), "(I)V".to_string(), false, Some(native_impl))
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.BufferedOutputStream".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.BufferedOutputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.BufferedOutputStream", BufferedOutputStream::init());
        jvm.method_area.add_native_method("java.io.BufferedOutputStream", BufferedOutputStream::write());
        jvm.method_area.add_native_method("java.io.BufferedOutputStream", BufferedOutputStream::flush());
        jvm.method_area.add_native_method("java.io.BufferedOutputStream", BufferedOutputStream::close());
    }
}

// ========== java.io.PrintWriter ==========

pub struct PrintWriter;

impl PrintWriter {
    pub fn init() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "<init>".to_string(), "(Ljava/io/OutputStream;)V".to_string(), false, None)
    }

    pub fn print() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "print".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn print_int() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "print".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn println() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "println".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn println_empty() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "println".to_string(), "()V".to_string(), false, None)
    }

    pub fn write() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "write".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.PrintWriter".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::init());
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::print());
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::print_int());
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::println());
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::println_empty());
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::write());
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::flush());
        jvm.method_area.add_native_method("java.io.PrintWriter", PrintWriter::close());
    }
}

// ========== java.io.BufferedReader ==========

pub struct BufferedReader;

impl BufferedReader {
    pub fn init() -> Method {
        Method::new_native("java.io.BufferedReader".to_string(), "<init>".to_string(), "(Ljava/io/Reader;)V".to_string(), false, None)
    }

    pub fn init_size() -> Method {
        Method::new_native("java.io.BufferedReader".to_string(), "<init>".to_string(), "(Ljava/io/Reader;I)V".to_string(), false, None)
    }

    pub fn read() -> Method {
        Method::new_native("java.io.BufferedReader".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn readLine() -> Method {
        Method::new_native("java.io.BufferedReader".to_string(), "readLine".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.BufferedReader".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.BufferedReader", BufferedReader::init());
        jvm.method_area.add_native_method("java.io.BufferedReader", BufferedReader::init_size());
        jvm.method_area.add_native_method("java.io.BufferedReader", BufferedReader::read());
        jvm.method_area.add_native_method("java.io.BufferedReader", BufferedReader::readLine());
        jvm.method_area.add_native_method("java.io.BufferedReader", BufferedReader::close());
    }
}

// ========== java.io.BufferedWriter ==========

pub struct BufferedWriter;

impl BufferedWriter {
    pub fn init() -> Method {
        Method::new_native("java.io.BufferedWriter".to_string(), "<init>".to_string(), "(Ljava/io/Writer;)V".to_string(), false, None)
    }

    pub fn init_size() -> Method {
        Method::new_native("java.io.BufferedWriter".to_string(), "<init>".to_string(), "(Ljava/io/Writer;I)V".to_string(), false, None)
    }

    pub fn write() -> Method {
        Method::new_native("java.io.BufferedWriter".to_string(), "write".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn write_int() -> Method {
        Method::new_native("java.io.BufferedWriter".to_string(), "write".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn newLine() -> Method {
        Method::new_native("java.io.BufferedWriter".to_string(), "newLine".to_string(), "()V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.BufferedWriter".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.BufferedWriter".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.BufferedWriter", BufferedWriter::init());
        jvm.method_area.add_native_method("java.io.BufferedWriter", BufferedWriter::init_size());
        jvm.method_area.add_native_method("java.io.BufferedWriter", BufferedWriter::write());
        jvm.method_area.add_native_method("java.io.BufferedWriter", BufferedWriter::write_int());
        jvm.method_area.add_native_method("java.io.BufferedWriter", BufferedWriter::newLine());
        jvm.method_area.add_native_method("java.io.BufferedWriter", BufferedWriter::flush());
        jvm.method_area.add_native_method("java.io.BufferedWriter", BufferedWriter::close());
    }
}

// ========== java.io.InputStreamReader ==========

pub struct InputStreamReader;

impl InputStreamReader {
    pub fn init() -> Method {
        Method::new_native("java.io.InputStreamReader".to_string(), "<init>".to_string(), "(Ljava/io/InputStream;)V".to_string(), false, None)
    }

    pub fn read() -> Method {
        Method::new_native("java.io.InputStreamReader".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.InputStreamReader".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.InputStreamReader", InputStreamReader::init());
        jvm.method_area.add_native_method("java.io.InputStreamReader", InputStreamReader::read());
        jvm.method_area.add_native_method("java.io.InputStreamReader", InputStreamReader::close());
    }
}

// ========== java.io.OutputStreamWriter ==========

pub struct OutputStreamWriter;

impl OutputStreamWriter {
    pub fn init() -> Method {
        Method::new_native("java.io.OutputStreamWriter".to_string(), "<init>".to_string(), "(Ljava/io/OutputStream;)V".to_string(), false, None)
    }

    pub fn write() -> Method {
        Method::new_native("java.io.OutputStreamWriter".to_string(), "write".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.OutputStreamWriter".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.OutputStreamWriter".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.OutputStreamWriter", OutputStreamWriter::init());
        jvm.method_area.add_native_method("java.io.OutputStreamWriter", OutputStreamWriter::write());
        jvm.method_area.add_native_method("java.io.OutputStreamWriter", OutputStreamWriter::flush());
        jvm.method_area.add_native_method("java.io.OutputStreamWriter", OutputStreamWriter::close());
    }
}

// ========== java.io.Reader ==========

pub struct Reader;

impl Reader {
    pub fn read() -> Method {
        Method::new_native("java.io.Reader".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn read_chars() -> Method {
        Method::new_native("java.io.Reader".to_string(), "read".to_string(), "([C)I".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.Reader".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.Reader", Reader::read());
        jvm.method_area.add_native_method("java.io.Reader", Reader::read_chars());
        jvm.method_area.add_native_method("java.io.Reader", Reader::close());
    }
}

// ========== java.io.Writer ==========

pub struct Writer;

impl Writer {
    pub fn write() -> Method {
        Method::new_native("java.io.Writer".to_string(), "write".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn write_int() -> Method {
        Method::new_native("java.io.Writer".to_string(), "write".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.Writer".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.Writer".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.Writer", Writer::write());
        jvm.method_area.add_native_method("java.io.Writer", Writer::write_int());
        jvm.method_area.add_native_method("java.io.Writer", Writer::flush());
        jvm.method_area.add_native_method("java.io.Writer", Writer::close());
    }
}

// ========== java.io.StringReader ==========

pub struct StringReader;

impl StringReader {
    pub fn init() -> Method {
        Method::new_native("java.io.StringReader".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn read() -> Method {
        Method::new_native("java.io.StringReader".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.StringReader".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.StringReader", StringReader::init());
        jvm.method_area.add_native_method("java.io.StringReader", StringReader::read());
        jvm.method_area.add_native_method("java.io.StringReader", StringReader::close());
    }
}

// ========== java.io.StringWriter ==========

pub struct StringWriter;

impl StringWriter {
    pub fn init() -> Method {
        Method::new_native("java.io.StringWriter".to_string(), "<init>".to_string(), "()V".to_string(), false, None)
    }

    pub fn write() -> Method {
        Method::new_native("java.io.StringWriter".to_string(), "write".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn toString() -> Method {
        Method::new_native("java.io.StringWriter".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.StringWriter".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.StringWriter".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.StringWriter", StringWriter::init());
        jvm.method_area.add_native_method("java.io.StringWriter", StringWriter::write());
        jvm.method_area.add_native_method("java.io.StringWriter", StringWriter::toString());
        jvm.method_area.add_native_method("java.io.StringWriter", StringWriter::flush());
        jvm.method_area.add_native_method("java.io.StringWriter", StringWriter::close());
    }
}

// ========== java.io.FilterInputStream ==========

pub struct FilterInputStream;

impl FilterInputStream {
    pub fn init() -> Method {
        Method::new_native("java.io.FilterInputStream".to_string(), "<init>".to_string(), "(Ljava/io/InputStream;)V".to_string(), false, None)
    }

    pub fn read() -> Method {
        Method::new_native("java.io.FilterInputStream".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn read_bytes() -> Method {
        Method::new_native("java.io.FilterInputStream".to_string(), "read".to_string(), "([B)I".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.FilterInputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.FilterInputStream", FilterInputStream::init());
        jvm.method_area.add_native_method("java.io.FilterInputStream", FilterInputStream::read());
        jvm.method_area.add_native_method("java.io.FilterInputStream", FilterInputStream::read_bytes());
        jvm.method_area.add_native_method("java.io.FilterInputStream", FilterInputStream::close());
    }
}

// ========== java.io.FilterOutputStream ==========

pub struct FilterOutputStream;

impl FilterOutputStream {
    pub fn init() -> Method {
        Method::new_native("java.io.FilterOutputStream".to_string(), "<init>".to_string(), "(Ljava/io/OutputStream;)V".to_string(), false, None)
    }

    pub fn write() -> Method {
        Method::new_native("java.io.FilterOutputStream".to_string(), "write".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn write_bytes() -> Method {
        Method::new_native("java.io.FilterOutputStream".to_string(), "write".to_string(), "([B)V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.FilterOutputStream".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.FilterOutputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.FilterOutputStream", FilterOutputStream::init());
        jvm.method_area.add_native_method("java.io.FilterOutputStream", FilterOutputStream::write());
        jvm.method_area.add_native_method("java.io.FilterOutputStream", FilterOutputStream::write_bytes());
        jvm.method_area.add_native_method("java.io.FilterOutputStream", FilterOutputStream::flush());
        jvm.method_area.add_native_method("java.io.FilterOutputStream", FilterOutputStream::close());
    }
}

// ========== java.io.DataInputStream ==========

pub struct DataInputStream;

impl DataInputStream {
    pub fn init() -> Method {
        Method::new_native("java.io.DataInputStream".to_string(), "<init>".to_string(), "(Ljava/io/InputStream;)V".to_string(), false, None)
    }

    pub fn readInt() -> Method {
        Method::new_native("java.io.DataInputStream".to_string(), "readInt".to_string(), "()I".to_string(), false, None)
    }

    pub fn readLong() -> Method {
        Method::new_native("java.io.DataInputStream".to_string(), "readLong".to_string(), "()J".to_string(), false, None)
    }

    pub fn readDouble() -> Method {
        Method::new_native("java.io.DataInputStream".to_string(), "readDouble".to_string(), "()D".to_string(), false, None)
    }

    pub fn readBoolean() -> Method {
        Method::new_native("java.io.DataInputStream".to_string(), "readBoolean".to_string(), "()Z".to_string(), false, None)
    }

    pub fn readUTF() -> Method {
        Method::new_native("java.io.DataInputStream".to_string(), "readUTF".to_string(), "()Ljava/lang/String;".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.DataInputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.DataInputStream", DataInputStream::init());
        jvm.method_area.add_native_method("java.io.DataInputStream", DataInputStream::readInt());
        jvm.method_area.add_native_method("java.io.DataInputStream", DataInputStream::readLong());
        jvm.method_area.add_native_method("java.io.DataInputStream", DataInputStream::readDouble());
        jvm.method_area.add_native_method("java.io.DataInputStream", DataInputStream::readBoolean());
        jvm.method_area.add_native_method("java.io.DataInputStream", DataInputStream::readUTF());
        jvm.method_area.add_native_method("java.io.DataInputStream", DataInputStream::close());
    }
}

// ========== java.io.DataOutputStream ==========

pub struct DataOutputStream;

impl DataOutputStream {
    pub fn init() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "<init>".to_string(), "(Ljava/io/OutputStream;)V".to_string(), false, None)
    }

    pub fn writeInt() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "writeInt".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn writeLong() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "writeLong".to_string(), "(J)V".to_string(), false, None)
    }

    pub fn writeDouble() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "writeDouble".to_string(), "(D)V".to_string(), false, None)
    }

    pub fn writeBoolean() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "writeBoolean".to_string(), "(Z)V".to_string(), false, None)
    }

    pub fn writeUTF() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "writeUTF".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.DataOutputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::init());
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::writeInt());
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::writeLong());
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::writeDouble());
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::writeBoolean());
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::writeUTF());
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::flush());
        jvm.method_area.add_native_method("java.io.DataOutputStream", DataOutputStream::close());
    }
}

// ========== java.io.PushbackInputStream ==========

pub struct PushbackInputStream;

impl PushbackInputStream {
    pub fn init() -> Method {
        Method::new_native("java.io.PushbackInputStream".to_string(), "<init>".to_string(), "(Ljava/io/InputStream;)V".to_string(), false, None)
    }

    pub fn read() -> Method {
        Method::new_native("java.io.PushbackInputStream".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn unread() -> Method {
        Method::new_native("java.io.PushbackInputStream".to_string(), "unread".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.PushbackInputStream".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.PushbackInputStream", PushbackInputStream::init());
        jvm.method_area.add_native_method("java.io.PushbackInputStream", PushbackInputStream::read());
        jvm.method_area.add_native_method("java.io.PushbackInputStream", PushbackInputStream::unread());
        jvm.method_area.add_native_method("java.io.PushbackInputStream", PushbackInputStream::close());
    }
}

// ========== java.io.PushbackReader ==========

pub struct PushbackReader;

impl PushbackReader {
    pub fn init() -> Method {
        Method::new_native("java.io.PushbackReader".to_string(), "<init>".to_string(), "(Ljava/io/Reader;)V".to_string(), false, None)
    }

    pub fn read() -> Method {
        Method::new_native("java.io.PushbackReader".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn unread() -> Method {
        Method::new_native("java.io.PushbackReader".to_string(), "unread".to_string(), "(I)V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.PushbackReader".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.PushbackReader", PushbackReader::init());
        jvm.method_area.add_native_method("java.io.PushbackReader", PushbackReader::read());
        jvm.method_area.add_native_method("java.io.PushbackReader", PushbackReader::unread());
        jvm.method_area.add_native_method("java.io.PushbackReader", PushbackReader::close());
    }
}

// ========== java.io.FileReader ==========

pub struct FileReader;

impl FileReader {
    pub fn init() -> Method {
        Method::new_native("java.io.FileReader".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn read() -> Method {
        Method::new_native("java.io.FileReader".to_string(), "read".to_string(), "()I".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.FileReader".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.FileReader", FileReader::init());
        jvm.method_area.add_native_method("java.io.FileReader", FileReader::read());
        jvm.method_area.add_native_method("java.io.FileReader", FileReader::close());
    }
}

// ========== java.io.FileWriter ==========

pub struct FileWriter;

impl FileWriter {
    pub fn init() -> Method {
        Method::new_native("java.io.FileWriter".to_string(), "<init>".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn write() -> Method {
        Method::new_native("java.io.FileWriter".to_string(), "write".to_string(), "(Ljava/lang/String;)V".to_string(), false, None)
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.FileWriter".to_string(), "flush".to_string(), "()V".to_string(), false, None)
    }

    pub fn close() -> Method {
        Method::new_native("java.io.FileWriter".to_string(), "close".to_string(), "()V".to_string(), false, None)
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.io.FileWriter", FileWriter::init());
        jvm.method_area.add_native_method("java.io.FileWriter", FileWriter::write());
        jvm.method_area.add_native_method("java.io.FileWriter", FileWriter::flush());
        jvm.method_area.add_native_method("java.io.FileWriter", FileWriter::close());
    }
}
