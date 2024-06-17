use rusqlite::Connection;

pub fn get_db_conn()-> Connection {
    let conn: Connection = Connection::open("resto-warp.db").expect("Failed to open SQLite Connection");
}

pub fn initialize_db() {
    println!("Initializing the database!");
    let conn = Connection::open("resto-warp.db").expect("Failed to open SQLite Connection!");

    conn.execute("PRAGMA foreign_keys = ON;", []).expect("Failed to enable foreign key support");

    println!("Creating Table: Table!");
    create_table_table(&conn).expect("Failed to create Table");
    println!("Creating Table: Menu!");
    create_menu_table(&conn).expect("Failed to create Table");
    println!("Creating Table: Order!");
    create_order_table(&conn).expect("Failed to create Table");
    println!("Creating Table: OrderItem!");
    create_order_item_table(&conn).expect("Failed to create Table");
}

fn create_table_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS tables (id INTEGER PRIMARY KEY, code TEXT NOT NULL UNIQUE)", [])?;
    Ok(())

}

fn create_menu_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS menus (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE)", [])?;
    Ok(())
}


fn create_order_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS orders (id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL, FOREIGN KEY (table_id) REFERENCES tables(id), UNIQUE(table_id))", [])?;
    Ok(())
}

fn create_order_item_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS order_items (id INTEGER PRIMARY KEY, order_id INTEGER NOT NULL, menu_id INTEGER NOT NULL, cooking_time NOT NULL, quantity INTEGER NOT NULL DEFAULT 1, FOREIGN KEY (order_id) REFERENCES orders(id), FOREIGN KEY (menu_id) REFERENCES menus(id))", [])?;
    Ok(())
}
