use crate::runtime::{Value, HeapObject, Method};

pub struct InputStream;

impl InputStream {
    pub fn read() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "read".to_string(), "()I")
    }

    pub fn read_bytes() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "read".to_string(), "([B)I")
    }

    pub fn read_bytes_offset() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "read".to_string(), "([BII)I")
    }

    pub fn skip() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "skip".to_string(), "(J)J")
    }

    pub fn available() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "available".to_string(), "()I")
    }

    pub fn close() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "close".to_string(), "()V")
    }

    pub fn mark() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "mark".to_string(), "(I)V")
    }

    pub fn reset() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "reset".to_string(), "()V")
    }

    pub fn markSupported() -> Method {
        Method::new_native("java.io.InputStream".to_string(), "markSupported".to_string(), "()Z")
    }
}

pub struct OutputStream;

impl OutputStream {
    pub fn write() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "write".to_string(), "(I)V")
    }

    pub fn write_bytes() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "write".to_string(), "([B)V")
    }

    pub fn write_bytes_offset() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "write".to_string(), "([BII)V")
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "flush".to_string(), "()V")
    }

    pub fn close() -> Method {
        Method::new_native("java.io.OutputStream".to_string(), "close".to_string(), "()V")
    }
}

pub struct PrintStream;

impl PrintStream {
    pub fn print_bool() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(Z)V")
    }

    pub fn print_char() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(C)V")
    }

    pub fn print_int() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(I)V")
    }

    pub fn print_long() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(J)V")
    }

    pub fn print_float() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(F)V")
    }

    pub fn print_double() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(D)V")
    }

    pub fn print_char_array() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "([C)V")
    }

    pub fn print_string() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(Ljava/lang/String;)V")
    }

    pub fn print_object() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "print".to_string(), "(Ljava/lang/Object;)V")
    }

    pub fn println_bool() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(Z)V")
    }

    pub fn println_char() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(C)V")
    }

    pub fn println_int() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(I)V")
    }

    pub fn println_long() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(J)V")
    }

    pub fn println_float() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(F)V")
    }

    pub fn println_double() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(D)V")
    }

    pub fn println_char_array() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "([C)V")
    }

    pub fn println_string() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(Ljava/lang/String;)V")
    }

    pub fn println_object() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "(Ljava/lang/Object;)V")
    }

    pub fn println() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "println".to_string(), "()V")
    }

    pub fn printf() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "printf".to_string(), "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
    }

    pub fn format() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "format".to_string(), "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;")
    }

    pub fn flush() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "flush".to_string(), "()V")
    }

    pub fn close() -> Method {
        Method::new_native("java.io.PrintStream".to_string(), "close".to_string(), "()V")
    }
}
