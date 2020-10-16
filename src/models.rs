use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use indexmap::IndexMap;
use itertools::Itertools;
use std::str::FromStr;
use strum_macros::EnumString;

pub type Amount = (BigDecimal, String);

#[derive(Debug, PartialEq)]
pub enum Directive {
    Open(NaiveDate, Account, Option<Vec<String>>),
    Close(NaiveDate, Account),
    Commodity(NaiveDate, String, IndexMap<String, String>),
    Transaction(Transaction),
    Balance(NaiveDate, Account, Amount),
    Pad(NaiveDate, Account, Account),
    Note(NaiveDate, Account, String),
    Document(NaiveDate, Account, String),
    Price(NaiveDate, String, Amount),
    Event(NaiveDate, String, String),
    Custom(String, Vec<String>),
    Option(String, String),
    Plugin(String, Option<String>),
    Include(String),
    Comment(String),
}

#[derive(Debug, EnumString, PartialEq, PartialOrd, strum_macros::ToString)]
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
    pub cost: Option<(Amount, Option<String>)>,
    pub single_price: Option<Amount>,
    pub total_price: Option<Amount>,
}

#[derive(EnumString, Debug, PartialEq, PartialOrd, strum_macros::ToString)]
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

impl ToString for Account {
    fn to_string(&self) -> String {
        let map = self.value.iter().map(|p| format!(":{}", p)).join("");
        format!("{}{}", self.account_type.to_string(), map)
    }
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
        tags: Vec<String>,
        links: Vec<String>,
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
            tags,
            links,
            lines,
        }
    }
}
pub(crate) type AmountInfo = (
    Amount,
    Option<(Amount, Option<String>)>,
    Option<Amount>,
    Option<Amount>,
);
impl TransactionLine {
    pub(crate) fn from_parser(
        flag: Option<Flag>,
        account: Account,
        amount_info: Option<AmountInfo>,
    ) -> Self {
        let flag = flag.unwrap_or(Flag::Complete);
        let (amount, cost, single_price, total_price) = match amount_info {
            None => (None, None, None, None),
            Some((a, c, s, t)) => (Some(a), c, s, t),
        };

        TransactionLine {
            flag,
            account,
            amount,
            cost,
            single_price,
            total_price,
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
        fn cost_and_cost_comment() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Narration"
                  Assets:123  -1 CNY {0.1 USD , "TEST"}
                  Expenses:TestCategory:One 1 CNY {0.1 USD}"#,
                )
                .unwrap();

            let a = TransactionLine {
                flag: Flag::Complete,
                account: Account::new(AccountType::Assets, vec!["123".to_owned()]),
                amount: Some((BigDecimal::from(-1i16), "CNY".to_string())),
                cost: Some((
                    (BigDecimal::from_f32(0.1f32).unwrap(), "USD".to_owned()),
                    Some("TEST".to_owned()),
                )),
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
                cost: Some((
                    (BigDecimal::from_f32(0.1f32).unwrap(), "USD".to_owned()),
                    None,
                )),
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

        #[test]
        fn with_optional_tags_without_payee() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 *  "Narration" #mytag #tag2
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
                payee: None,
                narration: Some("Narration".to_owned()),
                tags: vec!["mytag".to_owned(), "tag2".to_owned()],
                links: vec![],
                lines: vec![a, b],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }

        #[test]
        fn optional_tags() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Payee" "Narration" #mytag #tag2
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
                tags: vec!["mytag".to_owned(), "tag2".to_owned()],
                links: vec![],
                lines: vec![a, b],
            };
            let x1 = Box::new(Directive::Transaction(transaction));

            assert_eq!(x1, x);
        }

        #[test]
        fn optional_links() {
            let x = DirectiveExpressionParser::new()
                .parse(
                    r#"1970-01-01 * "Payee" "Narration" ^link1 ^link-2
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
                links: vec!["link1".to_owned(), "link-2".to_owned()],
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
        fn test() {
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
        fn test() {
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

    mod option {
        use crate::{models::Directive, parser::DirectiveExpressionParser};

        #[test]
        fn test() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"option "title"  "Personal""#)
                .unwrap();
            let directive = Box::new(Directive::Option("title".to_owned(), "Personal".to_owned()));

            assert_eq!(directive, x);
        }
    }

    mod plugin {
        use crate::{models::Directive, parser::DirectiveExpressionParser};

        #[test]
        fn has_plugin_data() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"plugin "module name"  "config data""#)
                .unwrap();
            let directive = Box::new(Directive::Plugin(
                "module name".to_owned(),
                Some("config data".to_owned()),
            ));

            assert_eq!(directive, x);
        }

        #[test]
        fn do_not_has_plugin_config_data() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"plugin "module name""#)
                .unwrap();
            let directive = Box::new(Directive::Plugin("module name".to_owned(), None));

            assert_eq!(directive, x);
        }
    }

    mod include {
        use crate::{models::Directive, parser::DirectiveExpressionParser};

        #[test]
        fn has_plugin_data() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"include "file path""#)
                .unwrap();
            let directive = Box::new(Directive::Include("file path".to_owned()));

            assert_eq!(directive, x);
        }
    }

    mod custom {
        use crate::{models::Directive, parser::DirectiveExpressionParser};

        #[test]
        fn custom() {
            let x = DirectiveExpressionParser::new()
                .parse(r#"1970-01-01 custom "budget" Expenses:Eat "monthly" CNY"#)
                .unwrap();
            let directive = Box::new(Directive::Custom(
                "budget".to_owned(),
                vec![
                    "Expenses:Eat".to_owned(),
                    "monthly".to_owned(),
                    "CNY".to_owned(),
                ],
            ));

            assert_eq!(directive, x);
        }
    }

    mod comment {

        use crate::{models::Directive, parser::DirectiveExpressionParser};

        #[test]
        fn comma() {
            let x = DirectiveExpressionParser::new().parse(";你好啊").unwrap();
            let directive = Box::new(Directive::Comment(";你好啊".to_owned()));
            assert_eq!(directive, x);
        }
    }

    mod entry {

        use crate::models::{Account, AccountType};
        use crate::{models::Directive, parser::EntryParser};
        use chrono::NaiveDate;

        #[test]
        fn conbine_test() {
            let content: String = vec!["\n\n;你好啊", "1970-01-01 open Assets:Book\n"].join("\n");

            let entry = EntryParser::new().parse(&content).unwrap();

            let directives = vec![
                Box::new(Directive::Comment(";你好啊".to_owned())),
                Box::new(Directive::Open(
                    NaiveDate::from_ymd(1970, 1, 1),
                    Account {
                        account_type: AccountType::Assets,
                        value: vec!["Book".to_owned()],
                    },
                    None,
                )),
            ];

            assert_eq!(directives, entry);
        }
    }
}
