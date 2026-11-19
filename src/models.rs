/*
This file contains the datastructs and its associated functions that will be used throughout the program.
Throughout the program, the Primary identifier will be Credit_Proposal_ID which is in the format: CBYYXXXX
*/

#[derive(Debug)]
pub struct CreditProposal {
  credit_proposal_id: String,
  customer_cif: String,
  customer_name: String,
 
}
impl CreditProposal {
  pub fn new (
    credit_proposal_id:String,
    customer_cif: String,
    customer_name: String,
  ) -> Self {
   Self{customer_cif,customer_name,credit_proposal_id}
  }
  pub fn get_cif(&self) -> &str {
    &self.customer_cif
  }
  pub fn get_name(&self) -> &str {
    &self.customer_name
  }
  pub fn get_proposal_id(&self) -> &str {
    &self.credit_proposal_id
  }
  pub fn from_database(
    credit_proposal_id:String,
    customer_cif:String,
    customer_name:String,
  ) -> Self {
    Self {
      credit_proposal_id,
      customer_cif,
      customer_name,
    }
  }
}

#[derive(Debug)]
pub struct Submission {
  ao_name: String,
  ao_code: u16,
  submission_date: String,
  proposal: CreditProposal,
  approver: Approver,
}

impl Submission {
  pub fn new (
    ao_name: String,
    ao_code: u16,
    submission_date: String,
    proposal: CreditProposal,
    approver: Approver,
  ) -> Self {
    Self{ao_name, ao_code,submission_date,proposal,approver}
  }

  pub fn get_approver(&self) -> Approver {
        self.approver
    }
  pub fn proposal(&self) -> &CreditProposal {
    &self.proposal
  }
}

#[derive(Debug)]
pub struct PDFExtract {
  extract_customer_name: String,
  extract_customer_cif: String,
}
impl PDFExtract {
  pub fn new (
    extract_customer_name: String,
    extract_customer_cif: String,
  ) -> Self {
    Self{extract_customer_name,extract_customer_cif}
  }
  pub fn get_extracted_cif(&self) -> &str {
    &self.extract_customer_cif
  }
  pub fn get_extracted_name(&self) -> &str {
    &self.extract_customer_name
  }
}

//declaring enums for immutable fields
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approver {
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