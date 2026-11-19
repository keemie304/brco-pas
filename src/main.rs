use brco_pas::models::{SubmittedProposal};
use brco_pas::parser::parse_pdf;
use brco_pas::validation::{validate};
use brco_pas::database::{initialise_database,create_entry,query_proposal};

const DB_FILEPATH: &str = "db/test_db_one.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let submission_one = SubmittedProposal::new(
    String::from("002834"),
    String::from("Test Customer Three")
  );

  let sample_one = parse_pdf("sample_data/sample_credit_proposal_three.pdf")?;

  let checks = validate(&submission_one,&sample_one);

  if checks {
    let mut conn = initialise_database(DB_FILEPATH)?;

    let proposal = create_entry(&mut conn, &submission_one, 2026)?;

    println!("Proposal has been registered in database");

    println!("The proposal is seen here {:?}",proposal);

    let retrieved = query_proposal(&conn, proposal.get_proposal_id())?;

    println!("Retrieved proposal: {:?}",retrieved);

    assert_eq!(
      proposal.get_proposal_id(),
      retrieved.get_proposal_id(),
    );
  } else {
    println!("Cannot input the proposal into the database");
  }


  Ok(())

}