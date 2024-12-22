pub mod export;
pub mod file;
pub mod note;
pub mod space;

use common::error::DataansResult;

pub type CommandResult<T> = Result<DataansResult<T>, ()>;
