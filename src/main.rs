use std::thread;
use sys_monitor::{get_processes, get_ram, get_system_info, get_temp_info};
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys: System = System::new_all();
    let sys_info = get_system_info(&mut sys);

    let ram_thread = thread::spawn(move || {
        let mut sysr = System::new_all();
        get_ram(&mut sysr);
    });

    let processes = thread::spawn(move || {
        let mut sysr = System::new_all();
        get_processes(&mut sysr);
    });

    let temp_thread = thread::spawn(move || {
        let mut sysr = System::new_all();
        get_temp_info(&mut sysr);
    });

    ram_thread.join().unwrap();
    processes.join().unwrap();
    temp_thread.join().unwrap();
}
