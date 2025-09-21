#[cfg(test)]
mod tests {
    use supa_rs::client::SupabaseClient;

    #[tokio::test]
    async fn table_delete() {
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
            .delete()
            .eq("name", "test5".into())
            .select("*")
            .execute()
            .await;

        println!("{:?}", res);
    }
}
