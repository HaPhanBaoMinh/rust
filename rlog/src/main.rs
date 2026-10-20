use std::{collections::{HashMap, HashSet}, fs::read_to_string, time::Instant};
use rusqlite::{Connection};

use crate::db::{get_doc, get_tf, total_docs};
use crate::db::get_df;

mod db;

fn main() {
    // Argument
    // let search_value_arg = std::env::args().nth(1).expect("No log level given");
    let option= std::env::args().nth(1).expect("No option given");

    match option.as_str() {
        "index" => {
            let path_arg = std::env::args().nth(2).expect("No path given");
            let file_content = read_to_string(path_arg).unwrap();
            let docs_vec: Vec<&str> = file_content.lines().filter(|l| !l.trim().is_empty()).collect();

            tf(&docs_vec);
            df(&docs_vec);
        }

        "search" => {
            let search_values = std::env::args().nth(2).expect("No value given");
            let mode = std::env::args().nth(3);

            // search "<query>"                    -> TF-IDF search on the index
            // search "<query>" --no-index <file>  -> loop over every line of the log file
            let start = Instant::now();
            let results = match mode.as_deref() {
                Some("--no-index") => {
                    let path_arg = std::env::args().nth(4).expect("No path given");
                    scan_search(&path_arg, &search_values)
                }
                _ => search(&search_values),
            };
            let elapsed = start.elapsed();

            if results.is_empty() {
                println!("No results");
            }
            for line in &results {
                println!("{}", line);
            }
            println!("{} results in {:.2?}", results.len(), elapsed);
        }

        _ => {
            eprintln!("index | search")
        }

    }
}

// Splits on any non-alphanumeric char and lowercases, so "[error]" and "Error" both become "error".
// Search queries must go through the same analyzer.
fn analyzer(doc: &str) -> Vec<String> {
    doc.split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_lowercase())
        .collect()
}

fn tf(docs: &[&str]) -> HashMap<(usize, String), f64> {
    let mut result: HashMap<(usize, String), f64> = HashMap::new();

    for (index, doc) in docs.iter().enumerate() {
        let tokens = analyzer(doc);

        for word in &tokens {
            let token_count = tokens.len();
            let count = tokens.iter().filter(|candidate| *candidate == word).count();
            let tf = count as f64 / token_count as f64;
            result.insert((index, word.clone()), tf);
        };
    };

    db::save_tf(db::DB_PATH, docs, &result).unwrap();
    result
}


fn df(docs: &[&str]) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    for doc in docs {
        let tokens: HashSet<String> = analyzer(doc).into_iter().collect();
        for word in tokens {
            *result.entry(word).or_insert(0) += 1;
        }
    };

    db::save_df(db::DB_PATH, &result).unwrap();
    result
}

// Search without the index: read the log file and check every line.
// A line matches when it contains all tokens of the query.
fn scan_search(path: &str, value: &str) -> Vec<String> {
    let mut search_result = Vec::new();
    let file_content = read_to_string(path).unwrap();
    let query_tokens = analyzer(value);

    if query_tokens.is_empty() {
        return search_result;
    }

    for line in file_content.lines() {
        let line_tokens: HashSet<String> = analyzer(line).into_iter().collect();

        let mut is_match = true;
        for token in &query_tokens {
            if !line_tokens.contains(token) {
                is_match = false;
                break;
            }
        }

        if is_match {
            search_result.push(line.to_string());
        }
    }

    search_result
}

fn search(value: &str) -> Vec<String> {
    let mut scores: HashMap<i64, f64> = HashMap::new();
    let conn = Connection::open(db::DB_PATH).expect("Cannot open DB");

    let tokens = analyzer(value);
    let total_docs = total_docs(&conn).expect("Query total docs failed");

    for token in tokens  {
       let token_tf = get_tf(&conn, &token).expect("Query tf failed");
       let df_result = get_df(&conn, &token).expect("Query df failed");

       let token_df = match df_result {
           Some(df) => df,       // term found in the index
           None => continue,     // term not in the index: no doc contains it, skip
       };
       let idf = (total_docs as f64 / token_df as f64).ln();

       for (doc_id, tf) in token_tf {
         let tfidf = tf * idf;
         *scores.entry(doc_id).or_insert(0.0) += tfidf;
      }
    }

    let mut ranked: Vec<(i64, f64)> = scores.into_iter().collect();
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1));
    ranked.truncate(10);

    let mut search_result = Vec::new();
    for (doc_id, score) in ranked {
        let doc = get_doc(&conn, doc_id).expect("Query doc failed");
        if let Some(content) = doc {
            search_result.push(format!("[{:.3}] {}", score, content));
        }
    }

    search_result
}


// Database table: one row per (term, document) pair
// Term | Doc_id | TF | IDF | TF-IDF | token_count
// - Term:   token produced by `analyzer`
// - Doc_id: line index of the document in the log file
// - TF:     term count in doc / total terms in doc
// - IDF:    ln(total docs / docs containing term)
// - TF-IDF: TF * IDF, used to rank search results
// - token_count: Total token in 1 doc
