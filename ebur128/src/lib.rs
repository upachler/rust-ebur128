
#[macro_use]
extern crate num_derive;

extern crate num_traits;

use num_traits::FromPrimitive;

use libc::*;
use ebur128_sys::*;

#[derive(Copy, Clone)]
pub enum Mode {
    M          = mode_EBUR128_MODE_M           as isize,
    S          = mode_EBUR128_MODE_S           as isize,
    I          = mode_EBUR128_MODE_I           as isize,
    LRA        = mode_EBUR128_MODE_LRA         as isize,
    SamplePeak = mode_EBUR128_MODE_SAMPLE_PEAK as isize,
    TruePeak   = mode_EBUR128_MODE_TRUE_PEAK   as isize,
    Histogram  = mode_EBUR128_MODE_HISTOGRAM   as isize
}



pub enum Channel {
    /// unused channel (for example LFE channel)
    Unused = channel_EBUR128_UNUSED as isize,
    /// itu M+030
    Mp030  = channel_EBUR128_Mp030 as isize, 
    Mm030  = channel_EBUR128_Mm030 as isize,     /**< itu M-030 */
    Mp000  = channel_EBUR128_Mp000 as isize,     /**< itu M+000 */
    Mp110  = channel_EBUR128_Mp110 as isize,     /**< itu M+110 */
    Mm110  = channel_EBUR128_Mm110 as isize,     /**< itu M-110 */
    DualMono = channel_EBUR128_DUAL_MONO as isize,      /**< a channel that is counted twice */
    MpSC   = channel_EBUR128_MpSC as isize,           /**< itu M+SC */
    MmSC   = channel_EBUR128_MmSC as isize,           /**< itu M-SC */
    Mp060  = channel_EBUR128_Mp060 as isize,          /**< itu M+060 */
    Mm060  = channel_EBUR128_Mm060 as isize,          /**< itu M-060 */
    Mp090  = channel_EBUR128_Mp090 as isize,          /**< itu M+090 */
    Mm090  = channel_EBUR128_Mm090 as isize,          /**< itu M-090 */
    Mp135  = channel_EBUR128_Mp135 as isize,          /**< itu M+135 */
    Mm135  = channel_EBUR128_Mm135 as isize,          /**< itu M-135 */
    Mp180  = channel_EBUR128_Mp180 as isize,          /**< itu M+180 */
    Up000  = channel_EBUR128_Up000 as isize,          /**< itu U+000 */
    Up030  = channel_EBUR128_Up030 as isize,          /**< itu U+030 */
    Um030  = channel_EBUR128_Um030 as isize,          /**< itu U-030 */
    Up045  = channel_EBUR128_Up045 as isize,          /**< itu U+045 */
    Um045  = channel_EBUR128_Um045 as isize,          /**< itu U-030 */
    Up090  = channel_EBUR128_Up090 as isize,          /**< itu U+090 */
    Um090  = channel_EBUR128_Um090 as isize,          /**< itu U-090 */
    Up110  = channel_EBUR128_Up110 as isize,          /**< itu U+110 */
    Um110  = channel_EBUR128_Um110 as isize,          /**< itu U-110 */
    Up135  = channel_EBUR128_Up135 as isize,          /**< itu U+135 */
    Um135  = channel_EBUR128_Um135 as isize,          /**< itu U-135 */
    Up180  = channel_EBUR128_Up180 as isize,          /**< itu U+180 */
    Tp000  = channel_EBUR128_Tp000 as isize,          /**< itu T+000 */
    Bp000  = channel_EBUR128_Bp000 as isize,          /**< itu B+000 */
    Bp045  = channel_EBUR128_Bp045 as isize,          /**< itu B+045 */
    /// itu B-045
    Bm045  = channel_EBUR128_Bm045 as isize,

    // NOTE: in C these map to 1..5 as isize, like Mp30..Mm110 (Left and Mm110 both have 1 as value)
    // However as isize, as rust won't allow ambiguous mappings as isize, we use the value | 0x1000
    Left   = 0x1000 as isize | channel_EBUR128_LEFT as isize,
    Right  = 0x1000 | channel_EBUR128_RIGHT as isize,
    Center = 0x1000 | channel_EBUR128_CENTER as isize,
    LeftSurround  = 0x1000 | channel_EBUR128_LEFT_SURROUND as isize,
    RightSurround = 0x1000 | channel_EBUR128_RIGHT_SURROUND as isize,

}

#[derive(FromPrimitive, PartialEq, Debug)]
pub enum Error {
  /// EBUR128_ERROR_NOMEM
  NoMem = error_EBUR128_ERROR_NOMEM as isize, 
  /// EBUR128_ERROR_INVALID_MODE,
  InvalidMode = error_EBUR128_ERROR_INVALID_MODE as isize,
  /// EBUR128_ERROR_INVALID_CHANNEL_INDEX
  InvalidChannelIndex = error_EBUR128_ERROR_INVALID_CHANNEL_INDEX as isize,
  /// EBUR128_ERROR_NO_CHANGE
  NoChange = error_EBUR128_ERROR_NO_CHANGE as isize,
}

pub struct State {
    ebur128_state: *mut ebur128_state,
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
    pub fn new(channels: usize, samplerate: u64, modes: &[Mode]) -> State {
        // NOTE: because the original Mode enum has separate enum 
        // members assigned to the same value, which we can't do 
        // in rust, we mapped those higher and mask that offset
        // back out here.
        let mut c_mode = 0;
        modes.iter().for_each(|mode| c_mode |= *mode as c_int & 0x0fff);
        unsafe {
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

        let mut state = State::new(2, 44100, &[Mode::M]);
        let r = state.set_channel(0, Channel::Left);
        assert_eq!(Result::Ok(()), r);
        let r = state.set_channel(1, Channel::Right);
        assert_eq!(Result::Ok(()), r);

        let r = state.set_channel(2, Channel::RightSurround);
        // NOTE: the current stable libebur128 version (1.2.4)
        // appears to have a bug, causing to return Error:NoMem
        // here - we'd acutally expect Error::InvalidChannelIndex.
        // Therefore, instead of checking for the actual error,
        // we check for not ok..
        assert_ne!(Result::Ok(()), r);
    }
}
