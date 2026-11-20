/*
This file parses the Credit Proposal PDFs uploaded by the AOs on the SharePoint list. 
After parsing the PDFs, the program extracts the markdown which contains the files inputs etc.
It is then passed on to a pattern 
The function parse_pdf eventually returns the extract_customer_cif & extract_customer_name 
which will then pass on to validation.rs which checks whether the submission details matches the 
information in the credit proposal.
 */

use regex::Regex;
use crate::models::{PDFExtract, RequestType};
use pdf_inspector::{process_pdf,PdfProcessResult};

// Declaring format of CIF & Customer Name as constants

const CUSTOMER_NAME_PATTERN: &str = r"\*\*Customer Name:\*\*\s*([^*]+)";
const CUSTOMER_CIF_PATTERN: &str  = r"\*\*Customer CIF:\*\*\s*(\d+)";
const AO_NAME_PATTERN: &str = r"\*\*AO Name:\*\*\s*([^*]+)";
const AO_CODE_PATTERN: &str = r"\*\*AO Code:\*\*\s*(\d+)";
const REQUEST_TYPE_PATTERN: &str = r"\*\*Request Type:\*\*\s*([^*]+)";

pub fn parse_pdf(filepath: &str) -> Result<PDFExtract,String> {

  let pdf: PdfProcessResult = parse_credit_proposal(filepath)?;

  let markdown: &str = pdf.markdown.as_deref().ok_or("No markdown extracted")?;



  let customer_name: String = extract_name(markdown)?;

  let customer_cif: String = extract_cif(markdown)?;

  let ao_name:String = extract_ao_name(markdown)?;
  let ao_code:String = extract_ao_code(markdown)?;
  let request_type:Vec<RequestType>= extract_request_type(markdown)?;

  let extracted_details: PDFExtract = PDFExtract::new(
    customer_cif,
    customer_name,
    ao_name,
    ao_code,
    request_type,
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

pub fn extract_cif(markdown:&str) -> Result<String,String> {

  //initialising the Regex CIF pattern
  let cif_pattern: Regex = Regex::new(CUSTOMER_CIF_PATTERN).map_err(|e| e.to_string())?;

  //finding cif
  let captured_cif: regex::Captures<'_> = cif_pattern.captures(markdown).ok_or("Customer CIF not found in uploaded Credit Propposal")?;

  //extract cif from captured_cif
  let cif: String = captured_cif.get(1).ok_or("CIF is missing")?.as_str().to_string();

  Ok(cif)


}

pub fn extract_name(markdown:&str) -> Result<String,String> {


  //initialising the Regex Customer Name pattern
  let name_pattern = Regex::new(CUSTOMER_NAME_PATTERN).map_err(|e| e.to_string())?;

  //finding cif
  let captured_name: regex::Captures<'_> = name_pattern.captures(markdown).ok_or("Customer name not found in uploaded Credit Proposal")?;

  //extract cif from captured_cif
  let customer_name = captured_name.get(1).ok_or("CIF is missing")?.as_str().trim().to_string();

  Ok(customer_name)

}

pub fn extract_ao_name(markdown:&str) ->Result<String,String> {

  //initialising the Regex Customer Name pattern
  let ao_name_pattern: Regex = Regex::new(AO_NAME_PATTERN).map_err(|e| e.to_string())?;

  //finding cif
  let captured_name: regex::Captures<'_> = ao_name_pattern.captures(markdown).ok_or("AO name not found in uploaded Credit Proposal")?;

  //extract cif from captured_cif
  let ao_name: String = captured_name.get(1).ok_or("CIF is missing")?.as_str().to_string();

  Ok(ao_name)

}

pub fn extract_ao_code(markdown:&str) -> Result<String,String> {


  //initialising the Regex Customer Name pattern
  let ao_code_pattern: Regex = Regex::new(AO_CODE_PATTERN).map_err(|e| e.to_string())?;

  //finding cif
  let captured_name: regex::Captures<'_> = ao_code_pattern.captures(markdown).ok_or("AO Code not found in uploaded Credit Proposal")?;

  //extract cif from captured_cif
  let ao_code: String = captured_name.get(1).ok_or("CIF is missing")?.as_str().trim().to_string();

  Ok(ao_code)

}

pub fn extract_request_type(markdown:&str) -> Result<Vec<RequestType>,String> {

  //initialising the Regex Customer Name pattern
  let request_type_pattern: Regex = Regex::new(REQUEST_TYPE_PATTERN).map_err(|e| e.to_string())?;

  //finding cif
  let captured_request_type: regex::Captures<'_> = request_type_pattern.captures(markdown).ok_or("Request Type not found in uploaded Credit Proposal")?;

  //extract cif from captured_cif
  let captured_type = captured_request_type.get(1).ok_or("Request Type is missing")?.as_str();

  let request_types: Vec<RequestType> = captured_type
    .split(',')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .map(|s| {
        match s {
            "New" => Ok(RequestType::New),
            "Increase" => Ok(RequestType::Increase),
            "Reduction" => Ok(RequestType::Reduction),
            "Amendment" => Ok(RequestType::Amendment),
            "Cancellation" => Ok(RequestType::Cancellation),
            "Annual Review" => Ok(RequestType::AnnualReview),
            "Interim Review" => Ok(RequestType::InterimReview),
            "Discussion" => Ok(RequestType::Discussion),
            _ => Err(format!("Unknown request type: {}", s)),
        }
    })
    .collect::<Result<Vec<RequestType>, String>>()?;

  Ok(request_types)

}