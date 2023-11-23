use anyhow;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_sys::EspError;

pub fn wifi_init(modem: Modem, ssid: &str, password: &str) -> Result<BlockingWifi<EspWifi<'static>>, EspError> {
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    wifi_configuration(&mut wifi, ssid, password)?;

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    while !wifi.is_connected().unwrap() {
        // Get and print connection configuration
        let config = wifi.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }

    Ok(wifi)
}

fn wifi_configuration(wifi: &mut BlockingWifi<EspWifi>, ssid: &str, password: &str) -> Result<(), EspError> {
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: password.into(),
        channel: None,
    }))?;
    Ok(())
}

pub fn get_ip_address(wifi: &mut BlockingWifi<EspWifi>) -> String {
    wifi.wifi_mut().sta_netif().get_ip_info().unwrap().ip.to_string()
}