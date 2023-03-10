use esp_cam_bindings::{FrameBufferLocation, FrameSize, GrabMode, InitConfig, Pic, PixelFormat};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::FromValueType;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio33)?;
    led.set_high()?;

    InitConfig {
        pin_pwdn: peripherals.pins.gpio32.downgrade(),
        pin_reset: None,
        pin_xclk: peripherals.pins.gpio0.downgrade(),
        pin_sccb_sda: peripherals.pins.gpio26.downgrade(),
        pin_sccb_scl: peripherals.pins.gpio27.downgrade(),
        pin_d7: peripherals.pins.gpio35.downgrade_input(),
        pin_d6: peripherals.pins.gpio34.downgrade_input(),
        pin_d5: peripherals.pins.gpio39.downgrade_input(),
        pin_d4: peripherals.pins.gpio36.downgrade_input(),
        pin_d3: peripherals.pins.gpio21.downgrade_input(),
        pin_d2: peripherals.pins.gpio19.downgrade_input(),
        pin_d1: peripherals.pins.gpio18.downgrade_input(),
        pin_d0: peripherals.pins.gpio5.downgrade_input(),
        pin_vsync: peripherals.pins.gpio25.downgrade(),
        pin_href: peripherals.pins.gpio23.downgrade(),
        pin_pclk: peripherals.pins.gpio22.downgrade(),

        xclk_freq_hz: 20u32.MHz().into(),
        ledc_timer: peripherals.ledc.timer0,
        ledc_channel: peripherals.ledc.channel0,

        pixel_format: PixelFormat::Jpeg,
        frame_size: FrameSize::Hd,
        jpeg_quality: 5,

        fb_count: 5,
        fb_location: FrameBufferLocation::Autodetect,
        grab_mode: GrabMode::Latest,

        sccb_i2c_port: 0,
    }
    .init()?;

    println!("Starting loop");
    loop {
        if let Some(pic) = Pic::new() {
            let width = pic.width();
            let height = pic.height();
            let data = pic.data();
            let len = data.len();
            println!("{width}x{height} {len}");
        }
        led.toggle()?;
        FreeRtos::delay_ms(1000);
    }
}
