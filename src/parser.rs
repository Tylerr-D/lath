#![allow(clippy::upper_case_acronyms, clippy::result_large_err)]
use pest::{self, Parser};

use crate::ast::{Node, Operator};

#[derive(pest_derive::Parser)]

#[grammar = "grammar.pest"]

pub struct CalcParser;

