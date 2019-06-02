// facility.rs

use postgres::Connection;
use csv::Reader;
use std::error::Error;



// The `Facility` struct will be used to do a single row inserts in to the 
// Postgres database. We need the lifetime specifier 'a or else the compiler
// complains for all the fields in the struct.
#[derive(Debug)]
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

impl<'a> Facility<'a> {
    pub fn new(fac_name: &'a str,
               fac_street: &'a str,
               fac_city: &'a str,
               fac_state: &'a str,
               fac_zip: &'a str,
               registry_id: &'a str,
               fac_county: &'a str,
               fac_epa_region: &'a str) -> Facility<'a> {
        
        let region_int = fac_epa_region.parse::<i32>().unwrap();

        let registry_id_int = registry_id.parse::<i64>().unwrap();

        Facility{fac_name,
                 fac_street,
                 fac_city, 
                 fac_state,
                 fac_zip,
                 registry_id: registry_id_int,
                 fac_county,
                 fac_epa_region: region_int}
    }
}




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

// fn null_if_blank<'a>(s: &'a str, null: &'a str) -> &'a str {
//     if s == "" {
//         return null
//     }
//     s
// }

// pub fn as_facility(rec: csv::StringRecord) ->  &'static Facility {
//     let fac_name = null_if_blank(rec.get(0), "NULL");
//     let fac_street = null_if_blank(rec.get(1), "NULL");
//     let fac_city = null_if_blank(rec.get(2), "NULL");
//     let fac_state = null_if_blank(rec.get(3), "NULL");
//     let fac_zip = null_if_blank(rec.get(4), "NULL");
//     let registry_id = null_if_blank(rec.get(5), "NULL");
//     let fac_county = null_if_blank(rec.get(6), "NULL");
//     let fac_epa_region = null_if_blank(rec.get(7), "NULL");

    // let fac = Facility {
    //     fac_name,
    //     fac_street,
    //     fac_city,
    //     fac_state,
    //     fac_zip,
    //     registry_id,
    //     fac_county,
    //     fac_epa_region,
    // };
//     fac
// }

// pub fn as_facility2<'a>(name: &'a str, 
//                         street: &'a str,
//                         city: &'a str,
//                         state: &'a str,
//                         zip: &'a str,
//                         id: &'a str,
//                         county: &'a str,
//                         region: &'a str) -> Facility {
//     Facility {
//         fac_name: null_if_blank(name, "NULL"),
//         fac_street: null_if_blank(street, "NULL"),
//         fac_city: null_if_blank(city, "NULL"),
//         fac_state: null_if_blank(state, "NULL"),
//         fac_zip: null_if_blank(zip, "NULL"),
//         registry_id: null_if_blank(id, "NULL"),
//         fac_county: null_if_blank(county, "NULL"),
//         fac_epa_region: null_if_blank(region, "NULL"),
//     }
// }



pub fn example() -> Result<(), Box<Error>> {
    let mut rdr = Reader::from_path("/Users/euclid/projects/rust_postgres_csv/FRS_FACILITIES.csv")?;
    let mut i = 1;
    for result in rdr.records() {
        let rec = result?;

        // let facility: Facility = record.deserialize(None)?;
        let facility = Facility::new(rec.get(0).unwrap(),
                                     rec.get(1).unwrap(),
                                     rec.get(2).unwrap(),
                                     rec.get(3).unwrap(),
                                     rec.get(4).unwrap(),
                                     rec.get(5).unwrap(),
                                     rec.get(6).unwrap(),
                                     rec.get(7).unwrap());
        println!("{:?}", facility);
        i += 1;
    }
    Ok(())
}