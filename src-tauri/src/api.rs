use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
struct BreachInfo {
    name: String,
    domain: String,
    breach_date: String,
    description: String,
}

// API Base URL
const HIBP_API_URL: &str = "https://haveibeenpwned.com/api/v3/breachedaccount/";

#[command]
pub async fn check_breach(email: String) -> Result<Vec<BreachInfo>, String> {
    let api_key = "YOUR_HIBP_API_KEY"; // Replace with your real API key

    // Validate email format
    if !email.contains("@") || !email.contains(".") {
        return Err("Invalid email format".to_string());
    }

    // Create HTTP client
    let client = Client::new();
    let url = format!("{}{}", HIBP_API_URL, email);

    // Make the API request
    let response = client
        .get(&url)
        .header("hibp-api-key", api_key)
        .header("User-Agent", "my-dark-web-monitor")
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let breaches: Vec<BreachInfo> = res.json().await.unwrap_or_else(|_| vec![]);
                Ok(breaches)
            } else if res.status().as_u16() == 404 {
                Ok(vec![]) // No breaches found
            } else {
                Err(format!("Error fetching data: {:?}", res.status()))
            }
        }
        Err(e) => Err(format!("Request failed: {:?}", e)),
    }
}
