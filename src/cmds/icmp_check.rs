use std::{net::IpAddr, time::Duration};

use clap::Args;

use crate::{I3Blocks, I3BlocksDisplay, I3BlocksError};

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

impl I3Blocks<IcmpCheckArgs> for IcmpCheck {
    fn get(command: &IcmpCheckArgs) -> Result<Option<I3BlocksDisplay>, I3BlocksError> {
        let icmp_check = IcmpCheck::check(command.ip, command.timeout_ms)?;
        match icmp_check.available {
            true => {
                if command.availability_text.is_some() {
                    let x = command.availability_text.clone().unwrap();
                    return Ok(Some(I3BlocksDisplay::new(x.clone(), x, None)));
                }
            }
            false => {
                if command.unavailability_text.is_some() {
                    let x = command.unavailability_text.clone().unwrap();
                    return Ok(Some(I3BlocksDisplay::new(x.clone(), x, None)));
                }
            }
        }
        Ok(None)
    }
}

impl IcmpCheck {
    fn check(host: IpAddr, timeout_ms: u64) -> Result<Self, I3BlocksError> {
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
                    return Err(I3BlocksError::from(e.to_string()));
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
