use clap::{Args, ValueEnum};
use std::{fmt, fs};

use crate::{I3Blocks, I3BlocksDisplay, I3BlocksError};

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

impl I3Blocks<PerfModeArgs> for PerformanceMode {
    fn get(command: &PerfModeArgs) -> Result<Option<I3BlocksDisplay>, I3BlocksError> {
        let lines = Self::i3blocks_print(command.display)?;
        Ok(Some(I3BlocksDisplay::new(lines.clone(), lines, None)))
    }
}

impl PerformanceMode {
    pub fn i3blocks_print(style: PerfModeStyle) -> Result<String, I3BlocksError> {
        let mode = Self::get_mode()?;
        Ok(match style {
            PerfModeStyle::Icons => match mode {
                PerformanceMode::Balanced => "",
                PerformanceMode::Performance => "異",
                PerformanceMode::LowPower => "",
            }
            .to_string(),
            PerfModeStyle::Text => mode.to_string(),
        })
    }

    fn get_mode() -> Result<PerformanceMode, I3BlocksError> {
        let content = fs::read_to_string(PERF_PROFILE)
            .map_err(|e| I3BlocksError::from(format!("can't read file {PERF_PROFILE}: {e}")))?;

        match content.as_str().trim() {
            "balanced" => Ok(PerformanceMode::Balanced),
            "performance" => Ok(PerformanceMode::Performance),
            "low-power" => Ok(PerformanceMode::LowPower),
            _ => Err(I3BlocksError::from(format!(
                "unknown performance mode: `{content}`"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_perfmode_print() {}
}
