use std::{collections::{HashMap, HashSet}, fs::read_to_string };

mod db;

fn main() {
    // Argument
    // let search_value_arg = std::env::args().nth(1).expect("No log level given");
    let path_arg = std::env::args().nth(1).expect("No path given");

    let file_content = read_to_string(path_arg).unwrap();
    let docs_vec: Vec<&str> = file_content.lines().filter(|l| !l.trim().is_empty()).collect();

    tf(&docs_vec);
    df(&docs_vec);
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
            println!("Doc: {} - Word: {} - TF: {}", index, word, tf);
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
        };
    };

    db::save_df(db::DB_PATH, &result).unwrap();
    result
}


// Database table: one row per (term, document) pair
// Term | Doc_id | TF | IDF | TF-IDF | token_count
// - Term:   token produced by `analyzer`
// - Doc_id: line index of the document in the log file
// - TF:     term count in doc / total terms in doc
// - IDF:    ln(total docs / docs containing term)
// - TF-IDF: TF * IDF, used to rank search results
// - token_count: Total token in 1 doc
