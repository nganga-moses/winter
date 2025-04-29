use sysinfo::{Disks};

#[tauri::command]
pub fn get_free_disk_space() -> Result<u64, String> {
    let disks = Disks::new_with_refreshed_list();

    let target_path = dirs::home_dir()
        .ok_or("Failed to get home directory")?
        .join("WinterData");

    for disk in disks.iter() {
        if target_path.starts_with(disk.mount_point()) {
            return Ok(disk.available_space());
        }
    }

    Err("Could not find a disk containing WinterData folder".into())
}