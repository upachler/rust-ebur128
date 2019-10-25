
#[macro_use]
extern crate num_derive;

extern crate num_traits;

use num_traits::FromPrimitive;

use libc::*;

extern {
    fn ebur128_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int);

    fn ebur128_init(channels: c_uint, samplerate: c_ulong, mode: c_int) -> *mut c_void;

    fn ebur128_destroy(ebur128_state_ptr: *mut *mut c_void);

    fn ebur128_set_channel(ebur128_state: *mut c_void, channel_number: c_uint, value: c_int) -> c_int;

    fn ebur128_change_parameters(ebur128_state: *mut c_void, channels: c_uint, samplerate: c_ulong) -> c_int;

    fn ebur128_set_max_window(ebur128_state: *mut c_void, window: c_ulong) -> c_int;

    fn ebur128_set_max_history(ebur128_state: *mut c_void, history: c_ulong) -> c_int;

    fn ebur128_add_frames_short(ebur128_state: *mut c_void, src: *const c_short, frames: size_t) -> c_int;

    fn ebur128_add_frames_int(ebur128_state: *mut c_void, src: *const c_int, frames: size_t) -> c_int;

    fn ebur128_add_frames_float(ebur128_state: *mut c_void, src: *const c_float, frames: size_t) -> c_int;

    fn ebur128_add_frames_double(ebur128_state: *mut c_void, src: *const c_double, frames: size_t) -> c_int;

    fn ebur128_loudness_global(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    fn ebur128_loudness_global_multiple(ebur128_states: *const *mut c_void, size: size_t, out: *mut c_double) -> c_int;

    fn ebur128_loudness_momentary(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    fn ebur128_loudness_shortterm(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    fn ebur128_loudness_window(ebur128_state: *mut c_void, window: c_ulong, out: *mut c_double) -> c_int;

    fn ebur128_loudness_range(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    fn ebur128_loudness_range_multiple(ebur128_states: *const *mut c_void, size: size_t, out: *mut c_double) -> c_int;

    fn ebur128_sample_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

    fn ebur128_prev_sample_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

    fn ebur128_true_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

    fn ebur128_prev_true_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

}

pub enum Mode {
    M          = (1 << 0),
    S          = (1 << 1) | Mode::M as isize,
    I          = (1 << 2) | Mode::M as isize,
    LRA        = (1 << 3) | Mode::S as isize,
    SamplePeak = (1 << 4) | Mode::M as isize,
    TruePeak   = (1 << 5) | Mode::M as isize | Mode::SamplePeak as isize,
    Histogram  = (1 << 6)
}



pub enum Channel {
    /// unused channel (for example LFE channel)
    Unused = 0,
    /// itu M+030
    Mp030  = 1, 
    Mm030  = 2,     /**< itu M-030 */
    Mp000  = 3,     /**< itu M+000 */
    Mp110  = 4,     /**< itu M+110 */
    Mm110  = 5,     /**< itu M-110 */
    DualMono,      /**< a channel that is counted twice */
    MpSC,           /**< itu M+SC */
    MmSC,           /**< itu M-SC */
    Mp060,          /**< itu M+060 */
    Mm060,          /**< itu M-060 */
    Mp090,          /**< itu M+090 */
    Mm090,          /**< itu M-090 */
    Mp135,          /**< itu M+135 */
    Mm135,          /**< itu M-135 */
    Mp180,          /**< itu M+180 */
    Up000,          /**< itu U+000 */
    Up030,          /**< itu U+030 */
    Um030,          /**< itu U-030 */
    Up045,          /**< itu U+045 */
    Um045,          /**< itu U-030 */
    Up090,          /**< itu U+090 */
    Um090,          /**< itu U-090 */
    Up110,          /**< itu U+110 */
    Um110,          /**< itu U-110 */
    Up135,          /**< itu U+135 */
    Um135,          /**< itu U-135 */
    Up180,          /**< itu U+180 */
    Tp000,          /**< itu T+000 */
    Bp000,          /**< itu B+000 */
    Bp045,          /**< itu B+045 */
    /// itu B-045
    Bm045,

    // NOTE: in C these map to 1..5, like Mp30..Mm110 (Left and Mm110 both have 1 as value)
    // However, as rust won't allow ambiguous mappings, we use the value | 0x1000
    Left   = 0x1001,
    Right  = 0x1002,
    Center = 0x1003,
    LeftSurround  = 0x1004,
    RightSurround = 0x1005,

}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum Error {
  /// EBUR128_ERROR_NOMEM
  NoMem = 1, 
  /// EBUR128_ERROR_INVALID_MODE,
  InvalidMode,
  /// EBUR128_ERROR_INVALID_CHANNEL_INDEX
  InvalidChannelIndex,
  /// EBUR128_ERROR_NO_CHANGE
  NoChange,
}

pub struct State {
    ebur128_state: *mut c_void,
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            ebur128_destroy(&mut self.ebur128_state)
        }
    }
}

fn to_result(ebur128_error: c_int) -> Result<(),Error> {
    to_result_t(ebur128_error, ())
}

fn to_result_f64(ebur128_error: c_int, out: f64) -> Result<f64,Error> {
    to_result_t(ebur128_error, out)
}

fn to_result_t<T>(ebur128_error: c_int, out: T) -> Result<T,Error> {
    match ebur128_error {
        0 => Result::Ok(out),
        n => Result::Err(Error::from_i32(n).unwrap())
    }
}


impl State {
    pub fn new(channels: usize, samplerate: u64, mode: Mode) -> State {
        unsafe {
            // NOTE: because the original Mode enum has separate enum 
            // members assigned to the same value, which we can't do 
            // in rust, we mapped those higher and mask that offset
            // back out here.
            let c_mode = mode as c_int & 0x0fff;
            State {
                ebur128_state: ebur128_init(channels as c_uint, samplerate as c_ulong, c_mode)
            }
        }
    }

    pub fn set_channel(&mut self, channel_number: u32, value: Channel) -> Result<(),Error> {
        to_result(unsafe {
            ebur128_set_channel(self.ebur128_state, channel_number as u32, value as i32)
        })
    }

    pub fn change_parameters(&mut self, channels: u32, samplerate: u64) -> Result<(),Error> {
        to_result(unsafe {
            ebur128_change_parameters(self.ebur128_state, channels as c_uint, samplerate as c_ulong)
        })
    }

    pub fn set_max_window(&mut self, window: usize) -> Result<(),Error> {
        to_result(unsafe {
            ebur128_set_max_window(self.ebur128_state, window as c_ulong)
        })
    }

    pub fn set_max_history(&mut self, history: usize) -> Result<(),Error> {
        to_result(unsafe{
            ebur128_set_max_history(self.ebur128_state, history as c_ulong)
        })
    }

    pub fn add_frames_short(&mut self, src: &[i16]) -> Result<(),Error> {
        to_result(unsafe {
            ebur128_add_frames_short(self.ebur128_state, src.as_ptr(), src.len())
        })
    }

    pub fn add_frames_int(&mut self, src: &[i32]) -> Result<(),Error> {
        to_result(unsafe {
            ebur128_add_frames_int(self.ebur128_state, src.as_ptr(), src.len())
        })
    }

    pub fn add_frames_float(&mut self, src: &[f32]) -> Result<(),Error> {
        to_result(unsafe {
            ebur128_add_frames_float(self.ebur128_state, src.as_ptr(), src.len())
        })
    }

    pub fn add_frames_double(&mut self, src: &[f64]) -> Result<(),Error> {
        to_result(unsafe {
            ebur128_add_frames_double(self.ebur128_state, src.as_ptr(), src.len())
        })
    }

/*
    fn loudness_global_multiple(ebur128_states: &[&mut Self]) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_loudness_global_multiple(ebur128_states.as_ptr(), ebur128_states.len(), &mut out)
        }, out)
    }

    fn loudness_range_multiple(ebur128_states: &mut[Self]) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            loudness_range_multiple(ebur128_states.as_ptr(), ebur128_states.len(), &out)
        }, out)
    }
*/
    pub fn loudness_global(&mut self) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_loudness_global(self.ebur128_state, &mut out)
        }, out)
    }

    pub fn loudness_momentary(&mut self) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_loudness_momentary(self.ebur128_state, &mut out)
        }, out)
    }

    pub fn loudness_shortterm(&mut self) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_loudness_shortterm(self.ebur128_state, &mut out)
        }, out)
    }

    pub fn loudness_window(&mut self, window: usize) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_loudness_window(self.ebur128_state, window as c_ulong, &mut out)
        }, out)
    }

    pub fn loudness_range(&mut self) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_loudness_range(self.ebur128_state, &mut out)
        }, out)
    }

    pub fn sample_peak(&mut self, channel_number: u32) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_sample_peak(self.ebur128_state, channel_number, &mut out)
        }, out)
    }

    pub fn prev_sample_peak(&mut self, channel_number: u32) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_prev_sample_peak(self.ebur128_state, channel_number,&mut out)
        }, out)
    }

    pub fn true_peak(&mut self, channel_number: u32) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_true_peak(self.ebur128_state, channel_number,&mut out)
        }, out)
    }

    pub fn prev_true_peak(&mut self, channel_number: u32) -> Result<f64,Error> {
        let mut out: f64 = 0.;
        to_result_f64(unsafe {
            ebur128_prev_true_peak(self.ebur128_state, channel_number,&mut out)
        }, out)
    }

}

pub fn get_version() -> (i32,i32,i32) {
    unsafe {
        let mut major: c_int = -1;
        let mut minor: c_int = -1;
        let mut patch: c_int = -1;

        crate::ebur128_get_version(&mut major, &mut minor, &mut patch);

        (major, minor, patch)
    }
}





#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {

        let (major, minor, patch) = crate::get_version();

        assert_eq!(1, major);
        assert_eq!(2, minor);
        assert_eq!(4, patch);

        let mut state = State::new(2, 44100, Mode::M);
        let r = state.set_channel(0, Channel::Left);
        assert_eq!(Result::Ok(()), r);
        let r = state.set_channel(1, Channel::Right);
        assert_eq!(Result::Ok(()), r);

        let r = state.set_channel(2, Channel::RightSurround);
        assert_eq!(Result::Err(Error::InvalidChannelIndex), r);
    }
}
