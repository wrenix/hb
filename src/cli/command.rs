//! Top level CLI command

use super::QueryOpts;
use crate::config::default_cfg_file;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

lazy_static! {
    static ref DEFAULT_CFG: String = default_cfg_file().to_str().unwrap().to_string();
}

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct CliOpts {
    #[structopt(
        short = "c",
        long = "config",
        help = "Path to hb configuration file",
        default_value = &DEFAULT_CFG
    )]
    pub path: PathBuf,

    // make optional subcommands
    #[structopt(subcommand)]
    pub subcmd: Option<SubCommand>,
}

impl CliOpts {
    /// Retrieve the CLI config path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Retrieve the subcommand given, if any
    pub fn subcommand(&self) -> Option<&SubCommand> {
        match &self.subcmd {
            Some(sc) => Some(sc),
            None => None,
        }
    }
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    Query(QueryOpts),
}
