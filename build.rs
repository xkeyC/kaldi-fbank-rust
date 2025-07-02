fn main() {
    // 编译 C++ 文件
    let mut cpp_builder = cc::Build::new();
    cpp_builder
        .cpp(true)
        .include("./kaldi-native-fbank")
        .include("./kaldi-native-fbank/kissfft")
        .files([
            "./kaldi-native-fbank/kaldi-native-fbank/c-api/c-api.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/feature-fbank.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/online-feature.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/feature-window.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/feature-functions.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/mel-computations.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/rfft.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/kaldi-math.cc",
            // "./kaldi-native-fbank/kaldi-native-fbank/csrc/fftsg.cc",
            // NOTE: needed to build on windows, need to figure out why
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/whisper-feature.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/feature-mfcc.cc",
        ])
        .warnings(false);
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        cpp_builder.flag("-std=c++14");
    }
    cpp_builder.compile("kaldi_fbank_cpp");

    let mut c_builder = cc::Build::new();
    c_builder
        .include("./kaldi-native-fbank")
        .include("./kaldi-native-fbank/kissfft")
        .files([
            "./kaldi-native-fbank/kissfft/kiss_fft.c",
            "./kaldi-native-fbank/kissfft/kiss_fftr.c",
        ])
        .warnings(false);
    c_builder.compile("kaldi_fbank_c");
}
