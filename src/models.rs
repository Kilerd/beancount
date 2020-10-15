use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use indexmap::IndexMap;
use std::str::FromStr;
use strum_macros::EnumString;

pub type Amount = (BigDecimal, String);

#[derive(Debug, PartialEq)]
pub enum Directive {
    Open(NaiveDate, Account, Option<Vec<String>>),
    Close(NaiveDate, Account),
    Commodity(NaiveDate, String, IndexMap<String, String>),
    Transaction(Transaction),
    Metadata,
    Balance(NaiveDate, Account, Amount),
    Tag,
    Pad(NaiveDate, Account, Account),
    Note(NaiveDate, Account, String),
    Document(NaiveDate, Account, String),
    Price(NaiveDate, String, Amount),
    Event(NaiveDate, String, String),
    Custom,
}

#[derive(Debug, EnumString, PartialEq, PartialOrd)]
pub enum AccountType {
    Assets,
    Liabilities,
    Equity,
    Income,
    Expenses,
}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Account {
    account_type: AccountType,
    value: Vec<String>,
}

impl Account {
    pub fn new(account_type: AccountType, value: Vec<String>) -> Self {
        Account {
            account_type,
            value,
        }
    }
}

// todo tags links
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Transaction {
    pub date: NaiveDate,
    pub flag: Flag,
    pub payee: Option<String>,
    pub narration: Option<String>,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub lines: Vec<TransactionLine>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TransactionLine {
    pub flag: Flag,
    pub account: Account,
    pub amount: Option<Amount>,
    pub cost: Option<Amount>,
    pub single_price: Option<Amount>,
    pub total_price: Option<Amount>,
}

#[derive(EnumString, Debug, PartialEq, PartialOrd)]
pub enum Flag {
    #[strum(serialize = "*", to_string = "*")]
    Complete,
    #[strum(serialize = "!", to_string = "!")]
    Incomplete,
}

pub(crate) fn amount_parse(input: &str) -> Amount {
    let parts: Vec<String> = input.splitn(2, ' ').map(|p| p.trim().to_owned()).collect();
    let price = BigDecimal::from_str(parts[0].as_str()).unwrap();
    (price, parts[1].to_owned())
}

impl Transaction {
    pub fn new(
        date: NaiveDate,
        flag: Flag,
        payee: Option<String>,
        narration: Option<String>,
        tags: Vec<String>,
        links: Vec<String>,
        lines: Vec<TransactionLine>,
    ) -> Self {
        Transaction {
            date,
            flag,
            payee,
            narration,
            tags,
            links,
            lines,
        }
    }

    pub(crate) fn from_parser(
        date: NaiveDate,
        flag: Flag,
        pn: Option<(String, Option<String>)>,
        lines: Vec<TransactionLine>,
    ) -> Transaction {
        let (payee, narration) = match pn {
            None => (None, None),
            Some((narration, None)) => (None, Some(narration)),
            Some((payee, Some(narration))) => (Some(payee), Some(narration)),
        };

        Transaction {
            date,
            flag,
            payee,
            narration,
            tags: vec![],
            links: vec![],
            lines,
        }
    }
}

#[cfg(test)]
mod test {

    mod open {
        use crate::{
            models::{Account, AccountType, Directive},
            parser::DirectiveExpressionParser,
        };
        use chrono::NaiveDate;
        #[test]
        fn test_open_directive() {
            let directive = Box::new(Directive::Open(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::new(
                    AccountType::Assets,
                    vec![
                        "123".to_owned(),
                        "234".to_owned(),
                        "English".to_owned(),
                        "中文".to_owned(),
                        "日本語".to_owned(),
                        "한국어".to_owned(),
                    ],
                ),
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
                Account::new(
                    AccountType::Assets,
                    vec![
                        "123".to_owned(),
                        "234".to_owned(),
                        "English".to_owned(),
                        "中文".to_owned(),
                        "日本語".to_owned(),
                        "한국어".to_owned(),
                    ],
                ),
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
                Account::new(
                    AccountType::Assets,
                    vec![
                        "123".to_owned(),
                        "234".to_owned(),
                        "English".to_owned(),
                        "中文".to_owned(),
                        "日本語".to_owned(),
                        "한국어".to_owned(),
                    ],
                ),
                Some(vec!["CNY".to_owned(), "USD".to_owned(), "CAD".to_owned()]),
            ));
            let x = DirectiveExpressionParser::new()
                .parse("1970-01-01 open Assets:123:234:English:中文:日本語:한국어 CNY, USD,CAD")
                .unwrap();
            assert_eq!(directive, x);
        }
    }

    mod close {
        use crate::{
            models::{Account, AccountType, Directive},
            parser::DirectiveExpressionParser,
        };
        use chrono::NaiveDate;
        #[test]
        fn test_close() {
            let directive = Box::new(Directive::Close(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::new(
                    AccountType::Assets,
                    vec!["123".to_owned(), "456".to_owned()],
                ),
            ));
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 close Assets:123:456  "#)
                .unwrap();
            assert_eq!(directive, x);
        }
    }

    mod note {
        use crate::{
            models::{Account, AccountType, Directive},
            parser::DirectiveExpressionParser,
        };
        use chrono::NaiveDate;
        #[test]
        fn test_note_directive() {
            let directive = Box::new(Directive::Note(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::new(AccountType::Assets, vec!["123".to_owned()]),
                "你 好 啊\\".to_owned(),
            ));
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 note Assets:123 "你 好 啊\\""#)
                .unwrap();
            assert_eq!(directive, x);
        }
    }

    mod commodity {
        use crate::{models::Directive, parser::DirectiveExpressionParser};
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

    mod transaction {
        use crate::{
            models::{Account, AccountType, Directive, Flag, Transaction, TransactionLine},
            parser::DirectiveExpressionParser,
        };
        use bigdecimal::{BigDecimal, FromPrimitive};
        use chrono::NaiveDate;

        #[test]
        fn simple_test() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Payee" "Narration"
                  Assets:123  -1 CNY
                  Expenses:TestCategory:One 1 CNY"#,
                )
                .unwrap();

            let a = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(AccountType::Assets, vec!["123".to_owned()]),
                amount: Some((BigDecimal::from(-1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };
            let b = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(
                    AccountType::Expenses,
                    vec!["TestCategory".to_owned(), "One".to_owned()],
                ),
                amount: Some((BigDecimal::from(1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };

            let transaction = Transaction {
                date: NaiveDate::from_ymd(1970, 1, 1),
                flag: Flag::Complete,
                payee: Some("Payee".to_owned()),
                narration: Some("Narration".to_owned()),
                tags: vec![],
                links: vec![],
                lines: vec![a, b],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }

        #[test]
        fn without_payee_with_narration() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Narration"
                  Assets:123  -1 CNY
                  Expenses:TestCategory:One 1 CNY"#,
                )
                .unwrap();

            let a = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(AccountType::Assets, vec!["123".to_owned()]),
                amount: Some((BigDecimal::from(-1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };
            let b = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(
                    AccountType::Expenses,
                    vec!["TestCategory".to_owned(), "One".to_owned()],
                ),
                amount: Some((BigDecimal::from(1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };

            let transaction = Transaction {
                date: NaiveDate::from_ymd(1970, 1, 1),
                flag: Flag::Complete,
                payee: None,
                narration: Some("Narration".to_owned()),
                tags: vec![],
                links: vec![],
                lines: vec![a, b],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }

        #[test]
        fn multiple_transaction_lines() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Payee" "Narration"
                  Assets:123  -1 CNY
                  Expenses:TestCategory:One 0.5 CNY
                  Expenses:TestCategory:Two 0.5 CNY"#,
                )
                .unwrap();

            let a = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(AccountType::Assets, vec!["123".to_owned()]),
                amount: Some((BigDecimal::from(-1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };
            let b = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(
                    AccountType::Expenses,
                    vec!["TestCategory".to_owned(), "One".to_owned()],
                ),
                amount: Some((BigDecimal::from_f32(0.5f32).unwrap(), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };
            let c = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(
                    AccountType::Expenses,
                    vec!["TestCategory".to_owned(), "Two".to_owned()],
                ),
                amount: Some((BigDecimal::from_f32(0.5f32).unwrap(), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };

            let transaction = Transaction {
                date: NaiveDate::from_ymd(1970, 1, 1),
                flag: Flag::Complete,
                payee: Some("Payee".to_owned()),
                narration: Some("Narration".to_owned()),
                tags: vec![],
                links: vec![],
                lines: vec![a, b, c],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }

        #[test]
        fn optional_amount_in_line() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Payee" "Narration"
                  Assets:123  -1 CNY
                  Expenses:TestCategory:One"#,
                )
                .unwrap();

            let a = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(AccountType::Assets, vec!["123".to_owned()]),
                amount: Some((BigDecimal::from(-1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };
            let b = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(
                    AccountType::Expenses,
                    vec!["TestCategory".to_owned(), "One".to_owned()],
                ),
                amount: None,
                cost: None,
                single_price: None,
                total_price: None,
            };

            let transaction = Transaction {
                date: NaiveDate::from_ymd(1970, 1, 1),
                flag: Flag::Complete,
                payee: Some("Payee".to_owned()),
                narration: Some("Narration".to_owned()),
                tags: vec![],
                links: vec![],
                lines: vec![a, b],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }

        #[test]
        fn optional_single_price() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Payee" "Narration"
                  Assets:123  -1 CNY
                  Expenses:TestCategory:One 1 CCC @ 1 CNY"#,
                )
                .unwrap();

            let a = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(AccountType::Assets, vec!["123".to_owned()]),
                amount: Some((BigDecimal::from(-1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };
            let b = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(
                    AccountType::Expenses,
                    vec!["TestCategory".to_owned(), "One".to_owned()],
                ),
                amount: Some((BigDecimal::from(1i16), "CCC".to_string())),
                cost: None,
                single_price: Some((BigDecimal::from(1i16), "CNY".to_string())),
                total_price: None,
            };

            let transaction = Transaction {
                date: NaiveDate::from_ymd(1970, 1, 1),
                flag: Flag::Complete,
                payee: Some("Payee".to_owned()),
                narration: Some("Narration".to_owned()),
                tags: vec![],
                links: vec![],
                lines: vec![a, b],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }

        #[test]
        fn optional_total_price() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Payee" "Narration"
                  Assets:123  -1 CNY
                  Expenses:TestCategory:One 1 CCC @@ 1 CNY"#,
                )
                .unwrap();

            let a = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(AccountType::Assets, vec!["123".to_owned()]),
                amount: Some((BigDecimal::from(-1i16), "CNY".to_string())),
                cost: None,
                single_price: None,
                total_price: None,
            };
            let b = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(
                    AccountType::Expenses,
                    vec!["TestCategory".to_owned(), "One".to_owned()],
                ),
                amount: Some((BigDecimal::from(1i16), "CCC".to_string())),
                cost: None,
                single_price: None,
                total_price: Some((BigDecimal::from(1i16), "CNY".to_string())),
            };

            let transaction = Transaction {
                date: NaiveDate::from_ymd(1970, 1, 1),
                flag: Flag::Complete,
                payee: Some("Payee".to_owned()),
                narration: Some("Narration".to_owned()),
                tags: vec![],
                links: vec![],
                lines: vec![a, b],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }
    }

    mod pad {
        use crate::{
            models::{Account, AccountType, Directive},
            parser::DirectiveExpressionParser,
        };
        use chrono::NaiveDate;
        #[test]
        fn pad_directive() {
            let x = DirectiveExpressionParser::new()
                .parse("1970-01-01 pad Assets:123:234:English:中文:日本語:한국어 Equity:ABC")
                .unwrap();
            let directive = Box::new(Directive::Pad(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::new(
                    AccountType::Assets,
                    vec![
                        "123".to_owned(),
                        "234".to_owned(),
                        "English".to_owned(),
                        "中文".to_owned(),
                        "日本語".to_owned(),
                        "한국어".to_owned(),
                    ],
                ),
                Account::new(AccountType::Equity, vec!["ABC".to_owned()]),
            ));

            assert_eq!(directive, x);
        }
    }

    mod balance {
        use crate::{
            models::{Account, AccountType, Directive},
            parser::DirectiveExpressionParser,
        };
        use bigdecimal::BigDecimal;
        use chrono::NaiveDate;

        #[test]
        fn balance_directive() {
            let x = DirectiveExpressionParser::new()
                .parse("1970-01-01 balance Assets:123:234:English:中文:日本語:한국어  1 CNY")
                .unwrap();
            let directive = Box::new(Directive::Balance(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::new(
                    AccountType::Assets,
                    vec![
                        "123".to_owned(),
                        "234".to_owned(),
                        "English".to_owned(),
                        "中文".to_owned(),
                        "日本語".to_owned(),
                        "한국어".to_owned(),
                    ],
                ),
                (BigDecimal::from(1i16), "CNY".to_owned()),
            ));

            assert_eq!(directive, x);
        }
    }

    mod utils {
        use crate::models::amount_parse;
        use bigdecimal::BigDecimal;

        #[test]
        fn test_amount_parse() {
            assert_eq!(
                (BigDecimal::from(1), "CNY".to_owned()),
                amount_parse("1 CNY")
            );
            assert_eq!(
                (BigDecimal::from(1), "CNY".to_owned()),
                amount_parse("1     CNY")
            );
            assert_eq!(
                (BigDecimal::from(-1), "CNY".to_owned()),
                amount_parse("-1     CNY")
            );
        }
    }

    mod document {
        use crate::{
            models::{Account, AccountType, Directive},
            parser::DirectiveExpressionParser,
        };
        use chrono::NaiveDate;
        #[test]
        fn empty_string() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 document Assets:123 """#)
                .unwrap();
            let directive = Box::new(Directive::Document(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::new(AccountType::Assets, vec!["123".to_owned()]),
                "".to_owned(),
            ));

            assert_eq!(directive, x);
        }

        #[test]
        fn has_document_content() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 document Assets:123 "here I am""#)
                .unwrap();
            let directive = Box::new(Directive::Document(
                NaiveDate::from_ymd(1970, 1, 1),
                Account::new(AccountType::Assets, vec!["123".to_owned()]),
                "here I am".to_owned(),
            ));

            assert_eq!(directive, x);
        }
    }

    mod price {
        use crate::{models::Directive, parser::DirectiveExpressionParser};
        use bigdecimal::BigDecimal;
        use chrono::NaiveDate;

        #[test]
        fn has_document_content() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 price USD   7 CNY"#)
                .unwrap();
            let directive = Box::new(Directive::Price(
                NaiveDate::from_ymd(1970, 1, 1),
                "USD".to_owned(),
                (BigDecimal::from(7i16), "CNY".to_owned()),
            ));

            assert_eq!(directive, x);
        }
    }

    mod event {
        use crate::{models::Directive, parser::DirectiveExpressionParser};
        use chrono::NaiveDate;

        #[test]
        fn has_document_content() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 event "location"  "China""#)
                .unwrap();
            let directive = Box::new(Directive::Event(
                NaiveDate::from_ymd(1970, 1, 1),
                "location".to_owned(),
                "China".to_owned(),
            ));

            assert_eq!(directive, x);
        }
    }
}
