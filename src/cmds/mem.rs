use clap::{Args, ValueEnum};
use sysinfo::{System, SystemExt};

use crate::{BytesUnit, I3Blocks, I3BlocksDisplay, I3BlocksError};

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

impl I3Blocks<MemArgs> for MemStats {
    fn get(command: &MemArgs) -> Result<Option<I3BlocksDisplay>, I3BlocksError> {
        let mem_stats = Self::get_mem_stats();
        let lines = mem_stats.i3blocks_print(command.unit, command.display);
        let color = define_threshold_color(
            command.warning,
            command.critical,
            mem_stats.used_percent as f32,
        );
        Ok(Some(I3BlocksDisplay::new(lines.clone(), lines, color)))
    }
}

impl MemStats {
    fn get_mem_stats() -> Self {
        let mut sys = System::new();
        sys.refresh_memory();

        // todo: remove buffered memory to be more accurate
        let usage_percent = sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0;
        MemStats {
            usage_bytes: sys.used_memory(),
            total_bytes: sys.total_memory(),
            used_percent: usage_percent as u8,
        }
    }

    fn i3blocks_print(&self, unit: BytesUnit, display: MemoryDisplay) -> String {
        match display {
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
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_mem_stats_print() {}
}
