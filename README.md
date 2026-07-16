# MiniJVM

A lightweight Java Virtual Machine implemented in Rust, supporting Java 17 features.

## Overview

MiniJVM is a from-scratch JVM implementation that can parse, verify, and execute Java class files. It supports Java 17 class format (version 61.0) and implements 182 bytecode instructions, including `invokedynamic`, `tableswitch`, `lookupswitch`, and Java 17 features like Records and Sealed Classes.

**Current codebase:** ~16,000 lines of Rust, 60 tests all passing.

## Features

### Core Engine
- **Class File Parser** — Full support for Java 17 class file format (version 61.0), all 21 constant pool types, and all standard attributes (Code, StackMapTable, LineNumberTable, BootstrapMethods, Record, PermittedSubclasses)
- **Bytecode Interpreter** — 182 instructions implemented, including `invokedynamic`/`invokeinterface`/`invokevirtual`/`invokespecial`/`invokestatic`, `tableswitch`/`lookupswitch`, `jsr`/`ret`, and wide instructions
- **Class Loading** — Parent-delegation class loader model with Bootstrap and Application loaders, lazy loading on demand
- **Bytecode Verifier** — Pre-execution verification of branch targets, instruction widths, and unknown opcodes

### Runtime System
- **Memory Management** — Heap with object allocation, array support, string interning, and garbage collection
- **Generational GC** — Young generation collection with age-based promotion to old generation, falling back to full mark-sweep when needed
- **Multi-threading** — Real OS thread support via `Thread.start()`, thread-local storage via `ThreadLocal`, cooperative scheduling
- **Synchronization** — Monitor-based object locking with `monitorenter`/`monitorexit`
- **Exception Handling** — `try-catch-finally` exception tables, stack unwinding, `athrow` instruction

### Java 17 Support
- **Records (JEP 395)** — Class file parsing of `Record` attribute, record component metadata
- **Sealed Classes (JEP 409)** — Class file parsing of `PermittedSubclasses` attribute
- **Pattern Matching (JEP 406)** — `instanceof` pattern matching variable support
- **Switch Expressions (JEP 361)** — Bytecode-level support
- **Text Blocks (JEP 378)** — Parse-time support
- **Helpful NPE (JEP 358)** — Null pointer detection

### Standard Library
The following Java packages are implemented (approximately 95 classes):

| Package | Classes |
|---------|---------|
| `java.lang` | Object, String, StringBuilder, System, Thread, ThreadLocal, Class, Runnable, Throwable, Integer, Long, Float, Double, Boolean, Math, Record, Enum |
| `java.io` | FileInputStream, FileOutputStream, File, ByteArrayInputStream, ByteArrayOutputStream, BufferedInputStream, BufferedOutputStream, PrintStream, InputStream, OutputStream |
| `java.util` | ArrayList, LinkedList, Stack, HashMap, LinkedHashMap, TreeMap, HashSet, LinkedHashSet, PriorityQueue, Random, UUID, Base64, Properties, Scanner, Arrays, Collections, Comparator, Iterator, Iterable, Objects, Optional, Date, BitSet, Locale, Queue, Deque |
| `java.util.concurrent.atomic` | AtomicInteger, AtomicLong, AtomicReference, AtomicBoolean |
| `java.util.concurrent.locks` | ReentrantLock |
| `java.util.regex` | Pattern, Matcher |
| `java.math` | BigInteger, BigDecimal |

## Architecture

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root
├── classfile/           # Class file parser
│   ├── parser.rs        # Binary .class file parser
│   ├── types.rs         # Class file data structures
│   ├── constant_pool.rs # Constant pool types and resolution
│   └── attributes.rs    # Attribute parsing (Code, StackMap, etc.)
├── classloader/         # Class loading subsystem
│   └── loader.rs        # Parent-delegation class loader
├── interpreter/         # Bytecode execution engine
│   ├── instruction_set.rs  # 182 instruction handlers
│   └── dispatch.rs      # Instruction dispatch logic
├── runtime/             # Runtime data structures
│   ├── heap.rs          # Object heap and allocation
│   ├── stack.rs         # JVM stack frames
│   ├── method_area.rs   # Class and method storage
│   └── value.rs         # Java value types
├── gc/                  # Garbage collection
│   └── collector.rs     # Mark-sweep and generational GC
├── threading/           # Thread management
│   ├── scheduler.rs     # Thread scheduling
│   ├── monitor.rs       # Object monitor for synchronization
│   └── thread.rs        # Thread data structures
├── verifier/            # Bytecode verification
│   └── checker.rs       # Pre-execution bytecode validation
├── stdlib/              # Standard library native methods
│   ├── lang.rs          # java.lang native implementations
│   ├── io.rs            # java.io native implementations
│   ├── util.rs          # java.util native implementations
│   ├── math.rs          # java.math native implementations
│   └── regex.rs         # java.util.regex native implementations
└── error.rs             # Error types and results
```

## Building & Running

### Prerequisites
- Rust 2021 edition (1.60+)
- Java 17+ (for compiling test Java files)

### Build
```bash
cargo build --release
```

### Run a Java class
```bash
# Compile a Java file
javac --release 17 HelloWorld.java

# Run with MiniJVM
cargo run -- HelloWorld.class
```

### Run tests
```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test jvm_integration

# Verifier tests only
cargo test verifier
```

## Test Status

```
Unit tests:      42 passed
Verifier tests:   9 passed
Integration tests: 9 passed
Total:            60 tests, all passing
```

## Example

```java
// HelloWorld.java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, MiniJVM!");
    }
}
```

```bash
$ javac --release 17 HelloWorld.java
$ cargo run -- HelloWorld.class
Hello, MiniJVM!
Execution completed successfully
```

## Limitations

- Single-threaded interpreter (no JIT compilation)
- Simplified GC (no compaction, no concurrent marking)
- Standard library coverage is functional but not exhaustive
- Limited runtime type checking for generic operations
- No native method support for AWT, Swing, or networking

## Project Structure

```
MiniJVM/
├── Cargo.toml          # Rust package configuration
├── src/                # Rust source code
├── tests/              # Integration test files
├── *.class             # Sample compiled Java classes
└── *.java              # Sample Java source files
```

## License

This project is for educational purposes.