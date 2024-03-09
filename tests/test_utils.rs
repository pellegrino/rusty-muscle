use sqlx::postgres::PgPoolOptions;

pub async fn setup_test_db() -> sqlx::PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
