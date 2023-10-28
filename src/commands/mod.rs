pub mod commit;
pub mod squash;
// use crate::config::Config;
mod shell;

pub use shell::{run, RunArgs};
