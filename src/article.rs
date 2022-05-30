use regex::Regex;

#[derive(Debug)]
pub struct Article {
    number: u8,
    href: String,
    summary: String,
    full_article: String,
    been_read: bool,
}

impl Article {
    // Create new article
    pub fn default(number: &str, href: &str, summary: &str) -> Self {
        let num: u8 = number.trim().parse().unwrap();

        Self {
            number: num,
            href: href.to_string(),
            summary: Article::decode_html(summary),
            full_article: "".to_string(), // empty for now, we're only going to grab it if the user wants to read it
            been_read: false,
        }
    }

    // Print the article headings out and flag them as read/unread
    pub fn display_them(&self) {
        let read = match self.been_read {
            true => "(READ) ",
            false => "",
        };

        println!("{:<2}  {:<80}{}", self.number, self.summary, read);
    }

    // Print out the article in full
    pub fn print_full(&mut self) -> String {
        if !self.been_read {
            let story_url = format!("https://www.bbc.co.uk{}", self.href);

            let full_article = match reqwest::get(&story_url) {
                Ok(mut val) => val.text().unwrap(),
                Err(error) => {
                    panic!(
                        "BBC News is unreachable. Are you connected to t'internet? Error: {error}."
                    )
                }
            };

            self.full_article = self.parse_article(full_article);
            self.been_read = true;
        }

        self.full_article.clone()
    }

    // Parse article from BBC website into something the terminal can handle
    fn parse_article(&self, full_article: String) -> String {
        let mut full_story = format!("\n-----\n{}\n-----\n\n", self.summary.to_ascii_uppercase());

        if self.href.contains("/sport/") {
            // TODO: I could code up some stuff to parse BBC Sport articles, but I kind of don't want to.
            full_story.push_str("Sorry, BBC Sport articles are not supported by this app.\n\n")
        }

        // use regex to grab paragraphs, I could tidy this regex up, but it's working quite robustly on news articles so it'll do for now.
        let re = Regex::new(r#"<div data-component="text-block" class="ssrcss-uf6wea-RichTextComponentWrapper e1xue1i86"><div class="ssrcss-7uxr49-RichTextContainer e5tfeyi1"><p class="ssrcss-1q0x1qg-Paragraph eq5iqo00">(?P<line>.*?)</p>"#).unwrap();

        for (line_no, cap) in re.captures_iter(&full_article).enumerate() {
            // This looks for paragraphs with social media nonsense so we can ignore them.
            let re4 = Regex::new(r#"<a href="https://twitter.com/[bbc|BBC].+"#).unwrap();
            let social_media = re4.is_match(&cap["line"]);

            let mut para = String::new();

            if !social_media {
                para = format!("{}\n\n", &cap["line"]);
            }

            // get rid of bold and href formatting
            let para2 = Article::remove_formatting(para);

            full_story.push_str(&para2);

            // Every 6 lines, insert some message. These will be interpreted by the main loop and used to break the article up a bit
            if line_no > 0 && line_no % 6 == 0 {
                full_story.push_str("\n[SEPARATE]\n");
            }
        }

        format!(
            "{} ----- \n ** END OF ARTICLE ** \n",
            Article::decode_html(&full_story)
        )
    }

    // Remove href and bold html formatting
    fn remove_formatting(input: String) -> String {
        // Look for hyperlinks
        let re5 = Regex::new(r#"<a href=".*?" class=".*?">(?P<thewords>.*?)</a>"#).unwrap();

        // and bold formatting
        let re6 = Regex::new(r#"<b class=".*?">(?P<thewords>.*?)</b>"#).unwrap();

        let output1 = re5.replace_all(&input, "$thewords"); // get rid href
        let output2 = re6.replace_all(&output1, "$thewords"); // get rid bold

        output2.to_string()
    }

    // Decode some html. BBC mainly uses apostrophes and quotes so we only need to decode these.
    fn decode_html(input: &str) -> String {
        let give_back = input.replace("&#x27;", "'");
        give_back.replace("&quot;", "\"")
    }
}
