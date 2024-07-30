fn main() {
    cc::Build::new()
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
        .warnings(false)
        .compile("kaldi_fbank");
}
