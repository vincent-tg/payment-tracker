extern crate regex;
use regex::Regex;

fn main() {
    let html = r#"<html><body><div>Giá trị: <b>58,000 VND</b></div></body></html>"#;

    // Simple HTML to text
    let text = html
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("</p>", "\n")
        .replace("</div>", "\n")
        .replace("</tr>", "\n")
        .replace("</td>", " ")
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"");

    let re = Regex::new(r"<[^>]+>").unwrap();
    let clean_text = re.replace_all(&text, "").to_string();

    println!("Clean text: {}", clean_text);

    // Try VND pattern
    let vnd_pattern = r"(?i)(\d{1,3}(?:,\d{3})*)\s*VND\b";
    let re_vnd = Regex::new(vnd_pattern).unwrap();

    if let Some(cap) = re_vnd.find(&clean_text) {
        println!("Found VND: {}", cap.as_str());
        let amount_str = cap.as_str().replace(" VND", "").replace(",", "");
        println!("Amount string: {}", amount_str);
        if let Ok(amount) = amount_str.parse::<f64>() {
            println!("Parsed amount: {}", amount);
        }
    } else {
        println!("No VND found");
    }
}
