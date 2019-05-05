// facility.rs

use postgres::Connection;

// The `Facility` struct will be used to do a single row inserts in to the 
// Postgres database. We need the lifetime specifier 'a or else the compiler
// complains for all the fields in the struct.
pub struct Facility<'a> {
    pub fac_name: &'a str,
    pub fac_street: &'a str,
    pub fac_city: &'a str,
    pub fac_state: &'a str,
    pub fac_zip: &'a str,
    pub registry_id: i64,
    pub fac_county: &'a str,
    pub fac_epa_region: i32,
}

// impl<'a> Facility<'a> {
//     pub fn new(fac_name: &'a str,
//                fac_street: &'a str,
//                fac_city: &'a str,
//                fac_state: &'a str,
//                fac_zip: &'a str,
//                registry_id: i64,
//                fac_county: &'a str,
//                fac_epa_region: i32) -> Facility<'a> {
        
//         Facility{fac_name,
//                  fac_street,
//                  fac_city, 
//                  fac_state,
//                  fac_zip,
//                  registry_id,
//                  fac_county,
//                  fac_epa_region}
//     }
// }


pub fn insert_frs_facility(fac: &Facility, conn: &Connection) {
    conn.execute(
        "
        INSERT INTO frs_facilities (FAC_NAME, 
                                    FAC_STREET, 
                                    FAC_CITY, 
                                    FAC_STATE, 
                                    FAC_ZIP, 
                                    REGISTRY_ID, 
                                    FAC_COUNTY, 
                                    FAC_EPA_REGION) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ;",
        &[
            &fac.fac_name,
            &fac.fac_street,
            &fac.fac_city,
            &fac.fac_state,
            &fac.fac_zip,
            &fac.registry_id,
            &fac.fac_county,
            &fac.fac_epa_region,
        ],
    )
    .unwrap();
}