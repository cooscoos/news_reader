#[derive(Debug)]
pub struct Article {
    number: u8,
    href: String,
    summary: String,
    full_article: String,
    been_read: bool,
}

impl Article {
    pub fn default(number: &str, href: &str, summary: &str) -> Self {
        let num: u8 = number.trim().parse().unwrap();

        Self {
            number: num,
            href: href.to_string(),
            summary: decode_html(summary),
            full_article: "".to_string(), // empty for now, we're only going to grab it if the user wants to read it
            been_read: false,
        }
    }

    pub fn display_them(&self) {
        println!("{}. {}", self.number, self.summary)
    }
}

// BBC news mainly only uses apostrophes so we just need to decode these
fn decode_html(input: &str) -> String {
    input.replace("&#x27;", "'")
}
