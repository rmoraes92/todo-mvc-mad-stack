use crate::schema;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::todo_list)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TodoList {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = schema::todo_list)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TodoListCreate {
    pub name: String,
}
