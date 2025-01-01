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
    println!("Usage:            {} %", rsrc.cpu_info.cpu_usage);
    println!("Available memory: {} GB", rsrc.available_memory_gb());
    println!("Used Memory:      {} GB", rsrc.used_memory_gb());
    println!("Logical Processors:");
    for core in rsrc.cpu_info.processes.iter() {
        println!("{}: {} MHz - Usage: {}% ", core.name, core.frequency, core.cpu_usage);
    }

}
