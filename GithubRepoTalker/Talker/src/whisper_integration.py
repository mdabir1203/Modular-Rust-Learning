import whisper

def convert_speech_to_text(audio_file_path):
    model = whisper.load_model("base")

    # Load audio and pad/trim it to fit 30 seconds
    audio = whisper.load_audio(audio_file_path)
    audio = whisper.pad_or_trim(audio)

    # Make log-Mel spectrogram and move to the same device as the model
    mel = whisper.log_mel_spectrogram(audio).to(model.device)

    # Detect the spoken language
    _, probs = model.detect_language(mel)
    detected_language = max(probs, key=probs.get)
    print(f"Detected language: {detected_language}")

    # Decode the audio
    options = whisper.DecodingOptions()
    result = whisper.decode(model, mel, options)

    # Return the recognized text
    return result.text

# Example usage
if __name__ == "__main__":
    audio_file_path = "path/to/your/audio.mp3"
    text = convert_speech_to_text(audio_file_path)
    print(f"Recognized text: {text}")
