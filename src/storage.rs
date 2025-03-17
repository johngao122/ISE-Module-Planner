use crate::models::{Curriculum, Plan, Student};
use anyhow::Result;

pub trait Storage {
    fn save_plan(&self, plan: &Plan) -> Result<()>;
    fn get_plan(&self, id: &str) -> Result<Option<Plan>>;
    fn list_plans(&self) -> Result<Vec<Plan>>;
    fn delete_plan(&self, id: &str) -> Result<bool>;

    fn save_student(&self, student: &Student) -> Result<()>;
    fn get_student(&self, name: &str) -> Result<Option<Student>>;
    fn list_students(&self) -> Result<Vec<Student>>;

    fn get_curriculum(&self, name: &str) -> Result<Option<Curriculum>>;
}
