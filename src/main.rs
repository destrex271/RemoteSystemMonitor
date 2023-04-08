use std::thread;
use sys_monitor::{get_processes, get_ram, get_system_info, get_temp_info};
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys: System = System::new_all();
    let sys_info = get_system_info(&mut sys);

    let ram_thread = thread::spawn();
}
