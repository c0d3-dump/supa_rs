#[cfg(test)]
mod tests {
    use serde_json::json;
    use supa_rs::client::SupabaseClient;

    #[tokio::test]
    async fn table_select() {
        let mut client = SupabaseClient::new(None, None);
        let res = client
            .email_login("prubruttadaja-3961@yopmail.com", "12345678")
            .await;

        let tokens = res.unwrap().data.unwrap();
        client
            .set_session(&tokens.access_token, &tokens.refresh_token)
            .await;
        let res = client.from("test").select("*").execute().await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_insert() {
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
                "name": "test5"
            }))
            .select("name")
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_eq() {
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
            .select("*")
            .eq("name", "test".into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_neq() {
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
            .select("*")
            .neq("name", "test9".into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_lt() {
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
            .select("*")
            .lt("id", 5.into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_lte() {
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
            .select("*")
            .lte("id", 4.into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_gt() {
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
            .select("*")
            .gt("id", 5.into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_gte() {
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
            .select("*")
            .gte("id", 8.into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_like() {
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
            .select("*")
            .like("name", "test%".into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_ilike() {
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
            .select("*")
            .ilike("name", "Test%".into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_is() {
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
            .select("*")
            .is("name", serde_json::Value::Null)
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_in_a() {
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
            .select("*")
            .in_a("name", json!(["test1", "test2"]))
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_contains() {
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
            .select("*")
            .contains("arr", json!(["test1"]))
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_contained_by() {
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
            .select("*")
            .contained_by("arr", json!(["test2"]))
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_not() {
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
            .select("*")
            .not("name", "like", "test1".into())
            .execute()
            .await;

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn table_or() {
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
            .select("*")
            .or(json!(["name.eq.test1", "name.eq.test2",]))
            .execute()
            .await;

        println!("{:?}", res);
    }
}
