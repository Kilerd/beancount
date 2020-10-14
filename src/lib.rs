use lalrpop_util::lalrpop_mod;

pub mod error;
pub mod models;
lalrpop_mod!(#[allow(clippy::all)] pub parser);
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn test_add() {
        assert_eq!(2, add(1, 1));
    }

}
