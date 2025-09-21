use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

use crate::{
    client::{Error, LoginResponse, Method, Response, SupabaseClient, UserResponse},
    utils::decode_jwt,
};

impl SupabaseClient {
    pub async fn set_session(&mut self, access_token: &str, refresh_token: &str) {
        self.access_token = Some(access_token.to_owned());
        self.refresh_token = Some(refresh_token.to_owned());

        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut has_expired = true;
        if let Ok(claims) = decode_jwt(&access_token) {
            if let Some(exp) = claims.exp {
                has_expired = exp <= time_now;
            }
        }

        if !has_expired {
            return;
        }

        match self.refresh_token().await {
            Ok(res) => {
                let data = res.data.unwrap();
                self.access_token = Some(data.access_token);
                self.refresh_token = Some(data.refresh_token);
            }
            Err(e) => {
                dbg!(e);
            }
        }
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

    async fn refresh_token(&self) -> Result<Response<LoginResponse>, Error> {
        let res = self
            .request(
                Method::POST,
                "auth/v1/token?grant_type=refresh_token",
                json!({
                    "refresh_token": self.refresh_token.clone().unwrap()
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
