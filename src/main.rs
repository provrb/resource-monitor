use sysinfo::Disks;

// api
mod api {
    pub mod monitor;
}

fn main() {
    let mut rsrc = api::monitor::SysResources::new();
    rsrc.load();

    println!("CPU Information");
    println!("Product Details");
    println!("Brand:     {}", rsrc.cpu_info.brand);
    println!("Vendor ID: {}", rsrc.cpu_info.vendor_id);
    println!("Frequency: {} GHz",  rsrc.cpu_info.frequency / 1000.0);
    println!("Cores:     {}", rsrc.cpu_info.core_count);
    println!();
    println!("Performance Details");
    println!("CPU Usage:         {} %", rsrc.get_cpu_usage() );
    println!("Running Processes: {}", rsrc.num_of_processes);
    println!("Total Memory:      {} GB", rsrc.total_memory_gb());
    println!("Available:         {} GB", rsrc.available_memory_gb());
    println!("Used:              {} GB", rsrc.used_memory_gb());
    println!("Logical Processors ({}):", rsrc.cpu_info.processes.len());
    for (index, core) in rsrc.cpu_info.processes.iter().enumerate() {
        if index > 2 {
            println!("    ... (truncated)");
            break; 
        }
        println!("    {}: {} MHz - Usage: {}% ", core.name, core.frequency, core.cpu_usage);
    }
    
    println!();

    println!("System Info");
    println!("Boot time (yyyy/mm/dd): {}", rsrc.get_boot_time().unwrap_or_default().format("%Y-%m-%d %I:%M:%S %p"));
    println!("Uptime   (dd/hh/mm/ss): {}", rsrc.get_uptime().unwrap_or_default().format("%d:%H:%M:%S"));
}
