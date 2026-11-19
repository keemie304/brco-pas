use rusqlite::{Connection,Result,params,TransactionBehavior};
use crate::models::{CreditProposal};

pub fn initialise_database(filepath: &str) -> Result<Connection> {
  let conn = Connection::open(filepath)?;

  conn.execute_batch(
    "CREATE TABLE IF NOT EXISTS CreditProposal (
    credit_proposal_id TEXT PRIMARY KEY,
    customer_name TEXT NOT NULL,
    customer_cif TEXT NOT NULL);

    CREATE TABLE IF NOT EXISTS proposal_id_sequences (
      registration_year INTEGER PRIMARY KEY,
      last_sequence INTEGER NOT NULL
        CHECK (last_sequence BETWEEN 1 AND 9999)
        );"
  )?;

  Ok(conn)
}

pub fn create_entry(
  conn: &mut Connection,
  proposal: &CreditProposal,
  year:i32,
) -> Result<String> {

  let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
  let sequence = next_sequence(&tx, year)?;
  let proposal_id =  format!(
    "CB{:02}{:04}",
    year % 100,
    sequence,
  );

  tx.execute (
    "INSERT INTO CreditProposal (credit_proposal_id,customer_cif,customer_name) VALUES (?1,?2,?3)",
    params![
      &proposal_id,
      proposal.get_cif(),
      proposal.get_name(),
    ],
  )?;
  tx.commit()?;

  println!("Credit proposal logged. Proposal ID is {}",sequence);
  Ok(proposal_id)
}

pub fn query_proposal(
  conn: &Connection,
  proposal: &CreditProposal,
) -> Result<CreditProposal> {
  let proposal_query = conn.query_row(
      "SELECT customer_cif, customer_name, credit_proposal_id
      FROM CreditProposal
      WHERE credit_proposal_id = ?1",
      [proposal.get_proposal_id()],
      |row| {
          Ok(CreditProposal::from_database(
              row.get(0)?,
              row.get(1)?,
              row.get(2)?,
          ))
      },
  )?;

  Ok(proposal_query)
}

//declaring non public functions for the proposal id generator
fn next_sequence (
  tx: &rusqlite::Transaction<'_>,
  year: i32,
) -> rusqlite::Result<i64> {
  let sequence: i64 = tx.query_row(
    "INSERT INTO proposal_id_sequences (
    registration_year,
    last_sequence)
    VALUES(?1,1)
    ON CONFLICT(registration_year)
    DO UPDATE SET last_sequence = last_sequence +1
    RETURNING last_sequence",
    params![year],
    |row| row.get(0),
  )?;
  Ok(sequence)
}