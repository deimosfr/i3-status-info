use clap::{Args, ValueEnum};
use sysinfo::{CpuExt, System, SystemExt};

use super::utils::define_threshold_color;

#[derive(Args)]
pub struct CpuArgs {
    #[arg(short, long, default_value_t=80, value_parser = clap::value_parser!(u8).range(2..100))]
    pub critical: u8,
    #[arg(short, long, default_value_t=60, value_parser = clap::value_parser!(u8).range(1..100))]
    pub warning: u8,
    #[arg(short, long, default_value = "all")]
    pub display: CpuDisplayStyle,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum CpuDisplayStyle {
    All,
    Average,
}

#[derive(Debug)]
pub struct CpuStats {
    pub cpu_usage_all_cores: Vec<u8>,
    pub cpu_usage_average: f32,
}

impl CpuStats {
    fn new(cpu_usage_all_cores: Vec<u8>, cpu_usage_average: f32) -> Self {
        Self {
            cpu_usage_all_cores,
            cpu_usage_average,
        }
    }

    pub fn print(warning: u8, critical: u8, display: CpuDisplayStyle) {
        let cpu_stats = Self::get_percent_usage();
        cpu_stats.i3blocks_print(display);
        if let Some(x) = define_threshold_color(warning, critical, cpu_stats.cpu_usage_average) {
            println!("{x}");
        }
    }

    /// Get CPU usage
    fn get_percent_usage() -> CpuStats {
        let mut sys = System::new();
        let mut all_cores_usage = Vec::with_capacity(
            sys.physical_core_count()
                .expect("can't get the number of physical cores"),
        );

        // Refreshing CPU information.
        for _ in 0..2 {
            sys.refresh_cpu();

            // Sleeping for 500 ms to let time for the system to run for long
            // enough to have useful information.
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        sys.cpus()
            .iter()
            .for_each(|cpu| all_cores_usage.push(cpu.cpu_usage()));
        let average_cores_usage =
            all_cores_usage.iter().sum::<f32>() / all_cores_usage.len() as f32;
        CpuStats::new(
            all_cores_usage.iter().map(|x| *x as u8).collect(),
            average_cores_usage,
        )
    }

    fn i3blocks_print(&self, display: CpuDisplayStyle) {
        let average = format!("{:.1}%", self.cpu_usage_average);
        match display {
            CpuDisplayStyle::All => {
                println!(
                    "{}",
                    self.cpu_usage_all_cores
                        .iter()
                        .map(|x| format!("{:02}%", x))
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            CpuDisplayStyle::Average => println!("{}", average),
        }
        println!("{}", average)
    }
}

#[cfg(test)]
mod tests {
    use super::{CpuDisplayStyle, CpuStats};

    #[test]
    fn test_cpu_stats_print() {
        CpuStats::print(80, 90, CpuDisplayStyle::All);
        CpuStats::print(80, 90, CpuDisplayStyle::Average);
    }
}
