# postgres-from-row

Derive `FromRow` to generate a mapping between a struct and postgres rows.

This crate is compatible with both [postgres](<https://docs.rs/postgres>) and [tokio-postgres](<https://docs.rs/tokio-postgres>).

```toml
[dependencies]
postgres_from_row = "0.5.1"
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

let row = client.query_one("SELECT todo_id, text, author_id FROM todos", &[]).unwrap();

// Pass a row with the correct columns.
let todo = Todo::from_row(&row);

let row = client.query_one("SELECT foo FROM bar", &[]).unwrap();

// Use `try_from_row` if the operation could fail.
let todo = Todo::try_from_row(&row);
assert!(todo.is_err());
```

Each field need's to implement `postgres::types::FromSql`, as this will be used to convert a
single column to the specified type. If you want to override this behavior and delegate it to a
nested structure that also implements `FromRow`, use `#[from_row(flatten)]`:

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

let row = client.query_one("SELECT todo_id, text, user_id, username FROM todos t, users u WHERE t.author_id = u.user_id", &[]).unwrap();
let todo = Todo::from_row(&row);
```

If a the struct contains a field with a name that differs from the name of the sql column, you can use the `#[from_row(rename = "..")]` attribute. 

When a field in your struct has a type `T` that doesn't implement `FromSql` or `FromRow` but 
it does impement `T: From<C>` or `T: TryFrom<c>`, and `C` does implment `FromSql` or `FromRow` 
you can use `#[from_row(from = "C")]` or `#[from_row(try_from = "C")]`. This will use type `C` to extract it from the row and 
then finally converts it into `T`. 

```rust

struct Todo {
    // If the postgres column is named `todo_id`.
    #[from_row(rename = "todo_id")]
    id: i32,
    // If the postgres column is `VARCHAR`, it will be decoded to `String`,
    // using `FromSql` and then converted to `Vec<u8>` using `std::convert::From`.
    #[from_row(from = "String")]
    todo: Vec<u8>
}

```


