use crate::epd_highlevel;
use crate::epd_highlevel::EpdiyHighlevelState;
use crate::firasans::FiraSans_12;

const EPD_WIDTH: usize = 960;
const EPD_HEIGHT: usize = 540;
const FB_SIZE: usize = EPD_WIDTH / 2 * EPD_HEIGHT;

#[derive(Debug)]
enum EpdState {
    Uninitialized,
    HighlevelState(EpdiyHighlevelState),
}

#[derive(Debug)]
pub struct Epd {
    epd_state: EpdState,
}

impl<'a> Epd {
    pub fn new() -> Self {
        Self {
            epd_state: EpdState::Uninitialized,
        }
    }

    pub fn init(&mut self) {
        const EPD_LUT_4K: u32 = 2;
        unsafe { epd_highlevel::epd_init(EPD_LUT_4K) };
        let state: epd_highlevel::EpdiyHighlevelState = unsafe { epd_highlevel::epd_hl_init() };

        //const EPD_ROT_LANDSCAPE: u32 = 0;
        const EPD_ROT_PORTRAIT: u32 = 1;
        unsafe { epd_highlevel::epd_set_rotation(EPD_ROT_PORTRAIT) };

        self.epd_state = EpdState::HighlevelState(state);
    }

    pub fn set_all(&mut self, color: u8) {
        match self.epd_state {
            EpdState::Uninitialized => {}
            EpdState::HighlevelState(state) => {
                let fb: &mut [u8] =
                    unsafe { std::slice::from_raw_parts_mut(state.front_fb, FB_SIZE) };
                fb.iter_mut().for_each(|x| *x = color);
            }
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u8) {
        match self.epd_state {
            EpdState::Uninitialized => {}
            EpdState::HighlevelState(state) => {
                //FIXME: Check rotation and move pixel around if necessary
                if x >= EPD_WIDTH {
                    return;
                }
                if y >= EPD_HEIGHT {
                    return;
                }

                let fb: &mut [u8] =
                    unsafe { std::slice::from_raw_parts_mut(state.front_fb, FB_SIZE) };

                let fb_index = y * EPD_WIDTH / 2 + x / 2;
                let mut fb_byte = fb[fb_index];
                if x % 2 == 0 {
                    fb_byte = (fb_byte & 0xF0) | (color >> 4);
                } else {
                    fb_byte = (fb_byte & 0x0F) | (color & 0xF0);
                }
                fb[fb_index] = fb_byte;
            }
        }
    }

    pub fn draw_hline(&mut self, x: usize, y: usize, length: usize, color: u8) {
        for i in 0..length {
            let xx = x + i;
            self.draw_pixel(xx, y, color);
        }
    }

    pub fn draw_vline(&mut self, x: usize, y: usize, length: usize, color: u8) {
        for i in 0..length {
            let yy = y + i;
            self.draw_pixel(x, yy, color);
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u8) {
        for i in y..y + h {
            self.draw_hline(x, i, w, color);
        }
    }

    pub fn clear(&self) -> () {
        match self.epd_state {
            EpdState::Uninitialized => {}
            EpdState::HighlevelState(_) => {
                unsafe { epd_highlevel::epd_clear() };
            }
        }
    }

    fn get_fb(&self) -> Option<&'a mut [u8]> {
        match self.epd_state {
            EpdState::Uninitialized => None,
            EpdState::HighlevelState(state) => {
                let fb: &mut [u8] =
                    unsafe { std::slice::from_raw_parts_mut(state.front_fb, FB_SIZE) };
                return Some(fb);
            }
        }
    }

    pub fn write_text(&mut self, x: usize, y: usize, text: String) {
        let t = text.as_ptr() as *const i8;
        let font = &FiraSans_12 as *const epd_highlevel::EpdFont;
        let x_ptr = &x as *const usize as *mut i32;
        let y_ptr = &y as *const usize as *mut i32;

        if let Some(fb) = self.get_fb() {
            unsafe { epd_highlevel::epd_write_default(font, t, x_ptr, y_ptr, fb.as_mut_ptr()) };
        }
    }

    pub fn update_screen(&self, temperature: i32) -> () {
        match self.epd_state {
            EpdState::Uninitialized => {}
            EpdState::HighlevelState(mut state) => {
                const MODE_GC16: epd_highlevel::EpdDrawMode = 0x2;

                println!("poweron");
                unsafe { epd_highlevel::epd_poweron() };

                let result: epd_highlevel::EpdDrawError = unsafe {
                    epd_highlevel::epd_hl_update_screen(&mut state, MODE_GC16, temperature)
                };
                println!("Draw result: {result:?}");

                println!("poweroff");
                unsafe { epd_highlevel::epd_poweroff() };
            }
        }
    }
}
