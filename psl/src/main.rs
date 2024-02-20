use std::str;
use url::{Host, ParseError, Position, Url};

fn suffix_new(domain: &str) -> String {
    let domain_as_byte = domain.as_bytes();
    let suffix = psl::suffix(domain_as_byte).unwrap();
    str::from_utf8(suffix.as_bytes()).unwrap().to_string()
}

fn get_url_components(url: &str) -> Result<Vec<String>, ParseError> {
    // Parse the URL.
    let parsed_url = Url::parse(url)?;

    // Initialize a vector to hold the URL components.
    let mut components = Vec::new();

    // Add the scheme.
    components.push(parsed_url.scheme().to_string());

    // Add the domain, if present.
    if let Some(domain) = parsed_url.domain() {
        components.push(domain.to_string());
    }

    // Add the path.
    components.push(parsed_url.path().to_string());

    // Add the query, if present.
    if let Some(query) = parsed_url.query() {
        components.push(query.to_string());
    }

    // Add the fragment, if present.
    if let Some(fragment) = parsed_url.fragment() {
        components.push(fragment.to_string());
    }

    // Return the vector of components.
    Ok(components)
}

fn main() {
    // println!("{}", suffix_new("www.example.com"));
    // get_url_components();
    match get_url_components("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open") {
        Ok(components) => println!("{:?}", components),
        Err(e) => println!("Error parsing URL: {}", e),
    }
}
