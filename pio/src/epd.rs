use crate::epd_highlevel;
use crate::epd_highlevel::EpdiyHighlevelState;

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
        self.epd_state = EpdState::HighlevelState(state);
    }

    pub fn clear(&self) -> () {
        match self.epd_state {
            EpdState::Uninitialized => {}
            EpdState::HighlevelState(_) => {
                unsafe { epd_highlevel::epd_clear() };
            }
        }
    }

    /// Get a mutable slice into the  display framebuffer, if it is initialized.
    pub fn get_framebuffer(&self) -> Option<&'a mut [u8]> {
        match self.epd_state {
            EpdState::Uninitialized => None,
            EpdState::HighlevelState(state) => {
                let fb: &mut [u8] =
                    unsafe { std::slice::from_raw_parts_mut(state.front_fb, FB_SIZE) };
                Some(fb)
            }
        }
    }

    }

    /// Update the screen to display the current contents of the framebuffer.
    pub fn update_screen(&self, temperature: i32) -> () {
        match self.epd_state {
            EpdState::Uninitialized => {}
            EpdState::HighlevelState(mut state) => {
                unsafe { epd_highlevel::epd_poweron() };

                const MODE_GC16: epd_highlevel::EpdDrawMode = 0x2;
                let result: epd_highlevel::EpdDrawError = unsafe {
                    epd_highlevel::epd_hl_update_screen(&mut state, MODE_GC16, temperature)
                };
                println!("Draw result: {result:?}");

                unsafe { epd_highlevel::epd_poweroff() };
            }
        }
    }
}
