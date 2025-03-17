use crate::models::{Module, ModuleRegistry};
use anyhow::Result;

pub trait NusmodsApi {
    fn get_module_registry(&self) -> Result<ModuleRegistry>;
    fn search_modules(&self, query: &str) -> Result<Vec<Module>>;
}

pub struct DefaultNusmodsApi;

impl DefaultNusmodsApi {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl NusmodsApi for DefaultNusmodsApi {
    fn get_module_registry(&self) -> Result<ModuleRegistry> {
        // In a real implementation, this would fetch data from the NUSMods API
        // For testing, we'll just return an empty registry
        Ok(ModuleRegistry::new())
    }

    fn search_modules(&self, _query: &str) -> Result<Vec<Module>> {
        // In a real implementation, this would search the NUSMods API
        // For testing, we'll just return an empty vector
        Ok(Vec::new())
    }
}
