use std::num::NonZeroI32;

use base64::{Engine as _, engine::general_purpose};
use embedded_svc::http::client::{Client, Response};
use embedded_svc::http::Method;
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
    let mut httpclient = get_http_client()?;

    let auth_header_value = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(format!("{}:{}", CLIENT_ID, CLIENT_SECRET))
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

    let (mut buf, body) = get_body(&mut response)?;

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

fn get_body(mut response: &mut Response<&mut EspHttpConnection>) -> Result<([u8; 1024], String), EspIOError> {
    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    let body = String::from_utf8(buf[0..bytes_read].to_vec()).unwrap();
    Ok((buf, body))
}

fn get_http_client() -> Result<Client<EspHttpConnection>, EspIOError> {
    let httpconnection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    // Create HTTPS Client
    let mut httpclient = Client::wrap(httpconnection);
    Ok(httpclient)
}

pub fn get_currently_playing_song(access_token: String) -> Result<CurrentlyPlayingTrack, EspIOError> {
    let mut http_client = get_http_client()?;

    let headers = [
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("Authorization", &access_token),
    ];

    let mut request = http_client.request(Method::Get, TOKEN_BASE_URL, &headers)?;

    let mut response = request.submit()?;

    let (mut buf, body) = get_body(&mut response)?;

    // Drain the remaining response bytes
    while response.read(&mut buf)? > 0 {}

    // Deserialize the response body into CurrentlyPlayingTrack
    let currently_playing_track: CurrentlyPlayingTrack = from_str(&body)
        .map_err(|_| EspIOError::from(
            EspError::from_non_zero(NonZeroI32::try_from(
                esp_idf_sys::ESP_FAIL).unwrap())
        ))?;

    Ok(currently_playing_track)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentlyPlayingTrack {
    pub context: Context,
    pub timestamp: i64,
    #[serde(rename = "progress_ms")]
    pub progress_ms: i64,
    #[serde(rename = "is_playing")]
    pub is_playing: bool,
    pub item: Item,
    #[serde(rename = "currently_playing_type")]
    pub currently_playing_type: String,
    pub actions: Actions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(rename = "type")]
    pub type_field: String,
    pub href: String,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub album: Album,
    pub artists: Vec<Artist2>,
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    #[serde(rename = "disc_number")]
    pub disc_number: i64,
    #[serde(rename = "duration_ms")]
    pub duration_ms: i64,
    pub explicit: bool,
    #[serde(rename = "external_ids")]
    pub external_ids: ExternalIds,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls5,
    pub href: String,
    pub id: String,
    pub name: String,
    pub popularity: i64,
    #[serde(rename = "preview_url")]
    pub preview_url: String,
    #[serde(rename = "track_number")]
    pub track_number: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    #[serde(rename = "is_local")]
    pub is_local: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "album_type")]
    pub album_type: String,
    #[serde(rename = "total_tracks")]
    pub total_tracks: i64,
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls2,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls2 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls3,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls3 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist2 {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls4,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls4 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub isrc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls5 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actions {
    pub disallows: Disallows,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disallows {
    pub resuming: bool,
}

