use std::fmt::{self};

use clap::{Parser, Subcommand, ValueEnum};
use cmds::{
    cpu::{CpuArgs, CpuStats},
    disk_io::{DiskIoArgs, DiskIoStats},
    mem::{MemArgs, MemStats},
    perfmode::{PerfModeArgs, PerformanceMode},
};

mod cmds;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Get CPU info")]
    Cpu(CpuArgs),
    #[command(about = "Get Memory info")]
    Mem(MemArgs),
    #[command(about = "Show Performance mode")]
    PerfMode(PerfModeArgs),
    #[command(about = "Get Disk IO info")]
    DiskIo(DiskIoArgs),
}

#[derive(Clone, Copy, ValueEnum)]
pub enum BytesUnit {
    Kb,
    Mb,
    Gb,
}

impl fmt::Display for BytesUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BytesUnit::Kb => write!(f, "KB"),
            BytesUnit::Mb => write!(f, "MB"),
            BytesUnit::Gb => write!(f, "GB"),
        }
    }
}

pub struct I3BlocksDisplay {
    pub long_line: String,
    pub short_line: String,
    pub color: Option<String>,
}

pub struct I3BlocksError {
    pub message: String,
}

impl From<String> for I3BlocksError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl I3BlocksDisplay {
    pub fn new(long_line: String, short_line: String, color: Option<String>) -> Self {
        Self {
            long_line,
            short_line,
            color,
        }
    }

    fn print(&self) {
        println!("{}\n{}", self.long_line, self.short_line);
        if let Some(color) = &self.color {
            println!("{}", color);
        }
    }
}

trait I3Blocks<T> {
    fn get(command: &T) -> Result<I3BlocksDisplay, I3BlocksError>;
}

fn main() {
    let cli = Cli::parse();

    let res = match &cli.command {
        Commands::Cpu(x) => CpuStats::get(x),
        Commands::Mem(x) => MemStats::get(x),
        Commands::DiskIo(x) => DiskIoStats::get(x),
        Commands::PerfMode(x) => PerformanceMode::get(x),
    };

    match res {
        Ok(x) => x.print(),
        Err(e) => eprintln!("{}", e.message),
    }
}
