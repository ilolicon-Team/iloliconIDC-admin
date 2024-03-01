use rusqlite::{Connection, Result};

pub fn install_sqlite3() -> Result<Connection> {
    let database_file = "data.db";
    let conn = Connection::open(database_file)?;
    conn.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            email TEXT NOT NULL,
            token TEXT NOT NULL,
            money REAL NOT NULL DEFAULT 0.00,
            aff INTEGER NOT NULL DEFAULT 0,
            is_admin INTEGER NOT NULL DEFAULT 0,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_login_ip TEXT NOT NULL,
            last_login_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE server (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            ip TEXT NOT NULL,
            port INTEGER NOT NULL DEFAULT 0,
            type TEXT NOT NULL,
            max_limit INTEGER NOT NULL DEFAULT 0,
            `group` INTEGER NOT NULL DEFAULT 0,
            status INTEGER NOT NULL DEFAULT 0,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE use_server (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL DEFAULT 0,
            server_id INTEGER NOT NULL DEFAULT 0,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            type TEXT NOT NULL,
            max_limit INTEGER NOT NULL DEFAULT 0,
            enable INTEGER NOT NULL DEFAULT 0,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            end_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL DEFAULT 0,
            server_id INTEGER NOT NULL DEFAULT 0,
            do TEXT NOT NULL,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE ticket (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL DEFAULT 0,
            own_admin_id INTEGER NOT NULL DEFAULT 0,
            server_id INTEGER NOT NULL DEFAULT 0,
            ticket_title TEXT NOT NULL,
            ticket_body TEXT NOT NULL,
            is_end INTEGER NOT NULL DEFAULT 0,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE cart (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            group_id INTEGER NOT NULL DEFAULT 0,
            cart_title TEXT NOT NULL,
            cart_body TEXT NOT NULL,
            money REAL NOT NULL DEFAULT 0.00,
            stock INTEGER NOT NULL DEFAULT 0,
            allow_payment TEXT NOT NULL,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE invoice (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            payment_id INTEGER NOT NULL DEFAULT 0,
            user_id INTEGER NOT NULL DEFAULT 0,
            invoice_remote INTEGER NOT NULL DEFAULT 0,
            invoice_local INTEGER NOT NULL DEFAULT 0,
            money REAL NOT NULL DEFAULT 0.00,
            pay_status INTEGER NOT NULL DEFAULT 0, /*1: Paid 2: Unpaid 3: Expired 4: Fraud 5: Cancelled*/
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            end_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE payment (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT NOT NULL,
            url TEXT NOT NULL,
            pay_id TEXT NOT NULL,
            key TEXT NOT NULL,
            private_key TEXT NOT NULL,
            public_key TEXT NOT NULL,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE setting (
            site_name TEXT NOT NULL,
            database_version INTEGER NOT NULL DEFAULT 0
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE announcement (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type INTEGER NOT NULL DEFAULT 0, /*1: Normal 2: Emergency*/
            announcement_title TEXT NOT NULL,
            announcement_body TEXT NOT NULL,
            display_portal INTEGER NOT NULL DEFAULT 0,
            created_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        [],
    )?;


    Ok(conn)
}
