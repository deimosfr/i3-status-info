use clap::{Args, Parser, Subcommand, ValueEnum};
use cmds::{cpu::CpuArgs, mem::MemArgs, perfmode::PerfModeArgs};

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
#[derive(Args)]
struct DiskIoArgs {
    #[arg(short, long, default_value_t=80, value_parser = clap::value_parser!(u8).range(2..100))]
    critical: u8,
    #[arg(short, long, default_value_t=60, value_parser = clap::value_parser!(u8).range(1..100))]
    warning: u8,
    #[arg(short, long, default_value = "gb")]
    unit: BytesUnit,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Cpu(x) => cmds::cpu::CpuStats::print(x.warning, x.critical, x.display),
        Commands::Mem(x) => cmds::mem::MemStats::print(x.warning, x.critical, x.unit, x.display),
        Commands::DiskIo(_) => cmds::disk_io::DiskIoStats::print(),
        Commands::PerfMode(x) => cmds::perfmode::PerformanceMode::print(x.display),
    }
}
