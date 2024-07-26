use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use reqwest::Error;
use crate::model::RepoInfo;


async fn fetch_repo_info(owner: &str, repo: &str) -> Result<RepoInfo, Error> {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "actix-web")
        .send()
        .await?
        .json::<RepoInfo>()
        .await?;

    Ok(response)
}

#[get("/repo/{owner}/{repo}")]
async fn get_repo_info(req: HttpRequest, path: web::Path<(String, String)>) -> impl Responder {
    let (owner, repo) = path.into_inner();
    let headers = req.headers();
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Extract the token part
                // Use the token as needed
                match fetch_repo_info(&owner, &repo).await {
                    Ok(repo_info) => HttpResponse::Ok().json(repo_info),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid token format")
            }
        } else {
            HttpResponse::Unauthorized().body("Invalid token format")
        }
    } else {
        HttpResponse::Unauthorized().body("No authorization header")
    }
}