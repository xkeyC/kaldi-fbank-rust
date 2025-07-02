fn main() {
    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .include("./kaldi-native-fbank")
        .files([
            "./kaldi-native-fbank/kaldi-native-fbank/c-api/c-api.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/feature-fbank.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/online-feature.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/feature-window.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/feature-functions.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/mel-computations.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/rfft.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/kaldi-math.cc",
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/fftsg.cc",
            // NOTE: needed to build on windows, need to figure out why
            "./kaldi-native-fbank/kaldi-native-fbank/csrc/whisper-feature.cc",
        ])
        // TODO: There are a bunch of those, maybe can fix them upstream
        .warnings(false);
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        builder.flag("-std=c++14");
    }
    builder.compile("kaldi_fbank");
}
