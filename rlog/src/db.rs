use std::collections::HashMap;

use rusqlite::{Connection, params};

pub const DB_PATH: &str = "data/search_index.db";

// Rebuilds the docs and tf tables from scratch.
// - docs: one row per log line
// - tf:   one row per (term, doc) pair
pub fn save_tf(
    path: &str,
    docs: &[&str],
    tf_map: &HashMap<(usize, String), f64>,
) -> rusqlite::Result<()> {
    let mut conn = Connection::open(path)?;

    conn.execute_batch(
        "DROP TABLE IF EXISTS postings;
         DROP TABLE IF EXISTS docs;
         DROP TABLE IF EXISTS tf;
         CREATE TABLE docs (
             doc_id  INTEGER PRIMARY KEY,
             content TEXT NOT NULL
         );
         CREATE TABLE tf (
             term   TEXT    NOT NULL,
             doc_id INTEGER NOT NULL,
             tf     REAL    NOT NULL,
             PRIMARY KEY (term, doc_id)
         );",
    )?;

    // One transaction for all inserts, otherwise every row is a separate disk write
    let tx = conn.transaction()?;
    {
        let mut insert_doc = tx.prepare("INSERT INTO docs (doc_id, content) VALUES (?1, ?2)")?;
        for (doc_id, content) in docs.iter().enumerate() {
            insert_doc.execute(params![doc_id as i64, content])?;
        }

        let mut insert_tf = tx.prepare("INSERT INTO tf (term, doc_id, tf) VALUES (?1, ?2, ?3)")?;
        for ((doc_id, term), tf) in tf_map {
            insert_tf.execute(params![term, *doc_id as i64, tf])?;
        }
    }
    tx.commit()
}

// Rebuilds the df table from scratch.
// - df: one row per term, number of docs containing it
pub fn save_df(path: &str, df_map: &HashMap<String, usize>) -> rusqlite::Result<()> {
    let mut conn = Connection::open(path)?;

    conn.execute_batch(
        "DROP TABLE IF EXISTS df;
         CREATE TABLE df (
             term TEXT    PRIMARY KEY,
             df   INTEGER NOT NULL
         );",
    )?;

    let tx = conn.transaction()?;
    {
        let mut insert_df = tx.prepare("INSERT INTO df (term, df) VALUES (?1, ?2)")?;
        for (term, df) in df_map {
            insert_df.execute(params![term, *df as i64])?;
        }
    }
    tx.commit()
}
