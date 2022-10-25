# postgres-from-row

Derive [`FromRow`] to generate a mapping between a struct and postgres rows.

This crate works with [postgres](<https://docs.rs/postgres>) by default.

```toml
[dependencies]
postgres_from_row = "0.1.0"
```

If you want to use it with [tokio-postgres](<https://docs.rs/tokio-postgres>), enable it like so:

```toml
[dependencies]
postgres_from_row = { version = "0.1.0", default_features = false, features = ["tokio-postgres"] }
```
## Examples
```rust
use postgres_from_row::FromRow;

#[derive(FromRow)]
struct Todo {
    todo_id: i32,
    text: String
    author_id: i32,
}

let row = client.query_one("SELECT todo_id, text, author_id FROM todos").unwrap();

// Pass a row with the correct columns.
let todo = Todo::from_row(&row);

let row = client.query_one("SELECT foo FROM bar").unwrap();

// Use `try_from_row` if the operation could fail.
let todo = Todo::try_from_row(&row);
assert!(todo.is_err());
```

Each field need's to implement [`postgres::types::FromSql`], as this will be used to convert a
single column to the specified type. If you want to override this behavior and delegate it to a
nested structure that also implements [`FromRow`], use `#[from_row(flatten)]`:

```rust
use postgres_from_row::FromRow;

#[derive(FromRow)]
struct Todo {
    todo_id: i32,
    text: String,
    #[from_row(flatten)]
    author: User
}

#[derive(FromRow)]
struct User {
    user_id: i32,
    username: String
}

let row = client.query_one("SELECT todo_id, text, user_id, username FROM todos t, users u WHERE t.author_id = u.user_id").unwrap();
let todo = Todo::from_row(&row);
```

