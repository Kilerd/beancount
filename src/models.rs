use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use indexmap::IndexMap;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq)]
pub enum Directive {
    Open(NaiveDate, Account, Option<Vec<String>>),
    Close(NaiveDate, Account),
    Commodity(NaiveDate, String, IndexMap<String, String>),
    Transaction(Transaction),
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
    date: NaiveDate,
    flag: Flag,
    payee: Option<String>,
    narration: Option<String>,
    tags: Vec<String>,
    links: Vec<String>,
    lines: Vec<TransactionLine>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TransactionLine {
    flag: Flag,
    account: Account,
    amount: Option<(BigDecimal, String)>,
    cost: Option<(BigDecimal, String)>,
    single_price: Option<(BigDecimal, String)>,
    total_price: Option<(BigDecimal, String)>,
}

#[derive(EnumString, Debug, PartialEq, PartialOrd)]
pub enum Flag {
    #[strum(serialize = "*", to_string = "*")]
    Complete,
    #[strum(serialize = "!", to_string = "!")]
    Incomplete,
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
        raw_lines: Vec<(Option<Flag>, Account, Option<String>)>,
    ) -> Transaction {
        let (payee, narration) = match pn {
            None => (None, None),
            Some((narration, None)) => (None, Some(narration)),
            Some((payee, Some(narration))) => (Some(payee), Some(narration)),
        };

        let x = raw_lines
            .into_iter()
            .map(|(flag, account, amount)| TransactionLine::from_parser(flag, account, amount))
            .collect();
        Transaction {
            date,
            flag,
            payee,
            narration,
            tags: vec![],
            links: vec![],
            lines: x,
        }
    }
}

impl TransactionLine {
    pub fn from_parser(
        flag: Option<Flag>,
        account: Account,
        amount: Option<String>,
    ) -> TransactionLine {
        let flag = flag.unwrap_or(Flag::Complete);

        let option = amount
            .map(|s| {
                s.splitn(2, ' ')
                    .map(|p| p.trim().to_owned())
                    .collect::<Vec<String>>()
            })
            .map(|v| {
                let price = BigDecimal::from_str(v[0].as_str()).unwrap();
                let commodity = v[1].to_owned();
                (price, commodity)
            });
        TransactionLine {
            flag,
            account,
            amount: option,
            cost: None,
            single_price: None,
            total_price: None,
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
    }
}
