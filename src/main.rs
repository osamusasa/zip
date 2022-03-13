extern crate osmzip;

use std::error::Error;

use osmzip::{reader, header};
use osmzip::reader::read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut zip_header = header::local_file_header::ZipHeader::new();
    zip_header.set_version(1, 0);
    zip_header.set_last_mod_date_time(12, 34, 11, 24, 8, 2021);
    println!("{:?}", zip_header);

    println!(
        "{}年{}月{}日　{}時{}分{}秒",
        zip_header.get_last_mod_year(),
        zip_header.get_last_mod_month(),
        zip_header.get_last_mod_day(),
        zip_header.get_last_mod_hour(),
        zip_header.get_last_mod_min(),
        zip_header.get_last_mod_seconds()
    );

    reader::read("D:\\source\\rust\\osmzip\\resources\\docker-compose.zip");

    Ok(())
}
