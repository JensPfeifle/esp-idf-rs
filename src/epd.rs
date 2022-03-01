use crate::epd_driver;

pub fn init_and_clear() {
    unsafe {
        let options: epd_driver::EpdInitOptions = 0u32; // EPD_OPTIONS_DEFAULT
        epd_driver::epd_init(options);
        epd_driver::epd_clear();
    }
}

const EPD_WIDTH: usize = 540;
const EPD_HEIGHT: usize = 960;
const FB_SIZE: usize = EPD_WIDTH / 2 * EPD_HEIGHT;

//ED047TC1 (LilyGo 4.7)

/// An area on the display.
struct EpdRect {
    /// Horizontal position.
    x: usize,
    /// Vertical position.
    y: usize,
    /// Area / image width, must be positive.
    width: usize,
    /// Area / image height, must be positive.
    height: usize,
}

impl EpdRect {
    const FULL_SCREEN: EpdRect = EpdRect {
        x: 0,
        y: 0,
        width: EPD_WIDTH,
        height: EPD_HEIGHT,
    };
}

impl Into<epd_driver::EpdRect> for EpdRect {
    fn into(self) -> epd_driver::EpdRect {
        epd_driver::EpdRect {
            x: self.x as i32,
            y: self.y as i32,
            width: self.width as i32,
            height: self.height as i32,
        }
    }
}

/// Holds internal state.
pub struct EpdState {
    /// The "front" framebuffer object.
    frame_buffer_1: [u8; FB_SIZE],
    frame_buffer_2: [u8; FB_SIZE],
    frame_buffer_diff: [u8; FB_SIZE],
    dirty_lines: [bool; EPD_HEIGHT],
}

impl EpdState {
    pub fn new() -> Self {
        // FIXME: use PSRAM/SPIRAM for frame buffers?
        EpdState {
            frame_buffer_1: [0xff; FB_SIZE],
            frame_buffer_2: [0xff; FB_SIZE],
            frame_buffer_diff: [0xff; FB_SIZE],
            dirty_lines: [false; EPD_HEIGHT],
        }
    }

    pub fn epd_draw_pixel(&mut self, x: usize, y: usize, color: u8) {
        //FIXME: Check rotation and move pixel around if necessary
        if x < 0 || x >= EPD_WIDTH {
            return;
        }
        if y < 0 || y >= EPD_HEIGHT {
            return;
        }

        let fb_index = y * EPD_WIDTH / 2 + x / 2;
        let mut fb_byte = self.frame_buffer_1[fb_index];
        if x % 2 == 0 {
            fb_byte = (fb_byte & 0xF0) | (color >> 4);
        } else {
            fb_byte = (fb_byte & 0x0F) | (color & 0xF0);
        }
        self.frame_buffer_1[fb_index] = fb_byte;
    }

    pub fn epd_draw_hline(&mut self, x: usize, y: usize, length: usize, color: u8) {
        for i in 0..length {
            let xx = x + i;
            self.epd_draw_pixel(xx, y, color);
        }
    }

    pub fn epd_draw_vline(&mut self, x: usize, y: usize, length: usize, color: u8) {
        for i in 0..length {
            let yy = y + i;
            self.epd_draw_pixel(x, yy, color);
        }
    }

    // * Update the EPD screen to match the content of the front frame buffer.
    // * Prior to this, power to the display must be enabled via `epd_poweron()`
    // * and should be disabled afterwards if no immediate additional updates follow.
    // *
    // * @param state: A reference to the `EpdiyHighlevelState` object used.
    // * @param mode: The update mode to use.
    // * 		Additional mode settings like the framebuffer format or
    // * 		previous display state are determined by the driver and must not be supplied here.
    // * 		In most cases, one of `MODE_GC16` and `MODE_GL16` should be used.
    // * @param temperature: Environmental temperature of the display in °C.
    // * @returns `EPD_DRAW_SUCCESS` on sucess, a combination of error flags otherwise.
    // */
    pub fn epd_hl_update_screen(&self, temperature: u8) -> () {
        // FIXME: Return Result

        // Go from any grayscale value to another with a flashing update.
        const MODE_GC16: u32 = 0x2;
        // Framebuffer packing modes
        // 4 bit-per pixel framebuffer with 0x0 = black, 0xF = white.
        // The upper nibble corresponds to the left pixel.
        // A byte cannot wrap over multiple rows, images of uneven width
        // must add a padding nibble per line.
        const MODE_PACKING_2PPB: u32 = 0x80;
        // Draw mode
        // Draw on a white background
        const PREVIOUSLY_WHITE: u32 = 0x200;

        let draw_mode = MODE_PACKING_2PPB | PREVIOUSLY_WHITE | MODE_GC16;
        // Assumes previously white.
        println!("{draw_mode}");

        //area: EpdRect,
        //data: *const u8,
        //crop_to: EpdRect,
        //mode: EpdDrawMode,
        //temperature: ::std::os::raw::c_int,
        //drawn_lines: *const bool,
        //waveform: *const EpdWaveform,
        unsafe {
            epd_driver::epd_draw_base(
                EpdRect::FULL_SCREEN.into(),
                self.frame_buffer_1.as_ptr(),
                EpdRect::FULL_SCREEN.into(),
                draw_mode,
                temperature as i32,
                self.dirty_lines.as_ptr(),
                &epd_driver::epdiy_ED047TC1 as *const epd_driver::EpdWaveform,
            );
        }
    }
}
//
//**
// * Update an area of the screen to match the content of the front framebuffer.
// * Supplying a small area to update can speed up the update process.
// * Prior to this, power to the display must be enabled via `epd_poweron()`
// * and should be disabled afterwards if no immediate additional updates follow.
// *
// * @param state: A reference to the `EpdiyHighlevelState` object used.
// * @param mode: See `epd_hl_update_screen()`.
// * @param temperature: Environmental temperature of the display in °C.
// * @param area: Area of the screen to update.
// * @returns `EPD_DRAW_SUCCESS` on sucess, a combination of error flags otherwise.
// */
//enum EpdDrawError epd_hl_update_area(EpdiyHighlevelState* state, enum EpdDrawMode mode, int temperature, EpdRect area);
//
//**
// * Reset the front framebuffer to a white state.
// *
// * @param state: A reference to the `EpdiyHighlevelState` object used.
// */
//void epd_hl_set_all_white(EpdiyHighlevelState* state);
//
//**
// * Bring the display to a fully white state and get rid of any
// * remaining artifacts.
// */
//void epd_fullclear(EpdiyHighlevelState* state, int temperature);
//
//#ifdef __cplusplus
//}
//#endif
//
