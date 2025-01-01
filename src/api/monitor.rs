use std::time::Duration;

use sysinfo::{self, MINIMUM_CPU_UPDATE_INTERVAL};
use chrono::{self, DateTime, NaiveDateTime};

#[derive(Default, Debug)]
pub struct CPU {
    // statistics
    pub processes: Vec<CPU>, // cores
    pub cpu_usage: f32,
    pub core_count: usize,
    
    // brand/product info
    pub frequency: f64, 
    pub name: String,
    pub vendor_id: String,
    pub brand: String
}

impl CPU {
    pub fn load_from_raw(raw_cpu: &sysinfo::Cpu) -> Self {        
        let mut cpu = CPU::default();
        cpu.brand = raw_cpu.brand().to_string();
        cpu.core_count = 0;
        cpu.cpu_usage = raw_cpu.cpu_usage();
        cpu.processes = Vec::new();
        cpu.frequency = raw_cpu.frequency() as f64;
        cpu.name = raw_cpu.name().to_string();
        cpu.vendor_id = raw_cpu.vendor_id().to_string();
        
        return cpu
    }
}

#[derive(Debug)]
pub struct SysResources {
    pub available_memory: u64,
    pub used_memory: u64,
    pub total_memory: u64,
    pub boot_time: u64,
    pub uptime: u64,
    pub cpu_info: CPU,
    pub num_of_processes: usize,

    system: sysinfo::System,
}

impl SysResources {    
    /**
     * Create a new, empty SysResources struct
     * with nothing loaded
     */
    pub fn new() -> SysResources {
        SysResources {
            available_memory: 0,
            used_memory: 0,
            total_memory: 0,
            boot_time: 0,
            uptime: 0,
            cpu_info: CPU::default(),
            num_of_processes: 0,
            system: sysinfo::System::new()
        }
    }

    /**
     * Return the percentage of the CPU
     * usage.
     */
    pub fn get_cpu_usage(&mut self) -> f32 {
        self.system.refresh_cpu_usage();
        std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        self.cpu_info.cpu_usage = self.system.global_cpu_usage();

        return self.cpu_info.cpu_usage
    }

    /**
     * Populate fields in SysResources with 
     * information about the system
     */
    pub fn load(&mut self) { 
        self.system = sysinfo::System::new_all();
        self.available_memory = self.system.available_memory();
        self.used_memory      = self.system.used_memory();
        self.boot_time        = sysinfo::System::boot_time();
        self.uptime           = sysinfo::System::uptime();
        self.num_of_processes = self.system.processes().len();
        self.total_memory     = self.system.total_memory();
        
        self.load_cpu_info();
    }

    /**
     * Reload all fields of information
     * that can change. Fields that don't need to be
     * reloaded will stay the same, e.g total memory
     */
    pub fn reload(mut self) {
        self.system.refresh_all();
        self.uptime = sysinfo::System::uptime();
        self.available_memory = self.system.available_memory();
        self.used_memory = self.system.used_memory();
        self.num_of_processes = self.system.processes().len();
        self.reload_cpu_info();
    }

    // used memory in bytes. divide by 1 billion to get gb
    pub fn used_memory_gb(&self) -> u64 {
        return self.used_memory / 1000000000; 
    }

    // avaiable memory in bytes. divide by 1 billion to get gb
    pub fn available_memory_gb(&self) -> u64 {
        return self.available_memory / 1000000000; 
    }

    // total memory in bytes. divide by 1 billion to get gb
    pub fn total_memory_gb(&self) -> u64 {
        return self.total_memory / 1000000000;    
    }

    /**
     * Get systems uptime in local time
     */
    pub fn get_uptime(&self) -> Option<NaiveDateTime> {
        let time =  DateTime::from_timestamp(self.uptime as i64, 0);
        match time {
            Some(utc) => return Some(utc.naive_local()),
            None => {}
        }
        return None;
    }

    /**
     * Get the time the system was booted
     * in local time.
     */
    pub fn get_boot_time(&self) -> Option<NaiveDateTime> {
        let time =  DateTime::from_timestamp(self.boot_time as i64, 0);
        match time {
            Some(utc) => return Some(utc.naive_local()),
            None => {}
        }
        return None;
    }

    /**
     * Update the usage and frequency of each
     * of the logical processes belonging to the
     * cpu. Information is saved directly to
     * the CPU struct.
     */
    fn reload_cpu_cores(&mut self) {
        let cores = self.system.cpus();

        // load info about cpu cores
        for (index, core ) in cores.iter().enumerate() {
            // update info using index
            if let Some(saved_cpu_core) = self.cpu_info.processes.get_mut(index) {
                saved_cpu_core.cpu_usage = core.cpu_usage();
                saved_cpu_core.frequency = core.frequency() as f64;
            }
        }
    }

    /**
     * Reload informationa about the CPU
     * that changes. Fields that dont change
     * like brand or vendor ID are not refreshed.
     * 
     * load_cpu_info should be called first.
     */
    fn reload_cpu_info(&mut self) {
        self.get_cpu_usage();
        
        let raw_cpu = self.system.cpus().get(0).unwrap();
    
        self.cpu_info.frequency  = raw_cpu.frequency() as f64;
        self.reload_cpu_cores();
    }

    /**
     * Load all information about the CPU
     * and fill in all fields.
     * Resource heavy, so only should be called once initially.
     * If you want to update CPU info, call 
     * reload_cpu_info instaed.
     */
    pub fn load_cpu_info(&mut self) {        
        self.system.refresh_cpu_all();

        let raw_cpu: &sysinfo::Cpu = &self.system.cpus()[0];
        
        self.cpu_info.brand      = raw_cpu.brand().to_string();
        self.cpu_info.core_count = self.system.physical_core_count().unwrap_or(0);
        self.cpu_info.cpu_usage  = raw_cpu.cpu_usage();
        self.cpu_info.frequency  = raw_cpu.frequency() as f64;
        self.cpu_info.name       = raw_cpu.name().to_string();
        self.cpu_info.vendor_id  = raw_cpu.vendor_id().to_string();
 
        for p in self.system.cpus() {
            self.cpu_info.processes.push(CPU::load_from_raw(p));
        }
    }
}