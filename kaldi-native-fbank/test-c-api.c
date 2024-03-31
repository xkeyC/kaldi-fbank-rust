#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>

#include "kaldi-native-fbank/c-api/c-api.h"

#define FRAME_SIZE 400

int main() {
  // Test OnlineFbankNew and OnlineFbankFree
  OnlineFbank *fbank = OnlineFbankNew(16000.0);
  assert(fbank != NULL);

  // Test OnlineFbankDim
  int dim = OnlineFbankDim(fbank);
  assert(dim > 0);  // replace with the expected dimension if known

  // Test OnlineFbankAcceptWaveform with random data
  float waveform[FRAME_SIZE];
  for (int i = 0; i < FRAME_SIZE; i++) {
    waveform[i] = (float)rand() / RAND_MAX;  // random float between 0 and 1
  }
  OnlineFbankAcceptWaveform(fbank, 16000.0, waveform, FRAME_SIZE);

  // Test OnlineFbankGetFrame
  const float *frame = OnlineFbankGetFrame(fbank, 0);
  assert(frame != NULL);  // we don't check the contents of the frame

  OnlineFbankFree(fbank);

  return 0;
}
