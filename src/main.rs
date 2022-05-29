

//use tokio;
//use reqwest;
use news_reader::*;
use regex::Regex;
use std::io;


#[tokio::main]
async fn main() {
    let result = match reqwest::get("http://www.bbc.co.uk/news") {
        Ok(mut val) => val.text().unwrap(),
        Err(error) => {
            panic!("BBC News is unreachable. Are you connected to t'internet? Error: {error}.")
        }
    };
 
    // Use regex to extract the most read article rating (number), the link, and the summary
    let re = Regex::new(r#"most-popular-read-(?P<number>\d{1,2}).*?href="(?P<link>.*?)">.*?gel-pica-bold">(?P<summary>.*?)</span>"#).unwrap();

    let mut stories: Vec<Article> = Vec::new();

    for cap in re.captures_iter(&result) {
        //println!("Number: {:?}, href: {:?}, summary: {:?}", &cap["number"],&cap["link"],&cap["summary"]);
        stories.push(Article::default(
            &cap["number"],
            &cap["link"],
            &cap["summary"],
        ));
    }

    loop {

        println!(r#"
 ____  ____   ____   _   _ _______        ______  
| __ )| __ ) / ___| | \ | | ____\ \      / / ___|  
|  _ \|  _ \| |     |  \| |  _|  \ \ /\ / /\___ \  
| |_) | |_) | |___  | |\  | |___  \ V  V /  ___) | 
|____/|____/ \____| |_| \_|_____|  \_/\_/  |____/ "#);

        println!("\n \n === WELCOME TO BBC NEWS === \n \n Here are the most read articles. \n");
        stories.iter().for_each(|a| a.display_them());

        println!("\n Input a number (1-10) to read the article, or type anything else to quit: \n");



        let input_no = get_user_input();




        println!("{}",stories[input_no-1].read());

        // sport articles have a completely different format that you need to regex through

        // I just want a keypress here, probably a better way
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Keyboard input bad.");
    }

    // show 5 lines at a time with enter to read more -- learn keypress first. - maybe can do for selection too
    
    // grey out if been_read

}