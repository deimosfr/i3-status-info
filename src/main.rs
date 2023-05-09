use std::fmt::{self, Display};

use clap::{Parser, Subcommand, ValueEnum};
use cmds::{
    cpu::{CpuArgs, CpuStats},
    disk_io::{DiskIoArgs, DiskIoStats},
    disk_uage::{DiskStats, DiskUsageArgs},
    icmp_check::{IcmpCheck, IcmpCheckArgs},
    mem::{MemArgs, MemStats},
    octoprint::{OctoprintArgs, OctoprintStatus},
    perfmode::{PerfModeArgs, PerformanceMode},
    tcp_check::{TcpCheck, TcpCheckArgs},
};
mod cmds;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[arg(short, long, value_enum, default_value = "i3-status-rust")]
    output: OutputType,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputType {
    I3Blocks,
    I3StatusRust,
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
    #[command(about = "Check hostname/ip with port availability")]
    DiskUsage(DiskUsageArgs),
    #[command(about = "Check disk usage")]
    TcpCheck(TcpCheckArgs),
    #[command(about = "Check hostname/ip availability")]
    IcmpCheck(IcmpCheckArgs),
    #[command(about = "Check octoprint job status")]
    Octoprint(OctoprintArgs),
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

pub struct I3Display {
    pub icon: Option<String>,
    pub long_line: String,
    pub short_line: String,
    pub color: Option<I3StatusRustColorState>,
}

#[derive(Clone)]
pub enum I3StatusRustColorState {
    I3StatusRustStateIdle,
    I3StatusRustStateInfo,
    I3StatusRustStateGood,
    I3StatusRustStateWarning,
    I3StatusRustStateCritical,
    HtmlColorCode(String),
}

impl Display for I3StatusRustColorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            I3StatusRustColorState::I3StatusRustStateIdle => write!(f, "Idle"),
            I3StatusRustColorState::I3StatusRustStateInfo => write!(f, "Info"),
            I3StatusRustColorState::I3StatusRustStateGood => write!(f, "Good"),
            I3StatusRustColorState::I3StatusRustStateWarning => write!(f, "Warning"),
            I3StatusRustColorState::I3StatusRustStateCritical => write!(f, "Critical"),
            I3StatusRustColorState::HtmlColorCode(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug)]
pub struct I3DisplayError {
    pub message: String,
}

impl From<String> for I3DisplayError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl I3Display {
    pub fn new(
        icon: Option<String>,
        long_line: String,
        short_line: String,
        color: Option<I3StatusRustColorState>,
    ) -> Self {
        Self {
            icon,
            long_line,
            short_line,
            color,
        }
    }

    fn print(&self, output: &OutputType) {
        match output {
            OutputType::I3Blocks => {
                let icon = match &self.icon {
                    Some(x) => format!("{} ", x),
                    None => "".to_string(),
                };
                println!("{} {}\n{}", icon, self.long_line, self.short_line);
                if let Some(color) = &self.color {
                    println!("{color}");
                }
            }
            OutputType::I3StatusRust => {
                // note: I do not use serde to avoid useless resources usage and reduce binary size
                let jsonify = |key, val| format!("\"{}\":\"{}\"", key, val);
                let mut output_content = Vec::with_capacity(4);

                output_content.push(jsonify("text", &self.long_line));
                output_content.push(jsonify("short_text", &self.short_line));
                if let Some(icon) = &self.icon {
                    output_content.push(jsonify("icon", icon));
                }
                if let Some(color) = &self.color {
                    output_content.push(jsonify("state", &color.to_string()));
                }

                println!("{{{}}}", output_content.join(","))
            }
        }
    }
}

trait CommandStatus<T> {
    fn get(command: &T) -> Result<Option<I3Display>, I3DisplayError>;
}

fn main() {
    let cli = Cli::parse();

    let res = match &cli.command {
        Commands::Cpu(x) => CpuStats::get(x),
        Commands::Mem(x) => MemStats::get(x),
        Commands::DiskIo(x) => DiskIoStats::get(x),
        Commands::PerfMode(x) => PerformanceMode::get(x),
        Commands::TcpCheck(x) => TcpCheck::get(x),
        Commands::IcmpCheck(x) => IcmpCheck::get(x),
        Commands::DiskUsage(x) => DiskStats::get(x),
        Commands::Octoprint(x) => OctoprintStatus::get(x),
    };

    match res {
        Ok(x) => match x {
            Some(data) => data.print(&cli.output),
            None => match cli.output {
                OutputType::I3Blocks => {}
                OutputType::I3StatusRust => println!("{{}}"),
            },
        },
        Err(e) => match cli.output {
            OutputType::I3Blocks => eprintln!("{}", e.message),
            OutputType::I3StatusRust => eprintln!("{{{}}}", e.message),
        },
    }
}
