use sys_monitor::{get_processes, get_ram, get_system_info, get_temp_info, send_data_to_django};
use sysinfo::{System, SystemExt};

#[tokio::main]
async fn main() {
    let mut sys: System = System::new_all();
    let sys_info = get_system_info(&mut sys);

    let mut sysr = System::new_all();
    let ram_data = get_ram(&mut sysr);

    let mut sysr = System::new_all();
    let procs = get_processes(&mut sysr);
    println!("{:?}", procs);

    let mut sysr = System::new_all();
    let temps = get_temp_info(&mut sysr);

    match send_data_to_django(sys_info, ram_data, procs, temps).await {
        Ok(()) => println!("Done!"),
        Err(()) => println!("Error!"),
    }
}
