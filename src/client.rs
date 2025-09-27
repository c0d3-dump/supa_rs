use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SupabaseClient {
    pub base_url: String,
    pub api_key: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

pub enum Method {
    GET,
    POST,
    DELETE,
    PATCH,
    PUT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u16,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: Option<String>,
    #[serde(rename = "statusCode")]
    pub status_code: Option<String>,
    pub error_code: Option<String>,
    pub error: Option<String>,
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
    pub fn new() -> Self {
        Self {
            base_url: "".to_string(),
            api_key: "".to_string(),
            access_token: None,
            refresh_token: None,
        }
    }

    pub fn load_env(self) -> Self {
        dotenv::dotenv().ok();

        Self {
            base_url: std::env::var("SUPABASE_URL").expect("require valid SUPABASE_URL"),
            api_key: std::env::var("SUPABASE_KEY").expect("require vaid SUPABASE_KEY"),
            access_token: None,
            refresh_token: None,
        }
    }

    pub fn base_url(self, url: &str) -> Self {
        Self {
            base_url: url.to_string(),
            api_key: self.api_key,
            access_token: None,
            refresh_token: None,
        }
    }

    pub fn api_key(self, key: &str) -> Self {
        Self {
            base_url: self.base_url,
            api_key: key.to_string(),
            access_token: None,
            refresh_token: None,
        }
    }
}

// base requests
impl SupabaseClient {
    pub async fn request(
        &self,
        method: Method,
        path: &str,
        body: serde_json::Value,
        access_token: Option<&str>,
        form: Option<reqwest::multipart::Form>,
    ) -> Result<Response<String>, Error> {
        let url = format!("{}/{}", self.base_url, path);

        let client = reqwest::Client::new();
        let builder = match method {
            Method::GET => client.get(&url),
            Method::POST => client.post(&url),
            Method::DELETE => client.delete(&url),
            Method::PATCH => client.patch(&url),
            Method::PUT => client.put(&url),
        };

        let prefer_return = if url.contains("select=") {
            "return=representation"
        } else {
            "return=minimal"
        };

        let builder = if access_token.unwrap() != "" {
            builder
                .header("apikey", self.api_key.clone())
                .header(
                    "Authorization",
                    format!("Bearer {}", access_token.unwrap()),
                )
        } else {
            builder
                .header("apikey", self.api_key.clone())
        };

        let builder = if let Some(form) = form {
            builder
                .multipart(form)
        } else {
            builder
                .header("Content-Type", "application/json")
                .header("Prefer", prefer_return)
                .json(&body)
        };

        let res = builder.send().await.map_err(|e| Error {
            code: Some(
                e.status()
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                    .as_str()
                    .to_string(),
            ),
            msg: Some(e.to_string()),
            error_code: None,
            details: None,
            message: None,
            error: None,
            status_code: None,
        })?;

        let status = res.status();
        let txt = res.text().await.unwrap_or("".to_string());

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
                code: Some(status.as_str().to_string()),
                error_code: res.error_code,
                msg: res.msg,
                message: res.message,
                details: res.details,
                error: res.error,
                status_code: None,
            }),
            Err(_) => Err(Error {
                code: Some(status.as_str().to_string()),
                msg: None,
                error_code: None,
                details: None,
                message: None,
                error: None,
                status_code: None,
            }),
        }
    }

    pub async fn request_bytes(
        &self,
        path: &str,
        access_token: Option<&str>,
    ) -> Result<Response<Vec<u8>>, Error> {
        let url = format!("{}/{}", self.base_url, path);

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .header("apikey", self.api_key.clone())
            .header(
                "Authorization",
                format!("Bearer {}", access_token.unwrap_or("")),
            )
            .send()
            .await
            .map_err(|e| Error {
                code: Some(
                    e.status()
                        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                        .as_str()
                        .to_string(),
                ),
                msg: Some(e.to_string()),
                error_code: None,
                details: None,
                message: None,
                error: None,
                status_code: None,
            })?;

        let status = res.status();
        let bytes = res.bytes().await.unwrap_or(vec![].into());

        if status == StatusCode::OK
            || status == StatusCode::CREATED
            || status == StatusCode::NO_CONTENT
        {
            return Ok(Response {
                code: status.as_u16(),
                data: Some(bytes.to_vec()),
            });
        }

        Err(Error {
            code: Some(status.as_str().to_string()),
            error_code: None,
            error: None,
            msg: None,
            message: None,
            details: None,
            status_code: None,
        })
    }
}
