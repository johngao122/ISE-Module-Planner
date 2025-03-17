mod curriculum;
mod module;
mod plan;
mod student;

pub use curriculum::*;
pub use module::*;
pub use plan::*;
pub use student::*;

use std::collections::HashMap;

pub struct ModuleRegistry {
    modules: HashMap<String, Module>,
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.insert(module.module_code.clone(), module);
    }

    pub fn get_module(&self, module_code: &str) -> Option<&Module> {
        self.modules.get(module_code)
    }

    pub fn contains_module(&self, module_code: &str) -> bool {
        self.modules.contains_key(module_code)
    }
}
