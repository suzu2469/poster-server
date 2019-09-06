use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

use crate::schema::todos;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub is_done: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct TodoCreate {
    pub name: String,
    pub is_done: bool,
}
