use rusqlite::{Connection,Result,params,TransactionBehavior};
use crate::models::{CreditProposal};

pub fn initialise_database(filepath: &str) -> Result<Connection> {
  let conn = Connection::open(filepath)?;

  conn.execute_batch(
    "CREATE TABLE IF NOT EXISTS CreditProposal (
    credit_proposal_id TEXT NOT NULL UNIQUE PRIMARY KEY,
    customer_name TEXT NOT NULL,
    customer_cif TEXT NOT NULL);

    CREATE TABLE IF NOT EXISTS proposal_id_sequences (
      registration_year INTEGER PRIMARY KEY,
      last_sequence INTEGER NOT NULL
        CHECK (last_sequence BETWEEN 1 AND 9999);"
  )?;

  Ok(conn)
}

pub fn create_entry(
  conn: &Connection,
  proposal: &CreditProposal,
) -> Result<usize> {
  conn.execute(
    "INSERT INTO CreditProposal (customer_cif,customer_name,credit_proposal_id) VALUES (?1, ?2, ?3)",
    params![
      proposal.get_proposal_id(),
      proposal.get_cif(),
      proposal.get_name(),
    ],
  )
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

//declaring non publi