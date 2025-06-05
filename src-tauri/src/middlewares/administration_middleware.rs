use sysinfo::{ ProcessStatus, System, Users, ProcessesToUpdate, RefreshKind, ProcessRefreshKind};
use std::{thread::sleep, time::Duration, collections::HashMap, string::String};

#[path="../models/admin_model.rs"]
mod admin_model;
use admin_model::AdministrationUser;

#[tauri::command]
pub fn get_administration_user() -> Vec<AdministrationUser>{
    let mut system = System::new_with_specifics(
        RefreshKind::everything()
        .with_processes(ProcessRefreshKind::everything().with_memory())
    );
    
    sleep(Duration::from_millis(500));
    system.refresh_all();
    system.refresh_processes(ProcessesToUpdate::All, false);
    let users = Users::new_with_refreshed_list();
    let mut user_map = HashMap::new();
    for user in users.iter(){
        user_map.insert(user.id(), user);
    }
    let mut administration_users: Vec<AdministrationUser> = system.processes().iter().filter_map(|(&pid, process)| {
        let user_id = process.user_id()?;
        let user = user_map.get(user_id)?;

        Some(AdministrationUser {
            id: pid.as_u32(),
            username: user.name().to_string(),
            name: process.name().to_string_lossy().into_owned(),
            group: user.groups().iter().map(|g| g.name().to_string()).collect::<Vec<_>>().join(", "),
            shell: String::from("N/A"), 
            uid: user.id().to_string(),
            gid: user.group_id().to_string(),
            base_directory: String::from("N/A"),
            ram: process.memory() as f32 / 1024.0 / 1024.0,
            cpu: process.cpu_usage(),
            disk: (process.disk_usage().read_bytes + process.disk_usage().written_bytes) as f32 / 1024.0 / 1024.0,
            status: match process.status() {
                ProcessStatus::Run => "Running".to_string(),
                ProcessStatus::Sleep => "Idle".to_string(),
                _=> "Stopped".to_string(),
            },
            user_shell: String::from("N/A"),
        })
    }).collect();
    administration_users.sort_by(|a, b| b.ram.partial_cmp(&a.ram).unwrap_or(std::cmp::Ordering::Equal));
    administration_users.truncate(100);
    administration_users
}
