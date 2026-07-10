pub mod value;
pub mod heap;
pub mod stack;
pub mod method_area;

pub use value::Value;
pub use heap::{Heap, HeapObject};
pub use stack::{JvmStack, Frame};
pub use method_area::MethodArea;

pub struct JVM {
    pub method_area: MethodArea,
    pub heap: Heap,
    pub stack: JvmStack,
}

impl JVM {
    pub fn new() -> Self {
        JVM {
            method_area: MethodArea::new(),
            heap: Heap::new(),
            stack: JvmStack::new(),
        }
    }
}
