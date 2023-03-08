use biblatex::Bibliography;
use log::LevelFilter;
use petgraph::graphmap::DiGraphMap;
use std::collections::{HashMap, HashSet};
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

    let mut keyword_map = HashMap::new();
    let mut author_map = HashMap::new();
    let mut refmap = DiGraphMap::new();

    let summary_dir = PathBuf::from("summaries");
    for entry in all.iter() {
        let name = entry.key.as_str();
        assert!(
            name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'),
            "while processing {name}: expected only keywords containing alphanumerics and hyphens!"
        );
        assert!(!name.is_empty(), "encountered empty paper name");
        if !summary_dir.join(name).exists() {
            log::warn!("Missing summary for {name}.");
            data_missing = true;
        }
        if let Some(keywords) = entry.fields.get("keywords") {
            assert_eq!(
                keywords.len(),
                1,
                "while processing {name}: expected exactly one keywords entry!"
            );
            for keyword in keywords[0].v.get().split(',') {
                let keyword = keyword.trim();
                assert!(
                    keyword
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-'),
                    "while processing {name}: expected only keywords containing alphanumerics and hyphens!"
                );
                assert!(
                    !keyword.is_empty(),
                    "while processing {name}: encountered empty keyword"
                );
                log::debug!("{name}: {keyword}");
                keyword_map
                    .entry(keyword)
                    .or_insert_with(HashSet::new)
                    .insert(name);
            }
        } else {
            log::error!("Missing keywords for {name}.");
            data_missing = true;
        }
        if let Some(authors) = entry.fields.get("author") {
            assert_eq!(
                authors.len(),
                1,
                "while processing {name}: expected exactly one authors entry!"
            );
            for author in authors[0].v.get().split(" and ") {
                let author = author.trim();
                let author_key = if let Some((last, first)) = author.split_once(", ") {
                    format!("{first} {last}")
                } else {
                    author.to_string()
                };
                log::debug!("{name}: {author_key}");
                if author_key.contains('.') {
                    log::warn!(
                        "while processing {name}: author {author_key} appears to be shortened"
                    );
                }
                author_map
                    .entry(author_key)
                    .or_insert_with(HashSet::new)
                    .insert(name);
            }
        } else {
            log::error!("Missing authors for {name}.");
            data_missing = true;
        }
        if entry.fields.get("url").is_none() {
            log::error!("Missing URL for {name}.");
            data_missing = true;
        }
        if let Some(crossrefs) = entry.fields.get("crossref") {
            assert_eq!(
                crossrefs.len(),
                1,
                "while processing {name}: expected exactly one crossref entry!"
            );
            for crossref in crossrefs[0].v.get().split(',') {
                let crossref = crossref.trim();
                assert!(
                    crossref
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '-'),
                    "while processing {name}: expected only keywords containing alphanumerics and hyphens!"
                );
                assert!(
                    !crossref.is_empty(),
                    "while processing {name}: encountered empty keyword"
                );
                log::debug!("{name}: {crossref}");
                refmap.add_edge(name, crossref, ());
            }
        }
    }

    log::debug!("{:#?}", keyword_map);
    log::debug!("{:#?}", author_map);
    log::debug!("{:#?}", refmap);

    if data_missing {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
