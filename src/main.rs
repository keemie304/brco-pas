use brco_pas::models::{CreditProposal,Submission,ProposalIDGenerator,Approver};
use brco_pas::parser::parse_pdf;
use brco_pas::validation::{validate_cif,validate_name,validate};


fn main() -> Result<(), Box<dyn std::error::Error>> {

  let result_one = parse_pdf("sample_data/sample_credit_proposal_one.pdf")?;

  println!("{:#?}",result_one);

  let result_two = parse_pdf("sample_data/sample_credit_proposal_two.pdf")?;

  println!("{:#?}",result_two);

  let result_three = parse_pdf("sample_data/sample_credit_proposal_three.pdf")?;

  println!("{:#?}",result_three);

  let mut generator = ProposalIDGenerator::new(2026, 1);


  let test_proposal = CreditProposal::new(
    String::from("002834"),
    String::from("Test Customer Three"),
    &mut generator,
  ).expect("Failed to generate credit proposal");

  println!("{:#?}",test_proposal);

  let test_submission = Submission::new(
    String::from("Test AO"),
    100,
    String::from("17Nov2026"),
    test_proposal,
    Approver::GCC,
  );

  let matches = validate_cif(&test_submission, &result_two);
  println!("{}",matches);

  let name_match = validate_name(&test_submission,&result_one);

  println!("{}",name_match);

  let test = validate(&test_submission, &result_three);
  println!("{}",test);
  
   
  Ok(())
}