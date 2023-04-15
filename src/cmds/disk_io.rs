use crate::{BytesUnit, I3Blocks, I3BlocksDisplay, I3BlocksError};
use clap::Args;
use procfs::diskstats;
use std::{fs, thread::sleep};

use super::utils::set_text_threshold_color;

const DISK_CHECK_NUM: u64 = 2;
const CHECK_INTERVAL_SEC: u64 = 1;
const DISK_CHECK_INTERVAL: f64 = (DISK_CHECK_NUM * CHECK_INTERVAL_SEC) as f64;
const PROC_STAT_PATH: &str = "/proc/stat";

#[derive(Args)]
pub struct DiskIoArgs {
    #[arg(short, long, default_value = "nvme0n1")]
    pub device: String,
    #[arg(short, long, default_value_t = 100)]
    pub critical_mb: u64,
    #[arg(short, long, default_value_t = 10)]
    pub warning_mb: u64,
    #[arg(short, long)]
    pub unit: Option<BytesUnit>,
}

#[derive(Debug)]
pub struct DiskIoStats {
    read_mb: f64,
    write_mb: f64,
    io_wait: f64,
}

struct PrettyDiskIoStats {
    read: f64,
    write: f64,
    read_unit: BytesUnit,
    write_unit: BytesUnit,
    io_wait_percentage: f64,
}

impl PrettyDiskIoStats {
    fn to_i3blocks_display(
        &self,
        disk_io_stat: DiskIoStats,
        warning: f64,
        critical: f64,
    ) -> I3BlocksDisplay {
        let read_value = set_text_threshold_color(
            warning,
            critical,
            disk_io_stat.read_mb,
            Some(format!("{:>5.1}{}/s", self.read, self.read_unit)),
        );
        let write_value = set_text_threshold_color(
            warning,
            critical,
            disk_io_stat.write_mb,
            Some(format!("{:>5.1}{}/s", self.write, self.write_unit)),
        );
        let iowait_value = set_text_threshold_color(
            5.0,
            10.0,
            disk_io_stat.io_wait,
            Some(format!("{:>3.1}%", self.io_wait_percentage)),
        );

        let lines = format!("{read_value} {write_value} {iowait_value}");
        I3BlocksDisplay::new(lines.clone(), lines, None)
    }
}

impl I3Blocks<DiskIoArgs> for DiskIoStats {
    fn get(command: &DiskIoArgs) -> Result<Option<I3BlocksDisplay>, I3BlocksError> {
        let io_stats = Self::get_stats(command.device.clone())?;
        let pretty_output = io_stats.pretty_content(command.unit);

        Ok(Some(pretty_output.to_i3blocks_display(
            io_stats,
            command.warning_mb as f64,
            command.critical_mb as f64,
        )))
    }
}

impl DiskIoStats {
    fn get_iowait() -> Result<f64, I3BlocksError> {
        let content = fs::read_to_string(PROC_STAT_PATH)
            .map_err(|e| I3BlocksError::from(format!("can't read file {PROC_STAT_PATH}: {e}")))?;

        let mut iowaits = Vec::new();

        // https://docs.kernel.org/filesystems/proc.html#miscellaneous-kernel-statistics-in-proc-stat
        let lines = content.lines();
        for line in lines {
            if line.starts_with("cpu") {
                let line_content = line.split_whitespace().collect::<Vec<&str>>();
                iowaits.push(
                    line_content[5].parse::<f64>().map_err(|e| {
                        I3BlocksError::from(format!("can't parse iowait value: {e}"))
                    })?,
                );
            }
        }

        Ok(iowaits.iter().sum::<f64>() / iowaits.len() as f64)
    }

    fn get_stats(device: String) -> Result<Self, I3BlocksError> {
        let mut reads = Vec::with_capacity(2);
        let mut writes = Vec::with_capacity(2);
        let mut iowait = Vec::with_capacity(2);

        for _ in 0..DISK_CHECK_NUM {
            // disk troughput: https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats
            let x = diskstats()
                .map_err(|e| I3BlocksError::from(format!("can't get disks stats: {}", e)))?;
            x.iter().for_each(|x| {
                if x.name == device {
                    reads.push(x.reads as f64);
                    writes.push(x.writes as f64);
                }
            });

            // ensure device exists
            if reads.is_empty() {
                return Err(I3BlocksError::from(format!("device `{device}` not found")));
            }

            // iowait
            iowait.push(Self::get_iowait()?);

            sleep(std::time::Duration::from_secs(CHECK_INTERVAL_SEC));
        }

        Ok(DiskIoStats {
            read_mb: (reads[1] - reads[0]) / DISK_CHECK_INTERVAL,
            write_mb: (writes[1] - writes[0]) / DISK_CHECK_INTERVAL,
            io_wait: (iowait[1] - iowait[0]) / DISK_CHECK_INTERVAL,
        })
    }

    fn pretty_content(&self, unit: Option<BytesUnit>) -> PrettyDiskIoStats {
        let default_unit = match unit {
            Some(x) => x,
            None => BytesUnit::Mb,
        };

        let mut pretty_stats = PrettyDiskIoStats {
            read: 0.0,
            write: 0.0,
            read_unit: default_unit,
            write_unit: default_unit,
            io_wait_percentage: self.io_wait,
        };

        // thoughput
        match unit {
            Some(BytesUnit::Kb) => {
                pretty_stats.read = self.read_mb * 1024.0;
                pretty_stats.write = self.write_mb * 1024.0;
            }
            Some(BytesUnit::Mb) => {
                pretty_stats.read = self.read_mb;
                pretty_stats.write = self.write_mb;
            }
            Some(BytesUnit::Gb) => {
                pretty_stats.read = self.read_mb / 1024.0;
                pretty_stats.write = self.write_mb / 1024.0;
            }
            None => {
                // auto adaptive read
                if self.read_mb > 1024.0 {
                    pretty_stats.read = self.read_mb / 1024.0;
                    pretty_stats.read_unit = BytesUnit::Gb;
                } else if self.read_mb < 1.0 {
                    pretty_stats.read = self.read_mb * 1024.0;
                    pretty_stats.read_unit = BytesUnit::Kb;
                } else {
                    pretty_stats.read = self.read_mb;
                    pretty_stats.read_unit = BytesUnit::Mb;
                }

                // auto adaptive write
                if self.write_mb > 1024.0 {
                    pretty_stats.write = self.write_mb / 1024.0;
                    pretty_stats.write_unit = BytesUnit::Gb;
                } else if self.write_mb < 1.0 {
                    pretty_stats.write = self.write_mb * 1024.0;
                    pretty_stats.write_unit = BytesUnit::Kb;
                } else {
                    pretty_stats.write = self.write_mb;
                    pretty_stats.write_unit = BytesUnit::Mb;
                }
            }
        };

        pretty_stats
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_diskio_stats_print() {}
}
