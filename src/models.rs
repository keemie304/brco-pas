/*
This file contains the datastructs and its associated functions that will be used throughout the program.
Throughout the program, the Primary identifier will be Credit_Proposal_ID which is in the format: CBYYXXXX
*/
#[derive(Debug)]
pub struct SubmittedProposal {
  customer_cif: String,
  customer_name: String,
  ao_name: String,
  ao_code: String,
  submission_type: Vec<RequestType>,
}
impl SubmittedProposal {
  pub fn new (
    customer_cif: String,
    customer_name: String,
    ao_name: String,
    ao_code: String,
    submission_type: Vec<RequestType>,
  ) -> Self {
    Self{customer_cif,customer_name,ao_name,ao_code, submission_type}
  }
  pub fn get_submitted_cif(&self) -> &str {
    &self.customer_cif
  }
  pub fn get_submitted_name(&self) -> &str {
    &self.customer_name
  }
  pub fn get_submitting_ao_name(&self) -> &str {
    &self.ao_name
  }
  pub fn get_submitting_ao_code(&self) -> &str {
    &self.ao_code
  }
  pub fn get_submission_type(&self) -> &Vec<RequestType> {
    &self.submission_type
  }
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VerificationStatus {
  Verified,
  Mismatch,
  ManualReview,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubmissionStatus {
  Accepted,
  RequiresAmendment,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
  New,
  Increase,
  Reduction,
  Amendment,
  Cancellation,
  AnnualReview,
  InterimReview,
  Discussion,
}