#[cfg(test)]
mod tests {
    use serde_json::json;
    use supa_rs::client::SupabaseClient;

    #[tokio::test]
    async fn table_update() {
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
            .update(json!({
                "name": "test9"
            }))
            .eq("name", "test".into())
            .select("*")
            .execute()
            .await;

        println!("{:?}", res);
    }
}
