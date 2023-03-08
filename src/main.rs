use biblatex::Bibliography;
use log::LevelFilter;
use std::error::Error;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Warn)
        .parse_default_env()
        .init();

    let all = Bibliography::parse(&read_to_string("fuzzing-papers.bib")?)
        .expect("Could not parse the bibliography!");

    let summary_dir = PathBuf::from("summaries");
    for entry in all.iter() {
        let name = &entry.key;
        if !summary_dir.join(name).exists() {
            log::warn!("Missing summary for {}.", name);
        }
        if entry.fields.get("keywords").is_none() {
            log::warn!("Missing keywords for {}.", name);
        }
    }

    Ok(())
}
