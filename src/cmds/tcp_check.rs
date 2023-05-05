use std::net::TcpStream;

use clap::Args;

use crate::{CommandStatus, I3Display, I3DisplayError};

#[derive(Args)]
pub struct TcpCheckArgs {
    #[arg(short = 'o', long)]
    pub host: String,
    #[arg(short, long)]
    pub port: u16,
    #[arg(short, long, default_value = "up")]
    pub availability_text: Option<String>,
    #[arg(short, long)]
    pub unavailability_text: Option<String>,
}

pub struct TcpCheck {
    available: bool,
}

impl CommandStatus<TcpCheckArgs> for TcpCheck {
    fn get(command: &TcpCheckArgs) -> Result<Option<I3Display>, I3DisplayError> {
        let tcp_check = TcpCheck::check(command.host.clone(), command.port);
        match tcp_check.available {
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

impl TcpCheck {
    fn check(host: String, port: u16) -> Self {
        let mut tcp_check = TcpCheck { available: false };
        let addr = format!("{}:{}", host, port);
        if TcpStream::connect(addr.as_str()).is_ok() {
            tcp_check.available = true;
        };
        tcp_check
    }
}

#[cfg(test)]
mod tests {
    use super::TcpCheck;

    #[test]
    fn test_tcp_check_connectivity() {
        let x = TcpCheck::check("google.com".to_string(), 80);
        assert_eq!(x.available, true);
    }
}
