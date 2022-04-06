use anyhow::Context;
use cli::{CliOpts, QueryOpts, SubCommand};
use config::Config;
use homebank_db::{HomeBankDb, Transaction};
use structopt::StructOpt;

use crate::cli::QueryType;

pub mod cli;
pub mod config;

fn main() -> Result<(), anyhow::Error> {
    let cli_opts = CliOpts::from_args();

    let cfg = Config::try_from(&cli_opts)?;
    let db = match HomeBankDb::try_from(cfg.path()) {
        Ok(db) => db,
        Err(e) => return Err(e).with_context(|| "Error parsing HomeBank file."),
    };

    match &cli_opts.subcommand() {
        Some(SubCommand::Query(q_opts)) => match q_opts.qtype() {
            QueryType::Transactions(query) => {
                let filt_transactions: Vec<&Transaction> = db
                    .transactions()
                    .iter()
                    // filter out dates before the given date
                    .filter(|&t| match query.date_from() {
                        Some(d) => t.date() >= d,
                        None => true,
                    })
                    // filter out dates on or after the given date
                    .filter(|&t| match query.date_to() {
                        Some(d) => t.date() < d,
                        None => true,
                    })
                    .collect();
                println!("{:#?}", query);
                println!("{:#?}", filt_transactions);
            }
            _ => {}
        },
        None => {}
    }

    Ok(())
}
