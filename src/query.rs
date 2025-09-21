use serde_json::json;

use crate::client::{Error, Method, Response, SupabaseClient};

#[derive(Debug, Clone)]
pub struct TableSchema {
    pub client: SupabaseClient,
    pub name: String,
    pub insert: Option<serde_json::Value>,
    pub update: Option<serde_json::Value>,
    pub delete: bool,
    pub params: Vec<(String, String, String)>,
}

impl SupabaseClient {
    pub fn from(&self, name: &str) -> TableSchema {
        TableSchema {
            client: self.clone(),
            name: name.to_string(),
            params: vec![],
            insert: None,
            update: None,
            delete: false,
        }
    }
}

impl TableSchema {
    pub fn select(&mut self, columns: &str) -> TableSchema {
        self.params
            .push(("select".to_string(), columns.to_string(), "".to_string()));
        self.clone()
    }

    pub fn eq(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        self.params
            .push(("eq".to_string(), column.to_string(), value.to_string()));
        self.clone()
    }

    pub fn neq(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        self.params
            .push(("neq".to_string(), column.to_string(), value.to_string()));
        self.clone()
    }

    pub fn gt(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        self.params
            .push(("gt".to_string(), column.to_string(), value.to_string()));
        self.clone()
    }

    pub fn gte(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        self.params
            .push(("gte".to_string(), column.to_string(), value.to_string()));
        self.clone()
    }

    pub fn lt(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        self.params
            .push(("lt".to_string(), column.to_string(), value.to_string()));
        self.clone()
    }

    pub fn lte(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        self.params
            .push(("lte".to_string(), column.to_string(), value.to_string()));
        self.clone()
    }

    pub fn like(&mut self, column: &str, value: String) -> TableSchema {
        self.params.push((
            "like".to_string(),
            column.to_string(),
            value.replace("%", "*"),
        ));
        self.clone()
    }

    pub fn ilike(&mut self, column: &str, value: String) -> TableSchema {
        self.params.push((
            "ilike".to_string(),
            column.to_string(),
            value.replace("%", "*"),
        ));
        self.clone()
    }

    pub fn is(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        self.params
            .push(("is".to_string(), column.to_string(), value.to_string()));
        self.clone()
    }

    pub fn in_a(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        let mut mapped_value = "(".to_string();
        match value.as_array() {
            Some(arr) => {
                for v in arr {
                    mapped_value += &format!("{},", v);
                }
                if !arr.is_empty() {
                    mapped_value.pop();
                }
            }
            None => panic!("Provide valid array"),
        }
        mapped_value += ")";

        self.params
            .push(("in".to_string(), column.to_string(), mapped_value));
        self.clone()
    }

    pub fn contains(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        let mut mapped_value = "{".to_string();
        match value.as_array() {
            Some(arr) => {
                for v in arr {
                    mapped_value += &format!("{},", v);
                }
                if !arr.is_empty() {
                    mapped_value.pop();
                }
            }
            None => panic!("Provide valid array"),
        }
        mapped_value += "}";

        self.params
            .push(("cs".to_string(), column.to_string(), mapped_value));
        self.clone()
    }

    pub fn contained_by(&mut self, column: &str, value: serde_json::Value) -> TableSchema {
        let mut mapped_value = "{".to_string();
        match value.as_array() {
            Some(arr) => {
                for v in arr {
                    mapped_value += &format!("{},", v);
                }
                if !arr.is_empty() {
                    mapped_value.pop();
                }
            }
            None => panic!("Provide valid array"),
        }
        mapped_value += "}";

        self.params
            .push(("cd".to_string(), column.to_string(), mapped_value));
        self.clone()
    }

    pub fn not(&mut self, column: &str, rule: &str, value: serde_json::Value) -> TableSchema {
        self.params.push((
            format!("not.{}", rule),
            column.to_string(),
            value.to_string(),
        ));
        self.clone()
    }

    pub fn or(&mut self, value: serde_json::Value) -> TableSchema {
        let mut mapped_value = "(".to_string();
        match value.as_array() {
            Some(arr) => {
                for v in arr {
                    mapped_value += &format!("{},", v);
                }
                if !arr.is_empty() {
                    mapped_value.pop();
                }
            }
            None => panic!("Provide valid array"),
        }
        mapped_value += ")";

        self.params
            .push(("or".to_string(), "".to_string(), mapped_value));
        self.clone()
    }

    pub fn insert(&mut self, data: serde_json::Value) -> TableSchema {
        self.insert = Some(data);
        self.clone()
    }

    pub fn delete(&mut self) -> TableSchema {
        self.delete = true;
        self.clone()
    }

    pub fn update(&mut self, data: serde_json::Value) -> TableSchema {
        self.update = Some(data);
        self.clone()
    }

    pub async fn execute(&self) -> Result<Response<serde_json::Value>, Error> {
        let mut url = format!("rest/v1/{}", self.name);

        let mut cmd = "?";
        for (rule, column, value) in &self.params {
            if rule == "select" {
                url += &format!("{}{}={}", cmd, rule, column);
            } else if rule == "or" {
                url += &format!("{}{}={}", cmd, rule, value.replace("\"", ""));
            } else {
                url += &format!("{}{}={}.{}", cmd, column, rule, value.replace("\"", ""));
            }

            cmd = "&";
        }

        let mut body: serde_json::Value = json!({});
        let method = if self.insert.is_some() {
            body = self.insert.clone().unwrap();
            Method::POST
        } else if self.delete == true {
            Method::DELETE
        } else if self.update.is_some() {
            body = self.update.clone().unwrap();
            Method::PATCH
        } else {
            Method::GET
        };

        let res = self
            .client
            .request(
                method,
                &url,
                body,
                Some(&self.client.access_token.clone().unwrap_or("".to_owned())),
                None,
            )
            .await?;

        let txt = res.data.unwrap();
        let data = if &txt == "" {
            json!({})
        } else {
            serde_json::from_str(&txt).unwrap()
        };

        Ok(Response {
            code: res.code,
            data: Some(data),
        })
    }
}
