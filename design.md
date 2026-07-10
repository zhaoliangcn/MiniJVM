这是一个非常棒的选择。Rust 的所有权模型、零成本抽象和内存安全性使其成为编写高性能系统软件（如虚拟机、浏览器引擎、操作系统）的理想语言。

以下是为您定制的 **《Rust 实现轻量级 Java 虚拟机 (MiniJVM) 架构设计文档》**。

---

# MiniJVM 架构设计文档 (v0.1)

## 1. 项目概述
**目标**：使用 Rust 实现一个符合 JVM 规范子集的轻量级虚拟机，能够加载标准的 `.class` 文件，解析字节码，并在解释器模式下执行简单的 Java 程序。

**核心原则**：
*   **安全性**：利用 Rust 的借用检查器防止内存泄漏和悬垂指针。
*   **模块化**：类加载、运行时数据区、执行引擎解耦。
*   **规范性**：严格遵循 Oracle JVM Specification (Java SE 17/21) 的核心章节。

## 2. 系统架构图

```mermaid
graph TD
    User[用户 Java 代码] -->|javac| ClassFile[.class 二进制文件]
    
    subgraph "MiniJVM Core"
        direction TB
        
        subgraph "Class Loader Subsystem"
            CL[Class Loader]
            Parser[Class File Parser]
            CP[Constant Pool Resolver]
            CL --> Parser
            Parser --> CP
        end
        
        subgraph "Runtime Data Areas"
            Heap[Heap (GC Managed)]
            MethodArea[Method Area / Class Metadata]
            Stack[Java VM Stack]
            
            Stack --> Frame[Stack Frame]
            Frame --> LV[Local Variables]
            Frame --> OS[Operand Stack]
            Frame --> PC[Program Counter]
        end
        
        subgraph "Execution Engine"
            Interpreter[Interpreter Loop]
            NativeLib[Native Interface Stub]
            
            Interpreter --> |Read Opcode| InstructionSet[Instruction Dispatch]
            InstructionSet --> |Update State| Stack
            InstructionSet --> |Alloc/Access| Heap
        end
    end
    
    ClassFile --> CL
    CL --> |Store Metadata| MethodArea
    Interpreter --> |Lookup Class| MethodArea
```

## 3. 模块详细设计

### 3.1 类加载子系统 (Class Loading Subsystem)

负责读取二进制 `.class` 文件并将其转换为 Rust 内部数据结构。

*   **输入**：`Vec<u8>` (字节流)
*   **核心组件**：
    *   `ClassFileParser`: 使用 `nom` 或手动解析二进制结构。需处理大端序 (Big-Endian)。
    *   `ConstantPool`: 存储字符串、类引用、方法引用等。在 Rust 中建议使用 `Vec<ConstantPoolEntry>` 枚举。
    *   `Clazz`: 表示一个已加载的类，包含字段、方法、父类信息。

*   **关键数据结构 (Rust)**:
    ```rust
    pub struct Clazz {
        pub name: String,
        pub super_name: Option<String>,
        pub constant_pool: ConstantPool,
        pub fields: Vec<FieldInfo>,
        pub methods: Vec<MethodInfo>,
        pub access_flags: AccessFlags,
    }
    
    pub enum ConstantPoolEntry {
        Utf8(String),
        Integer(i32),
        Float(f32),
        Long(i64),
        Double(f64),
        ClassRef(usize), // Index to Utf8
        StringRef(usize),
        MethodRef(ClassRefIndex, NameAndTypeIndex),
        // ... other types
    }
    ```

### 3.2 运行时数据区 (Runtime Data Areas)

#### A. 堆 (Heap)
用于存放对象实例和数组。
*   **设计策略**：初期可使用 `Vec<GcObject>` 模拟，后续引入标记-清除 (Mark-Sweep) 或 引用计数。
*   **对象布局**：
    ```rust
    pub struct Object {
        pub class_id: usize, // 指向 MethodArea 中的类定义
        pub fields: Vec<Value>, // 实例字段值
    }
    
    pub enum Value {
        Int(i32),
        Long(i64),
        Float(f32),
        Double(f64),
        ObjectRef(usize), // 堆索引
        Null,
    }
    ```

#### B. Java 虚拟机栈 (JVM Stack)
线程私有，生命周期与线程相同。由多个 **栈帧 (Frame)** 组成。

*   **栈帧 (Frame)**:
    ```rust
    pub struct Frame {
        pub method: Arc<MethodInfo>, // 引用方法元数据
        pub local_variables: Vec<Value>,
        pub operand_stack: Vec<Value>,
        pub pc: usize, // 当前字节码偏移量
    }
    ```
*   **栈管理**: 使用 `Vec<Frame>` 作为调用栈。

#### C. 方法区 (Method Area)
存储类结构信息、静态变量、常量池。
*   **实现**: 使用 `HashMap<String, Arc<Clazz>>` 存储已加载的类，确保线程安全（若支持多线程）或单线程独占。

