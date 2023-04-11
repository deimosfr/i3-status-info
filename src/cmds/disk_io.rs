use sysinfo::{ProcessExt, System, SystemExt};

pub struct DiskIoStats {
    read_bytes: u64,
    written_bytes: u64,
}

impl DiskIoStats {
    pub fn print() {
        let s = System::new_all();
        for (pid, process) in s.processes() {
            let disk_usage = process.disk_usage();
            println!(
                "[{}] read bytes   : new/total => {}/{} B",
                pid, disk_usage.read_bytes, disk_usage.total_read_bytes,
            );
            println!(
                "[{}] written bytes: new/total => {}/{} B",
                pid, disk_usage.written_bytes, disk_usage.total_written_bytes,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DiskIoStats;

    #[test]
    fn test_diskio_stats_print() {
        DiskIoStats::print();
    }
}
