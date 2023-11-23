use std::cell::RefCell;
use std::num::NonZeroI32;

use base64::encode;
use embedded_svc::http::client::Client;
use embedded_svc::io::Write;
use embedded_svc::utils::io;
use esp_idf_hal::io::EspIOError;
use esp_idf_svc::http::client::{Configuration as HttpConfig, EspHttpConnection};
use esp_idf_sys::EspError;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

static CLIENT_ID: &str = "1a9b6e0fefbd401fad250953f4a238ba";
static CLIENT_SECRET: &str = "e66e8a55d2564ef6841c2c5ae3207c89";

static TOKEN_BASE_URL: &str = "https://accounts.spotify.com/api/token/";
static API_BASE_URL: &str = "https://api.spotify.com/";

#[derive(Deserialize, Serialize)]
pub struct SpotifyAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}


pub fn get_auth_token() -> Result<SpotifyAuthToken, EspIOError> {
    let httpconnection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    // Create HTTPS Client
    let mut httpclient = Client::wrap(httpconnection);

    let auth_header_value = format!(
        "Basic {}",
        encode(format!("{}:{}", CLIENT_ID, CLIENT_SECRET))
    );

    let body = "grant_type=client_credentials";
    let content_length_header = body.len().to_string();

    let headers = [
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("Authorization", &auth_header_value),
        ("Content-Length", &content_length_header),
    ];

    // Prepare and send the POST request
    let mut request = httpclient.post(TOKEN_BASE_URL, &headers)?;
    request.write_all(body.as_bytes())?;
    request.flush()?;

    let mut response = request.submit()?;

    // Process response
    let status = response.status();

    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    let body = String::from_utf8(buf[0..bytes_read].to_vec()).unwrap();

    // Deserialize the response body into SpotifyAuthToken
    let auth_token: SpotifyAuthToken = from_str(&body)
        .map_err(|_| EspIOError::from(
            EspError::from_non_zero(NonZeroI32::try_from(
                esp_idf_sys::ESP_FAIL).unwrap())
        ))?;


    // Drain the remaining response bytes
    while response.read(&mut buf)? > 0 {}
    Ok(auth_token)
}
