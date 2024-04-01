#ifndef KALDI_NATIVE_C_API_C_API_H_
#define KALDI_NATIVE_C_API_C_API_H_

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct FrameExtractionOptions {
  float samp_freq;
  float frame_shift_ms;
  float frame_length_ms;
  float dither;
  float preemph_coeff;
  bool remove_dc_offset;
  const char *window_type;
  bool round_to_power_of_two;
  float blackman_coeff;
  bool snip_edges;
} FrameExtractionOptions;

typedef struct MelBanksOptions {
  int32_t num_bins;
  float low_freq;
  float high_freq;
  float vtln_low;
  float vtln_high;
  bool debug_mel;
  bool htk_mode;
  bool is_librosa;
  const char *norm;
} MelBanksOptions;

typedef struct FbankOptions {
  FrameExtractionOptions frame_opts;
  MelBanksOptions mel_opts;

  bool use_energy;
  float energy_floor;
  bool raw_energy;
  bool htk_compat;
  bool use_log_fbank;
  bool use_power;
} FbankOptions;

/// Holds the instance of the OnlineGenericBaseFeature class
typedef struct OnlineFbank OnlineFbank;

/// Create a new OnlineFbank object with the given sample rate
// TODO: Consider adding more options as a struct
// TODO: Return error code?
OnlineFbank *OnlineFbankNew(FbankOptions opts);

/// Free the memory allocated for the OnlineFbank object
void OnlineFbankFree(OnlineFbank *self);

/// Get the dimension of the feature vectors produced by the OnlineFbank object
int OnlineFbankDim(OnlineFbank *self);

/// Accept a waveform and add it to the buffer of the OnlineFbank object
void OnlineFbankAcceptWaveform(OnlineFbank *self, float sampling_rate,
                               const float *waveform, int32_t n);

/// Indicate that the input has finished and no more waveforms will be added
void OnlineFbankInputFinished(OnlineFbank *self);

/// Get the feature vector for a specific frame
const float *OnlineFbankGetFrame(OnlineFbank *self, int32_t frame);

/// Return the number of ready frames
int32_t OnlineFbankNumFramesReady(OnlineFbank *self);

/// Check if a specific frame is the last one
bool OnlineFbankIsLastFrame(OnlineFbank *self, int32_t frame);

/// Get the frame shift in seconds
float OnlineFbankFrameShiftInSeconds(OnlineFbank *self);

/// Discard the first n frames
void OnlineFbankPop(OnlineFbank *self, int32_t n);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif  // KALDI_NATIVE_C_API_C_API_H_
