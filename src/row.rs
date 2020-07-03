use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub id: f64,
    pub age: f64,
    pub blood_pressure: f64,
    pub resting_heart_rate: f64,
}
