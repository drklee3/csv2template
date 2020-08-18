use std::collections::BTreeMap;
use std::error::Error;
use std::io;
use tera::{Context, Tera};

mod error;
mod filters;

fn read_csv() -> Result<Vec<BTreeMap<String, String>>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());

    let mut entries = Vec::new();

    for result in rdr.deserialize() {
        let row: BTreeMap<String, String> = result?;
        log::debug!("row: {:?}", &row);
        entries.push(row);
    }

    log::debug!("data: {:?}", &entries);

    Ok(entries)
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let data = read_csv()?;

    let mut context = Context::new();
    context.insert("data", &data);

    let mut templates = Tera::new("templates/**/*")?;

    templates.register_filter("convertcase", filters::convert_case);

    let rendered = templates.render("test.beancount", &context)?;

    println!("{}", rendered);

    Ok(())
}
