#[cfg(test)]
mod tests {
    use supa_rs::{client::SupabaseClient, storage::UpdateRequest};
    use tokio::fs;

    #[tokio::test]
    async fn storage_bucket_create() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.create_bucket("test2", false).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_list() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.list_buckets().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_details() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.bucket("test1").get_bucket().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_update() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;

        let update_req = UpdateRequest {
            public: Some(false),
            file_size_limit: Some(1024),
            allowed_mime_types: None,
        };

        let res = client.bucket("test1").update_bucket(update_req).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_delete() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.bucket("test1").delete_bucket().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_upload() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let file_data = fs::read("./test.txt").await.unwrap();

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client
            .bucket("test")
            .upload("test.txt", file_data, false)
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_upload_with_upsert() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let file_data = fs::read("./test.txt").await.unwrap();

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client
            .bucket("test")
            .upload("test2.txt", file_data, true)
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_get() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.bucket("test").get("test4.txt").await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_list() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.bucket("test").list(100, 0).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_search() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.bucket("test").search("test4", 200).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_delete() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.bucket("test").delete("test.txt").await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_public_url() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.bucket("test").get_public_url("test.txt").await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_signed_url() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client
            .bucket("test1")
            .get_signed_url("test.txt", 3600)
            .await;

        println!("{:?}", res);
    }
}
