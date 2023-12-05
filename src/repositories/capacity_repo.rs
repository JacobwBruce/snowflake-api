use sqlx::MySqlPool;

#[derive(Clone)]
pub struct CapacityRepository {
    pub db: MySqlPool,
}

impl CapacityRepository {
    pub async fn get_capacity(
        &self,
    ) -> Result<Vec<crate::models::capacity::Capacity>, sqlx::Error> {
        match sqlx::query_as::<_, crate::models::capacity::Capacity>("SELECT * FROM capacity")
            .fetch_all(&self.db)
            .await
        {
            Ok(capacity) => Ok(capacity),
            Err(_) => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn update_capacity(
        &self,
        id: i32,
        payload: crate::models::capacity::NewCapacity,
    ) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
        let res = sqlx::query(
            r#"
        UPDATE capacity
        SET name = ?,
            location = ?,
            num_of_vendors_needed = ?,
            tsa_needed = ?
        WHERE id = ?
        "#,
        )
        .bind(&payload.name)
        .bind(&payload.location)
        .bind(&payload.num_of_vendors_needed)
        .bind(&payload.tsa_needed)
        .bind(&id)
        .execute(&self.db)
        .await;
        match res {
            Ok(status) => Ok(status),
            Err(_) => Err(sqlx::Error::RowNotFound),
        }
    }
}
