import unittest
from github_integration import create_github_issue

class TestGitHubIntegration(unittest.TestCase):
    def test_create_github_issue(self):
        result = create_github_issue('username/repo', 'Test Issue', 'This is a test issue')
        self.assertIsNotNone(result)

if __name__ == '__main__':
        unittest.main()
