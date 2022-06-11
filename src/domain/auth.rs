use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    #[serde(default)]
    pub tokens: Vec<Token>,
    #[serde(default)]
    pub passwords: Vec<UsernamePassword>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

impl Auth {
    pub fn is_valid<C: Credentials>(&self, cred: C) -> bool {
        if !self.enabled {
            true
        } else {
            cred.is_valid(self)
        }
    }

    pub fn get_token(&self, value: &str) -> Option<&Token> {
        self.tokens.iter().find(|t| t.value == value)
    }
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernamePassword {
    pub username: String,
    pub password: String,
}

pub trait Credentials {
    fn is_valid(&self, auth: &Auth) -> bool;
}

pub struct EmptyCredentials;

impl Credentials for EmptyCredentials {
    fn is_valid(&self, _: &Auth) -> bool {
        false
    }
}

#[derive(Deserialize)]
pub struct PasswordCredentials {
    pub username: String,
    pub password: String,
}

impl Credentials for PasswordCredentials {
    fn is_valid(&self, auth: &Auth) -> bool {
        auth.passwords
            .iter()
            .find(|p| p.username == self.username && p.password == self.password)
            .is_some()
    }
}

impl<U: Into<String>, P: Into<String>> From<(U, P)> for PasswordCredentials {
    fn from((username, password): (U, P)) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

#[derive(Deserialize)]
pub struct TokenCredentials {
    token: Option<String>,
}

impl Default for TokenCredentials {
    fn default() -> Self {
        Self {
            token: Option::None,
        }
    }
}

impl Credentials for TokenCredentials {
    fn is_valid(&self, auth: &Auth) -> bool {
        self.token
            .as_ref()
            .map(|t| auth.get_token(t).is_some())
            .unwrap_or(false)
    }
}

impl<S: Into<String>> From<S> for TokenCredentials {
    fn from(val: S) -> Self {
        Self {
            token: Some(val.into()),
        }
    }
}

impl TokenCredentials {
    pub fn from_opt<S: Into<String>>(token: Option<S>) -> Self {
        token.map(|t| Self::from(t)).unwrap_or_default()
    }

    pub fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::auth::{Auth, EmptyCredentials, Token, TokenCredentials, UsernamePassword, PasswordCredentials};

    #[test]
    fn no_auth_required_for_disabled_auth() {
        let auth = Auth {
            tokens: Default::default(),
            passwords: Default::default(),
            enabled: false,
        };
        assert!(
            auth.is_valid(EmptyCredentials),
            "Should be valid for empty credentials"
        );

        assert!(
            auth.is_valid(TokenCredentials::empty()),
            "Should be valid for empty token"
        );

        assert!(
            auth.is_valid(TokenCredentials::default()),
            "Should be valid for empty default token"
        );

        assert!(
            auth.is_valid(TokenCredentials::from("Token")),
            "Should be valid for random token"
        );
    }

    #[test]
    fn auth_required_for_enabled_auth_tokens_empty() {
        let auth = Auth {
            tokens: Default::default(),
            passwords: Default::default(),
            enabled: true,
        };
        assert!(
            !auth.is_valid(EmptyCredentials {}),
            "Should be invalid for no token"
        );

        assert!(
            !auth.is_valid(TokenCredentials::from("Token")),
            "Should be invalid for random token"
        );
    }

    #[test]
    fn auth_required_for_enabled_auth_tokens_single() {
        let auth = Auth {
            tokens: vec![Token {
                name: "random test token".into(),
                value: "test-token".into(),
            }],
            passwords: Default::default(),
            enabled: true,
        };
        assert!(
            !auth.is_valid(EmptyCredentials),
            "Should be invalid for no token"
        );

        assert!(
            !auth.is_valid(TokenCredentials::from("invalid-token")),
            "Should be invalid for invalid token"
        );

        assert!(
            auth.is_valid(TokenCredentials::from("test-token")),
            "Should be valid for valid token"
        );
    }

    #[test]
    fn auth_required_for_user_password() {
        let auth = Auth {
            tokens: Default::default(),
            passwords: vec![UsernamePassword {
                username: "test_user".into(),
                password: "test_password".into(),
            }],
            enabled: true,
        };

        assert!(
            !auth.is_valid(PasswordCredentials::from(("test_user", "wrong_password"))),
            "Should be invalid for invalid password"
        );

        assert!(
            !auth.is_valid(PasswordCredentials::from(("wrong_user", "test_password"))),
            "Should be invalid for invalid username"
        );

        assert!(
            auth.is_valid(PasswordCredentials::from(("test_user", "test_password"))),
            "Should be valid for valid token"
        );
    }
}
