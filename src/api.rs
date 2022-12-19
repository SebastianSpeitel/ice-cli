use reqwest::blocking::Client;
use thiserror::Error;
mod common;
pub use common::*;
mod status;
use status::*;
mod trip;
use trip::*;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error")]
    Request(#[from] reqwest::Error),
    #[error("Parse error")]
    Parse(#[from] serde_json::Error),
    #[error("Invalid content type")]
    InvalidContentType,
}

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("starship-ice")
            .timeout(std::time::Duration::from_millis(250))
            .build()
            .unwrap();

        Self { client }
    }

    pub fn available(&self) -> bool {
        let res = self
            .client
            .head("https://iceportal.de/api1/rs/status")
            .send();

        let res = match res {
            Ok(res) => res,
            Err(e) => {
                log::info!("Not available, because request failed: {}", e);
                return false;
            }
        };

        if let Err(_) = Self::verify_content_type(&res) {
            log::info!("Not available, because content type is invalid. You are probably not in an ICE right now");
            return false;
        }
        
        true
    }

    pub fn status(&self) -> Result<Status, ApiError> {
        let res = self
            .client
            .get("https://iceportal.de/api1/rs/status")
            .send()?;

        log::debug!("Response: {:#?}", res);

        Self::verify_content_type(&res)?;

        let json = res.bytes()?;

        let status: Status = serde_json::from_slice(&json)?;

        log::debug!("Status: {:#?}", status);

        Ok(status)
    }

    pub fn trip(&self) -> Result<TripInfo, ApiError> {
        let res = self
            .client
            .get("https://iceportal.de/api1/rs/tripInfo/trip")
            .send()?;

        log::debug!("Response: {:#?}", res);

        Self::verify_content_type(&res)?;

        let json = res.bytes()?;

        let trip: TripInfo = serde_json::from_slice(&json)?;

        log::debug!("Trip: {:#?}", trip);

        Ok(trip)
    }

    fn verify_content_type(res: &reqwest::blocking::Response) -> Result<(), ApiError> {
        for (h, v) in res.headers() {
            if h != &"content-type" {
                continue;
            }
            if let Ok(v) = v.to_str() {
                if v.starts_with("application/json") {
                    break;
                }
            }
            return Err(ApiError::InvalidContentType);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let api = Api::new();
        let status = api.status().unwrap();
        dbg!(status);
    }
}
