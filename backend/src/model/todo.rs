use super::db::Db;
use crate::model;
use sqlb::{HasFields, Raw};

// region:  Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    id: i64,
    cid: i64,
    title: String,
    status: TodoStatus,
}

#[derive(sqlb::Fields, Default, Debug, Clone)]
pub struct TodoPatch {
    title: Option<String>,
    cid: Option<i64>,
    status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}
sqlb::bindable!(TodoStatus);
// endregion:   Todo Types

// region:  TodoMac
pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, data: TodoPatch) -> Result<Todo, model::Error> {
        let sb = sqlb::insert()
            .table("todo")
            .data(data.fields())
            .returning(&["id", "cid", "title", "status"]);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db) -> Result<Vec<Todo>, model::Error> {
        let sql = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";

        // build sqlx query
        let query = sqlx::query_as(&sql);
        // execute query
        let todos = query.fetch_all(db).await?;
        Ok(todos)
    }
}
// endregion:  TodoMac

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;

// VID https://youtu.be/VIig9IcQ-w8?t=1533
