use chrono::NaiveDate;
use strum_macros::EnumString;
#[derive(Debug, PartialEq)]
pub enum Directive {
    Open(NaiveDate, Account, Option<Vec<String>>),
    Close,
    Commodity,
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
