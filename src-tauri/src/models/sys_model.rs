use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub ram: f32,
    pub cpu: f32,
    pub disk: f32,
    pub status: String
}