use std::path::Path;

use clap::{Args, ValueEnum};
use sysinfo::{DiskExt, System, SystemExt};

use crate::{BytesUnit, CommandStatus, I3Display, I3DisplayError};

use super::utils::define_threshold_color;

#[derive(Args)]
pub struct DiskUsageArgs {
    #[arg(short, long)]
    pub path: String,
    #[arg(short, long, default_value_t=80, value_parser = clap::value_parser!(u8).range(2..100))]
    pub critical_used_percentage: u8,
    #[arg(short, long, default_value_t=60, value_parser = clap::value_parser!(u8).range(1..100))]
    pub warning_used_percentage: u8,
    #[arg(short, long, default_value = "gb")]
    pub unit: BytesUnit,
    #[arg(short, long, default_value = "remaining")]
    pub display: DiskDisplay,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum DiskDisplay {
    Used,
    Remaining,
    UsedPercentage,
    RemainingPercentage,
}
#[derive(Debug)]
pub struct DiskStats {
    usage_mb: u64,
    total_mb: u64,
    used_percent: u8,
}

impl CommandStatus<DiskUsageArgs> for DiskStats {
    fn get(command: &DiskUsageArgs) -> Result<Option<I3Display>, I3DisplayError> {
        let disk_stats = Self::get_disk_stats(command.path.clone())?;
        let lines = disk_stats.i3blocks_print(command.unit, command.display);
        let color = define_threshold_color(
            command.warning_used_percentage,
            (command.critical_used_percentage + command.warning_used_percentage) / 2,
            command.critical_used_percentage,
            disk_stats.used_percent as f32,
        );
        Ok(Some(I3Display::new(
            None,
            lines.clone(),
            lines,
            Some(color),
        )))
    }
}

impl DiskStats {
    fn get_disk_stats(disk_path: String) -> Result<Self, I3DisplayError> {
        let disk_as_path = Path::new(&disk_path);

        let mut sys = System::new();
        sys.refresh_disks_list();

        for d in sys.disks() {
            if d.mount_point() == disk_as_path {
                return Ok(DiskStats {
                    total_mb: d.total_space(),
                    used_percent: (d.available_space() as f64 / d.total_space() as f64 * 100.0)
                        as u8,
                    usage_mb: d.total_space() - d.available_space(),
                });
            }
        }

        Err(I3DisplayError::from(format!("Disk {disk_path} not found")))
    }

    fn i3blocks_print(&self, unit: BytesUnit, display: DiskDisplay) -> String {
        match display {
            DiskDisplay::Used => match unit {
                BytesUnit::Kb => format!("{:.1}K", self.usage_mb / 1024),
                BytesUnit::Mb => format!("{:.1}M", self.usage_mb / 1024 / 1024),
                BytesUnit::Gb => {
                    format!("{:.1}G", self.usage_mb / 1024 / 1024 / 1024)
                }
            },
            DiskDisplay::Remaining => {
                let total = match unit {
                    BytesUnit::Kb => self.total_mb / 1024,
                    BytesUnit::Mb => self.total_mb / 1024 / 1024,
                    BytesUnit::Gb => self.total_mb / 1024 / 1024 / 1024,
                };
                match unit {
                    BytesUnit::Kb => {
                        format!("{:.1}K", total - self.usage_mb / 1024)
                    }
                    BytesUnit::Mb => format!("{:.1}M", total - self.usage_mb / 1024 / 1024),
                    BytesUnit::Gb => {
                        format!("{:.1}G", total - self.usage_mb / 1024 / 1024 / 1024)
                    }
                }
            }
            DiskDisplay::UsedPercentage => format!("{}%", self.used_percent),
            DiskDisplay::RemainingPercentage => format!("{}%", 100 - self.used_percent),
        }
    }
}

#[cfg(test)]
mod tests {}
