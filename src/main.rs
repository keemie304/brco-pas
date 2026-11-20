use brco_pas::models::{SubmittedProposal,RequestType};
use brco_pas::parser::{parse_pdf};


const DB_FILEPATH: &str = "db/test_db_one.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let filepath = parse_pdf("sample_data/sample_proposal_five_v2.pdf")?;

  println!("{:?}",filepath);

  Ok(())

}