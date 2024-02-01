use std::fmt::Display;

use clap::Args;
use compound_duration::format_dhms;
use diqwest::blocking::WithDigestAuth;
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{CommandStatus, I3Display, I3DisplayError, I3StatusRustColorState};

#[derive(Args)]
pub struct PrusaLinkArgs {
    #[arg(short, long)]
    pub url: String,
    #[arg(short, long)]
    pub login: Option<String>,
    #[arg(short, long)]
    pub password: Option<String>,
    #[arg(short, long)]
    pub token: Option<String>,
    #[arg(short = 'r', long, default_value_t = false)]
    pub hide_remaining_time: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PrusaLinkJobResponse {
    pub job: Option<PrusaLinkJob>,
    pub printer: PrusaPrinter,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PrusaLinkJob {
    pub progress: f64,
    pub time_printing: i64,
    pub time_remaining: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PrusaPrinter {
    pub state: PrusaPrintState,
}

// https://github.com/prusa3d/Prusa-Link/blob/583f3b613170ed05ede1673a952b5fb577c1cdcb/prusa/link/const.py#L43
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PrusaPrintState {
    Printing,
    Paused,
    Finished,
    Stopped,
    Idle,
    Busy,
    Ready,
    Attention,
}

impl Display for PrusaPrintState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrusaPrintState::Printing => write!(f, "󰹛"),
            PrusaPrintState::Paused => write!(f, "󰐫 "),
            PrusaPrintState::Finished => write!(f, "󰐫 "),
            PrusaPrintState::Stopped => write!(f, "󰐫 "),
            PrusaPrintState::Idle => write!(f, "󰐫 󰒲"),
            PrusaPrintState::Busy => write!(f, "󱢹"),
            PrusaPrintState::Ready => write!(f, "󰐫 󰒲"),
            PrusaPrintState::Attention => write!(f, "󱇁"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PrusaLinkStatus {
    status: PrusaPrintState,
    remaining_time: i64,
    completion: f64,
}

pub enum PrusaLinkError {
    InvalidCredentials,
    InvalidConnection(String),
    ConnectionTimeout,
    ConnectionRefused,
    InvalidResponse(String),
    DeserializationError(String),
}

impl Display for PrusaLinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrusaLinkError::InvalidCredentials => {
                write!(f, "No token or login/password provided")
            }
            PrusaLinkError::InvalidConnection(x) => write!(f, "Invalid connection: {x}"),
            PrusaLinkError::ConnectionTimeout => write!(f, "Connection timeout"),
            PrusaLinkError::ConnectionRefused => write!(f, "Connection refused"),
            PrusaLinkError::InvalidResponse(x) => write!(f, "Invalid response: {}", x),
            PrusaLinkError::DeserializationError(x) => write!(f, "Deserialization error: {}", x),
        }
    }
}

impl CommandStatus<PrusaLinkArgs> for PrusaLinkStatus {
    fn get(command: &PrusaLinkArgs) -> Result<Option<I3Display>, I3DisplayError> {
        // basic check
        if command.token.is_none() && (command.login.is_none() || command.password.is_none()) {
            return Err(I3DisplayError {
                message: PrusaLinkError::InvalidCredentials.to_string(),
            });
        }
        let res = match Self::get_job_status(
            &command.url,
            &command.token,
            &command.login,
            &command.password,
        ) {
            Ok(x) => x,
            Err(e) => match e {
                PrusaLinkError::ConnectionRefused | PrusaLinkError::ConnectionTimeout => {
                    return Ok(None)
                }
                _ => return Err(I3DisplayError::from(e.to_string())),
            },
        };
        let prusa_link_status = Self::to_prusa_link_status(res)
            .map_err(|e| I3DisplayError::from(format!("Error: {}", e)))?;

        let (line, color) = match prusa_link_status.status {
            PrusaPrintState::Printing => {
                let mut x = format!(
                    "{} {:.1}%",
                    prusa_link_status.status, prusa_link_status.completion
                );
                if !command.hide_remaining_time {
                    x = format!(
                        "{} {}",
                        x,
                        format_dhms(prusa_link_status.remaining_time as usize)
                    );
                }
                (x, None)
            }
            PrusaPrintState::Attention => (
                prusa_link_status.status.to_string(),
                Some(I3StatusRustColorState::I3StatusRustStateWarning),
            ),
            PrusaPrintState::Finished => (
                prusa_link_status.status.to_string(),
                Some(I3StatusRustColorState::I3StatusRustStateGood),
            ),
            _ => (prusa_link_status.status.to_string(), None),
        };

        Ok(Some(I3Display::new(None, line.clone(), line, color)))
    }
}

impl PrusaLinkStatus {
    fn get_job_status(
        url: &String,
        token: &Option<String>,
        login: &Option<String>,
        password: &Option<String>,
    ) -> Result<Response, PrusaLinkError> {
        // url
        let request_url = format!("{url}/api/v1/status");

        let client = reqwest::blocking::Client::builder()
            // .default_headers(headers)
            .timeout(std::time::Duration::from_secs(1))
            .build()
            .map_err(|e| PrusaLinkError::InvalidConnection(e.to_string()))?;

        match token {
            Some(x) => client
                .get(request_url)
                .header("X-Api-Key", x)
                .send()
                .map_err(|e| {
                    if e.is_timeout() {
                        PrusaLinkError::ConnectionTimeout
                    } else if e.is_connect() {
                        PrusaLinkError::ConnectionRefused
                    } else {
                        PrusaLinkError::InvalidResponse(e.to_string())
                    }
                }),
            None => client
                .get(request_url)
                .send_with_digest_auth(
                    login.clone().unwrap().as_str(),
                    password.clone().unwrap().as_str(),
                )
                .map_err(|e| PrusaLinkError::InvalidResponse(e.to_string())),
        }
    }

