#ifndef KALDI_NATIVE_C_API_C_API_H_
#define KALDI_NATIVE_C_API_C_API_H_

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Holds the instance of the OnlineGenericBaseFeature class
typedef struct OnlineFbank OnlineFbank;

/// Create a new OnlineFbank object with the given sample rate
// TODO: Consider adding more options as a struct
// TODO: Return error code?
OnlineFbank *OnlineFbankNew(float sample_rate);

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
