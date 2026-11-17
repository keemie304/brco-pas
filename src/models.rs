// File contains all the data structures required by the PAS

#[derive(Debug)]
pub struct CreditProposal {
  customer_cif: String,
  customer_name: String,
  credit_proposal_id: String,
}
impl CreditProposal {
  pub fn new (
    customer_cif: String,
    customer_name: String,
    generator: &mut ProposalIDGenerator,
  ) -> Result<Self, &'static str> {
    let credit_proposal_id: String = generator.generate()?;

    Ok(Self{customer_cif,customer_name,credit_proposal_id})
  }
}

#[derive(Debug)]
pub struct Submission {
  ao_name: String,
  ao_code: u16,
  submission_date: String,
  proposal: CreditProposal,
}

impl Submission {
  pub fn new (
    ao_name: String,
    ao_code: u16,
    submission_date: String,
    proposal: CreditProposal,
  ) -> Self {
    Self{ao_name, ao_code,submission_date,proposal}
  }
}

#[derive(Debug)]
pub struct PDFExtract {
  extract_customer_name: String,
  extract_customer_cif: String,
}

#[derive(Debug)]
pub struct ProposalIDGenerator {
  year: u16,
  sequence: u32,
}
impl ProposalIDGenerator {
  pub fn new(year: u16, sequence:u32) -> Self {
    Self {
      year,
      sequence,
    }
  }
  fn generate(&mut self) -> Result< String, &'static str> {
    if !(2000..=2099).contains(&self.year) {
      return Err("Invalid year");
    }
    if !(1..=9999).contains(&self.sequence) {
      return Err("Annual sequence exhausted");
    }

    let number = format!("CB {:02}{:04}",self.year%100,self.sequence);

    self.sequence +=1;

    Ok(number)

  }
}



//declaring enums for immutable fields
enum Approver {
  GCC,
  CCC,
  EXCO,
}

enum VerificationStatus {
  Verified,
  Mismatch,
  ManualReview,
}

enum SubmissionStatus {
  Accepted,
  RequiresAmendment,
}