use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let text = "Customer CIF: 123456";

  let pattern = Regex::new(r"Customer CIF:\s*(\d+)");

  let caputues = pattern.captures(text).ok_or("CIF not found");

  let cif = captures.get(1).ok_or("Capture group missing")?.as_str();

  println!("Extract CIF: {}", cif);

  Ok(())
}