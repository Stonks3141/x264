use crate::{Encoder, Encoding, Error, Result};
use core::mem::MaybeUninit;
use x264::*;

mod preset;
mod tune;

pub use self::preset::*;
pub use self::tune::*;

/// Builds a new encoder.
pub struct Setup {
    raw: x264_param_t,
}

impl Setup {
    /// Creates a new builder with the specified preset and tune.
    pub fn preset(preset: Preset, tune: Tune, fast_decode: bool, zero_latency: bool) -> Self {
        let mut raw = MaybeUninit::uninit();

        // Name validity verified at compile-time.
        assert_eq!(0, unsafe {
            x264_param_default_preset(
                raw.as_mut_ptr(),
                preset.to_cstr(),
                tune.to_cstr(fast_decode, zero_latency),
            )
        });

        Self {
            raw: unsafe { raw.assume_init() },
        }
    }

    /// Makes the first pass faster.
    pub fn fastfirstpass(mut self) -> Self {
        unsafe {
            x264_param_apply_fastfirstpass(&mut self.raw);
        }
        self
    }

    /// The video's framerate, represented as a rational number.
    ///
    /// The value is in frames per second.
    pub fn fps(mut self, num: u32, den: u32) -> Self {
        self.raw.i_fps_num = num;
        self.raw.i_fps_den = den;
        self
    }

    /// The encoder's timebase, used in rate control with timestamps.
    ///
    /// The value is in seconds per tick.
    pub fn timebase(mut self, num: u32, den: u32) -> Self {
        self.raw.i_timebase_num = num;
        self.raw.i_timebase_den = den;
        self
    }

    /// Please file an issue if you know what this does, because I have no idea.
    pub fn annexb(mut self, annexb: bool) -> Self {
        self.raw.b_annexb = annexb as i32;
        self
    }

    /// Approximately restricts the bitrate.
    ///
    /// The value is in metric kilobits per second.
    pub fn bitrate(mut self, bitrate: i32) -> Self {
        self.raw.rc.i_bitrate = bitrate;
        self
    }

    /// The lowest profile, with guaranteed compatibility with all decoders.
    pub fn baseline(mut self) -> Self {
        unsafe {
            x264_param_apply_profile(&mut self.raw, b"baseline\0" as *const u8 as *const i8);
        }
        self
    }

    /// A useless middleground between the baseline and high profiles.
    pub fn main(mut self) -> Self {
        unsafe {
            x264_param_apply_profile(&mut self.raw, b"main\0" as *const u8 as *const i8);
        }
        self
    }

    /// The highest profile, which almost all encoders support.
    pub fn high(mut self) -> Self {
        unsafe {
            x264_param_apply_profile(&mut self.raw, b"high\0" as *const u8 as *const i8);
        }
        self
    }

    pub fn max_keyframe_interval(mut self, interval: i32) -> Self {
        self.raw.i_keyint_max = interval;
        self
    }

    /// Set the minimum number of frames between keyframes.
    pub fn min_keyframe_interval(mut self, interval: i32) -> Self {
        self.raw.i_keyint_min = interval;
        self
    }

    /// Set the scenecut threshold. Set this to zero to guarantee a keyframe
    /// every `max_keyframe_interval`.
    pub fn scenecut_threshold(mut self, threshold: i32) -> Self {
        self.raw.i_scenecut_threshold = threshold;
        self
    }

    /// Build the encoder.
    pub fn build<C>(mut self, csp: C, width: i32, height: i32) -> Result<Encoder>
    where
        C: Into<Encoding>,
    {
        self.raw.i_csp = csp.into().into_raw();
        self.raw.i_width = width;
        self.raw.i_height = height;

        let raw = unsafe { x264_encoder_open(&mut self.raw) };

        if raw.is_null() {
            Err(Error)
        } else {
            Ok(unsafe { Encoder::from_raw(raw) })
        }
    }
}

impl Default for Setup {
    fn default() -> Self {
        let raw = unsafe {
            let mut raw = MaybeUninit::uninit();
            x264_param_default(raw.as_mut_ptr());
            raw.assume_init()
        };

        Self { raw }
    }
}
