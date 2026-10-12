use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
};
use std::{fs::read_to_string, time};

fn main() {
    // Argument
    // let search_value_arg = std::env::args().nth(1).expect("No log level given");
    let path_arg = std::env::args().nth(2).expect("No path given");

    // Benmark
    let now = time::Instant::now();

    let file_content = read_to_string(path_arg).unwrap();
    let logs: Vec<&str> = file_content.split("\n").collect();

    let _ = tf(logs);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn analyzer(doc: &str) -> Vec<&str> {
    doc.split(" ").collect()
}

fn tf(docs: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut total_word_in_docs: usize = 0;

    let mut tf_list: HashMap<&str, i64> = HashMap::new();
    let mut idf_list: HashMap<&str, i64> = HashMap::new();

    for doc in &docs {
        let mut words = analyzer(doc);

        total_word_in_docs += words.len();

        for &word in &words {
            let count = tf_list.entry(word).or_insert(0);
            *count += 1;
        }

        // dedup() chỉ loại bỏ phần tử trùng nằm cạnh nhau.
        words.sort_unstable();
        words.dedup();

        for &word in &words {
            let count = idf_list.entry(word).or_insert(0);
            *count += 1;
        }
    }

    tf_list.insert("TOTAL_WORD", total_word_in_docs as i64);
    tf_list.insert("TOTAL_DOC", docs.len() as i64);

    update_values("./db.toml", tf_list, "")?;
    update_values("./db.toml", idf_list, "idf_")?;
    Ok(())
}

pub fn read_value(file_path: &str, key: &str) -> Result<Option<f64>, Box<dyn Error>> {
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        }

        Err(error) => return Err(error.into()),
    };

    let table: toml::Table = toml::from_str(&content)?;

    match table.get(key) {
        None => Ok(None),

        Some(toml::Value::Float(value)) => Ok(Some(*value)),

        // Cho phép đọc cả giá trị TOML dạng integer rồi chuyển sang f64
        Some(toml::Value::Integer(value)) => Ok(Some(*value as f64)),

        Some(value) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Key '{key}' không chứa số: {value:?}"),
        )
        .into()),
    }
}

fn update_values<'a>(
    file_path: &str,
    values: HashMap<&'a str, i64>,
    prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let mut table: toml::Table = match fs::read_to_string(file_path) {
        Ok(content) => toml::from_str(&content)?,

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => toml::Table::new(),

        Err(error) => return Err(error.into()),
    };

    for (key, value) in values {
        let output_key = format!("{prefix}{key}");

        table.insert(output_key, toml::Value::Integer(value));
    }

    let content = toml::to_string_pretty(&table)?;
    fs::write(file_path, content)?;

    Ok(())
}
