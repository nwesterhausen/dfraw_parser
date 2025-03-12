pub(crate) mod sqlite;

pub fn init_db() {
    let conn = sqlite::load_database("test.db").unwrap();
    sqlite::apply_migrations(&conn).unwrap()
}