import unittest
from whisper_integration import convert_speech_to_text

class TestWhisperIntegration(unittest.TestCase):
    def test_convert_speech_to_text(self):
        result = convert_speech_to_text('path/to/test/audio/file')
        self.assertIsInstance(result, str)

if __name__ == '__main__':
    unittest.main()
