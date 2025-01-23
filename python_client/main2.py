import socket
import pyaudio
import numpy as np

UDP_IP = "127.0.0.1"
UDP_PORT = 8888

# Configure audio stream parameters
SAMPLE_RATE = 48000       # Match whatever rate your sender uses
CHANNELS = 1              # Match your sender's channel count
FORMAT = pyaudio.paFloat32
CHUNK_SIZE = 1024         # Buffer size for PyAudio

def main():
    # 1. Create a UDP socket and bind to the given IP/port
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((UDP_IP, UDP_PORT))
    print(f"Listening on {UDP_IP}:{UDP_PORT} for f32 samples...")

    # 2. Initialize PyAudio
    p = pyaudio.PyAudio()

    # 3. Open an audio output stream
    stream = p.open(
        format=FORMAT,
        channels=CHANNELS,
        rate=SAMPLE_RATE,
        output=True,
        frames_per_buffer=CHUNK_SIZE
    )

    try:
        while True:
            # 4. Receive datagrams (up to 4096 bytes)
            data, _ = sock.recvfrom(4096)

            # 5. Interpret the bytes as float32 samples
            samples = np.frombuffer(data, dtype=np.float32)

            # 6. Play the samples
            stream.write(samples.tobytes())

    except KeyboardInterrupt:
        pass
    finally:
        # Clean up
        stream.stop_stream()
        stream.close()
        p.terminate()
        sock.close()
        print("\nFinished.")

if __name__ == "__main__":
    main()

