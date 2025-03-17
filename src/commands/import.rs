use crate::models::Plan;
use crate::storage::Storage;
use anyhow::{anyhow, Result};
use dialoguer::{Confirm, Input};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct ImportCommand {
    pub storage: Box<dyn Storage>,
}

impl super::Command for ImportCommand {
    fn run(&self) -> Result<()> {
        let file_path: String = Input::new()
            .with_prompt("Enter path to JSON plan file")
            .interact()?;

        let path = Path::new(&file_path);
        if !path.exists() {
            return Err(anyhow!("File does not exist: {}", file_path));
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let imported_plan: Plan = serde_json::from_str(&contents)?;

        if self.storage.get_plan(&imported_plan.id)?.is_some() {
            let overwrite = Confirm::new()
                .with_prompt(format!(
                    "A plan with ID {} already exists. Overwrite?",
                    imported_plan.id
                ))
                .default(false)
                .interact()?;

            if !overwrite {
                return Err(anyhow!("Import cancelled"));
            }
        }

        self.storage.save_plan(&imported_plan)?;

        println!("✅ Imported plan: {}", imported_plan.name);

        Ok(())
    }
}
