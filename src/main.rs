mod extract_links;
use reqwest::Client;
use std::collections::{HashSet, VecDeque};
use url::Url;

fn check_links(base_url: &str, depth: u32) {
    let client = Client::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, u32)> = VecDeque::new();

    queue.push_back((base_url.to_owned(), 0));

    while let Some((url, current_depth)) = queue.pop_front() {
        if visited.contains(&url) || current_depth > depth {
            continue;
        }

        visited.insert(url.clone());

        let response = match client.get(&url).send() {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error fetching {}: {}", url, e);
                continue;
            }
        };

        let base_url = match Url::parse(&url) {
            Ok(url) => url,
            Err(e) => {
                eprintln!("Error parsing URL {}: {}", url, e);
                continue;
            }
        };

        let html = match response.text() {
            Ok(html) => html,
            Err(e) => {
                eprintln!("Error reading response body for {}: {}", url, e);
                continue;
            }
        };

        let links = extract_links::extract_links(&html, &base_url);

        for link in links {
            queue.push_back((link, current_depth + 1));
        }
    }
}

// fn extract_links(html: &str, base_url: &Url) -> Vec<String> {
//     // Code to extract links from the HTML content goes here
//     // This is just a placeholder for brevity
//     vec![]
// }

fn main() {
    let base_url = "https://example.com";
    let depth = 2;
    check_links(base_url, depth);
}
