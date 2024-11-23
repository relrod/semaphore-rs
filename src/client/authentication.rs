use super::Semaphore;

use serde::Deserialize;

#[cfg(feature = "tabled")]
use tabled::Tabled;

/// Describes how to authenticate to the Semaphore UI instance.
pub enum AuthInfo {
    /// If the user already has a token, this can be used to authenticate.
    Token(String),
    /// If the instance allows password authentication, this can be used to
    /// obtain a session which is used for further requests.
    Password(String, String),
    /// If a session has already been obtained the session cookie can be used to
    /// authenticate.
    SessionCookie(String),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tabled", derive(Tabled))]
pub struct OIDCProvider {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
//#[cfg_attr(feature = "tabled", derive(Tabled))]
pub struct LoginMetadata {
    pub oidc_providers: Vec<OIDCProvider>,
    pub login_with_password: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "tabled", derive(Tabled))]
pub struct Token {
    pub id: String,
    pub created: String,
    pub expired: bool,
    pub user_id: u64,
}

impl Semaphore {
    /// If authenticating with a token or pre-existing session, this method
    /// does nothing.
    ///
    /// If given a username/password combination, tries to exchange it for a
    /// session.
    pub async fn get_session(&self) -> Result<Option<String>, reqwest::Error> {
        match &self.auth {
            AuthInfo::Token(_) => Ok(None),
            AuthInfo::SessionCookie(_) => Ok(None),
            AuthInfo::Password(username, password) => {
                let url = format!("{}/api/auth/login", self.base_url);
                let response = self.client.post(&url)
                    .json(&serde_json::json!({
                        "auth": username,
                        "password": password,
                    }))
                    .send()
                    .await?;

                response.error_for_status_ref()?;
                let session_cookie = response.cookies().find(|c| c.name() == "semaphore").unwrap();
                Ok(Some(session_cookie.value().to_string()))
            }
        }
    }

    /// Returns login metadata for the instance, such as enabled OIDC
    /// providers and whether password authentication is enabled.
    pub fn login_metadata(&self) {

    }

    /// Returns tokens associated with the logged in user.
    pub async fn get_tokens(&self) -> Result<Vec<Token>, reqwest::Error> {
        let url = format!("{}/api/user/tokens", self.base_url);
        let response = self.client.get(&url).send().await?;
        response.error_for_status_ref()?;
        response.json().await
    }

    /// Creates a new token for the logged in user.
    pub async fn create_token(&self) -> Result<Token, reqwest::Error> {
        let url = format!("{}/api/user/tokens", self.base_url);
        let response = self.client.post(&url).send().await?;
        response.error_for_status_ref()?;
        response.json().await
    }
}
