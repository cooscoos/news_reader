use std::io;
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
            summary: decode_html(summary),
            full_article: "".to_string(), // empty for now, we're only going to grab it if the user wants to read it
            been_read: false,
        }
    }

    pub fn display_them(&self) {
        let read = match self.been_read{
            true => "(READ) ",
            false => "", 
        };

        println!("{:<2}  {:<80}{}", self.number, self.summary, read);
    }

    pub fn read(&mut self) -> String {
        
        if !self.been_read {
            let story_url = format!("https://www.bbc.co.uk{}",self.href);

            let full_article = match reqwest::get(&story_url) {
                Ok(mut val) => val.text().unwrap(),
                Err(error) => {
                    panic!("BBC News is unreachable. Are you connected to t'internet? Error: {error}.")
                }
            };
    
            let re = Regex::new(r#"<div data-component="text-block" class="ssrcss-uf6wea-RichTextComponentWrapper e1xue1i86"><div class="ssrcss-7uxr49-RichTextContainer e5tfeyi1"><p class="ssrcss-1q0x1qg-Paragraph eq5iqo00">(?P<line>.*?)</p>"#).unwrap();
            let mut full_story = format!("\n-----\n{}\n-----\n\n",self.summary.to_ascii_uppercase());
    
            for cap in re.captures_iter(&full_article) {
                
                let re2 = Regex::new(r#"l-BoldText e5tfeyi3">(?P<boldline>.*?)</b>"#).unwrap();
                let m = re2.captures(&cap["line"]);
    
                // this isn't always working is it because my thingy is lazy?
                let re3 = Regex::new(r#".*?<a href=.*?>(?P<ignorelink>.*?)</a>(?P<therest>.+)"#).unwrap();
                let n = re3.captures(&cap["line"]);

                // this looks for the social media nonsense in the line
                let re4 = Regex::new(r#"<i class="ssrcss-xbdn93-ItalicText e5tfeyi2">.+"#).unwrap();
                let p = re4.is_match(&cap["line"]);
                

                // if not social media nonsense
                if !p {

                if let Some(value) = m {
                    let bit = format!("* {} *\n\n",&value["boldline"]);
                    full_story.push_str(&bit);
                } else if let Some(value) = n {
                    let bit = format!("{}{}\n\n",&value["ignorelink"],&value["therest"]);
                    full_story.push_str(&bit);
                } else {
                    let bit = format!("{}\n\n",&cap["line"]);
                    full_story.push_str(&bit);
                }
    
            }
        }
            self.been_read = true;
    
            self.full_article = format!("{} ----- \n (END OF ARTICLE, press any key to return) \n",decode_html(&full_story));
    
            
        } else {
            println!("I doing from memory because I clever")
        }

        self.full_article.clone()

        
    }
}

// BBC news mainly only uses apostrophes so we just need to decode these
pub fn decode_html(input: &str) -> String {
    let give_back = input.replace("&#x27;", "'");
    give_back.replace("&quot;", "\"")


    
}


pub fn get_user_input() -> usize {
    let mut input = String::new();


    io::stdin().read_line(&mut input).expect("Keyboard bad");

    match input.trim().parse::<usize>() {
        Ok(val) if val < 11 && val > 0 => val,
        _ => {println!("Goodbye"); std::process::abort();},
    }
}