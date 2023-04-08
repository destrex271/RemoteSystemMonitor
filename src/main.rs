use sys_monitor::get_temp_info;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys: System = System::new_all();
    get_temp_info(&mut sys);
}

