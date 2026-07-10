use std::collections::HashMap;
use crate::error::{ClassFileError, InterpreterError, RuntimeError, JvmError, Result};
use JvmError::*;
use crate::runtime::{JVM, Value, Frame, HeapObject};

pub type InstructionHandler = fn(&mut Frame, &mut JVM) -> Result<usize>;

pub struct InstructionSet {
    handlers: HashMap<u8, InstructionHandler>,
}

impl InstructionSet {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();
        Self::register_handlers(&mut handlers);
        InstructionSet { handlers }
    }

    pub fn get_handler(&self, opcode: u8) -> Option<InstructionHandler> {
        self.handlers.get(&opcode).copied()
    }

    fn register_handlers(handlers: &mut HashMap<u8, InstructionHandler>) {
        handlers.insert(0x00, handle_nop);
        handlers.insert(0x01, handle_aconst_null);
        handlers.insert(0x02, handle_iconst_m1);
        handlers.insert(0x03, handle_iconst_0);
        handlers.insert(0x04, handle_iconst_1);
        handlers.insert(0x05, handle_iconst_2);
        handlers.insert(0x06, handle_iconst_3);
        handlers.insert(0x07, handle_iconst_4);
        handlers.insert(0x08, handle_iconst_5);
        handlers.insert(0x09, handle_lconst_0);
        handlers.insert(0x0A, handle_lconst_1);
        handlers.insert(0x0B, handle_fconst_0);
        handlers.insert(0x0C, handle_fconst_1);
        handlers.insert(0x0D, handle_fconst_2);
        handlers.insert(0x0E, handle_dconst_0);
        handlers.insert(0x0F, handle_dconst_1);
        handlers.insert(0x10, handle_bipush);
        handlers.insert(0x11, handle_sipush);
        handlers.insert(0x12, handle_ldc);
        handlers.insert(0x13, handle_ldc_w);
        handlers.insert(0x14, handle_ldc2_w);
        handlers.insert(0x15, handle_iload);
        handlers.insert(0x16, handle_lload);
        handlers.insert(0x17, handle_fload);
        handlers.insert(0x18, handle_dload);
        handlers.insert(0x19, handle_aload);
        handlers.insert(0x1A, handle_iload_0);
        handlers.insert(0x1B, handle_iload_1);
        handlers.insert(0x1C, handle_iload_2);
        handlers.insert(0x1D, handle_iload_3);
        handlers.insert(0x1E, handle_lload_0);
        handlers.insert(0x1F, handle_lload_1);
        handlers.insert(0x20, handle_lload_2);
        handlers.insert(0x21, handle_lload_3);
        handlers.insert(0x22, handle_fload_0);
        handlers.insert(0x23, handle_fload_1);
        handlers.insert(0x24, handle_fload_2);
        handlers.insert(0x25, handle_fload_3);
        handlers.insert(0x26, handle_dload_0);
        handlers.insert(0x27, handle_dload_1);
        handlers.insert(0x28, handle_dload_2);
        handlers.insert(0x29, handle_dload_3);
        handlers.insert(0x2A, handle_aload_0);
        handlers.insert(0x2B, handle_aload_1);
        handlers.insert(0x2C, handle_aload_2);
        handlers.insert(0x2D, handle_aload_3);
        handlers.insert(0x36, handle_istore);
        handlers.insert(0x37, handle_lstore);
        handlers.insert(0x38, handle_fstore);
        handlers.insert(0x39, handle_dstore);
        handlers.insert(0x3A, handle_astore);
        handlers.insert(0x3B, handle_istore_0);
        handlers.insert(0x3C, handle_istore_1);
        handlers.insert(0x3D, handle_istore_2);
        handlers.insert(0x3E, handle_istore_3);
        handlers.insert(0x4B, handle_astore_0);
        handlers.insert(0x4C, handle_astore_1);
        handlers.insert(0x4D, handle_astore_2);
        handlers.insert(0x4E, handle_astore_3);
        handlers.insert(0x57, handle_pop);
        handlers.insert(0x58, handle_pop2);
        handlers.insert(0x59, handle_dup);
        handlers.insert(0x5A, handle_dup_x1);
        handlers.insert(0x5B, handle_dup_x2);
        handlers.insert(0x5C, handle_dup2);
        handlers.insert(0x5D, handle_dup2_x1);
        handlers.insert(0x5E, handle_dup2_x2);
        handlers.insert(0x5F, handle_swap);
        handlers.insert(0x60, handle_iadd);
        handlers.insert(0x61, handle_ladd);
        handlers.insert(0x62, handle_fadd);
        handlers.insert(0x63, handle_dadd);
        handlers.insert(0x64, handle_isub);
        handlers.insert(0x65, handle_lsub);
        handlers.insert(0x66, handle_fsub);
        handlers.insert(0x67, handle_dsub);
        handlers.insert(0x68, handle_imul);
        handlers.insert(0x69, handle_lmul);
        handlers.insert(0x6A, handle_fmul);
        handlers.insert(0x6B, handle_dmul);
        handlers.insert(0x6C, handle_idiv);
        handlers.insert(0x6D, handle_ldiv);
        handlers.insert(0x6E, handle_fdiv);
        handlers.insert(0x6F, handle_ddiv);
        handlers.insert(0x70, handle_irem);
        handlers.insert(0x71, handle_lrem);
        handlers.insert(0x72, handle_frem);
        handlers.insert(0x73, handle_drem);
        handlers.insert(0x74, handle_ineg);
        handlers.insert(0x75, handle_lneg);
        handlers.insert(0x76, handle_fneg);
        handlers.insert(0x77, handle_dneg);
        handlers.insert(0x78, handle_ishl);
        handlers.insert(0x79, handle_lshl);
        handlers.insert(0x7A, handle_ishr);
        handlers.insert(0x7B, handle_lshr);
        handlers.insert(0x7C, handle_iushr);
        handlers.insert(0x7D, handle_lushr);
        handlers.insert(0x7E, handle_iand);
        handlers.insert(0x7F, handle_land);
        handlers.insert(0x80, handle_ior);
        handlers.insert(0x81, handle_lor);
        handlers.insert(0x82, handle_ixor);
        handlers.insert(0x83, handle_lxor);
        handlers.insert(0x84, handle_iinc);
        handlers.insert(0x9F, handle_if_icmpeq);
        handlers.insert(0xA0, handle_if_icmpne);
        handlers.insert(0xA1, handle_if_icmplt);
        handlers.insert(0xA2, handle_if_icmpge);
        handlers.insert(0xA3, handle_if_icmpgt);
        handlers.insert(0xA4, handle_if_icmple);
        handlers.insert(0xA5, handle_if_acmpeq);
        handlers.insert(0xA6, handle_if_acmpne);
        handlers.insert(0xA7, handle_goto);
        handlers.insert(0xAC, handle_ireturn);
        handlers.insert(0xAD, handle_lreturn);
        handlers.insert(0xAE, handle_freturn);
        handlers.insert(0xAF, handle_dreturn);
        handlers.insert(0xB0, handle_areturn);
        handlers.insert(0xB1, handle_return);
        handlers.insert(0xB2, handle_getstatic);
        handlers.insert(0xB3, handle_putstatic);
        handlers.insert(0xB4, handle_getfield);
        handlers.insert(0xB5, handle_putfield);
        handlers.insert(0xB6, handle_invokevirtual);
        handlers.insert(0xB7, handle_invokespecial);
        handlers.insert(0xB8, handle_invokestatic);
        handlers.insert(0xBB, handle_new);
        handlers.insert(0xBC, handle_newarray);
        handlers.insert(0xBD, handle_anewarray);
        handlers.insert(0xBE, handle_arraylength);
        handlers.insert(0xBF, handle_athrow);
        handlers.insert(0xC0, handle_checkcast);
        handlers.insert(0xC1, handle_instanceof);
        handlers.insert(0xC2, handle_monitorenter);
        handlers.insert(0xC3, handle_monitorexit);
        handlers.insert(0xC5, handle_multianewarray);
        handlers.insert(0xC6, handle_ifnull);
        handlers.insert(0xC7, handle_ifnonnull);
    }
}

