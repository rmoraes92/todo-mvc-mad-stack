use crate::controllers::database::DatabaseConnectionManager;
use crate::models::todo_list::{TodoList, TodoListCreate};
use crate::schema::todo_list::dsl::*;
use diesel::prelude::*;
use diesel::QueryDsl;

pub fn research_todo_list(
    db_conn: &mut r2d2::PooledConnection<DatabaseConnectionManager>,
) -> Result<Vec<TodoList>, diesel::result::Error> {
    let ret = todo_list.limit(10).load::<TodoList>(db_conn).unwrap();
    Ok(ret)
}

pub fn create_todo_list(
    db_conn: &mut r2d2::PooledConnection<DatabaseConnectionManager>,
    new_todolist: TodoListCreate,
) -> Result<TodoList, diesel::result::Error> {
    Ok(
        diesel::insert_into(todo_list)
            .values(&new_todolist)
            .returning(id)
            // .execute(db_conn)?
            .get_result::<i32>(db_conn)
            .map(
                |model_id| TodoList {
                    id: model_id,
                    name: new_todolist.name,
                },
            )?,
    )
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    use crate::test_utils::setup_test_db;

    #[test]
    fn test_create_todo_list() {
        let mut db_conn = setup_test_db();
        let new_todolist = TodoListCreate {
            name: "test 123".to_string(),
        };
        let r = create_todo_list(
            &mut db_conn,
            new_todolist,
        );
        assert_eq!(
            r.is_ok(),
            true
        );
        let m = r.unwrap();
        assert_eq!(
            m.id,
            1
        );
        let new_todolist = TodoListCreate {
            name: "test 456".to_string(),
        };
        let r = create_todo_list(
            &mut db_conn,
            new_todolist,
        );
        assert_eq!(
            r.is_ok(),
            true
        );
        let m = r.unwrap();
        assert_eq!(
            m.id,
            2
        );
    }

    #[test]
    fn test_research_todo_list() {
        let mut db_conn = setup_test_db();
        let new_todolist = TodoListCreate {
            name: "test 123".to_string(),
        };
        let _r = create_todo_list(
            &mut db_conn,
            new_todolist,
        );
        let new_todolist = TodoListCreate {
            name: "test 456".to_string(),
        };
        let _r = create_todo_list(
            &mut db_conn,
            new_todolist,
        );
        let r = research_todo_list(&mut db_conn);
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap().len(),
            2
        );
    }
}
