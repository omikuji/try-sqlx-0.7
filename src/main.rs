#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum Status {
    ProvisionalRegistration,
    OfficialRegistration,
}

#[tokio::main(worker_threads = 10)]
async fn main() {
    let db_pool = sqlx::mysql::MySqlPoolOptions::new()
        .connect("mysql://dev:dev@localhost:13306/dev")
        .await
        .unwrap();

    sqlx::query!(
        r#"
        INSERT INTO user (
            name,
            status,
            created_at
        ) VALUES (?, ?, now());
        "#,
        "sample".to_string(),
        Status::ProvisionalRegistration
    )
    .execute(&db_pool)
    .await
    .unwrap();

    println!("hoge");
}
