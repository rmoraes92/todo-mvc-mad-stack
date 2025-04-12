use crate::controllers::database::DatabaseConnectionManager;
use crate::models::todo_list::TodoList;
use crate::schema::todo_list::dsl::*;
use diesel::QueryDsl;
use diesel::prelude::*;

pub fn get_todo_list(
    db_conn: &mut r2d2::PooledConnection<DatabaseConnectionManager>,
) -> Result<Vec<TodoList>, diesel::result::Error> {
    let ret = todo_list.limit(10).load::<TodoList>(db_conn).unwrap();
    Ok(ret)
}

pub fn create_todo_list(
    db_conn: &mut r2d2::PooledConnection<DatabaseConnectionManager>,
    new_todolist: TodoList,
) -> Result<(), diesel::result::Error> {
    let _inserted_rows = diesel::insert_into(todo_list)
        .values(&new_todolist)
        .execute(db_conn)?;
    Ok(())
}