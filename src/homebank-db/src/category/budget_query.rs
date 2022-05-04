//! Query the budget in your HomeBank database.

use crate::{transaction::sum_transactions, Category, HomeBankDb, Query, QueryTransactions};
use chrono::{Datelike, Local, NaiveDate};
use kronos::{Grain, Grains, NthOf, TimeSequence};
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
    pub static ref FIRST_OF_NEXT_MONTH: NaiveDate = {
        let first_of_month = NthOf(1, Grains(Grain::Day), Grains(Grain::Month));
        let mut date_iter = first_of_month.future(&TODAY_FIRST_OF_MONTH.and_hms(0, 0, 0));

        // skip the first month
        date_iter.next();

        // save the next month
        let first_of_next_month = date_iter
            .next()
            .unwrap()
            .start
            .date();

        first_of_next_month
    };
    pub static ref FIRST_OF_NEXT_MONTH_STR: String =
        FIRST_OF_NEXT_MONTH.format("%Y-%m-%d").to_string();
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

    #[structopt(
        short = "D",
        long = "date-to",
        help = "Consider the budget from the month up to and excluding this date",
        default_value = &FIRST_OF_NEXT_MONTH_STR,
        parse(try_from_str = NaiveDate::from_str),
        value_name = "date"
    )]
    date_to: NaiveDate,
}

impl QueryBudget {
    /// Create a new query for budgets
    pub fn new(name: Option<Regex>, date_from: NaiveDate, date_to: NaiveDate) -> Self {
        Self {
            name,
            date_from,
            date_to,
        }
    }

    /// Retrieve the regular expression for the `Category` name
    fn name(&self) -> &Option<Regex> {
        &self.name
    }

    /// Retrieve the earliest date that the budget is including
    fn date_from(&self) -> &NaiveDate {
        &self.date_from
    }

    /// Retrieve the latest date that the budget is including
    fn date_to(&self) -> &NaiveDate {
        &self.date_to
    }
}

impl Query for QueryBudget {
    type T = (String, f32, Option<f32>);

    fn exec(&self, db: &HomeBankDb) -> Vec<Self::T> {
        let mut filt_categories: Vec<Category> = db
            .categories()
            .values()
            // filter out categories that don't match the regex
            .filter(|&cat| match self.name() {
                Some(re) => re.is_match(&cat.full_name(db)),
                None => true,
            })
            // filter out categories that don't have a budget
            .filter(|&cat| cat.has_budget())
            .map(|cat| cat.clone())
            .collect();

        filt_categories.sort_by(|a, b| a.full_name(db).cmp(&b.full_name(db)));

        let budget_spent: Vec<(String, f32, Option<f32>)> = filt_categories
            .iter()
            .map(|cat| {
                let cat_name_re = Regex::new(&cat.full_name(db)).unwrap();
                let transaction_query = QueryTransactions::new(
                    &Some(*self.date_from()),
                    &Some(*self.date_to()),
                    &None,
                    &None,
                    &None,
                    &Some(cat_name_re),
                    &None,
                    &None,
                    &None,
                    &None,
                    &None,
                    &None,
                    &None,
                );

                let filt_transactions = transaction_query.exec(db);
                let sum = sum_transactions(&filt_transactions);
                let allotment = cat.budget_amount_over_interval(*self.date_from(), *self.date_to());

                (cat.full_name(db), sum, allotment)
            })
            .collect();

        budget_spent
    }
}
