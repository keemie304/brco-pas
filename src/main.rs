use brco_pas::models::{CreditProposal,Submission,Approver,SubmittedProposal};
use brco_pas::parser::parse_pdf;
use brco_pas::validation::{validate_cif,validate_name,validate};
use brco_pas::database::{initialise_database,create_entry,query_proposal};
use rusqlite::{Connection,Result,params};

fn main() -> Result<()> {

  let submitted_proposal = SubmittedProposal::new(String::from("123456"),String::from("test customer one"));


  let mut conn = initialise_database("db/brco_pas.db")?;

  let proposal_id = create_entry(&mut conn, &submitted_proposal, 2026)?;


  let proposal = query_proposal(&conn, "CB260001")?;

  println!("{:#?}",proposal);


  Ok(())
}