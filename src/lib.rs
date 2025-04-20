use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use sqlite_wasm_rs::sahpool_vfs::install as install_opfs_vfs;
use wasm_bindgen::prelude::wasm_bindgen;

mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[wasm_bindgen]
pub struct Todo {
    id: i32,
    text: String,
}

#[wasm_bindgen]
impl Todo {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::todos)]
struct NewTodo<'a> {
    text: &'a str,
}

#[wasm_bindgen]
pub struct Repo {
    conn: SqliteConnection,
}

#[wasm_bindgen]
pub async fn new_repo() -> Repo {
    Repo::new().await
}

impl Repo {
    async fn new() -> Self {
        console_error_panic_hook::set_once();
        install_opfs_vfs(None, true)
            .await
            .expect("Failed to set up OPFS VFS");
        Self {
            conn: SqliteConnection::establish("testing-opfs-sahpool.db")
                .expect("Failed to open SQLite database."),
        }
    }
}

#[wasm_bindgen]
impl Repo {
    pub fn migrate(&mut self) {
        self.conn
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
    }

    pub fn put_todo(&mut self, todo_text: String) -> Todo {
        use schema::todos::dsl::*;
        let new_todo = NewTodo {
            text: todo_text.as_ref(),
        };
        let todo = diesel::insert_into(todos)
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(&mut self.conn)
            .unwrap();
        todo
    }

    pub fn delete_todo(&mut self, todo_id: i32) {
        use schema::todos::dsl::*;
        diesel::delete(todos.find(todo_id))
            .execute(&mut self.conn)
            .unwrap();
    }

    pub fn get_todos(&mut self) -> Vec<Todo> {
        use schema::todos::dsl::*;
        let todos_from_db = todos
            .select(Todo::as_select())
            .get_results(&mut self.conn)
            .unwrap();
        todos_from_db
    }
}
