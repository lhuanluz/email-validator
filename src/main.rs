use regex::Regex;
use reqwest::blocking::get;
use serde_json;
use std::collections::HashMap;

fn is_valid_email_format(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    email_regex.is_match(email)
}

fn is_valid_domain(email: &str) -> bool {
    if let Some(domain) = email.split('@').nth(1) {
        let url = format!("https://dns.google/resolve?name={}", domain);
        if let Ok(response) = get(&url) {
            if let Ok(json_response) = response.json::<HashMap<String, serde_json::Value>>() {
                return json_response.contains_key("Answer");
            }
        }
    }
    false
}

fn main() {
    let emails = vec![
        "user1@example.com",
        "user2@test.com",
        "invalid-email.com",
        "anotheruser@nonexistentdomain.xyz",
    ];

    for email in emails {
        let format_valid = is_valid_email_format(&email);
        let domain_valid = is_valid_domain(&email);

        if format_valid && domain_valid {
            println!("{} é válido.", email);
        } else {
            println!("{} é inválido.", email);
        }
    }
}
