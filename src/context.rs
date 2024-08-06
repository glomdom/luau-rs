use std::collections::HashMap;

use crate::luau::LuauType;

#[derive(Clone)]
pub struct Context {
    pub bindings: HashMap<String, LuauType>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            bindings: HashMap::new(),
        }
    }

    pub fn add_binding(&mut self, name: &str, typ: LuauType) {
        self.bindings.insert(name.to_string(), typ);
    }

    pub fn get_binding(&self, name: &str) -> Option<&LuauType> {
        self.bindings.get(name)
    }
}