fn handle_nop(_frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    Ok(1)
}

fn handle_aconst_null(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Null)?;
    Ok(1)
}

fn handle_iconst_m1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Int(-1))?;
    Ok(1)
}

fn handle_iconst_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Int(0))?;
    Ok(1)
}

fn handle_iconst_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Int(1))?;
    Ok(1)
}

fn handle_iconst_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Int(2))?;
    Ok(1)
}

fn handle_iconst_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Int(3))?;
    Ok(1)
}

fn handle_iconst_4(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Int(4))?;
    Ok(1)
}

fn handle_iconst_5(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Int(5))?;
    Ok(1)
}

fn handle_lconst_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Long(0))?;
    Ok(1)
}

fn handle_lconst_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Long(1))?;
    Ok(1)
}

fn handle_fconst_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Float(0.0))?;
    Ok(1)
}

fn handle_fconst_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Float(1.0))?;
    Ok(1)
}

fn handle_fconst_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Float(2.0))?;
    Ok(1)
}

fn handle_dconst_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Double(0.0))?;
    Ok(1)
}

fn handle_dconst_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.push(Value::Double(1.0))?;
    Ok(1)
}

fn handle_bipush(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let val = code[frame.pc + 1] as i8 as i32;
    frame.push(Value::Int(val))?;
    Ok(2)
}

