import pyaudio
import numpy as np

p = pyaudio.PyAudio()
DEVICE_INDEX = 7  # Replace with your output device index

stream = p.open(format=pyaudio.paFloat32,
                channels=1,
                rate=44100,
                output=True,
                output_device_index=DEVICE_INDEX)

# Generate a 440 Hz sine wave for testing
print("Playing a test tone...")
fs = 44100  # Sampling frequency
duration = 2.0  # seconds
frequency = 440.0  # Hz
samples = (np.sin(2 * np.pi * np.arange(fs * duration) * frequency / fs)).astype(np.float32)

stream.write(samples.tobytes())
stream.stop_stream()
stream.close()
p.terminate()

