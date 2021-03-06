extern crate postgres;
extern crate csv;
extern crate rust_postgres_csv;


use postgres::{Connection, TlsMode};
use self::rust_postgres_csv::table;
use self::rust_postgres_csv::facility::{self, Facility};


fn main() {
    // let x = facility::example();
    // println!("{:?}", x);

    let conn = Connection::connect("postgresql://postgres@localhost:5432", TlsMode::None).unwrap();

    // let fac = Facility {
    //     fac_name: "ikea",
    //     fac_street: "ikea drive w.",
    //     fac_city: "providence",
    //     fac_state: "ri",
    //     fac_zip: "02903",
    //     registry_id: 10000010,
    //     fac_county: "pvd",
    //     fac_epa_region: "01".to_string().parse::<i32>().unwrap(),
    // };
    let fac = Facility::new("ikea",
                            "ikea drive w.",
                            "providence",
                            "ri",
                            "02903",
                            "10000010",
                            "pvd county",
                            "01");

    // Create `FRS_FACILITY` table using `conn` connection
    table::create_frs_table(&conn);
    println!("ok");

    // Insert `fac` struct to `FRS_FACILITY` table
    facility::insert_frs_facility(&fac, &conn);

    // facility::example();
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
