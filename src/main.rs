use biblatex::Bibliography;
use log::LevelFilter;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Warn)
        .parse_default_env()
        .init();

    let mut data_missing = false;

    let all = Bibliography::parse(&read_to_string("fuzzing-papers.bib")?)
        .expect("Could not parse the bibliography!");

    let summary_dir = PathBuf::from("summaries");
    for entry in all.iter() {
        let name = &entry.key;
        if !summary_dir.join(name).exists() {
            log::warn!("Missing summary for {}.", name);
            data_missing = true;
        }
        if entry.fields.get("keywords").is_none() {
            log::warn!("Missing keywords for {}.", name);
            data_missing = true;
        }
    }

    if data_missing {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
