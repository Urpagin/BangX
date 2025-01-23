import pyaudio

p = pyaudio.PyAudio()
device_count = p.get_device_count()
print("Available audio devices:")
for i in range(device_count):
    info = p.get_device_info_by_index(i)
    print(
        f"Index {i}: {info.get('name')} "
        f"(inputs={info.get('maxInputChannels')}, "
        f"outputs={info.get('maxOutputChannels')})"
)

