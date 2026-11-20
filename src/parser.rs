/*
This file parses the Credit Proposal PDFs uploaded by the AOs on the SharePoint list. 
After parsing the PDFs, the program extracts the markdown which contains the files inputs etc.
It is then passed on to a pattern 
The function parse_pdf eventually returns the extract_customer_cif & extract_customer_name 
which will then pass on to validation.rs which checks whether the submission details matches the 
information in the credit proposal.
 */

use regex::Regex;
use crate::models::PDFExtract;
use pdf_inspector::{process_pdf,PdfProcessResult};

// Declaring format of CIF & Customer Name as constants

const CUSTOMER_NAME_PATTERN: &str = r"\*\*Customer Name:\*\*\s*(.*?)\s*\*\*Customer CIF:\*\*";
const CUSTOMER_CIF_PATTERN: &str  = r"\*\*Customer CIF:\*\*\s*(\d+)";
const AO_NAME_PATTERN: &str = r"\*\*AO Name:\*\*";
const AO_CODE_PATTERN: &str = r"";
const REQUEST_TYPE_PATTERN: &str = r"";

pub fn parse_pdf(filepath: &str) -> Result<PDFExtract,String> {

  let pdf: PdfProcessResult = parse_credit_proposal(filepath)?;

  let customer_name: String = extract_name(&pdf)?;

  let customer_cif: String = extract_cif(&pdf)?;

  let extracted_details: PDFExtract = PDFExtract::new(
    customer_name,
    customer_cif,
  );

  Ok(extracted_details)

}

pub fn parse_credit_proposal(filepath:&str) -> Result<PdfProcessResult,String> {

  let result: Result<PdfProcessResult, pdf_inspector::PdfError> = process_pdf(filepath);

  match result {
    Ok(pdf) => Ok(pdf),
    Err(error) => Err(error.to_string()),
  }
}

pub fn extract_cif(pdf:&PdfProcessResult) -> Result<String,String> {
  let markdown: &str = pdf.markdown.as_deref().ok_or("No markdown extracted")?;


  //initialising the Regex CIF pattern
  let cif_pattern: Regex = Regex::new(CUSTOMER_CIF_PATTERN).map_err(|e| e.to_string())?;

  //finding cif
  let captured_cif: regex::Captures<'_> = cif_pattern.captures(markdown).ok_or("Customer CIF not found")?;

  //extract cif from captured_cif
  let cif: String = captured_cif.get(1).ok_or("CIF is missing")?.as_str().to_string();

  Ok(cif)


}

pub fn extract_name(pdf:&PdfProcessResult) -> Result<String,String> {
  let markdown: &str = pdf.markdown.as_deref().ok_or("No markdown extracted")?;


  //initialising the Regex Customer Name pattern
  let name_pattern = Regex::new(CUSTOMER_NAME_PATTERN).map_err(|e| e.to_string())?;

  //finding cif
  let captured_name: regex::Captures<'_> = name_pattern.captures(markdown).ok_or("Customer name not found")?;

  //extract cif from captured_cif
  let customer_name = captured_name.get(1).ok_or("CIF is missing")?.as_str().to_string();

  Ok(customer_name)

}

pub fn extract_ao_name(pdf:&PdfProcessResult) ->Result<String,String> {

}