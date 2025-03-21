use crate::near::NearClient;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_greeting() {
        let client = NearClient::new();
        match client.get_greeting().await {
            Ok(greeting) => {
                println!("Received greeting: {}", greeting);
                assert!(!greeting.is_empty(), "Greeting should not be empty");
            }
            Err(e) => panic!("Failed to get greeting: {}", e),
        }
    }
}