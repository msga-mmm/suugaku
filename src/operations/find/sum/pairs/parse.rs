use std::fs;

use super::model;

pub(super) struct ParsedInput {
    pub from: Vec<i32>,
    pub target: i32,
}

impl model::Pairs {
    pub(super) fn parse(&self) -> ParsedInput {
        let numbers = if let Some(file) = &self.from_file {
            let numbers = fs::read_to_string(file).expect("file should exist");
            let numbers: Vec<i32> = numbers
                .trim()
                .split(',')
                .map(|ch| {
                    ch.parse().unwrap_or_else(|_| {
                        panic!(
                            "each number should be a valid number, found: {ch}"
                        )
                    })
                })
                .collect();

            numbers
        } else {
            self.from.clone()
        };

        ParsedInput {
            from: numbers,
            target: self.target,
        }
    }
}