fn handle_sipush(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let val = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    frame.push(Value::Int(val))?;
    Ok(3)
}

fn handle_ldc(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let cp = &class.class_file.constant_pool;
    
    match cp.get(index) {
        Some(crate::classfile::constant_pool::CpInfo::Integer(v)) => {
            frame.push(Value::Int(*v))?;
        }
        Some(crate::classfile::constant_pool::CpInfo::Float(v)) => {
            frame.push(Value::Float(*v))?;
        }
        Some(crate::classfile::constant_pool::CpInfo::String(utf8_index)) => {
            let s = cp.get_utf8(*utf8_index)
                .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(*utf8_index)))?;
            let obj = HeapObject::new_string("java.lang.String".to_string(), s);
            let ref_id = jvm.heap.allocate(obj)?;
            frame.push(Value::ObjectRef(ref_id))?;
        }
        _ => return Err(JvmError::InterpreterError(InterpreterError::InvalidInstructionFormat(frame.pc))),
    }
    
    Ok(2)
}

fn handle_ldc_w(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let cp = &class.class_file.constant_pool;
    
    match cp.get(index) {
        Some(crate::classfile::constant_pool::CpInfo::Integer(v)) => {
            frame.push(Value::Int(*v))?;
        }
        Some(crate::classfile::constant_pool::CpInfo::Float(v)) => {
            frame.push(Value::Float(*v))?;
        }
        Some(crate::classfile::constant_pool::CpInfo::String(utf8_index)) => {
            let s = cp.get_utf8(*utf8_index)
                .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(*utf8_index)))?;
            let obj = HeapObject::new_string("java.lang.String".to_string(), s);
            let ref_id = jvm.heap.allocate(obj)?;
            frame.push(Value::ObjectRef(ref_id))?;
        }
        _ => return Err(JvmError::InterpreterError(InterpreterError::InvalidInstructionFormat(frame.pc))),
    }
    
    Ok(3)
}

