extern crate postgres;

use postgres::{Connection, TlsMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::connect("postgresql://postgres@localhost:5432", TlsMode::None).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            data            BYTEA
        )",
        &[],
    )
    .unwrap();
    let me = Person {
        id: 0,
        name: "Jones".to_owned(),
        data: Some("Is this it?".as_bytes().to_owned()),
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&me.name, &me.data],
    )
    .unwrap();

    for row in &conn
        .query("SELECT id, name, data FROM person", &[])
        .unwrap()
    {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}", person.name);
    }
}
