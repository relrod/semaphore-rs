use reqwest::{Client, header};

pub mod authentication;

use authentication::AuthInfo;

pub struct Semaphore {
    pub base_url: String,
    pub client: Client,
    pub auth: AuthInfo,
}

impl Semaphore {
    pub fn new(base_url: &str, auth: AuthInfo) -> Self {
        let mut headers = header::HeaderMap::new();
        match &auth {
            AuthInfo::Token(token) => {
                headers.insert(
                    "Authorization",
                    header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap());
            }
            AuthInfo::SessionCookie(cookie) => {
                headers.insert(
                    "Cookie",
                    header::HeaderValue::from_str(&format!("semaphore={}", cookie)).unwrap());
            }
            _ => {}
        }
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            base_url: base_url.to_string(),
            client,
            auth,
        }
    }

    pub async fn get_raw_json(&self, endpoint: &str) -> String {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await.unwrap();
        response.text().await.unwrap()
    }
}
