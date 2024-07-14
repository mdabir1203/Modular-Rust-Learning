import unittest
from llama3_integration import generate_text

class TestLlama3Integration(unittest.TestCase):
    def test_generate_text(self):
        result = generate_text('Hello, how are you?')
        self.assertIsInstance(result, str)

if __name__ == '__main__':
    unittest.main()
