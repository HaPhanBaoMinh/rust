use std::{collections::HashMap, fs::read_to_string, ops::Index};

fn main() {
    // Argument
    // let search_value_arg = std::env::args().nth(1).expect("No log level given");
    let path_arg = std::env::args().nth(1).expect("No path given");

    let file_content = read_to_string(path_arg).unwrap();
    let docs_vec: Vec<&str> = file_content.split("\n").collect();
    let mut inverted_index: HashMap<(String, usize), TermStats> = HashMap::new();

    tf(docs_vec);
}

// TermStats
struct TermStats {
    tf: String,
    idf: String,
    ifidf: String
}

fn analyzer(doc: &str) -> Vec<&str> {
    doc.split_whitespace().collect()
}

fn tf(docs:Vec<&str>) -> HashMap<(usize, &str), f64> {
    let mut result: HashMap<(usize, &str), f64> = HashMap::new();

    for (index, doc) in docs.iter().enumerate() {
        let tokens = analyzer(doc);

        for &word in &tokens {
            let token_count = tokens.len();
            let count = tokens.iter().filter(|candidate| **candidate == word).count();
            let tf = count as f64 / token_count as f64;
            println!("Doc: {} - Word: {} - TF: {}", index, word, tf);
            result.insert((index,word), tf);
        };
    };
    result
}


fn idf(docs:Vec<&str>) -> HashMap<String, f64> {
    let mut result: HashMap<String, f64> = HashMap::new();

    for (index, doc) in docs.iter().enumerate() {
        let tokens = analyzer(doc);
        let doc_count = docs.len();

        for &word in &tokens {
            let count = doc.contains(&word);
        };
    };

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
