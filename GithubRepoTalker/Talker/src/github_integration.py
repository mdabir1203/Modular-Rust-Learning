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
