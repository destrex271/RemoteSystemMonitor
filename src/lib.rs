extern crate sysinfo;
use sysinfo::{System, SystemExt};

pub struct Raminfo {
    used_memory: u64,
    total_memory: u64,
    used_swap: u64,
    total_swap: u64,
}

pub fn get_ram(sys: &mut System) -> Raminfo {
    sys.refresh_all();

    let used_mem = sys.used_memory();
    let total_mem = sys.total_memory();
    let used_swap = sys.used_swap();
    let total_swap = sys.total_swap();

    Raminfo {
        used_memory: used_mem,
        total_memory: total_mem,
        used_swap,
        total_swap,
    }
}
