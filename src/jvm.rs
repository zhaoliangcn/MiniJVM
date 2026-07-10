use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;

// ==============================================================================
// 1. 堆与对象定义 (Heap & Objects)
// ==============================================================================

/// 堆中的对象实例
#[derive(Debug, Clone)]
pub struct HeapObject {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
    pub string_value: Option<String>,
}

/// 运行时值类型
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ObjectRef(usize), // 指向 Heap 的索引
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Long(v) => write!(f, "{}L", v),
            Value::Float(v) => write!(f, "{}f", v),
            Value::Double(v) => write!(f, "{}d", v),
            Value::ObjectRef(idx) => write!(f, "Ref@{}", idx),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i32 {
        if let Value::Int(v) = self { *v } else { panic!("Type mismatch: expected Int") }
    }
    
    pub fn as_ref(&self) -> usize {
        if let Value::ObjectRef(idx) = self { *idx } else { panic!("Type mismatch: expected Ref") }
    }
}

// ==============================================================================
// 2. 类元数据与常量池 (Metadata)
// ==============================================================================

#[derive(Debug, Clone)]
pub enum CpInfo {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(usize),
    String(usize),
    NameAndType { name_index: usize, descriptor_index: usize },
    FieldRef { class_index: usize, name_and_type_index: usize },
    MethodRef { class_index: usize, name_and_type_index: usize },
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub descriptor: String,
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub descriptor: String,
    pub code: Vec<u8>,
    pub max_stack: usize,
    pub max_locals: usize,
}

#[derive(Debug, Clone)]
pub struct Clazz {
    pub name: String,
    pub super_name: Option<String>,
    pub constant_pool: Vec<CpInfo>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
}

impl Clazz {
    pub fn get_method(&self, name: &str) -> Option<&MethodInfo> {
        self.methods.iter().find(|m| m.name == name)
    }

