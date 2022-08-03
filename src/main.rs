use serde_json;
use std::fs::File;
use std::io::BufReader;
mod edtr_types;

fn main() -> std::io::Result<()> {
    let input_file = BufReader::new(File::open("1555.json")?);
    let edtr_state: edtr_types::EdtrPlugin = serde_json::from_reader(input_file)?;

    println!("{:#?}", edtr_state);
    Ok(())
}
