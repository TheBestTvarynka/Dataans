pub mod export;
pub mod file;
pub mod note;
pub mod space;

use common::error::{DataansResult, DummyUnit};

type CommandResult<T> = Result<DataansResult<T>, ()>;
type CommandResultEmpty = Result<DataansResult<DummyUnit>, ()>;
