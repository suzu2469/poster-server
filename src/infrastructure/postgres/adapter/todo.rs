use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::domain::entity::todo::Todo;
use crate::domain::repository::todo::{TodoCreateDTO, TodoRepository, TodoUpdateDTO};
use crate::infrastructure::postgres::table::todo::{Todo as TodoTable, TodoCreate, TodoUpdate};
use crate::shared::DBConnection;

#[derive(Copy, Clone)]
pub struct PgAdapter {}

impl TodoRepository for PgAdapter {
    fn list(&self, conn: &DBConnection) -> Vec<Todo> {
        use crate::schema::todos::dsl::*;

        let db = conn.get().expect("Connection not found");
        let result: Vec<TodoTable> = todos
            .load::<TodoTable>(&db)
            .unwrap_or(Vec::<TodoTable>::new());

        result
            .iter()
            .map(move |t| Todo {
                id: t.id,
                is_done: t.is_done,
                name: t.name.clone(),
                created_at: DateTime::<Utc>::from_utc(t.created_at, Utc),
            })
            .collect()
    }

    fn create(&self, conn: &DBConnection, dto: &TodoCreateDTO) -> Result<(), ()> {
        use crate::schema::todos::dsl::*;

        let todo_create = TodoCreate {
            name: dto.name.clone(),
            is_done: dto.is_done,
        };

        let db = conn.get().expect("Connection not found");
        let _result = diesel::insert_into(todos)
            .values(todo_create)
            .execute(&db)
            .expect("Todo can not be created");

        Ok(())
    }

    fn update(&self, conn: &DBConnection, target_id: i32, dto: &TodoUpdateDTO) -> Result<(), ()> {
        use crate::schema::todos::dsl::*;

        let todo_update = TodoUpdate {
            is_done: dto.is_done,
            name: dto.name.clone(),
        };

        let db = conn.get().expect("Connection not found");
        let _result = diesel::update(todos)
            .filter(id.eq(target_id))
            .set(&todo_update)
            .execute(&db)
            .expect("Todo can not be updated");

        Ok(())
    }

    fn delete(&self, conn: &DBConnection, target_id: i32) -> Result<(), ()> {
        use crate::schema::todos::dsl::*;

        let db = conn.get().expect("Connection not found");
        diesel::delete(todos.filter(id.eq(target_id)))
            .execute(&db)
            .expect(&format!("This todo can not be deleted id = {}", target_id));
        Ok(())
    }
}
