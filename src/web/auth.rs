use actix_web::HttpRequest;
use anyhow::bail;

use crate::domain::auth::TokenCredentials;

use super::server::RequestCtx;

pub fn require_project_auth(ctx: &RequestCtx, req: &HttpRequest, name: &str) -> anyhow::Result<()> {
    let token = extract_token(req);
    let token_cred = TokenCredentials::from_opt(token);

    match ctx.get_project(name) {
        Some(proj) => match proj.auth.is_valid(token_cred) {
            true => Ok(()),
            false => bail!("Token invalid"),
        },
        None => bail!("Project not found!"),
    }
}


fn extract_token(req: &HttpRequest) -> Option<String> {
    let value = req
        .headers()
        .get("Authorization")
        .filter(|v| !v.is_empty())?;

    value
        .to_str()
        .ok() // Convert Result to Optional
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|v| v.to_string())
}
