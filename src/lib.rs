use lalrpop_util::lalrpop_mod;

pub mod error;
pub mod models;
lalrpop_mod!(#[allow(clippy::all)] pub parser);
