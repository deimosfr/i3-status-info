use clap::{Args, ValueEnum};
use std::{fmt, fs};

const PERF_PROFILE: &str = "/sys/firmware/acpi/platform_profile";

#[derive(Args)]
pub struct PerfModeArgs {
    #[arg(short, long, default_value = "icons")]
    pub display: PerfModeStyle,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum PerfModeStyle {
    Icons,
    Text,
}

#[derive(Debug)]
pub enum PerformanceMode {
    // Fr+,/m
    Balanced,
    // Fn+h
    Performance,
    // Fn+l
    LowPower,
}

impl fmt::Display for PerformanceMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PerformanceMode::Balanced => write!(f, "Balanced"),
            PerformanceMode::Performance => write!(f, "Performance"),
            PerformanceMode::LowPower => write!(f, "Low Power"),
        }
    }
}

impl PerformanceMode {
    pub fn print(style: PerfModeStyle) {
        match style {
            PerfModeStyle::Icons => {
                let icon = match Self::get_mode() {
                    PerformanceMode::Balanced => "",
                    PerformanceMode::Performance => "異",
                    PerformanceMode::LowPower => "",
                };
                println!("{icon}\n{icon}")
            }
            PerfModeStyle::Text => {
                let mode = Self::get_mode();
                println!("{mode}\n{mode}")
            }
        }
    }

    fn get_mode() -> PerformanceMode {
        let content = fs::read_to_string(PERF_PROFILE)
            .expect(format!("Failed to read {}", PERF_PROFILE).as_str());
        match content.as_str().trim() {
            "balanced" => PerformanceMode::Balanced,
            "performance" => PerformanceMode::Performance,
            "low-power" => PerformanceMode::LowPower,
            _ => panic!("Unknown performance mode: `{content}`"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PerfModeStyle, PerformanceMode};

    #[test]
    fn test_perfmode_print() {
        PerformanceMode::print(PerfModeStyle::Icons);
        PerformanceMode::print(PerfModeStyle::Text);
    }
}
