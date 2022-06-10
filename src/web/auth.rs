use actix_web::HttpRequest;
use anyhow::bail;

use crate::domain::project;

use super::server::RequestCtx;

pub fn require_project_auth(ctx: &RequestCtx, req: &HttpRequest, name: &str) -> anyhow::Result<()> {
    let token = extract_token(req);

    match ctx.get_project(name) {
        Some(proj) => match is_token_valid(&proj.auth, token) {
            true => Ok(()),
            false => bail!("Token invalid"),
        },
        None => bail!("Project not found!"),
    }
}

fn is_token_valid(auth: &project::Authorization, token: Option<String>) -> bool {
    if !auth.enabled {
        return true;
    }

    match token {
        Some(tok) => auth.tokens.iter().find(|t| t.value == tok).is_some(),
        None => false,
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

#[cfg(test)]
mod tests {
    use crate::domain::project::{self, Token};

    use super::is_token_valid;

    #[test]
    fn no_auth_required_for_disabled_auth() {
        let auth = project::Authorization {
            tokens: vec![],
            enabled: false,
        };
        assert!(
            is_token_valid(&auth, Option::None),
            "Should be valid for no token"
        );

        assert!(
            is_token_valid(&auth, Option::Some("Token".into())),
            "Should be valid for random token"
        );
    }

    #[test]
    fn auth_required_for_enabled_auth_tokens_empty() {
        let auth = project::Authorization {
            tokens: vec![],
            enabled: true,
        };
        assert!(
            !is_token_valid(&auth, Option::None),
            "Should be invalid for no token"
        );

        assert!(
            !is_token_valid(&auth, Option::Some("Token".into())),
            "Should be invalid for random token"
        );
    }

    #[test]
    fn auth_required_for_enabled_auth_tokens_single() {
        let auth = project::Authorization {
            tokens: vec![Token {
                name: "random test token".into(),
                value: "test-token".into(),
            }],
            enabled: true,
        };
        assert!(
            !is_token_valid(&auth, Option::None),
            "Should be invalid for no token"
        );

        assert!(
            !is_token_valid(&auth, Option::Some("invalid-token".into())),
            "Should be invalid for invalid token"
        );

        assert!(
            is_token_valid(&auth, Option::Some("test-token".into())),
            "Should be valid for valid token"
        );
    }
}
