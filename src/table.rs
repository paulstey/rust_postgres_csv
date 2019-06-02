// table.rs

use postgres::Connection;


pub fn create_frs_table(conn: &Connection) {
    conn.execute(
        "
        DROP TABLE IF EXISTS frs_facilities;
        ",
        &[],
    )
    .unwrap();

    conn.execute(
        "
        CREATE TABLE frs_facilities (
            FAC_NAME       varchar(80),
            FAC_STREET     varchar(80),
            FAC_CITY       varchar(80),
            FAC_STATE      varchar(2),
            FAC_ZIP        varchar(80),
            REGISTRY_ID    bigint primary key,
            FAC_COUNTY     varchar(80),
            FAC_EPA_REGION int
        );",
        &[],
    )
    .unwrap();
}