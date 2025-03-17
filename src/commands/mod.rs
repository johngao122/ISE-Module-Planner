mod create;
mod edit;
mod export;
mod import;
mod validate;
mod view;

pub use create::CreateCommand;
pub use edit::EditCommand;
pub use export::ExportCommand;
pub use import::ImportCommand;
pub use validate::ValidateCommand;
pub use view::ViewCommand;

pub trait Command {
    fn run(&self) -> anyhow::Result<()>;
}
