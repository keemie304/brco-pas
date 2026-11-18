use rusqlite::{Connection,Result,params};
use crate::models::{CreditProposal};

pub fn initialise_database(filepath: &str) -> Result<Connection> {
  let conn = Connection::open(filepath)?;

  conn.execute(
    "CREATE TABLE IF NOT EXISTS CreditProposal (
    customer_cif TEXT NOT NULL,
    customer_name TEXT NOT NULL,
    credit_proposal_id TEXT PRIMARY KEY
    )",
    [],
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
      proposal.get_cif(),
      proposal.get_name(),
      proposal.get_proposal_id()
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
