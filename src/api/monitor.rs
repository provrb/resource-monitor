use sysinfo;

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
    pub boot_time: u64,
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
            boot_time: 0,
            cpu_info: CPU::default(),
            num_of_processes: 0,
            system: sysinfo::System::new()
        }
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
        self.num_of_processes = self.system.processes().len();
        
        self.load_cpu_info();
    }

    pub fn reload(mut self) {
        self.system.refresh_all();
        self.available_memory = self.system.available_memory();
        self.used_memory = self.system.used_memory();
        self.num_of_processes = self.system.processes().len();
        self.reload_cpu_info();
    }

    pub fn used_memory_gb(&self) -> u64 {
        return self.used_memory / 1000000000; // used memory in bytes. divide by 1 billion to get gb
    }

    pub fn available_memory_gb(&self) -> u64 {
        return self.available_memory / 1000000000; // avaiable  memory in bytes. divide by 1 billion to get gb
    }

    fn reload_cpu_cores(&mut self) {
        let mut cores = self.system.cpus();

        // load info about cpu cores
        for (index, core ) in cores.iter().enumerate() {
            // update info using index
            if let Some(saved_cpu_core) = self.cpu_info.processes.get_mut(index) {
                saved_cpu_core.cpu_usage = core.cpu_usage();
                saved_cpu_core.frequency = core.frequency() as f64;
            }
        }
    }

    fn reload_cpu_info(&mut self) {
        let mut raw_cpu: &sysinfo::Cpu;
        let mut cores = self.system.cpus();

        match cores.get(0) {
            Some(cpu) => raw_cpu = cpu,
            None => return
        }

        self.cpu_info.cpu_usage  = self.system.global_cpu_usage();
        self.cpu_info.frequency  = raw_cpu.frequency() as f64;
        self.reload_cpu_cores();
    }

    fn load_cpu_info(&mut self) {        
        self.system.refresh_cpu_all();

        let mut raw_cpu: &sysinfo::Cpu = &self.system.cpus()[0];
        
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