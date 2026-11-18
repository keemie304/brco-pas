use regex::Regex;
use pdf_inspector::process_pdf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

   let result = process_pdf(
        "sample_data/sample_credit_proposal_one.pdf"
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

  let text = result.unwrap().markdown;
  println!("The text is:\n");
  println!("{:#?}",text);

  let markdown = text.as_deref().unwrap();

  println!("{}",markdown);

  let re = Regex::new(r"\*\*Customer CIF:\*\*\s*(\d+)").unwrap();

  let captures = re.captures(markdown).unwrap();

  let cif = captures.get(1).unwrap().as_str();

  println!("Extract CIF: {}", cif);

  let 


  /*
    let pattern = Regex::new(r"Customer CIF:\s*(\d+)")?;

  let captures = pattern.captures(text).ok_or("CIF not found")?;



  
   */




  Ok(())
}