use postgres_from_row::FromRow;

#[derive(FromRow)]
#[allow(dead_code)]
pub struct Todo {
    todo_id: i32,
    text: String,
    #[from_row(flatten)]
    user: User
}

#[derive(FromRow)]
#[allow(dead_code)]
pub struct User {
    user_id: i32,
}


