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
  request_type: Vec<RequestType>,
}
impl SubmittedProposal {
  pub fn new (
    customer_cif: String,
    customer_name: String,
    ao_name: String,
    ao_code: String,
    request_type: Vec<RequestType>,
  ) -> Self {
    Self{customer_cif,customer_name,ao_name,ao_code,request_type}
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
  pub fn get_request_type(&self) -> &Vec<RequestType> {
    &self.request_type
  }
}

#[derive(Debug)]
pub struct CreditProposal {
  credit_proposal_id: String,
  customer_cif: String,
  customer_name: String,
  ao_name: String,
  ao_code:String,
  request_type:Vec<RequestType>,
}
impl CreditProposal {
  pub fn new (
    credit_proposal_id:String,
    customer_cif: String,
    customer_name: String,
    ao_name: String,
    ao_code:String,
    request_type:Vec<RequestType>,
  ) -> Self {
   Self{customer_cif,customer_name,credit_proposal_id,ao_name,ao_code,request_type}
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
  pub fn get_proposal_ao_name(&self) -> &str {
    &self.ao_name
  }
  pub fn get_proposal_ao_code(&self) -> &str {
    &self.ao_code
  }
  pub fn get_proposal_request_type(&self) -> &Vec<RequestType> {
    &self.request_type
  }
}

#[derive(Debug)]
pub struct PDFExtract {
  extract_customer_name: String,
  extract_customer_cif: String,
  extract_ao_name: String,
  extract_ao_code: String,
  extract_request_type: Vec<RequestType>,
}
impl PDFExtract {
  pub fn new (
    extract_customer_name: String,
    extract_customer_cif: String,
    extract_ao_name: String,
    extract_ao_code: String,
    extract_request_type: Vec<RequestType>,
  ) -> Self {
    Self{extract_customer_name,extract_customer_cif,extract_ao_name,extract_ao_code,extract_request_type}
  }
  pub fn get_extracted_cif(&self) -> &str {
    &self.extract_customer_cif
  }
  pub fn get_extracted_name(&self) -> &str {
    &self.extract_customer_name
  }
  pub fn get_extracted_ao_name(&self) -> &str {
    &self.extract_ao_name
  }
  pub fn get_extracted_ao_code(&self) -> &str {
    &self.extract_ao_code
  }
  pub fn get_extracted_request_type(&self) -> &Vec<RequestType> {
    &self.extract_request_type
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