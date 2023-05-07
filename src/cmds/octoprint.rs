use std::fmt::Display;

use clap::Args;
use compound_duration::format_dhms;
use reqwest::{blocking::Response, StatusCode};

use crate::{CommandStatus, I3Display, I3DisplayError};
use reqwest::header::{self, HeaderValue};
use serde::Deserialize;

#[derive(Args)]
pub struct OctoprintArgs {
    #[arg(short, long)]
    pub apikey: String,
    #[arg(short, long)]
    pub url: String,
    #[arg(short = 'r', long, default_value_t = false)]
    pub hide_remaining_time: bool,
}

// Octoprint API
#[derive(Debug, Deserialize)]
pub struct OctoprintApiJobResponse {
    pub progress: ApiProgress,
    pub state: OctoprintJobState,
}

#[derive(Debug, Deserialize)]
pub struct ApiProgress {
    pub completion: Option<f64>,
    #[serde(rename(deserialize = "printTimeLeft"))]
    pub print_time_left: Option<i64>,
}

#[derive(Debug)]
pub struct OctoprintStatus {
    status: OctoprintJobState,
    remaining_time: i64,
    completion: f64,
}

#[derive(Debug)]
pub enum OctoprintStatusError {
    ConnectionRefused,
    ConnectionTimeout,
    InvalidApiKey(String),
    InvalidResponse(String),
    InvalidConnection(String),
    DeserializationError(String),
}

impl Display for OctoprintStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OctoprintStatusError::ConnectionRefused => write!(f, "Connection refused"),
            OctoprintStatusError::InvalidApiKey(x) => write!(f, "Invalid api key: {}", x),
            OctoprintStatusError::InvalidResponse(x) => write!(f, "Invalid response: {}", x),
            OctoprintStatusError::InvalidConnection(x) => write!(f, "Invalid connection: {}", x),
            OctoprintStatusError::DeserializationError(x) => {
                write!(f, "Deserialization error: {}", x)
            }
            OctoprintStatusError::ConnectionTimeout => write!(f, "Connection timeout"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum OctoprintJobState {
    Operational,
    Printing,
    Pausing,
    Paused,
    Cancelling,
    Error,
    Offline,
    Unssupported,
}

impl Display for OctoprintJobState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OctoprintJobState::Operational => write!(f, "󰒲"),
            OctoprintJobState::Printing => write!(f, "Printing"),
            OctoprintJobState::Pausing => write!(f, ""),
            OctoprintJobState::Paused => write!(f, ""),
            OctoprintJobState::Cancelling => write!(f, ""),
            OctoprintJobState::Error => write!(f, ""),
            OctoprintJobState::Offline => write!(f, "Offline"),
            _ => write!(f, "Unssupported"),
        }
    }
}

impl CommandStatus<OctoprintArgs> for OctoprintStatus {
    fn get(command: &OctoprintArgs) -> Result<Option<I3Display>, I3DisplayError> {
        let res = match Self::get_job_status(command.url.clone(), command.apikey.clone()) {
            Ok(x) => x,
            Err(e) => match e {
                OctoprintStatusError::ConnectionRefused
                | OctoprintStatusError::ConnectionTimeout => return Ok(None),
                _ => return Err(I3DisplayError::from(e.to_string())),
            },
        };

        let octoprint_status = Self::to_octoprint_status(res)
            .map_err(|e| I3DisplayError::from(format!("Error: {}", e.to_string())))?;

        let line = match octoprint_status.status {
            OctoprintJobState::Printing => {
                let mut x = format!("{:.1}%", octoprint_status.completion);
                if !command.hide_remaining_time {
                    x = format!(
                        "{} {}",
                        x,
                        format_dhms(octoprint_status.remaining_time as usize)
                    );
                }
                x
            }
            _ => octoprint_status.status.to_string(),
        };
        Ok(Some(I3Display::new(None, line.clone(), line, None)))
    }
}

impl OctoprintStatus {
    fn get_job_status(url: String, apikey: String) -> Result<Response, OctoprintStatusError> {
        // url
        let request_url = format!("{url}/api/job");
        // headers
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-Api-Key",
            HeaderValue::from_str(apikey.as_str())
                .map_err(|e| OctoprintStatusError::InvalidApiKey(e.to_string()))?,
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(1))
            .build()
            .map_err(|e| OctoprintStatusError::InvalidConnection(e.to_string()))?;

        client.get(request_url).send().map_err(|e| {
            if e.is_timeout() {
                OctoprintStatusError::ConnectionTimeout
            } else if e.is_connect() {
                OctoprintStatusError::ConnectionRefused
            } else {
                OctoprintStatusError::InvalidResponse(e.to_string())
            }
        })
    }

    fn to_octoprint_status(res: Response) -> Result<OctoprintStatus, OctoprintStatusError> {
        let content = match res.status() {
            StatusCode::OK => {
                let x: OctoprintApiJobResponse = res
                    .json()
                    .map_err(|e| OctoprintStatusError::DeserializationError(e.to_string()))?;
                x
            }
            StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => {
                return Err(OctoprintStatusError::ConnectionTimeout)
            }
            StatusCode::FORBIDDEN => {
                return Err(OctoprintStatusError::InvalidApiKey(
                    "Connection forbidden: invalid api key?".to_string(),
                ))
            }
            _ => {
                return Err(OctoprintStatusError::InvalidResponse(format!(
                    "Error: {}",
                    res.status()
                )))
            }
        };
        Ok(OctoprintStatus {
            status: content.state,
            completion: content.progress.completion.unwrap_or(0.0),
            remaining_time: content.progress.print_time_left.unwrap_or(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{OctoprintApiJobResponse, OctoprintStatus};
    const OCTOPRINT_URL: &str = "";
    const OCTOPRINT_API_KEY: &str = "";

    #[test]
    fn test_octoprint_api_connectivity() {
        let x = OctoprintStatus::get_job_status(
            OCTOPRINT_URL.to_string(),
            OCTOPRINT_API_KEY.to_string(),
        );
        assert!(x.is_ok());
        let res = OctoprintStatus::to_octoprint_status(x.unwrap());
        assert!(res.is_ok());
        println!("{:?}", res.unwrap());
    }

    #[test]
    fn test_octoprint_response_struct() {
        let api_result = r#"
        {
    #[test]
    fn test_octoprint() {
        let in_progress = r#"
        {
  "job": {
    "averagePrintTime": null,
    "estimatedPrintTime": 1960.060377407137,
    "filament": {
      "tool0": {
        "length": 1416.7206100000044,
        "volume": 0
      }
    },
    "file": {
      "date": 1683387742,
      "display": "x.gcode",
      "name": "x.gcode",
      "origin": "local",
      "path": "x.gcode",
      "size": 350914
    },
    "lastPrintTime": null,
    "user": "deimos"
  },
  "progress": {
    "completion": 0.06012869250015673,
    "filepos": 211,
    "printTime": 0,
    "printTimeLeft": 1959,
    "printTimeLeftOrigin": "analysis"
  },
  "state": "Printing"
}
        "#;

        let x = serde_json::from_str::<OctoprintApiJobResponse>(api_result);
        assert!(x.is_ok());
    }
}
