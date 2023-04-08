extern crate sysinfo;
use std::{collections::HashMap, sync::Arc};

use sysinfo::{Cpu, DiskUsage, Pid, ProcessExt, System, SystemExt};

pub struct Raminfo {
    used_memory: u64,
    total_memory: u64,
    used_swap: u64,
    total_swap: u64,
}

pub struct Proc {
    pid: Pid,
    name: String,
    disk_usage: DiskUsage,
}

pub struct TempInfo {
    values: Vec<String>,
}

pub struct SystemInfo {
    name: String,
    kernel_version: String,
    host_name: String,
    os_version: String,
    cpu_count: usize,
    cpus: Vec<Cpu>,
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

pub fn get_processes(sys: &mut System) -> HashMap<&str, Proc> {
    let mut map: HashMap<&str, Proc> = HashMap::new();
    for (pid, process) in sys.processes() {
        let proc: Proc = Proc {
            pid: process.pid(),
            name: String::from(process.name()),
            disk_usage: process.disk_usage(),
        };
        map.insert(process.name(), proc);
    }
    map
}

pub fn get_temp_info(sys: &mut System) -> TempInfo {
    let mut tempVec: Vec<String> = vec![];
    for component in sys.components() {
        tempVec.push(format!("{:?}", component));
        println!("{:?}", component);
    }
    TempInfo { values: tempVec }
}

pub fn get_system_info(sys: &mut System) -> SystemInfo {
    sys.refresh_cpu();
    let cpus: Vec<Cpu> = Vec::from();
    SystemInfo {
        name: sys.name().unwrap(),
        kernel_version: sys.kernel_version().unwrap(),
        host_name: sys.host_name().unwrap(),
        os_version: sys.os_version().unwrap(),
        cpu_count: sys.cpus().len(),
        cpus: cpus,
    }
}
