use sysinfo::{CpuRefreshKind, ProcessRefreshKind, ProcessStatus, ProcessesToUpdate, RefreshKind, System};
use std::time::Duration;
use std::thread::sleep;
// model
#[path="../models/sys_model.rs"]
mod sys_model;
use sys_model::Task;

#[tauri::command]
pub fn get_system_tasks() -> Vec<Task> {
    let mut system = System::new_with_specifics(
        RefreshKind::everything()
        .with_processes(ProcessRefreshKind::everything().with_memory())
    );
    let mut scpu = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    sleep(Duration::from_millis(500));
    scpu.refresh_cpu_all();
    system.refresh_processes(ProcessesToUpdate::All, false);
    let mut tasks: Vec<Task> = system.processes().iter().map(|(&pid, process)| {
        let _cpu_usage = scpu.process(pid).map_or(0.0, |p| p.cpu_usage());
        Task {
            id: pid.as_u32(),
            name: process.name().to_string_lossy().into_owned(),
            ram: process.memory() as f32 / 1024.0 / 1024.0,
            cpu: process.cpu_usage(),
            disk: (process.disk_usage().read_bytes + process.disk_usage().written_bytes) as f32 / 1024.0 / 1024.0,
            status: match process.status() {
                ProcessStatus::Run => "Running".to_string(),
                ProcessStatus::Sleep => "Idle".to_string(),
                _=>"Stopped".to_string(),
            },
        }
    }).collect();
    tasks.sort_by(|a, b| b.ram.partial_cmp(&a.ram).unwrap_or(std::cmp::Ordering::Equal));
    tasks.truncate(1000);
    tasks
}