    pub fn get_cp_utf8(&self, index: usize) -> Option<String> {
        match self.constant_pool.get(index) {
            Some(CpInfo::Utf8(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn resolve_field_ref(&self, field_ref_index: usize) -> Option<(String, String, String)> {
        if let Some(CpInfo::FieldRef { class_index, name_and_type_index }) = self.constant_pool.get(field_ref_index) {
            let class_name = if let Some(CpInfo::Class(name_idx)) = self.constant_pool.get(*class_index) {
                self.get_cp_utf8(*name_idx)?
            } else {
                return None;
            };
            if let Some(CpInfo::NameAndType { name_index, descriptor_index }) = self.constant_pool.get(*name_and_type_index) {
                let field_name = self.get_cp_utf8(*name_index)?;
                let descriptor = self.get_cp_utf8(*descriptor_index)?;
                return Some((class_name, field_name, descriptor));
            }
        }
        None
    }

    pub fn resolve_method_ref(&self, method_ref_index: usize) -> Option<(String, String, String)> {
        if let Some(CpInfo::MethodRef { class_index, name_and_type_index }) = self.constant_pool.get(method_ref_index) {
            let class_name = if let Some(CpInfo::Class(name_idx)) = self.constant_pool.get(*class_index) {
                self.get_cp_utf8(*name_idx)?
            } else {
                return None;
            };
            if let Some(CpInfo::NameAndType { name_index, descriptor_index }) = self.constant_pool.get(*name_and_type_index) {
                let method_name = self.get_cp_utf8(*name_index)?;
                let descriptor = self.get_cp_utf8(*descriptor_index)?;
                return Some((class_name.replace('/', "."), method_name, descriptor));
            }
        }
        None
    }
}

// ==============================================================================
// 3. 栈帧 (Frame)
// ==============================================================================

#[derive(Debug, Clone)]
pub struct Frame {
    pub method_name: String,
    pub local_variables: Vec<Value>,
    pub operand_stack: Vec<Value>,
    pub pc: usize,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize, method_name: String) -> Self {
        Frame {
            method_name,
            local_variables: vec![Value::Null; max_locals],
            operand_stack: Vec::with_capacity(max_stack),
            pc: 0,
        }
    }
    pub fn push_operand(&mut self, value: Value) {
        self.operand_stack.push(value);
    }
    pub fn pop_operand(&mut self) -> Value {
        self.operand_stack.pop().expect("Stack underflow")
    }
}

// ==============================================================================
// 4. 执行引擎 (Execution Engine with Heap)
// ==============================================================================

pub struct ExecutionContext {
    class: Rc<Clazz>,
    code: Vec<u8>,
}

pub struct JVM {
    pub classes: HashMap<String, Rc<Clazz>>,
    pub call_stack: Vec<Frame>,
    pub heap: Vec<Option<HeapObject>>,
    pub context_stack: Vec<ExecutionContext>,
}

impl JVM {
    pub fn new() -> Self {
        JVM {
            classes: HashMap::new(),
            call_stack: Vec::new(),
            heap: Vec::new(),
            context_stack: Vec::new(),
        }
    }

    pub fn load_class(&mut self, clazz: Clazz) {
        let name = clazz.name.clone();
        self.classes.insert(name, Rc::new(clazz));
    }

    pub fn run(&mut self, class_name: &str, method_name: &str) {
        let clazz = self.classes.get(class_name)
            .expect("Class not found")
            .clone();
        
        let method = clazz.get_method(method_name)
            .expect("Method not found")
            .clone();

        let frame = Frame::new(method.max_locals, method.max_stack, method.name.clone());
        self.call_stack.push(frame);

        self.context_stack.push(ExecutionContext {
            class: clazz,
            code: method.code,
        });

        self.execute_loop();
    }

    fn execute_loop(&mut self) {
        while let Some(context) = self.context_stack.last() {
            let code = &context.code;
            let current_class = &context.class;

            let frame = match self.call_stack.last_mut() {
                Some(f) => f,
                None => {
                    self.context_stack.pop();
                    continue;
                }
            };

            if frame.pc >= code.len() {
                self.call_stack.pop();
                self.context_stack.pop();
                continue;
            }

            let opcode_pos = frame.pc;
            let opcode = code[frame.pc];
            frame.pc += 1;

            match opcode {
                0x01 => frame.push_operand(Value::Null),
                0x02 => frame.push_operand(Value::Int(-1)),
                0x03 => frame.push_operand(Value::Int(0)),
                0x04 => frame.push_operand(Value::Int(1)),
                0x05 => frame.push_operand(Value::Int(2)),
                0x06 => frame.push_operand(Value::Int(3)),
                0x07 => frame.push_operand(Value::Int(4)),
                0x08 => frame.push_operand(Value::Int(5)),
                0x10 => {
                    let val = code[frame.pc] as i8 as i32;
                    frame.pc += 1;
                    frame.push_operand(Value::Int(val));
                }
                0x11 => {
                    let val = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    frame.push_operand(Value::Int(val));
                }
                0x12 => {
                    let index = code[frame.pc] as usize;
                    frame.pc += 1;
                    
                    match current_class.constant_pool.get(index) {
                        Some(CpInfo::Integer(val)) => {
                            frame.push_operand(Value::Int(*val));
                        }
                        Some(CpInfo::Float(val)) => {
                            frame.push_operand(Value::Float(*val));
                        }
                        Some(CpInfo::String(str_idx)) => {
                            if let Some(CpInfo::Utf8(s)) = current_class.constant_pool.get(*str_idx) {
                                let obj = HeapObject {
                                    class_name: "java.lang.String".to_string(),
                                    fields: HashMap::new(),
                                    string_value: Some(s.clone()),
                                };
                                let ref_idx = self.heap.len();
                                self.heap.push(Some(obj));
                                frame.push_operand(Value::ObjectRef(ref_idx));
                            } else {
                                eprintln!("Warning: Invalid string index {}", str_idx);
                                frame.push_operand(Value::Null);
                            }
                        }
                        Some(CpInfo::Class(name_idx)) => {
                            if let Some(CpInfo::Utf8(class_name)) = current_class.constant_pool.get(*name_idx) {
                                let class_name = class_name.replace('/', ".");
                                if !self.classes.contains_key(&class_name) {
                                    eprintln!("Warning: Class {} not loaded", class_name);
                                }
                                frame.push_operand(Value::Null);
                            }
                        }
                        _ => {
                            eprintln!("Warning: Unsupported ldc type at index {}", index);
                            frame.push_operand(Value::Null);
                        }
                    }
                }
                0x13 => {
                    let index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;
                    match current_class.constant_pool.get(index) {
                        Some(CpInfo::Integer(val)) => {
                            frame.push_operand(Value::Int(*val));
                        }
                        Some(CpInfo::Float(val)) => {
                            frame.push_operand(Value::Float(*val));
                        }
                        Some(CpInfo::Long(val)) => {
                            frame.push_operand(Value::Long(*val));
                        }
                        Some(CpInfo::Double(val)) => {
                            frame.push_operand(Value::Double(*val));
                        }
                        Some(CpInfo::String(str_idx)) => {
                            if let Some(CpInfo::Utf8(s)) = current_class.constant_pool.get(*str_idx) {
                                let obj = HeapObject {
                                    class_name: "java.lang.String".to_string(),
                                    fields: HashMap::new(),
                                    string_value: Some(s.clone()),
                                };
                                let ref_idx = self.heap.len();
                                self.heap.push(Some(obj));
                                frame.push_operand(Value::ObjectRef(ref_idx));
                            } else {
                                eprintln!("Warning: Invalid string index {}", str_idx);
                                frame.push_operand(Value::Null);
                            }
                        }
                        Some(CpInfo::Class(name_idx)) => {
                            if let Some(CpInfo::Utf8(class_name)) = current_class.constant_pool.get(*name_idx) {
                                let class_name = class_name.replace('/', ".");
                                if !self.classes.contains_key(&class_name) {
                                    eprintln!("Warning: Class {} not loaded", class_name);
                                }
                                frame.push_operand(Value::Null);
                            }
                        }
                        _ => {
                            eprintln!("Warning: Unsupported ldc_w type at index {}", index);
                            frame.push_operand(Value::Null);
                        }
                    }
                }
                0x1A => frame.push_operand(frame.local_variables[0].clone()),
                0x1B => frame.push_operand(frame.local_variables[1].clone()),
                0x1C => frame.push_operand(frame.local_variables[2].clone()),
                0x1D => frame.push_operand(frame.local_variables[3].clone()),
                0x15 => {
                    let index = code[frame.pc] as usize;
                    frame.pc += 1;
                    frame.push_operand(frame.local_variables[index].clone());
                }
                0x2A => frame.push_operand(frame.local_variables[0].clone()),
                0x2B => frame.push_operand(frame.local_variables[1].clone()),
                0x2C => frame.push_operand(frame.local_variables[2].clone()),
                0x2D => frame.push_operand(frame.local_variables[3].clone()),
                0x3B => {
                    let val = frame.pop_operand();
                    frame.local_variables[0] = val;
                }
                0x3C => {
                    let val = frame.pop_operand();
                    frame.local_variables[1] = val;
                }
                0x3D => {
                    let val = frame.pop_operand();
                    frame.local_variables[2] = val;
                }
                0x3E => {
                    let val = frame.pop_operand();
                    frame.local_variables[3] = val;
                }
                0x36 => {
                    let index = code[frame.pc] as usize;
                    frame.pc += 1;
                    let val = frame.pop_operand();
                    frame.local_variables[index] = val;
                }
                0x4B => {
                    let val = frame.pop_operand();
                    frame.local_variables[0] = val;
                }
                0x4C => {
                    let val = frame.pop_operand();
                    frame.local_variables[1] = val;
                }
                0x4D => {
                    let val = frame.pop_operand();
                    frame.local_variables[2] = val;
                }
                0x4E => {
                    let val = frame.pop_operand();
                    frame.local_variables[3] = val;
                }
                0x59 => {
                    let val = frame.pop_operand();
                    frame.push_operand(val.clone());
                    frame.push_operand(val);
                }
                0xBB => {
                    let class_index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;
                    
                    let class_name = if let Some(CpInfo::Class(name_idx)) = current_class.constant_pool.get(class_index) {
                        current_class.get_cp_utf8(*name_idx).unwrap_or("Unknown".to_string())
                    } else {
                        "Unknown".to_string()
                    };

                    let obj = HeapObject {
                        class_name: class_name.clone(),
                        fields: HashMap::new(),
                        string_value: None,
                    };
                    
                    let ref_idx = self.heap.len();
                    self.heap.push(Some(obj));
                    frame.push_operand(Value::ObjectRef(ref_idx));
                }
                0xB5 => {
                    let field_ref_index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;

                    let value = frame.pop_operand();
                    let obj_ref = frame.pop_operand().as_ref();

                    if let Some((_, field_name, _)) = current_class.resolve_field_ref(field_ref_index) {
                        if let Some(Some(obj)) = self.heap.get_mut(obj_ref) {
                            obj.fields.insert(field_name, value);
                        } else {
                            panic!("NullPointerException");
                        }
                    }
                }
                0xB4 => {
                    let field_ref_index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;

                    let obj_ref = frame.pop_operand().as_ref();

                    if let Some((_, field_name, _)) = current_class.resolve_field_ref(field_ref_index) {
                        if let Some(Some(obj)) = self.heap.get(obj_ref) {
                            let val = obj.fields.get(&field_name).cloned().unwrap_or(Value::Null);
                            frame.push_operand(val);
                        } else {
                            panic!("NullPointerException");
                        }
                    }
                }
                0x60 => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Value::Int(a.as_int() + b.as_int()));
                }
                0x64 => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Value::Int(a.as_int() - b.as_int()));
                }
                0x68 => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Value::Int(a.as_int() * b.as_int()));
                }
                0x6C => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Value::Int(a.as_int() / b.as_int()));
                }
                0x70 => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Value::Int(a.as_int() % b.as_int()));
                }
                0x74 => {
                    let a = frame.pop_operand();
                    frame.push_operand(Value::Int(-a.as_int()));
                }
                0xB2 => {
                    let field_ref_index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;
                    
                    if let Some((class_name, field_name, _)) = current_class.resolve_field_ref(field_ref_index) {
                        let class_name_dotted = class_name.replace('/', ".");
                        if class_name_dotted == "java.lang.System" && field_name == "out" {
                            let obj = HeapObject {
                                class_name: "java.io.PrintStream".to_string(),
                                fields: HashMap::new(),
                                string_value: None,
                            };
                            let ref_idx = self.heap.len();
                            self.heap.push(Some(obj));
                            frame.push_operand(Value::ObjectRef(ref_idx));
                        } else {
                            eprintln!("Warning: Unsupported static field: {}.{}", class_name, field_name);
                            frame.push_operand(Value::Null);
                        }
                    }
                }
                0xB8 => {
                    let method_ref_index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;
                    
                    if let Some((class_name, method_name, descriptor)) = current_class.resolve_method_ref(method_ref_index) {
                        self.do_invoke_static(&class_name, &method_name, &descriptor);
                    }
                }
                0xB6 => {
                    let method_ref_index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;
                    
                    if let Some((class_name, method_name, descriptor)) = current_class.resolve_method_ref(method_ref_index) {
                        self.do_invoke_virtual(&class_name, &method_name, &descriptor);
                    }
                }
                0xB7 => {
                    let method_ref_index = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as usize;
                    frame.pc += 2;
                    
                    if let Some((class_name, method_name, descriptor)) = current_class.resolve_method_ref(method_ref_index) {
                        self.do_invoke_special(&class_name, &method_name, &descriptor);
                    }
                }
                0xB1 => {
                    self.call_stack.pop();
                    self.context_stack.pop();
                }
                0xAC => {
                    let ret_val = self.call_stack.last_mut().unwrap().pop_operand();
                    self.call_stack.pop();
                    self.context_stack.pop();
                    if let Some(frame) = self.call_stack.last_mut() {
                        frame.push_operand(ret_val);
                    }
                }
                0xC6 => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    frame.pc = ((frame.pc as i32) + offset) as usize;
                }
                0x57 => {
                    frame.pop_operand();
                }
                0x84 => {
                    let index = code[frame.pc] as usize;
                    let increment = code[frame.pc + 1] as i8 as i32;
                    frame.pc += 2;
                    if let Value::Int(v) = frame.local_variables[index] {
                        frame.local_variables[index] = Value::Int(v + increment);
                    }
                }
                0x9B => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let a = frame.pop_operand().as_int();
                    if a < 0 {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0x9C => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let a = frame.pop_operand().as_int();
                    if a >= 0 {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0x9D => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let a = frame.pop_operand().as_int();
                    if a > 0 {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0x9E => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let b = frame.pop_operand().as_int();
                    let a = frame.pop_operand().as_int();
                    if a != b {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0x9F => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let b = frame.pop_operand().as_int();
                    let a = frame.pop_operand().as_int();
                    if a == b {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0xA1 => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let b = frame.pop_operand().as_int();
                    let a = frame.pop_operand().as_int();
                    if a < b {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0xA2 => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let b = frame.pop_operand().as_int();
                    let a = frame.pop_operand().as_int();
                    if a >= b {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0xA3 => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let b = frame.pop_operand().as_int();
                    let a = frame.pop_operand().as_int();
                    if a > b {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0xA4 => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let b = frame.pop_operand().as_int();
                    let a = frame.pop_operand().as_int();
                    if a <= b {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0xA5 => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    let b = frame.pop_operand().as_ref();
                    let a = frame.pop_operand().as_ref();
                    if a == b {
                        frame.pc = ((opcode_pos as i32) + offset) as usize;
                    }
                }
                0xA7 => {
                    let offset = ((code[frame.pc] as u16) << 8 | code[frame.pc + 1] as u16) as i16 as i32;
                    frame.pc += 2;
                    frame.pc = ((opcode_pos as i32) + offset) as usize;
                }
                0xB0 => {
                    let ret_val = self.call_stack.last_mut().unwrap().pop_operand();
                    self.call_stack.pop();
                    self.context_stack.pop();
                    if let Some(frame) = self.call_stack.last_mut() {
                        frame.push_operand(ret_val);
                    }
                }
                _ => {
                    eprintln!("Unknown opcode: 0x{:02X} at PC {}", opcode, frame.pc - 1);
                }
            }
        }
    }

    fn do_invoke_static(&mut self, class_name: &str, method_name: &str, descriptor: &str) {
        if class_name == "java.lang.System" && method_name == "out" {
            return;
        }
        
        if let Some(clazz) = self.classes.get(class_name) {
            if let Some(method) = clazz.methods.iter().find(|m| m.name == method_name && m.descriptor == descriptor) {
                let new_frame = Frame::new(method.max_locals, method.max_stack, method.name.clone());
                self.call_stack.push(new_frame);
                
                self.context_stack.push(ExecutionContext {
                    class: clazz.clone(),
                    code: method.code.clone(),
                });
            } else {
                eprintln!("Method not found: {}.{} {}", class_name, method_name, descriptor);
            }
        } else {
            eprintln!("Class not found: {}", class_name);
        }
    }

    fn do_invoke_virtual(&mut self, class_name: &str, method_name: &str, descriptor: &str) {
        if class_name == "java.io.PrintStream" && method_name == "println" {
            self.do_handle_println(descriptor);
            return;
        }
        
        if let Some(frame) = self.call_stack.last_mut() {
            let arg_count = JVM::count_method_args(descriptor);
            
            let mut args = Vec::with_capacity(arg_count);
            for _ in 0..arg_count {
                args.push(frame.pop_operand());
            }
            args.reverse();
            
            let obj_ref = frame.pop_operand().as_ref();
            
            if let Some(Some(obj)) = self.heap.get(obj_ref) {
                let actual_class_name = &obj.class_name;
                
                if let Some(clazz) = self.classes.get(actual_class_name) {
                    if let Some(method) = clazz.methods.iter().find(|m| m.name == method_name && m.descriptor == descriptor) {
                        let mut new_frame = Frame::new(method.max_locals, method.max_stack, method.name.clone());
                        new_frame.local_variables[0] = Value::ObjectRef(obj_ref);
                        for i in 0..args.len() {
                            if i + 1 < new_frame.local_variables.len() {
                                new_frame.local_variables[i + 1] = args[i].clone();
                            }
                        }
                        self.call_stack.push(new_frame);
                        
                        self.context_stack.push(ExecutionContext {
                            class: clazz.clone(),
                            code: method.code.clone(),
                        });
                    } else {
                        eprintln!("Method not found: {}.{} {}", actual_class_name, method_name, descriptor);
                    }
                } else {
                    eprintln!("Class not found: {}", actual_class_name);
                }
            }
        }
    }

    fn do_invoke_special(&mut self, class_name: &str, method_name: &str, descriptor: &str) {
        if let Some(frame) = self.call_stack.last_mut() {
            let arg_count = JVM::count_method_args(descriptor);
            
            let mut args = Vec::with_capacity(arg_count);
            for _ in 0..arg_count {
                args.push(frame.pop_operand());
            }
            args.reverse();
            
            let this_val = frame.pop_operand();
            let obj_ref = this_val.as_ref();
            
            if method_name == "<init>" {
                if let Some(Some(obj)) = self.heap.get(obj_ref) {
                    let actual_class_name = &obj.class_name;
                    
                    if class_name == "java.lang.Object" {
                        return;
                    }
                    
                    if let Some(clazz) = self.classes.get(actual_class_name) {
                        if let Some(method) = clazz.methods.iter().find(|m| m.name == method_name && m.descriptor == descriptor) {
                            let mut new_frame = Frame::new(method.max_locals, method.max_stack, method.name.clone());
                            new_frame.local_variables[0] = Value::ObjectRef(obj_ref);
                            for i in 0..args.len() {
                                if i + 1 < new_frame.local_variables.len() {
                                    new_frame.local_variables[i + 1] = args[i].clone();
                                }
                            }
                            self.call_stack.push(new_frame);
                            
                            self.context_stack.push(ExecutionContext {
                                class: clazz.clone(),
                                code: method.code.clone(),
                            });
                        } else {
                            eprintln!("Constructor not found: {}.{} {}", actual_class_name, method_name, descriptor);
                        }
                    } else {
                        eprintln!("Class not found: {}", actual_class_name);
                    }
                }
            }
        }
    }

    fn count_method_args(descriptor: &str) -> usize {
        let mut count = 0;
        let mut chars = descriptor.chars().skip(1);
        
        while let Some(c) = chars.next() {
            match c {
                ')' => break,
                'J' | 'D' => count += 2,
                'L' => {
                    count += 1;
                    while let Some(ch) = chars.next() {
                        if ch == ';' { break; }
                    }
                }
                _ => count += 1,
            }
        }
        
        count
    }

    fn do_handle_println(&mut self, descriptor: &str) {
        if let Some(frame) = self.call_stack.last_mut() {
            match descriptor {
                "(I)V" => {
                    let val = frame.pop_operand().as_int();
                    frame.pop_operand();
                    println!("{}", val);
                }
                "(Ljava/lang/String;)V" => {
                    let obj_ref = frame.pop_operand().as_ref();
                    frame.pop_operand();
                    if let Some(Some(obj)) = self.heap.get(obj_ref) {
                        if let Some(s) = &obj.string_value {
                            println!("{}", s);
                        } else {
                            println!("String@{}", obj_ref);
                        }
                    } else {
                        println!("null");
                    }
                }
                _ => {
                    eprintln!("Unsupported println descriptor: {}", descriptor);
                }
            }
        }
    }
}

// ==============================================================================
// 5. 类文件解析器 (Class File Parser)
// ==============================================================================

pub struct ClassFileParser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ClassFileParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ClassFileParser { data, pos: 0 }
    }

    fn read_u8(&mut self) -> u8 {
        let val = self.data[self.pos];
        self.pos += 1;
        val
    }

    fn read_u16(&mut self) -> u16 {
        let val = ((self.data[self.pos] as u16) << 8) | (self.data[self.pos + 1] as u16);
        self.pos += 2;
        val
    }

    fn read_u32(&mut self) -> u32 {
        let val = ((self.data[self.pos] as u32) << 24)
            | ((self.data[self.pos + 1] as u32) << 16)
            | ((self.data[self.pos + 2] as u32) << 8)
            | (self.data[self.pos + 3] as u32);
        self.pos += 4;
        val
    }

    fn read_bytes(&mut self, len: usize) -> &'a [u8] {
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        slice
    }

    fn read_utf8(&mut self, len: usize) -> String {
        let bytes = self.read_bytes(len);
        String::from_utf8_lossy(bytes).to_string()
    }

    pub fn parse(&mut self) -> Result<Clazz, String> {
        self.parse_magic()?;
        self.parse_version()?;
        
        let constant_pool = self.parse_constant_pool()?;
        let _access_flags = self.read_u16();
        let this_class_index = self.read_u16() as usize;
        let super_class_index = self.read_u16() as usize;
        
        let class_name = if let Some(CpInfo::Class(name_idx)) = constant_pool.get(this_class_index) {
            if let Some(CpInfo::Utf8(s)) = constant_pool.get(*name_idx) {
                s.replace('/', ".")
            } else {
                return Err("Invalid this_class name index".to_string());
            }
        } else {
            return Err("Invalid this_class index".to_string());
        };
        
        let super_name = if super_class_index != 0 {
            if let Some(CpInfo::Class(name_idx)) = constant_pool.get(super_class_index) {
                if let Some(CpInfo::Utf8(s)) = constant_pool.get(*name_idx) {
                    Some(s.replace('/', "."))
                } else {
                    return Err("Invalid super_class name index".to_string());
                }
            } else {
                return Err("Invalid super_class index".to_string());
            }
        } else {
            None
        };

        self.parse_interfaces()?;
        let fields = self.parse_fields(&constant_pool)?;
        let methods = self.parse_methods(&constant_pool)?;

        Ok(Clazz {
            name: class_name,
            super_name,
            constant_pool,
            fields,
            methods,
        })
    }

    fn parse_magic(&mut self) -> Result<(), String> {
        let magic = self.read_u32();
        if magic != 0xCAFEBABE {
            return Err(format!("Invalid magic number: 0x{:08X}", magic));
        }
        Ok(())
    }

    fn parse_version(&mut self) -> Result<(), String> {
        let minor = self.read_u16();
        let major = self.read_u16();
        if major < 45 || major > 65 {
            return Err(format!("Unsupported class version: {}.{}", major, minor));
        }
        Ok(())
    }

    fn parse_constant_pool(&mut self) -> Result<Vec<CpInfo>, String> {
        let count = self.read_u16() as usize;
        let mut pool = vec![CpInfo::Utf8("".to_string()); count];
        
        let mut i = 1;
        while i < count {
            let tag = self.read_u8();
            pool[i] = match tag {
                1 => {
                    let len = self.read_u16() as usize;
                    CpInfo::Utf8(self.read_utf8(len))
                }
                3 => {
                    let val = self.read_u32() as i32;
                    CpInfo::Integer(val)
                }
                4 => {
                    let val = f32::from_bits(self.read_u32());
                    CpInfo::Float(val)
                }
                5 => {
                    let high = self.read_u32();
                    let low = self.read_u32();
                    let val = ((high as u64) << 32) | (low as u64);
                    CpInfo::Long(val as i64)
                }
                6 => {
                    let high = self.read_u32();
                    let low = self.read_u32();
                    let val = ((high as u64) << 32) | (low as u64);
                    CpInfo::Double(f64::from_bits(val))
                }
                7 => {
                    let name_index = self.read_u16() as usize;
                    CpInfo::Class(name_index)
                }
                8 => {
                    let string_index = self.read_u16() as usize;
                    CpInfo::String(string_index)
                }
                9 => {
                    let class_index = self.read_u16() as usize;
                    let name_and_type_index = self.read_u16() as usize;
                    CpInfo::FieldRef { class_index, name_and_type_index }
                }
                10 => {
                    let class_index = self.read_u16() as usize;
                    let name_and_type_index = self.read_u16() as usize;
                    CpInfo::MethodRef { class_index, name_and_type_index }
                }
                12 => {
                    let name_index = self.read_u16() as usize;
                    let descriptor_index = self.read_u16() as usize;
                    CpInfo::NameAndType { name_index, descriptor_index }
                }
                15 => {
                    let _reference_kind = self.read_u8();
                    let _reference_index = self.read_u16();
                    eprintln!("Warning: MethodHandle (tag 15) not supported");
                    CpInfo::Utf8("MethodHandle".to_string())
                }
                18 => {
                    let _descriptor_index = self.read_u16();
                    eprintln!("Warning: MethodType (tag 18) not supported");
                    CpInfo::Utf8("MethodType".to_string())
                }
                22 => {
                    let _bootstrap_method_attr_index = self.read_u16();
                    let _name_and_type_index = self.read_u16();
                    eprintln!("Warning: InvokeDynamic (tag 22) not supported");
                    CpInfo::Utf8("InvokeDynamic".to_string())
                }
                _ => {
                    eprintln!("Warning: Unsupported constant pool tag: {}", tag);
                    CpInfo::Utf8("Unsupported".to_string())
                }
            };
            
            if tag == 5 || tag == 6 {
                pool[i + 1] = CpInfo::Utf8("".to_string());
                i += 1;
            }
            i += 1;
        }
        
        Ok(pool)
    }

    fn parse_interfaces(&mut self) -> Result<(), String> {
        let count = self.read_u16() as usize;
        for _ in 0..count {
            self.read_u16();
        }
        Ok(())
    }

    fn parse_fields(&mut self, cp: &Vec<CpInfo>) -> Result<Vec<FieldInfo>, String> {
        let count = self.read_u16() as usize;
        let mut fields = Vec::with_capacity(count);
        
        for _ in 0..count {
            let _access_flags = self.read_u16();
            let name_index = self.read_u16() as usize;
            let descriptor_index = self.read_u16() as usize;
            
            let name = if let Some(CpInfo::Utf8(s)) = cp.get(name_index) {
                s.clone()
            } else {
                return Err("Invalid field name index".to_string());
            };
            
            let descriptor = if let Some(CpInfo::Utf8(s)) = cp.get(descriptor_index) {
                s.clone()
            } else {
                return Err("Invalid field descriptor index".to_string());
            };
            
            let _attributes_count = self.read_u16();
            for _ in 0.._attributes_count {
                let _attr_name_index = self.read_u16();
                let _attr_len = self.read_u32();
                self.pos += _attr_len as usize;
            }
            
            fields.push(FieldInfo { name, descriptor });
        }
        
        Ok(fields)
    }

    fn parse_methods(&mut self, cp: &Vec<CpInfo>) -> Result<Vec<MethodInfo>, String> {
        let count = self.read_u16() as usize;
        let mut methods = Vec::with_capacity(count);
        
        for _ in 0..count {
            let _access_flags = self.read_u16();
            let name_index = self.read_u16() as usize;
            let descriptor_index = self.read_u16() as usize;
            
            let name = if let Some(CpInfo::Utf8(s)) = cp.get(name_index) {
                s.clone()
            } else {
                return Err("Invalid method name index".to_string());
            };
            
            let descriptor = if let Some(CpInfo::Utf8(s)) = cp.get(descriptor_index) {
                s.clone()
            } else {
                return Err("Invalid method descriptor index".to_string());
            };
            
            let attributes_count = self.read_u16();
            let mut code = Vec::new();
            let mut max_stack = 0;
            let mut max_locals = 0;
            
            for _ in 0..attributes_count {
                let attr_name_index = self.read_u16();
                let attr_len = self.read_u32();
                
                if let Some(CpInfo::Utf8(s)) = cp.get(attr_name_index as usize) {
                    if s == "Code" {
                        max_stack = self.read_u16() as usize;
                        max_locals = self.read_u16() as usize;
                        let code_len = self.read_u32() as usize;
                        code = self.read_bytes(code_len).to_vec();
                        let exception_table_len = self.read_u16();
                        for _ in 0..exception_table_len {
                            self.pos += 8;
                        }
                        let attributes_count = self.read_u16();
                        for _ in 0..attributes_count {
                            let _n = self.read_u16();
                            let _l = self.read_u32();
                            self.pos += _l as usize;
                        }
                    } else {
                        self.pos += attr_len as usize;
                    }
                } else {
                    self.pos += attr_len as usize;
                }
            }
            
            methods.push(MethodInfo {
                name,
                descriptor,
                code,
                max_stack,
                max_locals,
            });
        }
        
        Ok(methods)
    }
}

// ==============================================================================
// 6. 测试入口
// ==============================================================================

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: minijvm <class_file.class>");
        std::process::exit(1);
    }

    let class_file_path = &args[1];
    println!("Loading class file: {}", class_file_path);

    let data = match std::fs::read(class_file_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading class file: {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = ClassFileParser::new(&data);
    let clazz = match parser.parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing class file: {}", e);
            std::process::exit(1);
        }
    };

    println!("Loaded class: {}", clazz.name);
    println!("Methods: {}", clazz.methods.len());
    for m in &clazz.methods {
        println!("  - {} {}", m.name, m.descriptor);
    }

    let mut jvm = JVM::new();
    jvm.load_class(clazz);
    
    if let Err(e) = run_main(&mut jvm) {
        eprintln!("Execution error: {}", e);
        std::process::exit(1);
    }

    println!("--- Execution Finished ---");
}

fn run_main(jvm: &mut JVM) -> Result<(), String> {
    let class_name = if let Some((name, _)) = jvm.classes.iter().next() {
        name.clone()
    } else {
        return Err("No class loaded".to_string());
    };

    jvm.run(&class_name, "main");
    Ok(())
}