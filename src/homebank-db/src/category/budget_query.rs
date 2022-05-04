//! Query the budget in your HomeBank database.

use crate::{Category, HomeBankDb, Query, QueryTransactions, Transaction};
use chrono::{Datelike, Local, NaiveDate};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use structopt::StructOpt;

lazy_static! {
    pub static ref TODAY: NaiveDate = Local::today().naive_local();
    pub static ref TODAY_FIRST_OF_MONTH: NaiveDate =
        NaiveDate::from_ymd(TODAY.year(), TODAY.month(), 1);
    pub static ref TODAY_FIRST_OF_MONTH_STR: String =
        TODAY_FIRST_OF_MONTH.format("%Y-%m-%d").to_string();
}

#[derive(Debug, StructOpt)]
pub struct QueryBudget {
    #[structopt(help = "Name of the category", value_name = "regex")]
    name: Option<Regex>,

    #[structopt(
        short = "d",
        long = "date-from",
        help = "Consider the budget from the month including this date",
        default_value = &TODAY_FIRST_OF_MONTH_STR,
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_from: NaiveDate,
}

impl QueryBudget {
    /// Create a new query for budgets
    pub fn new(name: Option<Regex>, date_from: NaiveDate) -> Self {
        Self { name, date_from }
    }

    /// Retrieve the regular expression for the `Category` name
    fn name(&self) -> &Option<Regex> {
        &self.name
    }

    /// Retrieve the earliest date that the budget is including
    fn date_from(&self) -> &NaiveDate {
        &self.date_from
    }
}

impl Query for QueryBudget {
    type T = Transaction;

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let filt_categories: Vec<Category> = db
            .categories()
            .values()
            // filter out categories that don't match the regex
            .filter(|&p| match self.name() {
                Some(re) => re.is_match(&p.full_name(db)),
                None => true,
            })
            .map(|cat| cat.clone())
            .collect();

        let transaction_query = QueryTransactions::new(
            &Some(*self.date_from()),
            &None,
            &None,
            &None,
            &None,
            self.name(),
            &None,
            &None,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let filt_transactions = transaction_query.exec(db);

        filt_transactions
    }
}