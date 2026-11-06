// Rust - REST API with reqwest
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("APIFY_TOKEN")?;
    let input = r#"{"apps":["389801252",{"id":"310633997","country":"gb"}],"trackCharts":true}"#;
    let url = format!(
        "https://api.apify.com/v2/acts/data_foundry~aso-app-rank-tracker/runs?token={token}"
    );

    let client = reqwest::Client::new();
    let run: serde_json::Value = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(input)
        .send()
        .await?
        .json()
        .await?;

    let dataset = run["data"]["defaultDatasetId"].as_str().unwrap_or_default();
    println!("dataset: {dataset}");
    Ok(())
}
