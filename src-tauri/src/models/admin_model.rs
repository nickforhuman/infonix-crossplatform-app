use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdministrationUser {
    pub id: u32,
    pub username: String,
    pub name: String,
    pub group: String,
    pub shell: String,
    pub uid: String,
    pub gid: String,
    pub base_directory: String,
    pub user_shell: String,
    pub ram: f32,
    pub cpu: f32,
    pub disk: f32,
    pub status: String
}