### 3.3 执行引擎 (Execution Engine)

核心是一个巨大的 `match` 循环或跳转表，用于分发字节码指令。

*   **解释器循环**:
    1.  获取当前帧的 `pc`。
    2.  读取 `code[pc]` 得到操作码 (Opcode)。
    3.  `match opcode` 执行具体逻辑。
    4.  更新 `pc`。
    5.  检查是否遇到 `return` 或异常，若是则弹出当前帧。

*   **指令集实现示例**:
    *   `iconst_1`: `frame.operand_stack.push(Value::Int(1));`
    *   `iadd`: 
        ```rust
        let b = frame.operand_stack.pop().unwrap().as_int();
        let a = frame.operand_stack.pop().unwrap().as_int();
        frame.operand_stack.push(Value::Int(a + b));
        ```
    *   `invokevirtual`: 查找对象的实际类，找到对应方法，创建新 Frame 压栈。

### 3.4 垃圾回收 (Garbage Collection) - 阶段规划

*   **Phase 1 (无 GC)**: 仅分配，不释放。适用于演示极短生命周期的程序。
*   **Phase 2 (引用计数)**: 简单但无法处理循环引用。
*   **Phase 3 (Mark-Sweep)**: 经典的标记-清除算法。需要暂停世界 (Stop-the-World)。

## 4. 技术栈与依赖推荐

| 模块 | 推荐 Crate | 理由 |
| :--- | :--- | :--- |
| **命令行参数** | `clap` | 解析启动参数，如 `minijvm Main.class` |
| **日志调试** | `tracing` / `env_logger` | 追踪指令执行流程，调试必备 |
| **二进制解析** | `nom` 或 手动实现 | `nom` 功能强大但学习曲线陡；手动实现更利于理解 JVM 规范 |
| **错误处理** | `thiserror` | 优雅地定义解析错误、运行时异常 |
| **测试** | `rstest` | 参数化测试，方便批量测试字节码片段 |

## 5. 开发路线图 (Roadmap)

### 第一阶段：基础骨架 (Skeleton)
1.  定义 `Value`, `Frame`, `Clazz` 等核心数据结构。
2.  实现一个简单的硬编码字节码执行器（如上一个 Python 示例的逻辑，但用 Rust 重写）。
3.  实现基本的算术指令 (`iadd`, `isub`, `imul`) 和常数加载 (`iconst`, `bipush`)。

### 第二阶段：类文件解析器 (Parser)
1.  实现 `.class` 文件格式解析器。
2.  能够读取魔数、版本号、常量池。
3.  能够提取方法的 `Code` 属性（字节码数组）。
4.  **里程碑**：能够加载一个空的 `HelloWorld.class` 并不报错。

### 第三阶段：完整执行引擎
1.  实现局部变量表读写 (`iload`, `istore`)。
2.  实现对象创建 (`new`) 和字段访问 (`getfield`, `putfield`)。
3.  实现方法调用 (`invokestatic`, `invokevirtual`) 和返回 (`ireturn`, `return`)。
4.  **里程碑**：能够运行包含方法调用和简单对象操作的 Java 程序。

### 第四阶段：高级特性
1.  实现字符串常量池优化。
2.  实现简单的垃圾回收。
3.  支持异常处理 (`athrow`, try-catch 表解析)。

## 6. 潜在难点与解决方案

1.  **所有权与借用冲突**:
    *   *问题*: `Frame` 需要引用 `MethodInfo`，而 `MethodInfo` 存储在 `Clazz` 中，`Clazz` 又存储在 `MethodArea` 中。多层引用容易导致借用检查器报错。
    *   *解决*: 广泛使用 `Rc<RefCell<T>>` (单线程) 或 `Arc<Mutex<T>>` (多线程) 来共享不可变数据。对于方法元数据，使用 `Arc<MethodInfo>` 是常见做法。

2.  **字节码验证**:
    *   *问题*: 恶意或损坏的 `.class` 文件可能导致内存不安全。
    *   *解决*: 在执行前进行简单的类型检查（Type Checking），确保操作数栈类型匹配。初期可跳过严格验证，假设输入合法。

3.  **性能**:
    *   *问题*: Rust 的 `match` 分发虽然快，但相比 C++ 的跳转表可能略有差异。
    *   *解决*: 使用 `#[repr(u8)]` 枚举确保 Opcode 紧凑，开启 LTO (Link Time Optimization)。

---

### 下一步行动建议

如果您认可这份架构设计，我们可以开始 **第一阶段：基础骨架** 的代码实现。

我将为您生成：
1.  `Cargo.toml` 配置。
2.  核心数据类型定义 (`value.rs`, `frame.rs`)。
3.  一个简单的解释器循环框架。

您希望现在开始编写代码吗？