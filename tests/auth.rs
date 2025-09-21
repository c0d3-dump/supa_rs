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
    async fn anonymous_login() {
        let client = SupabaseClient::new(None, None);
        let res = client.anonymous_login().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn email_login() {
        let client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn user() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.user().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn logout() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        client.set_session(&res.unwrap().data.unwrap().access_token);
        let res = client.logout().await;

        println!("{:?}", res);
    }
}
