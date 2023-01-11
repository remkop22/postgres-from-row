use postgres_from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow)]
#[allow(dead_code)]
pub struct Todo {
    todo_id: i32,
    text: String,
    #[from_row(flatten)]
    user: User,
}

#[derive(FromRow)]
#[allow(dead_code)]
pub struct User {
    user_id: i32,
}

#[allow(dead_code)]
fn from_row(row: &Row) {
    let _ = Todo::from_row(row);
    let _ = Todo::try_from_row(row).unwrap();

    let _ = User::from_row(row);
    let _ = Todo::try_from_row(row).unwrap();
}
