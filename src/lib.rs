use lalrpop_util::lalrpop_mod;

pub mod error;
pub mod models;
lalrpop_mod!(#[allow(clippy::all)] pub parser);

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::parser::DirectiveExpressionParser;
    use chrono::NaiveDate;
    use regex::Regex;

    #[test]
    fn it_works() {
        let directive = Box::new(Directive::Open(
            NaiveDate::from_ymd(1970, 1, 1),
            Account::Assets(vec![
                "123".to_owned(),
                "234".to_owned(),
                "English".to_owned(),
                "中文".to_owned(),
                "日本語".to_owned(),
                "한국어".to_owned(),
            ]),
        ));
        let x = DirectiveExpressionParser::new()
            .parse("1970-01-01 open Assets:123:234:English:中文:日本語:한국어")
            .unwrap();
        assert_eq!(directive, x);
    }

    #[test]
    fn test_note_directive() {
        let directive = Box::new(Directive::Note(
            NaiveDate::from_ymd(1970, 1, 1),
            Account::Assets(vec![
                "123".to_owned(),
            ]),
            "你 好 啊\\".to_owned()
        ));
        let x = DirectiveExpressionParser::new()
            .parse(r#"1970-01-01 note Assets:123 "你 好 啊\\""#)
            .unwrap();
        assert_eq!(directive, x);
    }
}
