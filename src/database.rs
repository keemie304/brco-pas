
use rusqlite::{Connection,Result,params,TransactionBehavior};
use crate::models::{SubmittedProposal,CreditProposal,RequestType};

pub fn initialise_database(filepath: &str) -> Result<Connection> {
  let conn = Connection::open(filepath)?;

  conn.execute_batch("PRAGMA foreign_keys=ON")?;

  conn.execute_batch(
    "CREATE TABLE IF NOT EXISTS CreditProposal (
    credit_proposal_id TEXT PRIMARY KEY,
    customer_name TEXT NOT NULL,
    customer_cif TEXT NOT NULL,
    ao_name TEXT NOT NULL,
    ao_code TEXT NOT NULL
    );
    

    CREATE TABLE IF NOT EXISTS proposal_id_sequences (
      registration_year INTEGER PRIMARY KEY,
      last_sequence INTEGER NOT NULL
        CHECK (last_sequence BETWEEN 1 AND 9999)
        );
        
    CREATE TABLE IF NOT EXISTS ProposalRequestTypes (
      credit_proposal_id TEXT NOT NULL,
      request_type TEXT NOT NULL,

      PRIMARY KEY (credit_proposal_id, request_type),

      FOREIGN KEY (credit_proposal_id)
        REFERENCES CreditProposal(credit_proposal_id)  
      );
      "
  )?;

  Ok(conn)
}

pub fn create_entry(
  conn: &mut Connection,
  proposal: &SubmittedProposal,
  year:i32,
) -> Result<CreditProposal> {

  let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
  let sequence = next_sequence(&tx, year)?;
  let proposal_id =  format!(
    "CB{:02}{:04}",
    year % 100,
    sequence,
  );

  tx.execute (
    "INSERT INTO CreditProposal (credit_proposal_id,customer_cif,customer_name,ao_name,ao_code) VALUES (?1,?2,?3,?4,?5)",
    params![
      &proposal_id,
      proposal.get_submitted_cif(),
      proposal.get_submitted_name(),
      proposal.get_submitted_ao_name(),
      proposal.get_submitted_ao_code(),
    ],
  )?;

  for request_type in proposal.get_submitted_request_type() {
    tx.execute(
        " INSERT INTO ProposalRequestTypes (
          credit_proposal_id,
          request_type
        ) VALUES (?1,?2)",
        params![
          &proposal_id,
          request_type.as_str(),
        ],
    )?;
  }
  tx.commit()?;

  println!("Credit proposal logged. Proposal ID is {}",proposal_id);
  Ok(CreditProposal::new(
    proposal_id, proposal.get_submitted_cif().to_string(), proposal.get_submitted_name().to_string(),proposal.get_submitted_ao_name().to_string(),proposal.get_submitted_ao_code().to_string(),proposal.get_submitted_request_type().clone()
    
    ))
}


pub fn query_proposal(
    conn: &Connection,
    proposal_id: &str,
) -> Result<CreditProposal> {

    // 1. Retrieve the main credit proposal
    let (id, cif, name, ao_name, ao_code):
        (String, String, String, String, String) = conn.query_row(
        "SELECT
            credit_proposal_id,
            customer_cif,
            customer_name,
            ao_name,
            ao_code
         FROM CreditProposal
         WHERE credit_proposal_id = ?1",
        [proposal_id],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        },
    )?;

    // 2. Retrieve all request types for this proposal
    let mut stmt = conn.prepare(
        "SELECT request_type
         FROM ProposalRequestTypes
         WHERE credit_proposal_id = ?1
         ORDER BY request_type"
    )?;

    // 3. Convert database strings into RequestType variants
    let request_types: Vec<RequestType> = stmt
        .query_map([proposal_id], |row| {
            let value: String = row.get(0)?;

            let request_type = match value.as_str() {
                "New" => RequestType::New,
                "Increase" => RequestType::Increase,
                "Reduction" => RequestType::Reduction,
                "Amendment" => RequestType::Amendment,
                "Cancellation" => RequestType::Cancellation,
                "Annual Review" => RequestType::AnnualReview,
                "Interim Review" => RequestType::InterimReview,
                "Discussion" => RequestType::Discussion,

                _ => return Err(
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Unknown request type: {value}"),
                        )),
                    )
                ),
            };

            Ok(request_type)
        })?
        .collect::<Result<Vec<RequestType>>>()?;

    // 4. Reconstruct the complete CreditProposal
    Ok(CreditProposal::new(
        id,
        cif,
        name,
        ao_name,
        ao_code,
        request_types,
    ))
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