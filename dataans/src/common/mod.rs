mod attachment;
mod confirmation;
mod file;
mod modal;
mod switch;
mod textarea;

pub use attachment::*;
pub use confirmation::*;
pub use file::*;
pub use modal::*;
// The `Switch` component is unused.
// But we don't want to remove it because this component may be useful in the future.
#[allow(unused_imports)]
pub use switch::*;
pub use textarea::*;
