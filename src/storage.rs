use crate::client::{Error, Method, Response, SupabaseClient};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketResponse {
    pub id: String,
    pub name: String,
    pub public: bool,
    pub file_size_limit: Option<i64>,
    pub allowed_mime_types: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub public: Option<bool>,
    pub file_size_limit: Option<i64>,
    pub allowed_mime_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUrlResponse {
    #[serde(rename = "signedURL")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResponse {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesResponse {
    pub id: String,
    pub name: String,
    pub updated_at: String,
    pub created_at: String,
    pub last_accessed_at: String,
    pub metadata: FileMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    #[serde(rename = "eTag")]
    pub e_tag: String,
    pub size: i64,
    pub mimetype: String,
    #[serde(rename = "cacheControl")]
    pub cache_control: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
}

#[derive(Debug, Clone)]
pub struct BucketSchema {
    pub client: SupabaseClient,
    pub name: String,
}

impl SupabaseClient {
    pub fn bucket(&self, name: &str) -> BucketSchema {
        BucketSchema {
            client: self.clone(),
            name: name.to_string(),
        }
    }

    pub async fn create_bucket(&self, name: &str, public: bool) -> Result<Response<String>, Error> {
        self.request(
            Method::POST,
            "storage/v1/bucket",
            json!({
                "id": name,
                "name": name,
                "public": public
            }),
            Some(&self.access_token.clone().unwrap_or("".to_owned())),
            None,
        )
        .await
    }

    pub async fn list_buckets(&self) -> Result<Response<Vec<BucketResponse>>, Error> {
        let res = self
            .request(
                Method::GET,
                "storage/v1/bucket",
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

impl BucketSchema {
    pub async fn get_bucket(&self) -> Result<Response<BucketResponse>, Error> {
        let res = self
            .client
            .request(
                Method::GET,
                &format!("storage/v1/bucket/{}", self.name),
                json!({}),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn update_bucket(
        &self,
        data: UpdateRequest,
    ) -> Result<Response<MessageResponse>, Error> {
        let res = self
            .client
            .request(
                Method::PUT,
                &format!("storage/v1/bucket/{}", self.name),
                serde_json::to_value(data).unwrap(),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn delete_bucket(&self) -> Result<Response<MessageResponse>, Error> {
        let res = self
            .client
            .request(
                Method::DELETE,
                &format!("storage/v1/bucket/{}", self.name),
                json!({}),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn upload(
        &self,
        file_path: &str,
        file_data: Vec<u8>,
        upsert: bool,
    ) -> Result<Response<FileResponse>, Error> {
        let form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(file_data)
                .file_name("filename")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        // TODO: upsert is not working
        let res = self
            .client
            .request(
                Method::POST,
                &format!(
                    "storage/v1/object/{}/{}?upsert={}",
                    self.name, file_path, upsert
                ),
                json!({}),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                Some(form),
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn get(&self, file_path: &str) -> Result<Response<Vec<u8>>, Error> {
        self.client
            .request_bytes(
                &format!("storage/v1/object/{}/{}", self.name, file_path),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
            )
            .await
    }

    pub async fn list(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Response<Vec<FilesResponse>>, Error> {
        let res = self
            .client
            .request(
                Method::POST,
                &format!("storage/v1/object/list/{}", self.name),
                json!({
                        "limit": limit,
                        "offset": offset,
                        "prefix": "",
                        "sortBy": {
                            "column": "name",
                            "order": "asc",
                        },
                }),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn search(
        &self,
        term: &str,
        limit: i32,
    ) -> Result<Response<Vec<FilesResponse>>, Error> {
        let res = self
            .client
            .request(
                Method::POST,
                &format!("storage/v1/object/list/{}", self.name),
                json!({
                    "search": term,
                    "limit": limit,
                    "prefix": ""
                }),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn delete(&self, file_path: &str) -> Result<Response<MessageResponse>, Error> {
        let res = self
            .client
            .request(
                Method::DELETE,
                &format!("storage/v1/object/{}/{}", self.name, file_path),
                json!({}),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn get_public_url(&self, file_path: &str) -> Result<Response<String>, Error> {
        Ok(Response {
            code: 200,
            data: format!(
                "{}/storage/v1/object/public/{}/{}",
                self.client.base_url, self.name, file_path
            )
            .into(),
        })
    }

    pub async fn get_signed_url(
        &self,
        file_path: &str,
        expires_in: i64,
    ) -> Result<Response<String>, Error> {
        let res = self
            .client
            .request(
                Method::POST,
                &format!("storage/v1/object/sign/{}/{}", self.name, file_path),
                json!({
                    "expiresIn": expires_in
                }),
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        let signed_url: SignedUrlResponse = serde_json::from_str(&res.data.unwrap()).unwrap();

        Ok(Response {
            code: res.code,
            data: Some(format!(
                "{}/storage/v1{}",
                self.client.base_url, signed_url.url
            )),
        })
    }
}
