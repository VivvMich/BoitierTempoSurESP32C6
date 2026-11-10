use anyhow::Result;
use embedded_svc::http::{client::Client, Method};
use esp_idf_svc::http::client::{Configuration as HttpConfig, EspHttpConnection};
use esp_idf_svc::io::Read;

pub fn get(url: &str) -> Result<String> {
    let connection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = Client::wrap(connection);

    let request = client.request(Method::Get, url, &[("accept", "application/json")])?;
    let mut response = request.submit()?;
    log::info!("Statut HTTP :: {:?}", response.status());

    let mut body = Vec::new();
    let mut buf = [0u8; 512];
    loop {
        let n = response.read(&mut buf)?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&buf[..n]);
    }
    Ok(String::from_utf8_lossy(&body).to_string())
}