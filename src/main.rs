use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:secret-pw@localhost/test_db")
        .await?;
    let row: (i64,) = sqlx::query_as("SELECT id FROM teachers where id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 1);

    Ok(())
}
