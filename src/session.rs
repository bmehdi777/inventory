use redis::{AsyncCommands, Client};

pub struct SessionStore {
    client: Client,
}

impl SessionStore {
    const USER_ID_KEY: &'static str = "user_id";

    pub async fn new(uri: String) -> anyhow::Result<Self> {
        Ok(SessionStore {
            client: Client::open(uri).expect("Failed to connect to redis database"),
        })
    }
    async fn get_connection(&self) -> redis::RedisResult<redis::aio::Connection> {
        match self.client.get_tokio_connection().await {
            Ok(c) => Ok(c),
            Err(e) => {
                tracing::error!("Failed to connect to redis database.");
                return Err(e);
            }
        }
    }

    pub async fn has_user_id(&self, uid: String) -> redis::RedisResult<String> {
        Ok(self.get_connection().await?.get(uid).await?)
    }
    pub async fn insert_user_id(&self, uid: String) -> redis::RedisResult<()> {
        Ok(self
            .get_connection()
            .await?
            .sadd(uid, Self::USER_ID_KEY)
            .await?)
    }
    pub async fn log_out(&self, uid: String) -> redis::RedisResult<()> {
        Ok(self
            .get_connection()
            .await?
            .srem(uid, Self::USER_ID_KEY)
            .await?)
    }
}
