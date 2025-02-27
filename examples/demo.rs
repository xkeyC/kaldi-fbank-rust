use std::ffi::CStr;

use kaldi_fbank_rust::FbankOptions;
use kaldi_fbank_rust::FrameExtractionOptions;
use kaldi_fbank_rust::MelBanksOptions;
use kaldi_fbank_rust::OnlineFbank;

fn main() {
    let opt = FbankOptions {
        frame_opts: FrameExtractionOptions {
            samp_freq: 16000.0,
            window_type: CStr::from_bytes_with_nul(b"hamming\0").unwrap().as_ptr(),
            dither: 0.0,
            frame_shift_ms: 10.0,
            frame_length_ms: 25.0,
            ..Default::default()
        },
        mel_opts: MelBanksOptions {
            num_bins: 80,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut fbank = OnlineFbank::new(opt);
    fbank.accept_waveform(16_000f32, &[0.0; 16000 * 10]);
    let frames = fbank.num_ready_frames();
    // 提取特徵
    for i in 0..frames {
        let frame = fbank.get_frame(i);
        // frame 是長度為 num_bins 的 float 數組
        for j in 0..5 {
            // 打印前 5 個值作為示例
            print!("{} ", frame[j as usize]);
        }
        println!();
    }
}
