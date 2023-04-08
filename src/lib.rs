extern crate sysinfo;
use std::{collections::HashMap, rc::Rc};

use sysinfo::{Cpu, DiskUsage, Pid, ProcessExt, System, SystemExt};

#[derive(Debug)]
pub struct Raminfo {
    used_memory: u64,
    total_memory: u64,
    used_swap: u64,
    total_swap: u64,
}

#[derive(Debug)]
pub struct Proc {
    pid: Pid,
    name: String,
    disk_usage: DiskUsage,
}

#[derive(Debug)]
pub struct TempInfo {
    values: Vec<String>,
}

#[derive(Debug)]
pub struct SystemInfo<'a> {
    name: String,
    kernel_version: String,
    host_name: String,
    os_version: String,
    cpu_count: usize,
    cpus: Rc<&'a [Cpu]>,
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
    for (_, process) in sys.processes() {
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
    let mut temp_vec: Vec<String> = vec![];
    for component in sys.components() {
        temp_vec.push(format!("{:?}", component));
        println!("{:?}", component);
    }
    TempInfo { values: temp_vec }
}

pub fn get_system_info(sys: &mut System) -> SystemInfo {
    sys.refresh_cpu();
    let cps = Rc::new(sys.cpus());
    let cpus = cps.to_owned();
    SystemInfo {
        name: sys.name().unwrap(),
        kernel_version: sys.kernel_version().unwrap(),
        host_name: sys.host_name().unwrap(),
        os_version: sys.os_version().unwrap(),
        cpu_count: sys.cpus().len(),
        cpus,
    }
}
