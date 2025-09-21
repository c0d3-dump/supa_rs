#[cfg(test)]
mod tests {
    use serde_json::json;
    use supa_rs::client::SupabaseClient;

    #[tokio::test]
    async fn table_insert_one() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client
            .from("test")
            .insert(json!({
                "name": "test"
            }))
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_insert_many() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client
            .from("test")
            .insert(json!([
                {"name": "test1"},
                {"name": "test2"}
            ]))
            .select("*")
            .execute()
            .await;

        println!("{:?}", res);
    }
}
