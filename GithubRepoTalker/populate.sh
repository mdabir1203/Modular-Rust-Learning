#!/bin/bash

# Project root directory
PROJECT_ROOT="Talker"

# main.py
cat <<EOL > $PROJECT_ROOT/src/main.py
from whisper_integration import convert_speech_to_text
from llama3_integration import generate_text
from github_integration import create_github_issue, create_github_pull_request

def main():
    audio_file_path = 'path/to/audio/file'
    repo_name = 'username/repo'

    # Step 1: Convert Speech to Text
    text = convert_speech_to_text(audio_file_path)
    print(f"Transcribed Text: {text}")

    # Step 2: Generate Response using Llama3
    response_text = generate_text(text)
    print(f"Generated Text: {response_text}")

    # Step 3: Create GitHub Issue
    issue = create_github_issue(repo_name, "Generated Issue", response_text)
    print(f"Issue created: {issue.html_url}")

    # Step 4: (Optional) Create GitHub Pull Request
    pr = create_github_pull_request(repo_name, "Generated PR", response_text, 'feature-branch')
    print(f"Pull Request created: {pr.html_url}")

if __name__ == "__main__":
    main()
EOL

# whisper_integration.py
cat <<EOL > $PROJECT_ROOT/src/whisper_integration.py
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
EOL

# llama3_integration.py
cat <<EOL > $PROJECT_ROOT/src/llama3_integration.py
import openai

def generate_text(prompt):
    openai.api_key = 'YOUR_LLAMA3_API_KEY'
    response = openai.Completion.create(
        model="text-davinci-003",
        prompt=prompt,
        max_tokens=150
    )
    return response['choices'][0]['text'].strip()
EOL

# github_integration.py
cat <<EOL > $PROJECT_ROOT/src/github_integration.py
from github import Github

def create_github_issue(repo_name, title, body):
    g = Github("YOUR_GITHUB_ACCESS_TOKEN")
    repo = g.get_repo(repo_name)
    issue = repo.create_issue(
        title=title,
        body=body
    )
    return issue

def create_github_pull_request(repo_name, title, body, head, base='main'):
    g = Github("YOUR_GITHUB_ACCESS_TOKEN")
    repo = g.get_repo(repo_name)
    pr = repo.create_pull(
        title=title,
        body=body,
        head=head,
        base=base
    )
    return pr
EOL

# test_whisper.py
cat <<EOL > $PROJECT_ROOT/tests/test_whisper.py
import unittest
from whisper_integration import convert_speech_to_text

class TestWhisperIntegration(unittest.TestCase):
    def test_convert_speech_to_text(self):
        result = convert_speech_to_text('path/to/test/audio/file')
        self.assertIsInstance(result, str)

if __name__ == '__main__':
    unittest.main()
EOL

# test_llama3.py
cat <<EOL > $PROJECT_ROOT/tests/test_llama3.py
import unittest
from llama3_integration import generate_text

class TestLlama3Integration(unittest.TestCase):
    def test_generate_text(self):
        result = generate_text('Hello, how are you?')
        self.assertIsInstance(result, str)

if __name__ == '__main__':
    unittest.main()
EOL

# test_github.py
cat <<EOL > $PROJECT_ROOT/tests/test_github.py
import unittest
from github_integration import create_github_issue

class TestGitHubIntegration(unittest.TestCase):
    def test_create_github_issue(self):
        result = create_github_issue('username/repo', 'Test Issue', 'This is a test issue')
        self.assertIsNotNone(result)

if __name__ == '__main__':
        unittest.main()
EOL

# requirements.txt
cat <<EOL > $PROJECT_ROOT/requirements.txt
openai
whisper
PyGithub
EOL

# README.md
cat <<EOL > $PROJECT_ROOT/README.md
# Talker

Talker is an open-source project that integrates Whisper API, Llama3 API, and GitHub API to provide speech-to-text conversion, text generation, and GitHub issue and pull request creation functionalities.

## Features

- Convert speech to text using Whisper API
- Generate text using Llama3 API
- Create GitHub issues and pull requests using GitHub API

## Installation

\`\`\`bash
pip install -r requirements.txt
\`\`\`

## Usage

\`\`\`bash
python src/main.py
\`\`\`

## License

This project is licensed under the MIT License.
EOL

# LICENSE
cat <<EOL > $PROJECT_ROOT/LICENSE
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOL

echo "Files populated successfully."
