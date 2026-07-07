pub async fn set_mode(address: &str, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Convert ws:// address to http:// for the REST endpoint
    let http_addr = address
        .replace("ws://", "http://")
        .replace("/config", "")
        .replace("/raw", "");
    let url = format!("{}/mode?set={}", http_addr, mode);
    let resp = reqwest::Client::new().post(&url).send().await?;
    let body = resp.text().await?;
    match body.trim() {
        "live" => println!("Mode: live (bridge controls audio, MOD UI disconnected)"),
        "design" => println!("Mode: design (MOD UI can connect to mod-host:5555)"),
        other => eprintln!("Unexpected response: {}", other),
    }
    Ok(())
}
