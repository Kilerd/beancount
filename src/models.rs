use chrono::NaiveDate;
use strum_macros::EnumString;
#[derive(Debug, PartialEq)]
pub enum Directive {
    Open(NaiveDate, Account),
    Close,
    Commodity,
    Transaction,
    Metadata,
    Balance,
    Tag,
    Pad,
    Note,
    Document,
    Price,
    Event,
    Custom,
}

#[derive(Debug, EnumString, PartialEq)]
pub enum Account {
    Assets(Vec<String>),
    Liabilities,
    Equity,
    Income,
    Expenses,
}
