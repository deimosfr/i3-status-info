use std::{net::IpAddr, time::Duration};

use clap::Args;

use crate::{CommandStatus, I3Display, I3DisplayError};

#[derive(Args)]
pub struct IcmpCheckArgs {
    #[arg(short, long)]
    pub ip: IpAddr,
    #[arg(short, long, default_value = "up")]
    pub availability_text: Option<String>,
    #[arg(short, long)]
    pub unavailability_text: Option<String>,
    #[arg(short, long, default_value = "100")]
    pub timeout_ms: u64,
}

pub struct IcmpCheck {
    available: bool,
}

impl CommandStatus<IcmpCheckArgs> for IcmpCheck {
    fn get(command: &IcmpCheckArgs) -> Result<Option<I3Display>, I3DisplayError> {
        let icmp_check = Self::check(command.ip, command.timeout_ms)?;
        match icmp_check.available {
            true => {
                if command.availability_text.is_some() {
                    let x = command.availability_text.clone().unwrap();
                    return Ok(Some(I3Display::new(None, x.clone(), x, None)));
                }
            }
            false => {
                if command.unavailability_text.is_some() {
                    let x = command.unavailability_text.clone().unwrap();
                    return Ok(Some(I3Display::new(None, x.clone(), x, None)));
                }
            }
        }
        Ok(None)
    }
}

impl IcmpCheck {
    fn check(host: IpAddr, timeout_ms: u64) -> Result<Self, I3DisplayError> {
        let mut icmp_check = IcmpCheck { available: false };
        match ping::ping(
            host,
            Some(Duration::from_millis(timeout_ms)),
            None,
            None,
            None,
            None,
        ) {
            Ok(_) => icmp_check.available = true,
            Err(e) => {
                if !e.to_string().contains("Resource temporarily unavailable") {
                    return Err(I3DisplayError::from(e.to_string()));
                }
            }
        }
        Ok(icmp_check)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_icmp_check_connectivity() {}
}
