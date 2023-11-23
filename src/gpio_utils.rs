use esp_idf_hal::gpio::{OutputMode, OutputPin, Pin, PinDriver};
use esp_idf_hal::io::EspIOError;


pub fn blinky<T, MODE>(pin: &mut PinDriver<T, MODE>) -> Result<(), EspIOError>
    where
        T: Pin + OutputPin,
        MODE: OutputMode,
{
    if pin.is_set_high() {
        pin.set_low()?;
    } else {
        pin.set_high()?;
    }

    Ok(())
}