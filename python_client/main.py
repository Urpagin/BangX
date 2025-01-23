import socket
import pyaudio
import numpy as np

SAMPLE_RATE = 44100          # Update if your Rust code uses a different rate
CHANNELS = 1                 # Update if your Rust code is sending mono/stereo
FORMAT = pyaudio.paFloat32   # Rust code sends f32 samples

# Replace this with the device index you found
DEVICE_INDEX = 7  # Example: if your "FiiO K3" was index 2

def main():
    # Create a UDP socket and bind to 127.0.0.1:8888
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(("127.0.0.1", 8888))
    
    p = pyaudio.PyAudio()
    
    # Open the PyAudio stream using the specified device index
    stream = p.open(format=FORMAT,
                    channels=CHANNELS,
                    rate=SAMPLE_RATE,
                    output=True,
                    output_device_index=DEVICE_INDEX)
    
    print("Listening for UDP audio on 127.0.0.1:8888...")
    
    try:
        while True:
            data, addr = sock.recvfrom(6000)
            if not data:
                break
            stream.write(data)
    except KeyboardInterrupt:
        pass
    finally:
        stream.stop_stream()
        stream.close()
        p.terminate()
        sock.close()

if __name__ == "__main__":
    main()

