use lalrpop_util::lalrpop_mod;
pub mod to_file;
pub mod error;
pub mod models;

pub(crate) mod utils;
lalrpop_mod!(#[allow(clippy::all)] pub parser);
