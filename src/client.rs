use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SupabaseClient {
    pub base_url: String,
    pub api_key: String,
    pub access_token: Option<String>,
}

pub enum Method {
    GET,
    POST,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u16,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub error_code: Option<String>,
    pub msg: Option<String>,
    pub message: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_at: i64,
    pub expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub is_anonymous: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl SupabaseClient {
    pub fn new(mut base_url: Option<String>, mut api_key: Option<String>) -> Self {
        dotenv::dotenv().ok();

        if base_url.is_none() && api_key.is_none() {
            base_url = Some(std::env::var("SUPABASE_URL").expect("require valid SUPABASE_URL"));
            api_key = Some(std::env::var("SUPABASE_KEY").expect("require vaid SUPABASE_KEY"));
        }

        Self {
            base_url: base_url.unwrap(),
            api_key: api_key.unwrap(),
            access_token: None,
        }
    }

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        body: serde_json::Value,
        access_token: Option<&str>,
    ) -> Result<Response<String>, Error> {
        let url = format!("{}/{}", self.base_url, path);

        let client = reqwest::Client::new();
        let builder = match method {
            Method::GET => client.get(&url),
            Method::POST => client.post(&url),
            Method::DELETE => client.delete(&url),
            Method::PATCH => client.patch(&url),
        };

        let prefer_return = if url.contains("select=") {
            "return=representation"
        } else {
            "return=minimal"
        };

        let res = builder
            .header("apikey", self.api_key.clone())
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", access_token.unwrap_or("")),
            )
            .header("Prefer", prefer_return)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error {
                code: e
                    .status()
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                    .as_str()
                    .to_string(),
                msg: Some(e.to_string()),
                error_code: None,
                details: None,
                message: None,
            })?;

        dbg!(&res);

        let status = res.status();
        let txt = res.text().await.unwrap();

        if status == StatusCode::OK
            || status == StatusCode::CREATED
            || status == StatusCode::NO_CONTENT
        {
            return Ok(Response {
                code: status.as_u16(),
                data: Some(txt),
            });
        }

        match serde_json::from_str::<Error>(&txt) {
            Ok(res) => Err(Error {
                code: status.as_str().to_string(),
                error_code: res.error_code,
                msg: res.msg,
                message: res.message,
                details: res.details,
            }),
            Err(_) => Err(Error {
                code: status.as_str().to_string(),
                msg: None,
                error_code: None,
                details: None,
                message: None,
            }),
        }
    }
}
