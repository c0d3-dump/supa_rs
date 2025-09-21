use serde_json::json;

use crate::client::{Error, LoginResponse, Method, Response, SupabaseClient, UserResponse};

impl SupabaseClient {
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
            None,
        )
        .await
    }

    pub async fn anonymous_login(&self) -> Result<Response<LoginResponse>, Error> {
        let res = self
            .request(Method::POST, "auth/v1/signup", json!({}), None, None)
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn email_login(
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
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn logout(&self) -> Result<Response<String>, Error> {
        self.request(
            Method::POST,
            "auth/v1/logout",
            json!({}),
            Some(&self.access_token.clone().unwrap_or("".to_owned())),
            None,
        )
        .await
    }

    pub async fn user(&self) -> Result<Response<UserResponse>, Error> {
        let res = self
            .request(
                Method::GET,
                "auth/v1/user",
                json!({}),
                Some(&self.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }
}