fn handle_ldc2_w(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let cp = &class.class_file.constant_pool;
    
    match cp.get(index) {
        Some(crate::classfile::constant_pool::CpInfo::Long(v)) => {
            frame.push(Value::Long(*v))?;
        }
        Some(crate::classfile::constant_pool::CpInfo::Double(v)) => {
            frame.push(Value::Double(*v))?;
        }
        _ => return Err(JvmError::InterpreterError(InterpreterError::InvalidInstructionFormat(frame.pc))),
    }
    
    Ok(3)
}

fn handle_iload(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.get_local(index)?.clone();
    frame.push(val)?;
    Ok(2)
}

fn handle_lload(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.get_local(index)?.clone();
    frame.push(val)?;
    Ok(2)
}

fn handle_fload(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.get_local(index)?.clone();
    frame.push(val)?;
    Ok(2)
}

fn handle_dload(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.get_local(index)?.clone();
    frame.push(val)?;
    Ok(2)
}

fn handle_aload(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.get_local(index)?.clone();
    frame.push(val)?;
    Ok(2)
}

fn handle_iload_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(0)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_iload_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(1)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_iload_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(2)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_iload_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(3)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_lload_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(0)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_lload_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(1)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_lload_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(2)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_lload_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(3)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_fload_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(0)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_fload_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(1)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_fload_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(2)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_fload_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(3)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_dload_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(0)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_dload_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(1)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_dload_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(2)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_dload_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(3)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_aload_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(0)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_aload_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(1)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_aload_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(2)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_aload_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.get_local(3)?.clone();
    frame.push(val)?;
    Ok(1)
}

fn handle_istore(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.pop()?;
    frame.set_local(index, val)?;
    Ok(2)
}

fn handle_lstore(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.pop()?;
    frame.set_local(index, val)?;
    Ok(2)
}

fn handle_fstore(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.pop()?;
    frame.set_local(index, val)?;
    Ok(2)
}

fn handle_dstore(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.pop()?;
    frame.set_local(index, val)?;
    Ok(2)
}

fn handle_astore(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let val = frame.pop()?;
    frame.set_local(index, val)?;
    Ok(2)
}

fn handle_istore_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(0, val)?;
    Ok(1)
}

fn handle_istore_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(1, val)?;
    Ok(1)
}

fn handle_istore_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(2, val)?;
    Ok(1)
}

fn handle_istore_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(3, val)?;
    Ok(1)
}

fn handle_astore_0(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(0, val)?;
    Ok(1)
}

fn handle_astore_1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(1, val)?;
    Ok(1)
}

fn handle_astore_2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(2, val)?;
    Ok(1)
}

fn handle_astore_3(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?;
    frame.set_local(3, val)?;
    Ok(1)
}

fn handle_pop(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    Ok(1)
}

fn handle_pop2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pop()?;
    Ok(1)
}

fn handle_dup(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.dup()?;
    Ok(1)
}

fn handle_dup_x1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.dup_x1()?;
    Ok(1)
}

fn handle_dup_x2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.dup_x2()?;
    Ok(1)
}

fn handle_dup2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.dup2()?;
    Ok(1)
}

fn handle_dup2_x1(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.dup2_x1()?;
    Ok(1)
}

fn handle_dup2_x2(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.dup2_x2()?;
    Ok(1)
}

fn handle_swap(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.swap()?;
    Ok(1)
}

fn handle_iadd(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a + b))?;
    Ok(1)
}

fn handle_ladd(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a + b))?;
    Ok(1)
}

fn handle_fadd(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_float();
    let a = frame.pop()?.as_float();
    frame.push(Value::Float(a + b))?;
    Ok(1)
}

fn handle_dadd(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_double();
    let a = frame.pop()?.as_double();
    frame.push(Value::Double(a + b))?;
    Ok(1)
}

fn handle_isub(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a - b))?;
    Ok(1)
}

fn handle_lsub(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a - b))?;
    Ok(1)
}

fn handle_fsub(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_float();
    let a = frame.pop()?.as_float();
    frame.push(Value::Float(a - b))?;
    Ok(1)
}

