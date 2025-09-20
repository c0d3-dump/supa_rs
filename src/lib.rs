use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone)]
pub struct SupabaseClient {
    pub base_url: String,
    pub api_key: String,
    access_token: Option<String>,
}

pub enum Method {
    GET,
    POST,
    DELETE,
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
            Method::GET => client.get(url),
            Method::POST => client.post(url),
            Method::DELETE => client.delete(url),
        };

        let res = builder
            .header("apikey", self.api_key.clone())
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", access_token.unwrap_or("")),
            )
            .header("Prefer", "return=minimal")
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

impl SupabaseClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            access_token: None,
        }
    }

    pub fn set_session(&mut self, access_token: &str) {
        self.access_token = Some(access_token.to_owned());
    }

    pub async fn signup(&self, email: &str, password: &str) -> Result<Response<String>, Error> {
        self.request(
            Method::POST,
            "auth/v1/signup",
            json!({
                "email": email,
                "password": password
            }),
            None,
        )
        .await
    }

    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Response<LoginResponse>, Error> {
        let res = self
            .request(
                Method::POST,
                "auth/v1/token?grant_type=password",
                json!({
                    "email": email,
                    "password": password
                }),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn logout(&self, access_token: &str) -> Result<Response<String>, Error> {
        self.request(
            Method::POST,
            "auth/v1/logout",
            json!({}),
            Some(access_token),
        )
        .await
    }

    pub async fn user(&self, access_token: &str) -> Result<Response<UserResponse>, Error> {
        let res = self
            .request(Method::GET, "auth/v1/user", json!({}), Some(access_token))
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TableSchema {
    pub client: SupabaseClient,
    pub name: String,
    pub select: Option<String>,
    pub insert: Option<serde_json::Value>,
    pub filters: Vec<(String, String, Value)>,
}

impl SupabaseClient {
    pub fn from(&self, name: &str) -> TableSchema {
        TableSchema {
            client: self.clone(),
            name: name.to_string(),
            select: None,
            filters: vec![],
            insert: None,
        }
    }
}

impl TableSchema {
    pub fn select(&mut self, columns: &str) -> TableSchema {
        self.select = Some(columns.to_owned());
        self.clone()
    }

    pub fn insert(&mut self, data: serde_json::Value) -> TableSchema {
        self.insert = Some(data);
        self.clone()
    }

    pub async fn execute(&self) -> Result<Response<serde_json::Value>, Error> {
        let mut url = format!("rest/v1/{}", self.name);
        if self.select.is_some() {
            url += &format!("?select={}", &self.select.clone().unwrap());
        }

        let method = if self.insert.is_some() {
            Method::POST
        } else {
            Method::GET
        };

        let mut body = json!({});
        match &self.insert {
            Some(ins) => {
                body = ins.clone();
            }
            None => {}
        }

        let res = self
            .client
            .request(
                method,
                &url,
                body,
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
            )
            .await?;

        dbg!(&res);

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::SupabaseClient;

    const SUPABASE_URL: &str = "";
    const SUPABASE_KEY: &str = "";

    #[tokio::test]
    async fn signup() {
        let client = SupabaseClient::new(SUPABASE_URL.to_string(), SUPABASE_KEY.to_string());
        let res = client.signup("test1@gmail.com", "12345678").await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn login() {
        let client = SupabaseClient::new(SUPABASE_URL.to_string(), SUPABASE_KEY.to_string());
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn user() {
        let client = SupabaseClient::new(SUPABASE_URL.to_string(), SUPABASE_KEY.to_string());
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let res = client.user(&res.unwrap().data.unwrap().access_token).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn logout() {
        let client = SupabaseClient::new(SUPABASE_URL.to_string(), SUPABASE_KEY.to_string());
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let res = client
            .logout(&res.unwrap().data.unwrap().access_token)
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_select() {
        let mut client = SupabaseClient::new(SUPABASE_URL.to_string(), SUPABASE_KEY.to_string());
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.from("test").select("*").execute().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_insert() {
        let mut client = SupabaseClient::new(SUPABASE_URL.to_string(), SUPABASE_KEY.to_string());
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client
            .from("test")
            .insert(json!({
                "name": "test5"
            }))
            .select("*")
            .execute()
            .await;

        println!("{:?}", res);
    }
}
