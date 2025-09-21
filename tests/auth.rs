#[cfg(test)]
mod tests {
    use supa_rs::client::SupabaseClient;

    #[tokio::test]
    async fn signup() {
        let client = SupabaseClient::new(None, None);
        let res = client.signup("test1@gmail.com", "12345678").await;
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn login() {
        let client = SupabaseClient::new(None, None);
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn user() {
        let client = SupabaseClient::new(None, None);
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let res = client.user(&res.unwrap().data.unwrap().access_token).await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn logout() {
        let client = SupabaseClient::new(None, None);
        let res = client
            .login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let res = client
            .logout(&res.unwrap().data.unwrap().access_token)
            .await;

        println!("{:?}", res);
    }
}
