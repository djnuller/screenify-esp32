use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use anyhow;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_6X9};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::text::{Baseline, Text};
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{Gpio0, Gpio18, Gpio19, Gpio2, Gpio23, Gpio5, Output, PinDriver};
use esp_idf_hal::io::EspIOError;
use esp_idf_hal::spi;
use esp_idf_hal::spi::{SPI2, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::units::FromValueType;
use st7735_lcd;
use st7735_lcd::{Orientation, ST7735};

pub fn display_init(spi2: SPI2, sclk: Gpio18, sda: Gpio23,
                    cs19: Gpio19, reset: Gpio2, data_command: Gpio5)
                    -> Result<ST7735<SpiDeviceDriver<'static,
                        SpiDriver<'static>>,
                        PinDriver<'static, Gpio5, Output>,
                        PinDriver<'static, Gpio2, Output>>,
                        EspIOError> {
    let sdi = Option::<Gpio0>::None;
    let cs = Some(cs19);

    // SPI configuration
    let driver_config = Default::default();
    let spi_config = spi::SpiConfig::new().baudrate(30.MHz().into());

    // Initialize the SPI interface
    let spi = spi::SpiDeviceDriver::new_single(
        spi2,
        sclk,
        sda,
        sdi,
        cs,
        &driver_config,
        &spi_config,
    )?;

    // Initialize the control pins (RST and DC)
    let rst = PinDriver::output(reset)?;
    let dc = PinDriver::output(data_command)?;

    let rgb = true;
    let inverted = false;
    let width = 128;
    let height = 160;

    let mut delay = Ets {};

    let mut display = ST7735::new(spi, dc, rst, rgb, inverted, width, height);

    display.init(&mut delay).unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    display
        .set_orientation(&Orientation::PortraitSwapped)
        .unwrap();
    Ok(display)
}

pub fn draw_welcome_screen(ip_addr: String,
    mut display: ST7735<SpiDeviceDriver<'static,
        SpiDriver<'static>>,
        PinDriver<'static, Gpio5, Output>,
        PinDriver<'static, Gpio2, Output>>
) -> Result<ST7735<SpiDeviceDriver<'static, SpiDriver>, PinDriver<'static, Gpio5, Output>, PinDriver<'static, Gpio2, Output>>, ()> {

    sleep(Duration::from_millis(1000));

    let text_style = MonoTextStyle::new(&FONT_6X9, Rgb565::WHITE);
    Text::new(ip_addr.deref(), Point::new(10, 15), text_style)
        .draw(&mut display)?;

    display.set_offset(0, 25);

    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../assets/ferris.raw"), 86);
    let image = Image::new(&image_raw, Point::new(26, 8));
    image.draw(&mut display).unwrap();
    Ok(display)
}