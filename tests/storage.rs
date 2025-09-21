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

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.create_bucket("test2", false).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_list() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.list_buckets().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_details() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.get_bucket("test2").await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_update() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);

        let update_req = UpdateRequest {
            public: Some(false),
            file_size_limit: Some(1024),
            allowed_mime_types: None,
        };

        let res = client.update_bucket("test2", update_req).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_bucket_delete() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.delete_bucket("test2").await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_upload() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let file_data = fs::read("./test.txt").await.unwrap();

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client
            .upload_file("test", "test2.txt", file_data, false)
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

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client
            .upload_file("test", "test2.txt", file_data, true)
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_get() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.get_file("test", "test2.txt").await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn storage_file_list() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.list_files_in_bucket("test", 100, 0).await;

        println!("{:?}", res);
    }
}
