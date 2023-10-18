pub mod share_code;
pub use share_code::*;

pub mod cli;
pub use cli::*;

pub mod developers;
pub use developers::*;

pub mod database;
pub use database::*;

pub mod auth;
pub use auth::*;
use crate::cli::cli::cli_cmd;

fn main() {
    let xencode = cli_cmd().get_matches();
}