fn handle_dsub(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_double();
    let a = frame.pop()?.as_double();
    frame.push(Value::Double(a - b))?;
    Ok(1)
}

fn handle_imul(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a * b))?;
    Ok(1)
}

fn handle_lmul(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a * b))?;
    Ok(1)
}

fn handle_fmul(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_float();
    let a = frame.pop()?.as_float();
    frame.push(Value::Float(a * b))?;
    Ok(1)
}

fn handle_dmul(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_double();
    let a = frame.pop()?.as_double();
    frame.push(Value::Double(a * b))?;
    Ok(1)
}

fn handle_idiv(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    if b == 0 {
        return Err(RuntimeError(RuntimeError::ArithmeticException));
    }
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a / b))?;
    Ok(1)
}

fn handle_ldiv(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    if b == 0 {
        return Err(RuntimeError(RuntimeError::ArithmeticException));
    }
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a / b))?;
    Ok(1)
}

fn handle_fdiv(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_float();
    let a = frame.pop()?.as_float();
    frame.push(Value::Float(a / b))?;
    Ok(1)
}

fn handle_ddiv(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_double();
    let a = frame.pop()?.as_double();
    frame.push(Value::Double(a / b))?;
    Ok(1)
}

fn handle_irem(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    if b == 0 {
        return Err(RuntimeError(RuntimeError::ArithmeticException));
    }
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a % b))?;
    Ok(1)
}

fn handle_lrem(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    if b == 0 {
        return Err(RuntimeError(RuntimeError::ArithmeticException));
    }
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a % b))?;
    Ok(1)
}

fn handle_frem(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_float();
    let a = frame.pop()?.as_float();
    frame.push(Value::Float(a % b))?;
    Ok(1)
}

fn handle_drem(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_double();
    let a = frame.pop()?.as_double();
    frame.push(Value::Double(a % b))?;
    Ok(1)
}

fn handle_ineg(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?.as_int();
    frame.push(Value::Int(-val))?;
    Ok(1)
}

fn handle_lneg(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?.as_long();
    frame.push(Value::Long(-val))?;
    Ok(1)
}

fn handle_fneg(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?.as_float();
    frame.push(Value::Float(-val))?;
    Ok(1)
}

fn handle_dneg(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let val = frame.pop()?.as_double();
    frame.push(Value::Double(-val))?;
    Ok(1)
}

fn handle_ishl(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let shift = frame.pop()?.as_int() & 0x1F;
    let val = frame.pop()?.as_int();
    frame.push(Value::Int(val << shift))?;
    Ok(1)
}

fn handle_lshl(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let shift = frame.pop()?.as_int() & 0x3F;
    let val = frame.pop()?.as_long();
    frame.push(Value::Long(val << shift))?;
    Ok(1)
}

fn handle_ishr(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let shift = frame.pop()?.as_int() & 0x1F;
    let val = frame.pop()?.as_int();
    frame.push(Value::Int(val >> shift))?;
    Ok(1)
}

fn handle_lshr(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let shift = frame.pop()?.as_int() & 0x3F;
    let val = frame.pop()?.as_long();
    frame.push(Value::Long(val >> shift))?;
    Ok(1)
}

fn handle_iushr(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let shift = frame.pop()?.as_int() & 0x1F;
    let val = frame.pop()?.as_int();
    frame.push(Value::Int((val as u32 >> shift) as i32))?;
    Ok(1)
}

fn handle_lushr(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let shift = frame.pop()?.as_int() & 0x3F;
    let val = frame.pop()?.as_long();
    frame.push(Value::Long((val as u64 >> shift) as i64))?;
    Ok(1)
}

fn handle_iand(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a & b))?;
    Ok(1)
}

fn handle_land(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a & b))?;
    Ok(1)
}

fn handle_ior(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a | b))?;
    Ok(1)
}

fn handle_lor(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a | b))?;
    Ok(1)
}

