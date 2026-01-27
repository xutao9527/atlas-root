use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

pub struct TypeCollector {
    pub composites: BTreeSet<String>,
}

impl TypeCollector {
    pub fn new() -> Self {
        Self {
            composites: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>) {
        self.composites.insert(name.into());
    }
}

pub static TYPE_COLLECTOR: OnceLock<Mutex<TypeCollector>> = OnceLock::new();

pub fn type_collector() -> &'static Mutex<TypeCollector> {
    TYPE_COLLECTOR.get_or_init(|| {
        Mutex::new(TypeCollector::new())
    })
}