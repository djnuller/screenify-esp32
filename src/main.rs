use std::{thread::sleep, time::Duration};

use anyhow;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;
use st7735_lcd;

mod http_server_handler;
mod wifi_handler;
mod gpio_utils;
mod spotify_handler;
mod display;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();

    // let mut onboard_led = PinDriver::output(peripherals.pins.gpio2)?;

    // It is important wifi is up before we start the http server.
    let mut wifi = wifi_handler::wifi_init(peripherals.modem,
                                        "JDSJ", "@Frank123!")?;
    let _httpserver = http_server_handler::httpserver_init()?;

    let mut display = display::display_init(
        peripherals.spi2,
        peripherals.pins.gpio18,
        peripherals.pins.gpio23,
        peripherals.pins.gpio19,
        peripherals.pins.gpio2,
        peripherals.pins.gpio5,
    ).unwrap();

    display = display::draw_welcome_screen(wifi_handler::get_ip_address(&mut wifi), display).unwrap();


    // Loop to Avoid Program Termination
    loop {
        // gpio_utils::blinky(&mut onboard_led)?;
        sleep(Duration::from_millis(1000));
    }
}








