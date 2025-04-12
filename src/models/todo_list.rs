use crate::schema;
use diesel::prelude::*;
#[derive(Queryable, Debug, Insertable, Selectable)]
#[diesel(table_name = schema::todo_list)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TodoList {
    pub id: i32,
    pub name: String,
}

//#[derive(Debug, Insertable)]
//#[diesel(table_name = self::schema::users)]
//struct NewUser<'a> {
//    id: &'a str,
//    name: &'a str,
//}
