use std::sync::Arc;
use crate::runtime::{JVM, Frame, Value, method_area::{Method, NativeImplementation}};

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
                        println!("{}", str_val);
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
            println!();
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
