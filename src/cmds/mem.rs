use clap::{Args, ValueEnum};
use sysinfo::{System, SystemExt};

use crate::BytesUnit;

use super::utils::define_threshold_color;

#[derive(Args)]
pub struct MemArgs {
    #[arg(short, long, default_value_t=80, value_parser = clap::value_parser!(u8).range(2..100))]
    pub critical: u8,
    #[arg(short, long, default_value_t=60, value_parser = clap::value_parser!(u8).range(1..100))]
    pub warning: u8,
    #[arg(short, long, default_value = "gb")]
    pub unit: BytesUnit,
    #[arg(short, long, default_value = "used")]
    pub display: MemoryDisplay,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum MemoryDisplay {
    Used,
    Remaining,
    UsedPercentage,
    RemainingPercentage,
}
#[derive(Debug)]
pub struct MemStats {
    usage_bytes: u64,
    total_bytes: u64,
    used_percent: u8,
}

impl MemStats {
    fn new(usage_bytes: u64, total_bytes: u64, used_percent: u8) -> Self {
        Self {
            usage_bytes,
            total_bytes,
            used_percent,
        }
    }

    pub fn print(warning: u8, critical: u8, unit: BytesUnit, display: MemoryDisplay) {
        let mem_stats = Self::get_mem_stats();
        mem_stats.i3blocks_print(unit, display);
        if let Some(x) = define_threshold_color(warning, critical, mem_stats.used_percent as f32) {
            println!("{x}");
        }
    }

    fn get_mem_stats() -> MemStats {
        let mut sys = System::new();
        sys.refresh_memory();

        // todo: remove buffered memory to be more accurate
        let usage_percent = sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0;
        MemStats::new(sys.used_memory(), sys.total_memory(), usage_percent as u8)
    }

    fn i3blocks_print(&self, unit: BytesUnit, display: MemoryDisplay) {
        let memory = match display {
            MemoryDisplay::Used => match unit {
                BytesUnit::Kb => format!("{:.1}K", self.usage_bytes as f64 / 1024.0),
                BytesUnit::Mb => format!("{:.1}M", self.usage_bytes as f64 / 1024.0 / 1024.0),
                BytesUnit::Gb => {
                    format!("{:.1}G", self.usage_bytes as f64 / 1024.0 / 1024.0 / 1024.0)
                }
            },
            MemoryDisplay::Remaining => {
                let total_memory = match unit {
                    BytesUnit::Kb => self.total_bytes as f64 / 1024.0,
                    BytesUnit::Mb => self.total_bytes as f64 / 1024.0 / 1024.0,
                    BytesUnit::Gb => self.total_bytes as f64 / 1024.0 / 1024.0 / 1024.0,
                };
                match unit {
                    BytesUnit::Kb => {
                        format!("{:.1}K", total_memory - self.usage_bytes as f64 / 1024.0)
                    }
                    BytesUnit::Mb => format!(
                        "{:.1}M",
                        total_memory - self.usage_bytes as f64 / 1024.0 / 1024.0
                    ),
                    BytesUnit::Gb => {
                        format!(
                            "{:.1}G",
                            total_memory - self.usage_bytes as f64 / 1024.0 / 1024.0 / 1024.0
                        )
                    }
                }
            }
            MemoryDisplay::UsedPercentage => format!("{}%", self.used_percent),
            MemoryDisplay::RemainingPercentage => format!("{}%", 100 - self.used_percent),
        };
        println!("{memory}\n{memory}");
    }
}

#[cfg(test)]
mod tests {
    use super::{MemStats, MemoryDisplay};

    #[test]
    fn test_mem_stats_print() {
        println!("{:?}", MemStats::get_mem_stats());
        MemStats::print(80, 90, crate::BytesUnit::Gb, MemoryDisplay::Used);
        MemStats::print(80, 90, crate::BytesUnit::Gb, MemoryDisplay::UsedPercentage);
        MemStats::print(80, 90, crate::BytesUnit::Gb, MemoryDisplay::Remaining);
        MemStats::print(
            80,
            90,
            crate::BytesUnit::Gb,
            MemoryDisplay::RemainingPercentage,
        );
    }
}
