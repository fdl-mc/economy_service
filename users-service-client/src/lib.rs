use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub admin: bool,
}

#[derive(Clone, Debug)]
pub enum GetUserResponse {
    Ok(User),
    BadRequest,
    NotFound,
}
impl GetUserResponse {
    async fn from_http_response(response: reqwest::Response) -> Result<Self, reqwest::Error> {
        match response.status() {
            StatusCode::OK => Ok(Self::Ok(response.json().await?)),
            StatusCode::BAD_REQUEST => Ok(Self::BadRequest),
            StatusCode::NOT_FOUND => Ok(Self::NotFound),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum GetSelfResponse {
    Ok(User),
    Unauthenticated,
}
impl GetSelfResponse {
    async fn from_http_response(response: reqwest::Response) -> Result<Self, reqwest::Error> {
        match response.status() {
            StatusCode::OK => Ok(Self::Ok(response.json().await?)),
            StatusCode::UNAUTHORIZED => Ok(Self::Unauthenticated),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct UsersServiceClient {
    base_url: String,
    client: reqwest::Client,
}
impl UsersServiceClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        UsersServiceClient {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn get_user(&self, id: i32) -> Result<GetUserResponse, reqwest::Error> {
        GetUserResponse::from_http_response(
            self.client
                .get(format!("{}/{}", self.base_url, id))
                .send()
                .await?,
        )
        .await
    }

    pub async fn get_self(
        &self,
        token: impl Into<String>,
    ) -> Result<GetSelfResponse, reqwest::Error> {
        GetSelfResponse::from_http_response(
            self.client
                .get(format!("{}/me", self.base_url))
                .header("x-token", token.into())
                .send()
                .await?,
        )
        .await
    }
}
