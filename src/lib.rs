use lalrpop_util::lalrpop_mod;

pub mod error;
pub mod models;
lalrpop_mod!(pub parser);

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::parser::DirectiveExpressionParser;
    use chrono::NaiveDate;

    #[test]
    fn it_works() {
        let directive = Box::new(Directive::Open(NaiveDate::from_ymd(1970, 1, 1), (Account::Assets(vec![]))));
        let x = DirectiveExpressionParser::new()
            .parse("1970-01-01 open Assets")
            .unwrap();
        assert_eq!(directive, x);

        let x = DirectiveExpressionParser::new()
            .parse("1970-01-01")
            .is_err();
        assert_eq!(true, x);
    }
}
