use std::error::Error;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records(){
        let record = result?;
        println!("Records: {:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./COVID_data.csv"){
        println!("Error: {}", e);
    }
}
