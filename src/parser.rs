#![allow(clippy::upper_case_acronyms, clippy::result_large_err)]

#[derive(pest_derive::Parser)]

#[grammar = "grammar.pest"]

pub struct CalcParser;

