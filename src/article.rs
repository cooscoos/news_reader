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

    // Print the articles out and flag them as already read
    pub fn display_them(&self) {
        let read = match self.been_read {
            true => "(READ) ",
            false => "",
        };

        println!("{:<2}  {:<80}{}", self.number, self.summary, read);
    }

    // Print out the article in full
    pub fn read(&mut self) -> String {
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
            // TODO: I could code up some stuff to parse BBC Sport articles but I don't want to.
            full_story.push_str("Sorry, BBC Sport articles are not supported by this app.\n\n")
        }

        // use regex to grab paragraphs, I could tidy this regex up a lot but it's working so it'll do for now.
        let re = Regex::new(r#"<div data-component="text-block" class="ssrcss-uf6wea-RichTextComponentWrapper e1xue1i86"><div class="ssrcss-7uxr49-RichTextContainer e5tfeyi1"><p class="ssrcss-1q0x1qg-Paragraph eq5iqo00">(?P<line>.*?)</p>"#).unwrap();

        for (line_no, cap) in re.captures_iter(&full_article).enumerate() {
            // If the paragraph is bold, remove the bold bit
            let re2 = Regex::new(r#"l-BoldText e5tfeyi3">(?P<boldline>.*?)</b>"#).unwrap();
            let bold_text = re2.captures(&cap["line"]);

            // If the paragraph has html links, remove these
            // TODO: this will die if there is any more than one link in the paragraph. Probably better to remove all html link code rather than using capture groups.
            let re3 = Regex::new(
                r#"(?P<thestart>.*?)<a href=.*?>(?P<ignorelink>.*?)</a>(?P<therest>.*?)"#,
            )
            .unwrap();
            let with_links = re3.captures(&cap["line"]);

            // This looks for paragraphs with social media nonsense so we can remove them.
            let re4 = Regex::new(r#"<a href="https://twitter.com/[bbc|BBC].+"#).unwrap();
            let social_media = re4.is_match(&cap["line"]);

            let mut para = String::new();

            if !social_media {
                if let Some(value) = bold_text {
                    para = format!("* {} *\n\n", &value["boldline"]);
                } else if let Some(value) = with_links {
                    para = format!(
                        "{}{}{}\n\n",
                        &value["thestart"], &value["ignorelink"], &value["therest"]
                    );
                } else {
                    para = format!("{}\n\n", &cap["line"]);
                }
            }

            full_story.push_str(&para);
  
            // Every 6 lines, insert some message. These will be interpreted by the main loop and used to break the article up a bit
            if line_no>0 && line_no%6 == 0 {
                full_story.push_str("\n[SEPARATE]\n");
            }
        }

        format!(
            "{} ----- \n ** END OF ARTICLE ** \n",
            Article::decode_html(&full_story)
        )
    }

    // Decode some html. BBC mainly uses apostrophes and quotes so we only need to decode these.
    fn decode_html(input: &str) -> String {
        let give_back = input.replace("&#x27;", "'");
        give_back.replace("&quot;", "\"")
    }
}
