use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = "YOUR_PROJECT_ID";
    let bucket_name = "YOUR_BUCKET_NAME";
    let _access_token = "YOUR_ACCESS_TOKEN";

    // Create a JSON object with the bucket configuration
    let bucket_config = json!({
        "name": bucket_name,
        "location": "US" // You can change the location according to your preference
    });

    // Set up the HTTP client
    let client = Client::new();
    let url = format!(
        "https://storage.googleapis.com/storage/v1/b?project={}",
        project_id
    );

    // Create a header with the Authorization token
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    // headers.insert(
    //     reqwest::header::AUTHORIZATION,
    //     HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    // );

    // Send a POST request to create the bucket
    let response = client
        .post(&url)
        .headers(headers)
        .body(bucket_config.to_string())
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        println!("Bucket {} created successfully.", bucket_name);
    } else {
        println!(
            "Failed to create bucket {}: {:?}",
            bucket_name,
            response.text().await?
        );
    }

    Ok(())
}
