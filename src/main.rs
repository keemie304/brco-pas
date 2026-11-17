use brco_pas::models::{CreditProposal,Submission,ProposalIDGenerator};

fn main() {

  let mut generator = ProposalIDGenerator::new(2026, 1);


  let test_proposal = CreditProposal::new(
    String::from("123456"),
    String::from("Test Company Sdn Bhd"),
    &mut generator,
  ).expect("Failed to generate credit proposal");

  println!("{:#?}",test_proposal);

  let test_submission = Submission::new(
    String::from("Test AO"),
    100,
    String::from("17Nov2026"),
    test_proposal,
  );

  println!("{:#?}",test_submission);
}