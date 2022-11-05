use super::{TodoMac, TodoPatch};
use crate::model::db::init_db;
use crate::model::todo::TodoStatus;

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let data_fx = TodoPatch {
        title: Some("test - Model_todo_create 1".to_string()),
        cid: Some(123),
        ..Default::default()
    };

    // -- ACTION
    let todo_created = TodoMac::create(&db, data_fx.clone()).await?;

    // -- CHECK
    assert!(todo_created.id >= 1000, "Id should be >= 1000");
    assert_eq!(TodoStatus::Open, todo_created.status);

    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // FIXTURE
    let db = init_db().await?;

    // ACTION
    let todos = TodoMac::list(&db).await?;

    // CHECK
    assert_eq!(2, todos.len());

    Ok(())
}
