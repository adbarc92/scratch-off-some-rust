#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the URL you want to send the GET request to
    let url = "https://google.com";

    // Send the GET request and await the response
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Get the response body as bytes
        let body = response.bytes().await?;

        // Convert the bytes into a string (assuming it's UTF-8 encoded)
        let body_as_string = String::from_utf8(body.to_vec())?;

        // Print the response body
        println!("{}", body_as_string);
    } else {
        // Print an error message if the request was not successful
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
