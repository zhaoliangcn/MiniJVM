use crate::runtime::{Value, HeapObject, Method};

pub struct Object;

impl Object {
    pub fn get_class() -> Method {
        Method::new_native("java.lang.Object".to_string(), "getClass".to_string(), "()Ljava/lang/Class;")
    }

    pub fn hashCode() -> Method {
        Method::new_native("java.lang.Object".to_string(), "hashCode".to_string(), "()I")
    }

    pub fn equals() -> Method {
        Method::new_native("java.lang.Object".to_string(), "equals".to_string(), "(Ljava/lang/Object;)Z")
    }

    pub fn toString() -> Method {
        Method::new_native("java.lang.Object".to_string(), "toString".to_string(), "()Ljava/lang/String;")
    }

    pub fn clone() -> Method {
        Method::new_native("java.lang.Object".to_string(), "clone".to_string(), "()Ljava/lang/Object;")
    }

    pub fn notify() -> Method {
        Method::new_native("java.lang.Object".to_string(), "notify".to_string(), "()V")
    }

    pub fn notifyAll() -> Method {
        Method::new_native("java.lang.Object".to_string(), "notifyAll".to_string(), "()V")
    }

    pub fn wait() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "()V")
    }

    pub fn wait_timeout() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(J)V")
    }

    pub fn wait_nanos() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(JI)V")
    }

    pub fn finalize() -> Method {
        Method::new_native("java.lang.Object".to_string(), "finalize".to_string(), "()V")
    }
}

pub struct String;

impl String {
    pub fn length() -> Method {
        Method::new_native("java.lang.String".to_string(), "length".to_string(), "()I")
    }

    pub fn charAt() -> Method {
        Method::new_native("java.lang.String".to_string(), "charAt".to_string(), "(I)C")
    }

    pub fn getBytes() -> Method {
        Method::new_native("java.lang.String".to_string(), "getBytes".to_string(), "()[B")
    }

    pub fn getBytes_charset() -> Method {
        Method::new_native("java.lang.String".to_string(), "getBytes".to_string(), "(Ljava/lang/String;)[B")
    }

    pub fn equals() -> Method {
        Method::new_native("java.lang.String".to_string(), "equals".to_string(), "(Ljava/lang/Object;)Z")
    }

    pub fn compareTo() -> Method {
        Method::new_native("java.lang.String".to_string(), "compareTo".to_string(), "(Ljava/lang/String;)I")
    }

    pub fn indexOf() -> Method {
        Method::new_native("java.lang.String".to_string(), "indexOf".to_string(), "(Ljava/lang/String;)I")
    }

    pub fn substring() -> Method {
        Method::new_native("java.lang.String".to_string(), "substring".to_string(), "(II)Ljava/lang/String;")
    }

    pub fn concat() -> Method {
        Method::new_native("java.lang.String".to_string(), "concat".to_string(), "(Ljava/lang/String;)Ljava/lang/String;")
    }

    pub fn replace() -> Method {
        Method::new_native("java.lang.String".to_string(), "replace".to_string(), "(CC)Ljava/lang/String;")
    }

    pub fn trim() -> Method {
        Method::new_native("java.lang.String".to_string(), "trim".to_string(), "()Ljava/lang/String;")
    }

    pub fn toLowerCase() -> Method {
        Method::new_native("java.lang.String".to_string(), "toLowerCase".to_string(), "()Ljava/lang/String;")
    }

    pub fn toUpperCase() -> Method {
        Method::new_native("java.lang.String".to_string(), "toUpperCase".to_string(), "()Ljava/lang/String;")
    }

