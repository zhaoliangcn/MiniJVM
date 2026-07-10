use crate::runtime::method_area::Method;

pub struct Object;

impl Object {
    pub fn get_class() -> Method {
        Method::new_native("java.lang.Object".to_string(), "getClass".to_string(), "()Ljava/lang/Class;".to_string())
    }

    pub fn hashCode() -> Method {
        Method::new_native("java.lang.Object".to_string(), "hashCode".to_string(), "()I".to_string())
    }

    pub fn equals() -> Method {
        Method::new_native("java.lang.Object".to_string(), "equals".to_string(), "(Ljava/lang/Object;)Z".to_string())
    }

    pub fn toString() -> Method {
        Method::new_native("java.lang.Object".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn clone() -> Method {
        Method::new_native("java.lang.Object".to_string(), "clone".to_string(), "()Ljava/lang/Object;".to_string())
    }

    pub fn notify() -> Method {
        Method::new_native("java.lang.Object".to_string(), "notify".to_string(), "()V".to_string())
    }

    pub fn notifyAll() -> Method {
        Method::new_native("java.lang.Object".to_string(), "notifyAll".to_string(), "()V".to_string())
    }

    pub fn wait() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "()V".to_string())
    }

    pub fn wait_timeout() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(J)V".to_string())
    }

    pub fn wait_nanos() -> Method {
        Method::new_native("java.lang.Object".to_string(), "wait".to_string(), "(JI)V".to_string())
    }

    pub fn finalize() -> Method {
        Method::new_native("java.lang.Object".to_string(), "finalize".to_string(), "()V".to_string())
    }
}

pub struct String;

impl String {
    pub fn length() -> Method {
        Method::new_native("java.lang.String".to_string(), "length".to_string(), "()I".to_string())
    }

    pub fn charAt() -> Method {
        Method::new_native("java.lang.String".to_string(), "charAt".to_string(), "(I)C".to_string())
    }

    pub fn getBytes() -> Method {
        Method::new_native("java.lang.String".to_string(), "getBytes".to_string(), "()[B".to_string())
    }

    pub fn getBytes_charset() -> Method {
        Method::new_native("java.lang.String".to_string(), "getBytes".to_string(), "(Ljava/lang/String;)[B".to_string())
    }

    pub fn equals() -> Method {
        Method::new_native("java.lang.String".to_string(), "equals".to_string(), "(Ljava/lang/Object;)Z".to_string())
    }

    pub fn compareTo() -> Method {
        Method::new_native("java.lang.String".to_string(), "compareTo".to_string(), "(Ljava/lang/String;)I".to_string())
    }

    pub fn indexOf() -> Method {
        Method::new_native("java.lang.String".to_string(), "indexOf".to_string(), "(Ljava/lang/String;)I".to_string())
    }

    pub fn substring() -> Method {
        Method::new_native("java.lang.String".to_string(), "substring".to_string(), "(II)Ljava/lang/String;".to_string())
    }

    pub fn concat() -> Method {
        Method::new_native("java.lang.String".to_string(), "concat".to_string(), "(Ljava/lang/String;)Ljava/lang/String;".to_string())
    }

    pub fn replace() -> Method {
        Method::new_native("java.lang.String".to_string(), "replace".to_string(), "(CC)Ljava/lang/String;".to_string())
    }

