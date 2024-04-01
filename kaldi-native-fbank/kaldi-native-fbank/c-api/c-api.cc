#include "kaldi-native-fbank/c-api/c-api.h"

#include <cstdint>

#include "kaldi-native-fbank/csrc/feature-fbank.h"
#include "kaldi-native-fbank/csrc/feature-window.h"
#include "kaldi-native-fbank/csrc/mel-computations.h"
#include "kaldi-native-fbank/csrc/online-feature.h"

struct OnlineFbank {
  knf::OnlineGenericBaseFeature<knf::FbankComputer> *instance;
};

knf::FbankOptions _OptionsFromC(FbankOptions copts) {
  knf::FbankOptions opts;

  opts.frame_opts.samp_freq = copts.frame_opts.samp_freq;
  opts.frame_opts.frame_shift_ms = copts.frame_opts.frame_shift_ms;
  opts.frame_opts.frame_length_ms = copts.frame_opts.frame_length_ms;
  opts.frame_opts.dither = copts.frame_opts.dither;
  opts.frame_opts.preemph_coeff = copts.frame_opts.preemph_coeff;
  opts.frame_opts.remove_dc_offset = copts.frame_opts.remove_dc_offset;
  opts.frame_opts.window_type = std::string(copts.frame_opts.window_type);
  opts.frame_opts.round_to_power_of_two =
      copts.frame_opts.round_to_power_of_two;
  opts.frame_opts.snip_edges = copts.frame_opts.snip_edges;

  opts.mel_opts.num_bins = copts.mel_opts.num_bins;
  opts.mel_opts.low_freq = copts.mel_opts.low_freq;
  opts.mel_opts.high_freq = copts.mel_opts.high_freq;
  opts.mel_opts.vtln_low = copts.mel_opts.vtln_low;
  opts.mel_opts.vtln_high = copts.mel_opts.vtln_high;
  opts.mel_opts.debug_mel = copts.mel_opts.debug_mel;
  opts.mel_opts.htk_mode = copts.mel_opts.htk_mode;
  opts.mel_opts.is_librosa = copts.mel_opts.is_librosa;
  opts.mel_opts.norm = std::string(copts.mel_opts.norm);

  opts.use_energy = copts.use_energy;
  opts.energy_floor = copts.energy_floor;
  opts.htk_compat = copts.htk_compat;
  opts.use_log_fbank = copts.use_log_fbank;
  opts.use_power = copts.use_power;

  return opts;
}

OnlineFbank *OnlineFbankNew(FbankOptions opts) {
  OnlineFbank *obj = new OnlineFbank;
  // TODO: Make customizable

  knf::FbankOptions opts_cpp = _OptionsFromC(opts);
  obj->instance =
      new knf::OnlineGenericBaseFeature<knf::FbankComputer>(opts_cpp);
  return obj;
}

void OnlineFbankFree(OnlineFbank *self) {
  delete self->instance;
  delete self;
}

int OnlineFbankDim(OnlineFbank *self) { return self->instance->Dim(); }

void OnlineFbankAcceptWaveform(OnlineFbank *self, float sampling_rate,
                               const float *waveform, int32_t n) {
  self->instance->AcceptWaveform(sampling_rate, waveform, n);
}

void OnlineFbankInputFinished(OnlineFbank *self) {
  self->instance->InputFinished();
}

const float *OnlineFbankGetFrame(OnlineFbank *self, int32_t frame) {
  return self->instance->GetFrame(frame);
}

int32_t OnlineFbankNumFramesReady(OnlineFbank *self) {
  return self->instance->NumFramesReady();
}

bool OnlineFbankIsLastFrame(OnlineFbank *self, int32_t frame) {
  return self->instance->IsLastFrame(frame);
}

float OnlineFbankFrameShiftInSeconds(OnlineFbank *self) {
  return self->instance->FrameShiftInSeconds();
}

void OnlineFbankPop(OnlineFbank *self, int32_t n) {
  return self->instance->Pop(n);
}
