
use libc::*;

extern {
    pub fn ebur128_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int);

    pub fn ebur128_init(channels: c_uint, samplerate: c_ulong, mode: c_int) -> *mut c_void;

    pub fn ebur128_destroy(ebur128_state_ptr: *mut *mut c_void);

    pub fn ebur128_set_channel(ebur128_state: *mut c_void, channel_number: c_uint, value: c_int) -> c_int;

    pub fn ebur128_change_parameters(ebur128_state: *mut c_void, channels: c_uint, samplerate: c_ulong) -> c_int;

    pub fn ebur128_set_max_window(ebur128_state: *mut c_void, window: c_ulong) -> c_int;

    pub fn ebur128_set_max_history(ebur128_state: *mut c_void, history: c_ulong) -> c_int;

    pub fn ebur128_add_frames_short(ebur128_state: *mut c_void, src: *const c_short, frames: size_t) -> c_int;

    pub fn ebur128_add_frames_int(ebur128_state: *mut c_void, src: *const c_int, frames: size_t) -> c_int;

    pub fn ebur128_add_frames_float(ebur128_state: *mut c_void, src: *const c_float, frames: size_t) -> c_int;

    pub fn ebur128_add_frames_double(ebur128_state: *mut c_void, src: *const c_double, frames: size_t) -> c_int;

    pub fn ebur128_loudness_global(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    pub fn ebur128_loudness_global_multiple(ebur128_states: *const *mut c_void, size: size_t, out: *mut c_double) -> c_int;

    pub fn ebur128_loudness_momentary(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    pub fn ebur128_loudness_shortterm(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    pub fn ebur128_loudness_window(ebur128_state: *mut c_void, window: c_ulong, out: *mut c_double) -> c_int;

    pub fn ebur128_loudness_range(ebur128_state: *mut c_void, out: *mut c_double) -> c_int;

    pub fn ebur128_loudness_range_multiple(ebur128_states: *const *mut c_void, size: size_t, out: *mut c_double) -> c_int;

    pub fn ebur128_sample_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

    pub fn ebur128_prev_sample_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

    pub fn ebur128_true_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

    pub fn ebur128_prev_true_peak(ebur128_state: *mut c_void, channel_number: c_uint, out: *mut c_double) -> c_int;

}