    pub fn trim() -> Method {
        Method::new_native("java.lang.String".to_string(), "trim".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn toLowerCase() -> Method {
        Method::new_native("java.lang.String".to_string(), "toLowerCase".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn toUpperCase() -> Method {
        Method::new_native("java.lang.String".to_string(), "toUpperCase".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn valueOf_int() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(I)Ljava/lang/String;".to_string())
    }

    pub fn valueOf_long() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(J)Ljava/lang/String;".to_string())
    }

    pub fn valueOf_float() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(F)Ljava/lang/String;".to_string())
    }

    pub fn valueOf_double() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(D)Ljava/lang/String;".to_string())
    }

    pub fn valueOf_bool() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(Z)Ljava/lang/String;".to_string())
    }

    pub fn valueOf_obj() -> Method {
        Method::new_native("java.lang.String".to_string(), "valueOf".to_string(), "(Ljava/lang/Object;)Ljava/lang/String;".to_string())
    }

    pub fn format() -> Method {
        Method::new_native("java.lang.String".to_string(), "format".to_string(), "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;".to_string())
    }

    pub fn startsWith() -> Method {
        Method::new_native("java.lang.String".to_string(), "startsWith".to_string(), "(Ljava/lang/String;)Z".to_string())
    }

    pub fn endsWith() -> Method {
        Method::new_native("java.lang.String".to_string(), "endsWith".to_string(), "(Ljava/lang/String;)Z".to_string())
    }

    pub fn contains() -> Method {
        Method::new_native("java.lang.String".to_string(), "contains".to_string(), "(Ljava/lang/CharSequence;)Z".to_string())
    }

    pub fn isEmpty() -> Method {
        Method::new_native("java.lang.String".to_string(), "isEmpty".to_string(), "()Z".to_string())
    }

    pub fn split() -> Method {
        Method::new_native("java.lang.String".to_string(), "split".to_string(), "(Ljava/lang/String;)[Ljava/lang/String;".to_string())
    }
}

pub struct System;

impl System {
    pub fn arraycopy() -> Method {
        Method::new_native("java.lang.System".to_string(), "arraycopy".to_string(), "(Ljava/lang/Object;ILjava/lang/Object;II)V".to_string())
    }

    pub fn currentTimeMillis() -> Method {
        Method::new_native("java.lang.System".to_string(), "currentTimeMillis".to_string(), "()J".to_string())
    }

    pub fn nanoTime() -> Method {
        Method::new_native("java.lang.System".to_string(), "nanoTime".to_string(), "()J".to_string())
    }

    pub fn identityHashCode() -> Method {
        Method::new_native("java.lang.System".to_string(), "identityHashCode".to_string(), "(Ljava/lang/Object;)I".to_string())
    }

    pub fn setErr() -> Method {
        Method::new_native("java.lang.System".to_string(), "setErr".to_string(), "(Ljava/io/PrintStream;)V".to_string())
    }

    pub fn setIn() -> Method {
        Method::new_native("java.lang.System".to_string(), "setIn".to_string(), "(Ljava/io/InputStream;)V".to_string())
    }

    pub fn setOut() -> Method {
        Method::new_native("java.lang.System".to_string(), "setOut".to_string(), "(Ljava/io/PrintStream;)V".to_string())
    }
}

pub struct Thread;

impl Thread {
    pub fn start() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "start".to_string(), "()V".to_string())
    }

    pub fn run() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "run".to_string(), "()V".to_string())
    }

    pub fn sleep() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "sleep".to_string(), "(J)V".to_string())
    }

    pub fn join() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "join".to_string(), "()V".to_string())
    }

    pub fn r#yield() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "yield".to_string(), "()V".to_string())
    }

    pub fn currentThread() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "currentThread".to_string(), "()Ljava/lang/Thread;".to_string())
    }

    pub fn getName() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getName".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn setName() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "setName".to_string(), "(Ljava/lang/String;)V".to_string())
    }

    pub fn getPriority() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getPriority".to_string(), "()I".to_string())
    }

    pub fn setPriority() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "setPriority".to_string(), "(I)V".to_string())
    }

    pub fn getId() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getId".to_string(), "()J".to_string())
    }

    pub fn getState() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "getState".to_string(), "()Ljava/lang/Thread$State;".to_string())
    }

    pub fn interrupt() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "interrupt".to_string(), "()V".to_string())
    }

    pub fn isInterrupted() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "isInterrupted".to_string(), "()Z".to_string())
    }

    pub fn interrupted() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "interrupted".to_string(), "()Z".to_string())
    }

    pub fn isAlive() -> Method {
        Method::new_native("java.lang.Thread".to_string(), "isAlive".to_string(), "()Z".to_string())
    }
}

pub struct Throwable;

impl Throwable {
    pub fn getMessage() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getMessage".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn getLocalizedMessage() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getLocalizedMessage".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn toString() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "toString".to_string(), "()Ljava/lang/String;".to_string())
    }

    pub fn printStackTrace() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "printStackTrace".to_string(), "()V".to_string())
    }

    pub fn fillInStackTrace() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "fillInStackTrace".to_string(), "()Ljava/lang/Throwable;".to_string())
    }

    pub fn getCause() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "getCause".to_string(), "()Ljava/lang/Throwable;".to_string())
    }

    pub fn initCause() -> Method {
        Method::new_native("java.lang.Throwable".to_string(), "initCause".to_string(), "(Ljava/lang/Throwable;)Ljava/lang/Throwable;".to_string())
    }
}
