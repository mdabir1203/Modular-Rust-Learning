#!/bin/bash

# Project root directory
PROJECT_ROOT="Talker"

# Create directory structure
mkdir -p $PROJECT_ROOT/src
mkdir -p $PROJECT_ROOT/tests

# Create files
touch $PROJECT_ROOT/src/main.py
touch $PROJECT_ROOT/src/whisper_integration.py
touch $PROJECT_ROOT/src/llama3_integration.py
touch $PROJECT_ROOT/src/github_integration.py
touch $PROJECT_ROOT/tests/test_whisper.py
touch $PROJECT_ROOT/tests/test_llama3.py
touch $PROJECT_ROOT/tests/test_github.py
touch $PROJECT_ROOT/requirements.txt
touch $PROJECT_ROOT/README.md
touch $PROJECT_ROOT/LICENSE

echo "Project structure created successfully."