fn handle_ixor(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    frame.push(Value::Int(a ^ b))?;
    Ok(1)
}

fn handle_lxor(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let b = frame.pop()?.as_long();
    let a = frame.pop()?.as_long();
    frame.push(Value::Long(a ^ b))?;
    Ok(1)
}

fn handle_iinc(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = code[frame.pc + 1] as usize;
    let inc = code[frame.pc + 2] as i8 as i32;
    let val = frame.get_local(index)?.as_int();
    frame.set_local(index, Value::Int(val + inc))?;
    Ok(3)
}

fn handle_if_icmpeq(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    if a == b {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_if_icmpne(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    if a != b {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_if_icmplt(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    if a < b {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_if_icmpge(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    if a >= b {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_if_icmpgt(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    if a > b {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_if_icmple(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?.as_int();
    let a = frame.pop()?.as_int();
    if a <= b {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_if_acmpeq(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?;
    let a = frame.pop()?;
    if a.is_null() && b.is_null() {
        frame.pc = (frame.pc as i32 + offset) as usize;
    } else if !a.is_null() && !b.is_null() && a.as_ref() == b.as_ref() {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_if_acmpne(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let b = frame.pop()?;
    let a = frame.pop()?;
    if !((a.is_null() && b.is_null()) || (!a.is_null() && !b.is_null() && a.as_ref() == b.as_ref())) {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_goto(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    frame.pc = (frame.pc as i32 + offset) as usize;
    Ok(3)
}

fn handle_ireturn(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pc = frame.method.code.len();
    Ok(1)
}

fn handle_lreturn(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pc = frame.method.code.len();
    Ok(1)
}

fn handle_freturn(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pc = frame.method.code.len();
    Ok(1)
}

fn handle_dreturn(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pc = frame.method.code.len();
    Ok(1)
}

fn handle_areturn(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pc = frame.method.code.len();
    Ok(1)
}

fn handle_return(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pc = frame.method.code.len();
    Ok(1)
}

fn handle_getstatic(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let (class_name, field_name, descriptor) = class.class_file.constant_pool.resolve_field_ref(index)?;
    
    let target_class = jvm.method_area.get_class(&class_name)
        .ok_or(RuntimeError::NoSuchClass(class_name.clone()))?;
    
    if let Some(val) = target_class.static_fields.get(&format!("{}:{}", field_name, descriptor)) {
        frame.push(val.clone())?;
    } else {
        frame.push(Value::Null)?;
    }
    
    Ok(3)
}

fn handle_putstatic(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let (class_name, field_name, descriptor) = class.class_file.constant_pool.resolve_field_ref(index)?;
    
    let target_class = jvm.method_area.get_class_mut(&class_name)
        .ok_or(RuntimeError::NoSuchClass(class_name.clone()))?;
    
    let val = frame.pop()?;
    target_class.static_fields.insert(format!("{}:{}", field_name, descriptor), val);
    
    Ok(3)
}

fn handle_getfield(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let (_class_name, field_name, descriptor) = class.class_file.constant_pool.resolve_field_ref(index)?;
    
    let obj_ref = frame.pop()?;
    if obj_ref.is_null() {
        return Err(RuntimeError(RuntimeError::NullPointerException));
    }
    
    let obj = jvm.heap.get(obj_ref.as_ref())
        .ok_or(RuntimeError::NullPointerException)?;
    
    if let Some(val) = obj.get_field(&format!("{}:{}", field_name, descriptor)) {
        frame.push(val.clone())?;
    } else {
        frame.push(Value::Null)?;
    }
    
    Ok(3)
}

fn handle_putfield(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let (_class_name, field_name, descriptor) = class.class_file.constant_pool.resolve_field_ref(index)?;
    
    let val = frame.pop()?;
    let obj_ref = frame.pop()?;
    if obj_ref.is_null() {
        return Err(RuntimeError(RuntimeError::NullPointerException));
    }
    
    let obj = jvm.heap.get_mut(obj_ref.as_ref())
        .ok_or(RuntimeError::NullPointerException)?;
    
    obj.set_field(&format!("{}:{}", field_name, descriptor), val);
    
    Ok(3)
}

fn handle_invokevirtual(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let (class_name, method_name, descriptor) = class.class_file.constant_pool.resolve_method_ref(index)?;
    
    let target_class = jvm.method_area.get_class(&class_name)
        .ok_or(RuntimeError::NoSuchClass(class_name.clone()))?;
    
    let method = target_class.get_method(&method_name, &descriptor)
        .ok_or(RuntimeError::MethodNotFound(class_name, method_name.clone()))?;
    
    let arg_count = parse_method_descriptor(&descriptor).0;
    let mut args = Vec::with_capacity(arg_count);
    for _ in 0..arg_count {
        args.push(frame.pop()?);
    }
    args.reverse();
    let this_ref = frame.pop()?;
    
    let mut new_frame = Frame::new(method.clone());
    if !method.is_static {
        new_frame.set_local(0, this_ref)?;
        for (i, arg) in args.into_iter().enumerate() {
            new_frame.set_local(i + 1, arg)?;
        }
    } else {
        for (i, arg) in args.into_iter().enumerate() {
            new_frame.set_local(i, arg)?;
        }
    }
    
    jvm.stack.push(new_frame)?;
    
    Ok(3)
}

fn handle_invokespecial(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let (class_name, method_name, descriptor) = class.class_file.constant_pool.resolve_method_ref(index)?;
    
    let target_class = jvm.method_area.get_class(&class_name)
        .ok_or(RuntimeError::NoSuchClass(class_name.clone()))?;
    
    let method = target_class.get_method(&method_name, &descriptor)
        .ok_or(RuntimeError::MethodNotFound(class_name, method_name.clone()))?;
    
    let arg_count = parse_method_descriptor(&descriptor).0;
    for _ in 0..arg_count {
        frame.pop()?;
    }
    let this_ref = frame.pop()?;
    
    let mut new_frame = Frame::new(method.clone());
    if !method.is_static {
        new_frame.set_local(0, this_ref)?;
    }
    
    jvm.stack.push(new_frame)?;
    
    Ok(3)
}

fn handle_invokestatic(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let (class_name, method_name, descriptor) = class.class_file.constant_pool.resolve_method_ref(index)?;
    
    let target_class = jvm.method_area.get_class(&class_name)
        .ok_or(RuntimeError::NoSuchClass(class_name.clone()))?;
    
    let method = target_class.get_method(&method_name, &descriptor)
        .ok_or(RuntimeError::MethodNotFound(class_name, method_name.clone()))?;
    
    let arg_count = parse_method_descriptor(&descriptor).0;
    for _ in 0..arg_count {
        frame.pop()?;
    }
    
    let mut new_frame = Frame::new(method.clone());
    jvm.stack.push(new_frame)?;
    
    Ok(3)
}

fn handle_new(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let class_name = class.class_file.constant_pool.get_class_name(index)
        .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(index)))?;
    
    let target_class = jvm.method_area.get_class(&class_name)
        .ok_or(RuntimeError::NoSuchClass(class_name.clone()))?;
    
    let mut obj = HeapObject::new(class_name.clone());
    for field_key in &target_class.instance_fields {
        obj.fields.insert(field_key.clone(), Value::Null);
    }
    
    let ref_id = jvm.heap.allocate(obj)?;
    frame.push(Value::ObjectRef(ref_id))?;
    
    Ok(3)
}

fn handle_newarray(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let _atype = code[frame.pc + 1];
    let length = frame.pop()?.as_int() as usize;
    
    if length < 0 {
        return Err(RuntimeError(RuntimeError::NegativeArraySize));
    }
    
    let obj = HeapObject::new_array("[[I".to_string(), length);
    let ref_id = jvm.heap.allocate(obj)?;
    frame.push(Value::ArrayRef(ref_id))?;
    
    Ok(2)
}

fn handle_anewarray(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let class_name = class.class_file.constant_pool.get_class_name(index)
        .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(index)))?;
    
    let length = frame.pop()?.as_int() as usize;
    
    if length < 0 {
        return Err(RuntimeError(RuntimeError::NegativeArraySize));
    }
    
    let obj = HeapObject::new_array(format!("[L{};", class_name), length);
    let ref_id = jvm.heap.allocate(obj)?;
    frame.push(Value::ArrayRef(ref_id))?;
    
    Ok(3)
}

fn handle_arraylength(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let arr_ref = frame.pop()?;
    if arr_ref.is_null() {
        return Err(RuntimeError(RuntimeError::NullPointerException));
    }
    
    let obj = jvm.heap.get(arr_ref.as_ref())
        .ok_or(RuntimeError::NullPointerException)?;
    
    frame.push(Value::Int(obj.array_length as i32))?;
    
    Ok(1)
}

fn handle_athrow(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pc = frame.method.code.len();
    Ok(1)
}

fn handle_checkcast(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    Ok(3)
}

fn handle_instanceof(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    frame.pop()?;
    frame.pop()?;
    frame.push(Value::Int(0))?;
    Ok(3)
}

fn handle_monitorenter(_frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    Ok(1)
}

fn handle_monitorexit(_frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    Ok(1)
}

fn handle_multianewarray(frame: &mut Frame, jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let index = u16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as usize;
    let _dims = code[frame.pc + 3];
    
    let class = jvm.method_area.get_class(&frame.method.class_name).unwrap();
    let class_name = class.class_file.constant_pool.get_class_name(index)
        .ok_or(JvmError::ClassFileError(ClassFileError::ConstantPoolIndexOutOfBounds(index)))?;
    
    let length = frame.pop()?.as_int() as usize;
    if length < 0 {
        return Err(RuntimeError(RuntimeError::NegativeArraySize));
    }
    
    let obj = HeapObject::new_array(class_name, length);
    let ref_id = jvm.heap.allocate(obj)?;
    frame.push(Value::ArrayRef(ref_id))?;
    
    Ok(4)
}

fn handle_ifnull(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let ref_val = frame.pop()?;
    if ref_val.is_null() {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

fn handle_ifnonnull(frame: &mut Frame, _jvm: &mut JVM) -> Result<usize> {
    let code = &frame.method.code;
    let offset = i16::from_be_bytes([code[frame.pc + 1], code[frame.pc + 2]]) as i32;
    let ref_val = frame.pop()?;
    if !ref_val.is_null() {
        frame.pc = (frame.pc as i32 + offset) as usize;
    }
    Ok(3)
}

pub fn parse_method_descriptor(descriptor: &str) -> (usize, bool) {
    if !descriptor.starts_with('(') {
        return (0, false);
    }
    
    let mut params = 0;
    let mut i = 1;
    
    while i < descriptor.len() && descriptor.chars().nth(i) != Some(')') {
        match descriptor.chars().nth(i).unwrap() {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' => {
                params += 1;
                i += 1;
            }
            '[' => {
                while descriptor.chars().nth(i) == Some('[') {
                    i += 1;
                }
                if descriptor.chars().nth(i) == Some('L') {
                    while i < descriptor.len() && descriptor.chars().nth(i) != Some(';') {
                        i += 1;
                    }
                    i += 1;
                } else {
                    i += 1;
                }
                params += 1;
            }
            'L' => {
                while i < descriptor.len() && descriptor.chars().nth(i) != Some(';') {
                    i += 1;
                }
                i += 1;
                params += 1;
            }
            _ => i += 1,
        }
    }
    
    i += 1;
    let return_type = descriptor[i..].to_string();
    let has_return = !return_type.is_empty() && return_type != "V";
    
    (params, has_return)
}