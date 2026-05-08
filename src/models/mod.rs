use serde::{Deserialize, Serialize};

pub const FILE_PATH: &str = "./tasks.json";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub is_deleted: bool,
    pub todo: String,
    pub data: Option<String>,
    pub status: Option<String>,
}
