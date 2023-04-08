extern crate sysinfo;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, rc::Rc};
use sysinfo::{Cpu, DiskUsage, Pid, ProcessExt, System, SystemExt};

pub async fn send_data_to_django(
    sysinfo: SystemInfo,
    raminfo: Raminfo,
    prcs: Vec<Proc>,
    tp_inf: TempInfo,
) -> Result<(), ()> {
    let client = Client::new();
    let url1 = "http://127.0.0.1:8000/items/temp";
    let url2 = "http://127.0.0.1:8000/items/sys";
    let url3 = "http://127.0.0.1:8000/items/ram";
    let url4 = "http://127.0.0.1:8000/items/processes";
    let payload1 = json!(tp_inf);
    let payload2 = json!(sysinfo);
    let payload3 = json!(raminfo);
    let payload4 = json!(prcs[0]);
    println!("{:?}", payload4);
    let response1 = client.post(url1).json(&payload1).send().await.unwrap();
    let response2 = client.post(url2).json(&payload2).send().await.unwrap();
    let response3 = client.post(url3).json(&payload3).send().await.unwrap();
    let response4 = client.post(url4).json(&payload4).send().await.unwrap();
    if response1.status().is_success()
        && response2.status().is_success()
        && response3.status().is_success()
        && response4.status().is_success()
    {
        Ok(())
    } else {
        Err(())
    }
}

#[derive(Debug, Serialize)]
pub struct Raminfo {
    used_memory: u64,
    total_memory: u64,
    used_swap: u64,
    total_swap: u64,
}

#[derive(Debug, Serialize)]
pub struct Proc {
    pid: String,
    name: String,
    written_bytes: u64,
    total_written_bytes: u64,
    read_bytes: u64,
    total_read_bytes: u64,
}

#[derive(Debug, Serialize)]
pub struct ProcList {
    li: Vec<Proc>,
}

#[derive(Debug, Serialize)]
pub struct TempInfo {
    values: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    name: String,
    kernel_version: String,
    host_name: String,
    os_version: String,
    cpu_count: usize,
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

pub fn get_processes(sys: &mut System) -> Vec<Proc> {
    let mut map: Vec<Proc> = vec![];
    for (_, process) in sys.processes() {
        let disk_us = process.disk_usage();
        let proc: Proc = Proc {
            pid: String::from(format!("{:?}", process.pid())),
            name: String::from(process.name()),
            read_bytes: disk_us.read_bytes,
            total_read_bytes: disk_us.total_read_bytes,
            written_bytes: disk_us.written_bytes,
            total_written_bytes: disk_us.total_written_bytes,
        };
        map.push(proc);
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
    // let cps = Rc::new(sys.cpus());
    // let cpus = cps.to_owned();
    SystemInfo {
        name: sys.name().unwrap(),
        kernel_version: sys.kernel_version().unwrap(),
        host_name: sys.host_name().unwrap(),
        os_version: sys.os_version().unwrap(),
        cpu_count: sys.cpus().len(),
    }
}
