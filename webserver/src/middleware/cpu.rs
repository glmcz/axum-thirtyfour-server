

// runtime check of CPU info 
use log;
use sysinfo::System;


pub struct CPU {
    sys: System,
    limit: f32,
    cpu_usage: f32,
    cpu_freq: u64,
    total_memory: u64,
    memory_usage: u64
}
pub fn init_cpu_measurement() -> CPU {
    CPU {
        sys: System::new(),
        limit: 100.0,
        cpu_usage: 0.0,
        cpu_freq: 0,
        total_memory: 0,
        memory_usage: 0
    }
} 

impl CPU {

    pub fn get_host_platform() -> Option<String> {
       System::cpu_arch() 
    }
    
    pub fn get_cpu_usage(&mut self) {
        self.sys.refresh_cpu(); 
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.sys.refresh_cpu(); 
        for cpu in self.sys.cpus() {
            log::info!("CPU usage is {}% ", cpu.cpu_usage());
            log::info!("cpu freq is {}", self.sys.global_cpu_info().frequency());
        }
        // self.sys.refresh_cpu_frequency();
    }

    pub fn memory_usage_is_critical(&mut self) {
        &mut self.sys.refresh_memory();
        if &self.sys.used_memory() >= &90 {
            log::warn!("Total mem is {} used is {}", &self.sys.total_memory(), &self.sys.used_memory());
        }
    }
    pub fn get_memory_usage(&mut self) {
        &mut self.sys.refresh_memory();
        &self.sys.total_memory();
        &self.sys.used_memory();
        log::info!("Total mem is {} used is {}", &self.sys.total_memory(), &self.sys.used_memory());
        println!("Total mem is {} used is {}", &self.sys.total_memory(), &self.sys.used_memory());
    }
}

