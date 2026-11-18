use brco_pas::models::{CreditProposal,Submission,ProposalIDGenerator,Approver};
use brco_pas::parser::parse_pdf;
use brco_pas::validation::{validate_cif,validate_name,validate};
use brco_pas::database::{initialise_database,create_entry,query_proposal};
use rusqlite::{Connection,Result,params};

fn main() -> Result<()> {
  let conn = initialise_database("db/brco_pas.db")?;


  let mut generator = ProposalIDGenerator::new(2026, 1);

  let credit_proposal_one = CreditProposal::new(
    String::from("123456"),
    String::from("Test Company Sdn Bhd"),
    &mut generator,
  ).expect("Failed to generate credit proposal");


  let proposal = query_proposal(&conn, &credit_proposal_one)?;

  println!("{:#?}",proposal);


  Ok(())
}