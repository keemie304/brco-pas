/*
This module takes the data inputted by the AOs in the Microsoft Forms and the customer data extracted
from the Credit Proposal and evaluates whether they are a match.

A match will return 'true' while a mismatch will return 'false'.

 */
use crate::models::{Submission,PDFExtract};

pub fn validate(submission: &Submission, extracted: &PDFExtract) -> &'static str {

  let cif_check = validate_cif(&submission, &extracted);

  let name_check = validate_name(&submission, &extracted);

  if cif_check && name_check {
    "Both data in Credit Proposal Submission and Credit Proposal matches"
  } else {
    "Data Mismatch"
  }

}

pub fn validate_cif(submission: &Submission,extracted:&PDFExtract) -> bool {
  let submission_cif = submission.proposal().get_cif();

  let extract_cif = extracted.get_extracted_cif();

  let matched: bool = submission_cif == extract_cif;

  matched

}

pub fn validate_name(submission: &Submission,extracted:&PDFExtract) -> bool {
  let submission_name = submission.proposal().get_name();

  let extract_name = extracted.get_extracted_name();

  let matched: bool = submission_name == extract_name;

  matched

}
