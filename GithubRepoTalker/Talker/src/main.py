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
