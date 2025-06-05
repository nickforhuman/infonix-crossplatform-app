use sysinfo::{ ProcessesToUpdate, System, Pid};

#[tauri::command]
pub fn kill_process(pid: f32) -> Result<(), String> {
    let mut system = System::new_all();
    let pid  = pid as usize;
    system.refresh_processes(ProcessesToUpdate::All, false);
    if let Some(process) = system.process(Pid::from(pid)) {
        if process.kill() {
            Ok(())
        } else {
            Err(format!("Failed to kill process {}", pid))
        }
    }else {
        Err(format!("Process {} not found", pid))
    }
}
#[tauri::command]
pub fn start_process(pid: u32) -> Result<(), String> {
    let mut system = System::new_all();
    let pid = pid as usize;
    system.refresh_processes(ProcessesToUpdate::All, false);
    if let Some(_process) = system.process(Pid::from(pid)) {
        Ok(())
    } else {
        Err(format!("process {} not found", pid))
    }
}