    fn to_prusa_link_status(res: Response) -> Result<PrusaLinkStatus, PrusaLinkError> {
        let content = match res.status() {
            StatusCode::OK => {
                let x: PrusaLinkJobResponse = res
                    .json()
                    .map_err(|e| PrusaLinkError::DeserializationError(e.to_string()))?;
                x
            }
            StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => {
                return Err(PrusaLinkError::ConnectionTimeout)
            }
            StatusCode::FORBIDDEN => return Err(PrusaLinkError::InvalidCredentials),
            _ => {
                return Err(PrusaLinkError::InvalidResponse(format!(
                    "Error: {}",
                    res.status()
                )))
            }
        };

        Ok(PrusaLinkStatus {
            status: content.printer.state.clone(),
            remaining_time: match content.job.clone() {
                Some(x) => x.time_remaining,
                None => 0,
            },
            completion: match content.printer.state {
                PrusaPrintState::Printing
                | PrusaPrintState::Paused
                | PrusaPrintState::Stopped
                | PrusaPrintState::Attention => match content.job {
                    Some(x) => x.progress,
                    None => return Err(PrusaLinkError::InvalidResponse(format!("{:?}", content))),
                },
                PrusaPrintState::Idle | PrusaPrintState::Busy | PrusaPrintState::Ready => 0.0,
                PrusaPrintState::Finished => 100.0,
            },
        })
    }
}

pub mod tests {
    #[test]
    fn test_prusa_link() {
        // api/v1/status
        let api_result_printing = r#"
        {
            "job": {
                "id": 12,
                "progress": 62.00,
                "time_printing": 59544,
                "time_remaining": 34320
            },
            "printer": {
                "axis_z": 121.8,
                "fan_hotend": 8111,
                "fan_print": 6105,
                "flow": 100,
                "speed": 100,
                "state": "PRINTING",
                "target_bed": 60,
                "target_nozzle": 220,
                "temp_bed": 60,
                "temp_nozzle": 219.2
            },
            "storage": {
                "name": "usb",
                "path": "/usb/",
                "read_only": false
            }
        }
        "#;
        let api_result_finished = r#"
        {
            "storage": {
                "path": "/usb/",
                "name": "usb",
                "read_only": false
            },
            "printer": {
                "state": "FINISHED",
                "temp_bed": 52.3,
                "target_bed": 0.0,
                "temp_nozzle": 98.3,
                "target_nozzle": 0.0,
                "axis_z": 62.4,
                "axis_x": 241.0,
                "axis_y": 170.0,
                "flow": 100,
                "speed": 100,
                "fan_hotend": 6793,
                "fan_print": 0
            }
        }
        "#;

        let api_result_stopped = r#"
        { "printer": { "state": "StOPPED" } }}
        "#;

        let printing = serde_json::from_str::<crate::cmds::prusa_link::PrusaLinkJobResponse>(
            api_result_printing,
        );
        assert!(printing.is_ok());

        let finished = serde_json::from_str::<crate::cmds::prusa_link::PrusaLinkJobResponse>(
            api_result_finished,
        );
        assert!(finished.is_ok());

        let stopped = serde_json::from_str::<crate::cmds::prusa_link::PrusaLinkJobResponse>(
            api_result_stopped,
        );
        assert!(stopped.is_ok());
    }
}
