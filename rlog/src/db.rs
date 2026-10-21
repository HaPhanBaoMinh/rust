use std::collections::HashMap;

use rusqlite::{Connection, OptionalExtension, params};

pub const DB_PATH: &str = "data/search_index.db";

// Rebuilds the docs, tf and meta tables from scratch.
// - docs: one row per log line
// - tf:   one row per (term, doc) pair
// - meta: index-wide values, e.g. total_docs (N in idf)
pub fn save_tf(
    path: &str,
    docs: &[&str],
    tf_map: &HashMap<(usize, String), f64>,
) -> rusqlite::Result<()> {
    let mut conn = Connection::open(path)?;

    // The index is always rebuilt from scratch, so a crash mid-build only means re-running index
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
         PRAGMA synchronous = OFF;",
    )?;

    // WITHOUT ROWID: rows are stored ordered by (term, doc_id),
    // so all postings of one term sit next to each other on disk
    conn.execute_batch(
        "DROP TABLE IF EXISTS postings;
         DROP TABLE IF EXISTS docs;
         DROP TABLE IF EXISTS tf;
         DROP TABLE IF EXISTS meta;
         CREATE TABLE docs (
             doc_id  INTEGER PRIMARY KEY,
             content TEXT NOT NULL
         );
         CREATE TABLE tf (
             term   TEXT    NOT NULL,
             doc_id INTEGER NOT NULL,
             tf     REAL    NOT NULL,
             PRIMARY KEY (term, doc_id)
         ) WITHOUT ROWID;
         CREATE TABLE meta (
             key   TEXT    PRIMARY KEY,
             value INTEGER NOT NULL
         );",
    )?;

    // Insert in primary key order so SQLite appends to the B-tree instead of splitting pages
    let mut tf_rows: Vec<(&str, usize, f64)> = tf_map
        .iter()
        .map(|((doc_id, term), tf)| (term.as_str(), *doc_id, *tf))
        .collect();
    tf_rows.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));

    // One transaction for all inserts, otherwise every row is a separate disk write
    let tx = conn.transaction()?;
    {
        let mut insert_doc = tx.prepare("INSERT INTO docs (doc_id, content) VALUES (?1, ?2)")?;
        for (doc_id, content) in docs.iter().enumerate() {
            insert_doc.execute(params![doc_id as i64, content])?;
        }

        let mut insert_tf = tx.prepare("INSERT INTO tf (term, doc_id, tf) VALUES (?1, ?2, ?3)")?;
        for (term, doc_id, tf) in tf_rows {
            insert_tf.execute(params![term, doc_id as i64, tf])?;
        }

        // Stored once here so search does not need COUNT(*) over docs
        tx.execute(
            "INSERT INTO meta (key, value) VALUES ('total_docs', ?1)",
            params![docs.len() as i64],
        )?;
    }
    tx.commit()
}

// Rebuilds the df table from scratch.
// - df: one row per term, number of docs containing it
pub fn save_df(path: &str, df_map: &HashMap<String, usize>) -> rusqlite::Result<()> {
    let mut conn = Connection::open(path)?;

    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
         PRAGMA synchronous = OFF;
         DROP TABLE IF EXISTS df;
         CREATE TABLE df (
             term TEXT    PRIMARY KEY,
             df   INTEGER NOT NULL
         ) WITHOUT ROWID;",
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
    let mut stmt = conn.prepare_cached("SELECT df FROM df WHERE term = ?1")?;
    stmt.query_row(params![term], |row| row.get(0)).optional()
}

// Posting list of `term`: every (doc_id, tf) pair where the term appears.
pub fn get_tf(conn: &Connection, term: &str) -> rusqlite::Result<Vec<(i64, f64)>> {
    let mut stmt = conn.prepare_cached("SELECT doc_id, tf FROM tf WHERE term = ?1")?;
    let rows = stmt.query_map(params![term], |row| Ok((row.get(0)?, row.get(1)?)))?;
    rows.collect()
}

// Total number of indexed docs (N in idf = ln(N / df)), saved by save_tf.
pub fn total_docs(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row("SELECT value FROM meta WHERE key = 'total_docs'", [], |row| row.get(0))
}

// Original log line of `doc_id`, or None if no such doc.
pub fn get_doc(conn: &Connection, doc_id: i64) -> rusqlite::Result<Option<String>> {
    let mut stmt = conn.prepare_cached("SELECT content FROM docs WHERE doc_id = ?1")?;
    stmt.query_row(params![doc_id], |row| row.get(0)).optional()
}
