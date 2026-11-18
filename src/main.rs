use regex::Regex;
use pdf_inspector::process_pdf;
use brco_pas::parser::{parse_credit_proposal,extract_cif,extract_name};

fn main() -> Result<(), Box<dyn std::error::Error>> {

/*
let result = process_pdf(
        "sample_data/sample_credit_proposal_three.pdf"
    );

    match &result {
        Ok(text) => {
            println!("PDF processed successfully!");
            println!("{:#?}", text);
        }

        Err(error) => {
            eprintln!("Failed to process PDF: {}", error);
        }
    }
 */
  let result = parse_credit_proposal("sample_data/sample_credit_proposal_three.pdf")?;

  let cif = extract_cif(&result)?;

  println!("The customer CIF is {}",cif);

  let customer_name = extract_name(&result)?;

  println!("The customer name is {}", customer_name);
   

  

  Ok(())
}