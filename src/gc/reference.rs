use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
    Strong,
    Soft,
    Weak,
    Phantom,
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceType::Strong => write!(f, "Strong"),
            ReferenceType::Soft => write!(f, "Soft"),
            ReferenceType::Weak => write!(f, "Weak"),
            ReferenceType::Phantom => write!(f, "Phantom"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub reference_type: ReferenceType,
    pub referent: usize,
    pub next: Option<usize>,
    pub queue: Option<usize>,
}

impl Reference {
    pub fn new(reference_type: ReferenceType, referent: usize) -> Self {
        Reference {
            reference_type,
            referent,
            next: None,
            queue: None,
        }
    }

    pub fn is_reachable(&self) -> bool {
        self.referent != 0
    }

    pub fn clear(&mut self) {
        self.referent = 0;
    }
}

#[derive(Debug, Clone)]
pub struct ReferenceQueue {
    pub references: Vec<Reference>,
}

impl ReferenceQueue {
    pub fn new() -> Self {
        ReferenceQueue {
            references: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, reference: Reference) {
        self.references.push(reference);
    }

    pub fn dequeue(&mut self) -> Option<Reference> {
        self.references.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.references.is_empty()
    }

    pub fn len(&self) -> usize {
        self.references.len()
    }
}
