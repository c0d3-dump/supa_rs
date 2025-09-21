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

impl SupabaseClient {
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

    pub async fn get_bucket(&self, bucket_id: &str) -> Result<Response<BucketResponse>, Error> {
        let res = self
            .request(
                Method::GET,
                &format!("storage/v1/bucket/{}", bucket_id),
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

    pub async fn update_bucket(
        &self,
        bucket_id: &str,
        data: UpdateRequest,
    ) -> Result<Response<MessageResponse>, Error> {
        let res = self
            .request(
                Method::PUT,
                &format!("storage/v1/bucket/{}", bucket_id),
                serde_json::to_value(data).unwrap(),
                Some(&self.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn delete_bucket(&self, bucket_id: &str) -> Result<Response<MessageResponse>, Error> {
        let res = self
            .request(
                Method::DELETE,
                &format!("storage/v1/bucket/{}", bucket_id),
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

    pub async fn upload_file(
        &self,
        bucket_name: &str,
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
            .request(
                Method::POST,
                &format!(
                    "storage/v1/object/{}/{}?upsert={}",
                    bucket_name, file_path, upsert
                ),
                json!({}),
                Some(&self.access_token.clone().unwrap_or("".to_owned())),
                Some(form),
            )
            .await?;

        dbg!(&res);

        Ok(Response {
            code: res.code,
            data: serde_json::from_str(&res.data.unwrap()).unwrap(),
        })
    }

    pub async fn get_file(
        &self,
        bucket_name: &str,
        file_path: &str,
    ) -> Result<Response<Vec<u8>>, Error> {
        self.request_bytes(
            &format!("storage/v1/object/{}/{}", bucket_name, file_path),
            Some(&self.access_token.clone().unwrap_or("".to_owned())),
        )
        .await
    }

    pub async fn list_files_in_bucket(
        &self,
        bucket_name: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Response<Vec<FilesResponse>>, Error> {
        let res = self
            .request(
                Method::POST,
                &format!("storage/v1/object/list/{}", bucket_name),
                json!({
                        "limit": limit,
                        "offset": offset,
                        "prefix": "",
                        "sortBy": {
                            "column": "name",
                            "order": "asc",
                        },
                }),
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
