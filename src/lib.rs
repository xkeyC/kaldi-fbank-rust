use std::ffi::CStr;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused)]
mod lib_sys;

impl Default for lib_sys::FbankOptions {
    fn default() -> Self {
        lib_sys::FbankOptions {
            frame_opts: lib_sys::FrameExtractionOptions {
                samp_freq: 16_000.0,
                frame_shift_ms: 10.0,
                frame_length_ms: 25.0,
                dither: 0.00003,
                preemph_coeff: 0.97,
                remove_dc_offset: true,
                window_type: CStr::from_bytes_with_nul(b"povey\0").unwrap().as_ptr(),
                round_to_power_of_two: true,
                blackman_coeff: 0.42,
                snip_edges: true,
            },
            mel_opts: lib_sys::MelBanksOptions {
                num_bins: 25,
                low_freq: 20.0,
                high_freq: 0.0,
                vtln_low: 100.0,
                vtln_high: -500.0,
                debug_mel: false,
                htk_mode: false,
                is_librosa: false,
                norm: CStr::from_bytes_with_nul(b"slaney\0").unwrap().as_ptr(),
            },
            use_energy: false,
            energy_floor: 0.0,
            raw_energy: true,
            htk_compat: false,
            use_log_fbank: true,
            use_power: true,
        }
    }
}

pub type FbankOptions = lib_sys::FbankOptions;

pub struct OnlineFbank {
    ptr: *mut lib_sys::OnlineFbank,
}

impl OnlineFbank {
    /// Create a new OnlineFbank object with the given sample rate
    pub fn new(options: FbankOptions) -> Self {
        let ptr = unsafe { lib_sys::OnlineFbankNew(options) };
        Self { ptr }
    }

    /// Get the dimension of the feature vectors produced by the OnlineFbank object
    pub fn dim(&self) -> i32 {
        unsafe { lib_sys::OnlineFbankDim(self.ptr) }
    }

    /// Accept a waveform and add it to the buffer for feature calulation
    pub fn accept_waveform(&mut self, sampling_rate: f32, waveform: &[f32]) {
        unsafe {
            lib_sys::OnlineFbankAcceptWaveform(
                self.ptr,
                sampling_rate,
                waveform.as_ptr(),
                waveform.len() as i32,
            )
        }
    }

    /// Indicate that the input has finished and no more waveforms will be added
    pub fn input_finished(&self) {
        unsafe {
            lib_sys::OnlineFbankInputFinished(self.ptr);
        }
    }

    /// Get the feature vector for a specific frame
    /// Returns None if there is no frame ready yet
    pub fn get_frame(&self, frame: i32) -> Option<&[f32]> {
        if frame >= self.num_ready_frames() {
            return None;
        }
        unsafe {
            let ptr = lib_sys::OnlineFbankGetFrame(self.ptr, frame);
            Some(std::slice::from_raw_parts(ptr, self.dim() as usize))
        }
    }

    /// Check if a specific frame is the last one
    pub fn is_last_frame(&self, frame: i32) -> bool {
        unsafe { lib_sys::OnlineFbankIsLastFrame(self.ptr, frame) }
    }

    /// Return the number of ready frames
    pub fn num_ready_frames(&self) -> i32 {
        unsafe { lib_sys::OnlineFbankNumFramesReady(self.ptr) }
    }

    /// Get the frame shift in seconds
    pub fn frame_shift_in_seconds(&self) -> f32 {
        unsafe { lib_sys::OnlineFbankFrameShiftInSeconds(self.ptr) }
    }

    /// Discard the first n frames
    pub fn pop(&self, n: i32) {
        unsafe {
            lib_sys::OnlineFbankPop(self.ptr, n);
        }
    }
}

impl Drop for OnlineFbank {
    fn drop(&mut self) {
        unsafe { lib_sys::OnlineFbankFree(self.ptr) }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib_sys;
    use lib_sys::*;

    #[test]
    fn test_not_enough_samples() {
        let options = super::FbankOptions::default();
        let mut fbank = super::OnlineFbank::new(options);
        fbank.accept_waveform(16_000f32, &[0.0; 160]);
        fbank.input_finished();
        let frame = fbank.get_frame(0);
        assert!(frame.is_none());
    }

    #[test]
    fn test_some_samples() {
        let mut options = super::FbankOptions::default();
        options.mel_opts.num_bins = 80;
        let mut fbank = super::OnlineFbank::new(options);
        fbank.accept_waveform(16_000f32, &[0.0; 16000 * 10]);
        fbank.input_finished();
        assert!(fbank.num_ready_frames() > 0);
        let frame = fbank.get_frame(0);
        assert!(frame.is_some());
        assert_eq!(frame.unwrap().len(), 80);
    }

    use std::ffi::*;
    #[test]
    fn test_raw_create() {
        unsafe {
            let options = super::lib_sys::FbankOptions {
                frame_opts: FrameExtractionOptions {
                    samp_freq: 16_000.0,
                    frame_shift_ms: 10.0,
                    frame_length_ms: 25.0,
                    dither: 0.00003,
                    preemph_coeff: 0.97,
                    remove_dc_offset: true,
                    window_type: CStr::from_bytes_with_nul(b"povey\0").unwrap().as_ptr(),
                    round_to_power_of_two: true,
                    blackman_coeff: 0.42,
                    snip_edges: true,
                },
                mel_opts: MelBanksOptions {
                    num_bins: 25,
                    low_freq: 20.0,
                    high_freq: 0.0,
                    vtln_low: 100.0,
                    vtln_high: -500.0,
                    debug_mel: false,
                    htk_mode: false,
                    is_librosa: false,
                    norm: CStr::from_bytes_with_nul(b"slaney\0").unwrap().as_ptr(),
                },
                use_energy: false,
                energy_floor: 0.0,
                raw_energy: true,
                htk_compat: false,
                use_log_fbank: true,
                use_power: true,
            };
            let fbank = OnlineFbankNew(options);
            let dim = OnlineFbankDim(fbank);
            dbg!(dim);
            OnlineFbankFree(fbank);
        }
    }
}
