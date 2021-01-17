use std::collections::HashMap;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Environment {
    db_pool: PgPool,
}

impl Environment {
    pub async fn new() -> anyhow::Result<Self> {
        let db_pool =
            PgPool::connect(&std::env::var("DATABASE_URL").expect("Must specify DATABASE_URL"))
                .await?;
        Ok(Self { db_pool })
    }

    pub fn db(&self) -> &PgPool {
        &self.db_pool
    }
}
