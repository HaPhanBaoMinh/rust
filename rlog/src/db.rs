use std::collections::HashMap;

use rusqlite::{Connection, OptionalExtension, params};

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

// Number of docs containing `term`, or None if the term was never indexed.
pub fn get_df(conn: &Connection, term: &str) -> rusqlite::Result<Option<i64>> {
    conn.query_row("SELECT df FROM df WHERE term = ?1", params![term], |row| row.get(0))
        .optional()
}

// Posting list of `term`: every (doc_id, tf) pair where the term appears.
pub fn get_tf(conn: &Connection, term: &str) -> rusqlite::Result<Vec<(i64, f64)>> {
    let mut stmt = conn.prepare("SELECT doc_id, tf FROM tf WHERE term = ?1")?;
    let rows = stmt.query_map(params![term], |row| Ok((row.get(0)?, row.get(1)?)))?;
    rows.collect()
}

// Total number of indexed docs (N in idf = ln(N / df)).
pub fn total_docs(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row("SELECT COUNT(*) FROM docs", [], |row| row.get(0))
}

// Original log line of `doc_id`, or None if no such doc.
pub fn get_doc(conn: &Connection, doc_id: i64) -> rusqlite::Result<Option<String>> {
    conn.query_row("SELECT content FROM docs WHERE doc_id = ?1", params![doc_id], |row| row.get(0))
        .optional()
}