    pub fn valueOf_int() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(I)Ljava/lang/String;")
    }

    pub fn valueOf_long() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(J)Ljava/lang/String;")
    }

    pub fn valueOf_float() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(F)Ljava/lang/String;")
    }

    pub fn valueOf_double() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(D)Ljava/lang/String;")
    }

    pub fn valueOf_bool() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(Z)Ljava/lang/String;")
    }

    pub fn valueOf_obj() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(Ljava/lang/Object;)Ljava/lang/String;")
    }

    pub fn format() -> Method {
        Method::new_native("java.lang.String".to_string(), "format".to_string(), "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;")
    }

    pub fn startsWith() -> Method {
        Method::new_native("java.lang.String".to_string(), "startsWith".to_string(), "(Ljava/lang/String;)Z")
    }

    pub fn endsWith() -> Method {
        Method::new_native("java.lang.String".to_string(), "endsWith".to_string(), "(Ljava/lang/String;)Z")
    }

    pub fn contains() -> Method {
        Method::new_native("java.lang.String".to_string(), "contains".to_string(), "(Ljava/lang/CharSequence;)Z")
    }

    pub fn isEmpty() -> Method {
        Method::new_native("java.lang.String".to_string(), "isEmpty".to_string(), "()Z")
    }

    pub fn split() -> Method {
        Method::new_native("java.lang.String".to_string(), "split".to_string(), "(Ljava/lang/String;)[Ljava/lang/String;")
    }
}

pub struct System;

impl System {
    pub fn arraycopy() -> Method {
        Method::new_native("java.lang.System".to_string(), "arraycopy".to_string(), "(Ljava/lang/Object;ILjava/lang/Object;II)V")
    }

    pub fn currentTimeMillis() -> Method {
        Method::new_native("java.lang.System".to_string(), "currentTimeMillis".to_string(), "()J")
    }

    pub fn nanoTime() -> Method {
        Method::new_native("java.lang.System".to_string(), "nanoTime".to_string(), "()J")
    }

    pub fn identityHashCode() -> Method {
        Method::new_native("java.lang.System".to_string(), "identityHashCode".to_string(), "(Ljava/lang/Object;)I")
    }

    pub fn setErr() -> Method {
        Method::new_native("java.lang.System".to_string(), "setErr".to_string(), "(Ljava/io/PrintStream;)V")
    }

    pub fn setIn() -> Method {
        Method::new_native("java.lang.System".to_string(), "setIn".to_string(), "(Ljava/io/InputStream;)V")
    }

    pub fn setOut() -> Method {
        Method::new_native("java.lang.System".to_string(), "setOut".to_string(), "(Ljava/io/PrintStream;)V")
    }
}

pub struct Thread;

impl Thread {
    pub fn start() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "start".to_string(), "()V")
    }

    pub fn run() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "run".to_string(), "()V")
    }

    pub fn sleep() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "sleep".to_string(), "(J)V")
    }

    pub fn join() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "join".to_string(), "()V")
    }

    pub fn yield() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "yield".to_string(), "()V")
    }

    pub fn currentThread() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "currentThread".to_string(), "()Ljava/lang/Thread;")
    }

    pub fn getName() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getName".to_string(), "()Ljava/lang/String;")
    }

    pub fn setName() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "setName".to_string(), "(Ljava/lang/String;)V")
    }

    pub fn getPriority() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getPriority".to_string(), "()I")
    }

    pub fn setPriority() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "setPriority".to_string(), "(I)V")
    }

    pub fn getId() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getId".to_string(), "()J")
    }

    pub fn getState() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getState".to_string(), "()Ljava/lang/Thread$State;")
    }

    pub fn interrupt() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "interrupt".to_string(), "()V")
    }

    pub fn isInterrupted() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "isInterrupted".to_string(), "()Z")
    }

    pub fn interrupted() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "interrupted".to_string(), "()Z")
    }

    pub fn isAlive() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "isAlive".to_string(), "()Z")
    }
}

pub struct Throwable;

impl Throwable {
    pub fn getMessage() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getMessage".to_string(), "()Ljava/lang/String;")
    }

    pub fn getLocalizedMessage() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getLocalizedMessage".to_string(), "()Ljava/lang/String;")
    }

    pub fn toString() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "toString".to_string(), "()Ljava/lang/String;")
    }

    pub fn printStackTrace() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "printStackTrace".to_string(), "()V")
    }

    pub fn fillInStackTrace() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "fillInStackTrace".to_string(), "()Ljava/lang/Throwable;")
    }

    pub fn getCause() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getCause".to_string(), "()Ljava/lang/Throwable;")
    }

    pub fn initCause() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "initCause".to_string(), "(Ljava/lang/Throwable;)Ljava/lang/Throwable;")
    }
}
