use sysinfo::{self, MINIMUM_CPU_UPDATE_INTERVAL};
use chrono::{self, DateTime, NaiveDateTime};

/* Constants  */
const BYTES_PER_GB: u64 = 1024 * 1024 * 1024; // 1,073,741,824 bytes per gb. convert from b to gb
const MHZ_TO_GHZ:   f64 = 0.001;              // number used when converting mhz frequency to ghz

#[derive(Debug)]
pub struct SysResources {
    pub available_memory: u64,   // memory that can be used by the system (gb)
    pub used_memory:      u64,   // memory in use by the system (gb)
    pub total_memory:     u64,   // total memory installed in the system (gb)
    pub boot_time:        u64,   // epoch time from when the system was booted
    pub uptime:           u64,   // system uptime (days:hours:minutes:seconds)
    pub cpu:              CPU,   // struct containing information about the system cpu
    pub num_of_processes: usize, // number of running processes

    // private
    system: sysinfo::System,     // internal system struct used to gather info
}

#[derive(Default, Debug)]
pub struct CPU {
    pub processes:  Vec<CPU>,  // logical processes, if any. if this is a logical processor it will be empty.
    pub cpu_usage:  f32,       // usage of the cpu or logical processor as a percent
    pub core_count: usize,     // phyiscal core count. if this is a logical processor, it will be 0
    pub frequency:  f64,       // frequency of the cpu or logical processor in mhz    
    pub name:       String,    // Not reloaded. name of the cpu, i.e cpu 1, cpu 2.
    pub vendor_id:  String,    // Not reloaded. vendor id, i.e AuthenticAMD
    pub brand:      String     // Not reloaded. the model of the cpu, i.e AMD Ryzen 7 5700
}

impl CPU {
    /**
     * Create a CPU struct from a sysinfo internel 
     * 'Cpu' struct. Core count and processes are set
     * to default values.
     */
    pub fn load_from_raw(raw_cpu: &sysinfo::Cpu) -> Self {        
        let mut cpu = Self::default();
        cpu.brand = raw_cpu.brand().to_string();
        cpu.core_count = 0;
        cpu.cpu_usage = raw_cpu.cpu_usage();
        cpu.processes = Vec::new();
        cpu.frequency = raw_cpu.frequency() as f64;
        cpu.name = raw_cpu.name().to_string();
        cpu.vendor_id = raw_cpu.vendor_id().to_string();
        
        return cpu
    }

    /**
     * Get the frequency of a cpu (or logical processor) 
     * in Ghz, converting from Mhz.
     */
    pub fn get_cpu_frequency_ghz(&self) -> f64 {
        return &self.frequency * MHZ_TO_GHZ;
    }
}

impl SysResources {    
    /**
     * Create a new, empty SysResources struct
     * with nothing loaded
     */
    pub fn new() -> SysResources {
        Self {
            available_memory: 0,
            used_memory: 0,
            total_memory: 0,
            boot_time: 0,
            uptime: 0,
            cpu: CPU::default(),
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
        self.system.refresh_cpu_usage();
        self.cpu.cpu_usage = self.system.global_cpu_usage();

        return self.cpu.cpu_usage
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
    pub fn reload(&mut self) {
        self.system.refresh_all();
        self.uptime = sysinfo::System::uptime();
        self.available_memory = self.system.available_memory();
        self.used_memory = self.system.used_memory();
        self.num_of_processes = self.system.processes().len();
        self.reload_cpu_info();
    }

    // used memory in bytes. divide by BYTES_PER_GB to get gb
    pub fn used_memory_gb(&self) -> u64 {
        return self.used_memory / BYTES_PER_GB; 
    }

    // avaiable memory in bytes. divide by BYTES_PER_GB to get gb
    pub fn available_memory_gb(&self) -> u64 {
        return self.available_memory / BYTES_PER_GB; 
    }

    // total memory in bytes. divide by BYTES_PER_GB to get gb
    pub fn total_memory_gb(&self) -> u64 {
        return self.total_memory / BYTES_PER_GB;    
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
     * Load all information about the CPU
     * and fill in all fields.
     * Resource heavy, so only should be called once initially.
     * If you want to update CPU info, call 
     * reload_cpu_info instaed.
     */
    pub fn load_cpu_info(&mut self) {        
        self.system.refresh_cpu_all();
        
        let raw_cpu: &sysinfo::Cpu = &self.system.cpus()[0];
        
        self.cpu.brand      = raw_cpu.brand().to_string();
        self.cpu.core_count = self.system.physical_core_count().unwrap_or(0);
        self.cpu.cpu_usage  = raw_cpu.cpu_usage();
        self.cpu.frequency  = raw_cpu.frequency() as f64;
        self.cpu.name       = raw_cpu.name().to_string();
        self.cpu.vendor_id  = raw_cpu.vendor_id().to_string();
        self.cpu.processes  = self.system.cpus().iter().map(CPU::load_from_raw).collect();
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
            if let Some(saved_cpu_core) = self.cpu.processes.get_mut(index) {
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
        
        self.cpu.frequency  = raw_cpu.frequency() as f64;
        self.reload_cpu_cores();
    }
}