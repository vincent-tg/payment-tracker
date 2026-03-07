use base64::Engine;

fn main() {
    let base64_text = r#"V2VsbHMgRmFyZ28gVHJhbnNhY3Rpb24gQWxlcnQKCllvdXIgYWNjb3VudCBoYXMgYmVlbiBkZWJpdGVk
IGZvciAkNTkuOTkuCgpNZXJjaGFudDogV0FMTUFSVApEYXRlOiAwMS8xNS8yMDI0CgpUaGFuayB5b3U="#;

    println!("Original base64 text:");
    println!("{}", base64_text);
    println!();

    // Try decoding
    match base64::engine::general_purpose::STANDARD.decode(base64_text) {
        Ok(decoded) => {
            println!("Successfully decoded!");
            println!("Decoded text: {}", String::from_utf8_lossy(&decoded));
        }
        Err(e) => {
            println!("Decoding error: {}", e);

            // Try with whitespace removed
            let cleaned = base64_text
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();

            println!("\nTrying with whitespace removed:");
            println!("Cleaned: {}", cleaned);

            match base64::engine::general_purpose::STANDARD.decode(&cleaned) {
                Ok(decoded) => {
                    println!("Successfully decoded after cleaning!");
                    println!("Decoded text: {}", String::from_utf8_lossy(&decoded));
                }
                Err(e2) => {
                    println!("Still error: {}", e2);
                }
            }
        }
    }
}
