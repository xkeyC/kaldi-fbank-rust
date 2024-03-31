#include "kaldi-native-fbank/c-api/c-api.h"

#include <cstdint>

#include "kaldi-native-fbank/csrc/feature-fbank.h"
#include "kaldi-native-fbank/csrc/online-feature.h"

struct OnlineFbank {
  knf::OnlineGenericBaseFeature<knf::FbankComputer> *instance;
};

OnlineFbank *OnlineFbankNew(float sample_rate) {
  OnlineFbank *obj = new OnlineFbank;
  // TODO: Make customizable
  knf::FbankOptions opts;
  opts.frame_opts.samp_freq = sample_rate;

  obj->instance = new knf::OnlineGenericBaseFeature<knf::FbankComputer>(opts);
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
