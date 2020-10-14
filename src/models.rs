use chrono::NaiveDate;
use indexmap::IndexMap;
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug, PartialEq)]
pub enum Directive {
    Open(NaiveDate, Account, Option<Vec<String>>),
    Close(NaiveDate, Account),
    Commodity(NaiveDate, String, IndexMap<String, String>),
    Transaction,
    Metadata,
    Balance,
    Tag,
    Pad,
    Note(NaiveDate, Account, String),
    Document,
    Price,
    Event,
    Custom,
}

#[derive(Debug, EnumString, PartialEq)]
pub enum Account {
    Assets(Vec<String>),
    Liabilities(Vec<String>),
    Equity(Vec<String>),
    Income(Vec<String>),
    Expenses(Vec<String>),
}

#[cfg(test)]
mod test {

    mod open {
        use crate::models::{Account, Directive};
        use crate::parser::DirectiveExpressionParser;
        use chrono::NaiveDate;
        #[test]
        fn test_open_directive() {
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
                None,
            ));
            let x = DirectiveExpressionParser::new()
                .parse("1970-01-01 open Assets:123:234:English:中文:日本語:한국어")
                .unwrap();
            assert_eq!(directive, x);
        }
        #[test]
        fn test_open_with_commodity() {
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
                Some(vec!["CNY".to_owned()]),
            ));
            let x = DirectiveExpressionParser::new()
                .parse("1970-01-01 open Assets:123:234:English:中文:日本語:한국어 CNY")
                .unwrap();
            assert_eq!(directive, x);
        }
        #[test]
        fn test_open_with_commodities() {
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
                Some(vec!["CNY".to_owned(), "USD".to_owned(), "CAD".to_owned()]),
            ));
            let x = DirectiveExpressionParser::new()
                .parse("1970-01-01 open Assets:123:234:English:中文:日本語:한국어 CNY, USD,CAD")
                .unwrap();
            assert_eq!(directive, x);
        }
    }

    mod close {
        use crate::models::{Account, Directive};
        use crate::parser::DirectiveExpressionParser;
        use chrono::NaiveDate;
        #[test]
        fn test_close() {
            let directive = Box::new(Directive::Close(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::Assets(vec!["123".to_owned(), "456".to_owned()]),
            ));
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 close Assets:123:456  "#)
                .unwrap();
            assert_eq!(directive, x);
        }
    }

    mod note {
        use crate::models::{Account, Directive};
        use crate::parser::DirectiveExpressionParser;
        use chrono::NaiveDate;
        #[test]
        fn test_note_directive() {
            let directive = Box::new(Directive::Note(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::Assets(vec!["123".to_owned()]),
                "你 好 啊\\".to_owned(),
            ));
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 note Assets:123 "你 好 啊\\""#)
                .unwrap();
            assert_eq!(directive, x);
        }
    }

    mod commodity {
        use crate::models::{Account, Directive};
        use crate::parser::DirectiveExpressionParser;
        use chrono::NaiveDate;
        use indexmap::IndexMap;
        #[test]
        fn test_commodity_without_attribute() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 commodity CNY  "#)
                .unwrap();

            let directive = Box::new(Directive::Commodity(
                NaiveDate::from_ymd(1970, 1, 1),
                "CNY".to_owned(),
                IndexMap::new(),
            ));
            assert_eq!(directive, x);
        }

        #[test]
        fn test_commodity_with_single_attribute() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 commodity CNY
                  a: "b""#,
                )
                .unwrap();

            let mut map = IndexMap::new();
            map.insert("a".to_owned(), "b".to_owned());
            let directive = Box::new(Directive::Commodity(
                NaiveDate::from_ymd(1970, 1, 1),
                "CNY".to_owned(),
                map,
            ));
            assert_eq!(directive, x);
        }
        #[test]
        fn test_commodity_with_attributes() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 commodity CNY
                  a: "b"
                  中文-test  :  "한국어 我也不知道我在说啥""#,
                )
                .unwrap();

            let mut map = IndexMap::new();
            map.insert("a".to_owned(), "b".to_owned());
            map.insert(
                "中文-test".to_owned(),
                "한국어 我也不知道我在说啥".to_owned(),
            );
            let directive = Box::new(Directive::Commodity(
                NaiveDate::from_ymd(1970, 1, 1),
                "CNY".to_owned(),
                map,
            ));
            assert_eq!(directive, x);
        }
    }
}
