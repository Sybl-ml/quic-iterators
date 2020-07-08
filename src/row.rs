use serde::{Deserialize, Serialize};

pub struct RowGenerator {
    total: usize,
    output: usize,
}

impl RowGenerator {
    pub fn new(total: usize) -> Self {
        Self { total, output: 0 }
    }
}

impl Iterator for RowGenerator {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.output == self.total {
            return None;
        }

        self.output += 1;

        Some(Row {
            id: self.output,
            age: 20,
            shoe_size: 9,
            resting_heart_rate: 60,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub id: usize,
    pub age: usize,
    pub shoe_size: usize,
    pub resting_heart_rate: usize,
}
