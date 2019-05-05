extern crate postgres;
extern crate rust_postgres_csv;

use postgres::{Connection, TlsMode};


use self::rust_postgres_csv::table;
use self::rust_postgres_csv::facility::{self, Facility};

fn main() {
    let conn = Connection::connect("postgresql://postgres@localhost:5432", TlsMode::None).unwrap();

    let fac = Facility {
        fac_name: "ikea",
        fac_street: "ikea drive w.",
        fac_city: "providence",
        fac_state: "ri",
        fac_zip: "02903",
        registry_id: 10000008,
        fac_county: "pvd",
        fac_epa_region: 1,
    };

    // Create `FRS_FACILITY` table using `conn` connection
    table::create_frs_table(&conn);

    // Insert `fac` struct to `FRS_FACILITY` table
    facility::insert_frs_facility(&fac, &conn);

    // for row in &conn
    //     .query("SELECT id, name, data FROM person", &[])
    //     .unwrap()
    // {
    //     let person = Person {
    //         id: row.get(0),
    //         name: row.get(1),
    //         data: row.get(2),
    //     };
    //     println!("Found person {}", person.name);
    // }
}
