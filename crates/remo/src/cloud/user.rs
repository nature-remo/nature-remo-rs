use super::error::APIError;
use super::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub nickname: String,
}

impl Client {
    pub async fn get_user(&self) -> Result<User, APIError> {
        self.get::<User>("/1/users/me").await
    }
}

#[cfg(test)]
mod tests {
    use super::APIError;
    use super::Client;
    use crate::tests::get_test_token;

    #[tokio::test]
    async fn it_get_authorized_user() {
        let token = get_test_token();
        let client = Client::new(Some(token));
        let res = client.get_user().await.unwrap();
        assert_eq!(res.nickname, "uetchy");
    }

    #[tokio::test]
    async fn it_fail_to_get_authorized_user() {
        let client = Client::new(None);

        let res = client.get_user().await;
        match res {
            Ok(_) => panic!("should not be ok"),
            Err(e) => match e {
                APIError::AuthError => assert!(true),
                _ => panic!("unintended error"),
            },
        }
    }
}
