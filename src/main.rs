use brco_pas::models::{SubmittedProposal,RequestType};


const DB_FILEPATH: &str = "db/test_db_one.db";

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let submission_one = SubmittedProposal::new(
    String::from("123456"),
    String::from("Test Customer One"),
    String::from("ABC"),
    String::from("100"),
    vec![RequestType::Increase,RequestType::AnnualReview],
  );

  println!("{:?}",submission_one);

  let request_type = submission_one.get_request_type();

  println!("{:?}",request_type);


  Ok(())

}