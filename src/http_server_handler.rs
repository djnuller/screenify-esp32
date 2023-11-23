use embedded_svc::http::Method;
use esp_idf_hal::io::EspIOError;
use esp_idf_sys::EspError;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use crate::spotify_handler;
use crate::spotify_handler::SpotifyAuthToken;

pub fn httpserver_init() -> Result<EspHttpServer<'static>, EspIOError> {
    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default())?;
    http_handler(&mut httpserver)?;
    Ok(httpserver)
}

fn http_handler(httpserver: &mut EspHttpServer) -> Result<(), EspError> {
// Define Server Request Handler Behaviour on Get for Root URL
    serve_index(httpserver)?;
    test_spotify(httpserver)?;
    test_callback(httpserver)?;
    Ok(())
}

fn test_callback(httpserver: &mut EspHttpServer) ->  Result<(), EspError> {
    httpserver.fn_handler("/callback", Method::Get, |request| {
        // Retrieve html String
        let html = index_html();

        // Respond with OK status
        let mut response = request.into_ok_response()?;
        // Return Requested Object (Index Page)
        response.write(html.as_bytes())?;
        Ok(())
    })?;
    Ok(())
}

fn test_spotify(httpserver: &mut EspHttpServer) ->  Result<(), EspError> {
    httpserver.fn_handler("/test", Method::Get, |request| {
        // Retrieve html String
        let token = spotify_handler::get_auth_token()?;
        let html = test_html(token);

        // Respond with OK status
        let mut response = request.into_ok_response()?;
        // Return Requested Object (Index Page)
        response.write(html.as_bytes())?;
        Ok(())
    })?;
    Ok(())
}

fn serve_index(httpserver: &mut EspHttpServer) ->  Result<(), EspError> {
    httpserver.fn_handler("/", Method::Get, |request| {
        // Retrieve html String
        let html = index_html();
        // Respond with OK status
        let mut response = request.into_ok_response()?;
        // Return Requested Object (Index Page)
        response.write(html.as_bytes())?;
        Ok(())
    })?;
    Ok(())
}


fn test_html(token: SpotifyAuthToken) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>ESP Spotify Example</title>
    </head>
    <body>
        <h1>Hello World from ESP!</h1>
        <p>Spotify Access Token: {}</p>
    </body>
</html>
"#,
        token.access_token
    )
}

fn index_html() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
    Hello World from ESP!
    </body>
</html>
"#
    )
}