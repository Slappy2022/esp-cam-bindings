use esp_idf_hal::gpio::{AnyIOPin, AnyInputPin, Pin};
use esp_idf_hal::ledc::{CHANNEL0, TIMER0};
use esp_idf_hal::units::Hertz;
use esp_idf_sys::*;

#[derive(Clone, Copy)]
pub enum FrameBufferLocation {
    Autodetect = -1,
    Psram = camera_fb_location_t_CAMERA_FB_IN_PSRAM as isize,
    Dram = camera_fb_location_t_CAMERA_FB_IN_DRAM as isize,
}

#[derive(Clone, Copy)]
pub enum GrabMode {
    WhenEmpty = camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY as isize,
    Latest = camera_grab_mode_t_CAMERA_GRAB_LATEST as isize,
}

#[derive(Clone, Copy)]
pub enum PixelFormat {
    Rgb565 = pixformat_t_PIXFORMAT_RGB565 as isize,
    Yuv422 = pixformat_t_PIXFORMAT_YUV422 as isize,
    Yuv420 = pixformat_t_PIXFORMAT_YUV420 as isize,
    Grayscale = pixformat_t_PIXFORMAT_GRAYSCALE as isize,
    Jpeg = pixformat_t_PIXFORMAT_JPEG as isize,
    Rpb888 = pixformat_t_PIXFORMAT_RGB888 as isize,
    Raw = pixformat_t_PIXFORMAT_RAW as isize,
    Rgb444 = pixformat_t_PIXFORMAT_RGB444 as isize,
    Rgb555 = pixformat_t_PIXFORMAT_RGB555 as isize,
}

#[derive(Clone, Copy)]
pub enum FrameSize {
    Size96x96 = framesize_t_FRAMESIZE_96X96 as isize,
    Qqvga = framesize_t_FRAMESIZE_QQVGA as isize,
    Qcif = framesize_t_FRAMESIZE_QCIF as isize,
    Hqvga = framesize_t_FRAMESIZE_HQVGA as isize,
    Size240x240 = framesize_t_FRAMESIZE_240X240 as isize,
    Qvga = framesize_t_FRAMESIZE_QVGA as isize,
    Cif = framesize_t_FRAMESIZE_CIF as isize,
    Hvga = framesize_t_FRAMESIZE_HVGA as isize,
    Vga = framesize_t_FRAMESIZE_VGA as isize,
    Svga = framesize_t_FRAMESIZE_SVGA as isize,
    Xga = framesize_t_FRAMESIZE_XGA as isize,
    Hd = framesize_t_FRAMESIZE_HD as isize,
    Sxga = framesize_t_FRAMESIZE_SXGA as isize,
    Uxga = framesize_t_FRAMESIZE_UXGA as isize,
    Fhd = framesize_t_FRAMESIZE_FHD as isize,
    Phd = framesize_t_FRAMESIZE_P_HD as isize,
    P3mp = framesize_t_FRAMESIZE_P_3MP as isize,
    Qxga = framesize_t_FRAMESIZE_QXGA as isize,
    Qhd = framesize_t_FRAMESIZE_QHD as isize,
    Wqxga = framesize_t_FRAMESIZE_WQXGA as isize,
    Pfhd = framesize_t_FRAMESIZE_P_FHD as isize,
    Qsxga = framesize_t_FRAMESIZE_QSXGA as isize,
    Invalid = framesize_t_FRAMESIZE_INVALID as isize,
}

pub struct InitConfig {
    pub pin_pwdn: AnyIOPin,
    pub pin_reset: Option<AnyIOPin>,
    pub pin_xclk: AnyIOPin,
    pub pin_sccb_sda: AnyIOPin,
    pub pin_sccb_scl: AnyIOPin,
    pub pin_d7: AnyInputPin,
    pub pin_d6: AnyInputPin,
    pub pin_d5: AnyInputPin,
    pub pin_d4: AnyInputPin,
    pub pin_d3: AnyInputPin,
    pub pin_d2: AnyInputPin,
    pub pin_d1: AnyInputPin,
    pub pin_d0: AnyInputPin,
    pub pin_vsync: AnyIOPin,
    pub pin_href: AnyIOPin,
    pub pin_pclk: AnyIOPin,

    pub xclk_freq_hz: Hertz,

    pub ledc_timer: TIMER0,
    pub ledc_channel: CHANNEL0,

    pub pixel_format: PixelFormat,
    pub frame_size: FrameSize,
    pub jpeg_quality: u8,

    pub fb_count: usize,
    pub fb_location: FrameBufferLocation,
    pub grab_mode: GrabMode,
    pub sccb_i2c_port: u8,
}

impl InitConfig {
    pub fn init(&self) -> Result<(), EspError> {
        let ledc_timer = ledc_timer_t_LEDC_TIMER_0;
        let ledc_channel = ledc_channel_t_LEDC_CHANNEL_0;

        let fb_location = match self.fb_location {
            FrameBufferLocation::Autodetect => {
                match unsafe { esp_idf_sys::heap_caps_get_total_size(MALLOC_CAP_SPIRAM) } {
                    0 => FrameBufferLocation::Dram,
                    _ => FrameBufferLocation::Psram,
                }
            }
            l => l,
        } as u32;

        esp!(unsafe {
            esp_camera_init(&camera_config_t {
                pin_pwdn: self.pin_pwdn.pin() as i32,
                pin_reset: self.pin_reset.as_ref().map_or(-1, |x| x.pin()) as i32,
                pin_xclk: self.pin_xclk.pin() as i32,
                pin_sccb_sda: self.pin_sccb_sda.pin() as i32,
                pin_sccb_scl: self.pin_sccb_scl.pin() as i32,
                pin_d7: self.pin_d7.pin() as i32,
                pin_d6: self.pin_d6.pin() as i32,
                pin_d5: self.pin_d5.pin() as i32,
                pin_d4: self.pin_d4.pin() as i32,
                pin_d3: self.pin_d3.pin() as i32,
                pin_d2: self.pin_d2.pin() as i32,
                pin_d1: self.pin_d1.pin() as i32,
                pin_d0: self.pin_d0.pin() as i32,
                pin_vsync: self.pin_vsync.pin() as i32,
                pin_href: self.pin_href.pin() as i32,
                pin_pclk: self.pin_pclk.pin() as i32,
                xclk_freq_hz: u32::from(self.xclk_freq_hz) as i32,

                ledc_timer,
                ledc_channel,

                pixel_format: self.pixel_format as u32,
                frame_size: self.frame_size as u32,
                jpeg_quality: self.jpeg_quality as i32,
                fb_count: self.fb_count,
                fb_location,
                grab_mode: self.grab_mode as u32,

                sccb_i2c_port: self.sccb_i2c_port as i32,
            })
        })
    }
}

pub struct Pic {
    data: *mut camera_fb_t,
}
impl Pic {
    pub fn new() -> Option<Self> {
        unsafe { esp_camera_fb_get().as_mut().map(|data| Self { data }) }
    }
    pub fn width(&self) -> usize {
        unsafe { (*self.data).width as usize }
    }
    pub fn height(&self) -> usize {
        unsafe { (*self.data).height as usize }
    }
    pub fn data(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts((*self.data).buf, (*self.data).len as usize) }
    }
}

impl Drop for Pic {
    fn drop(&mut self) {
        unsafe { esp_camera_fb_return(self.data) }
    }
}
