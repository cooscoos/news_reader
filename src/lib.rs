use regex::Regex;
use std::io;

mod article;
use article::Article;

// Get most read stories from main BBC news page
pub fn get_top_stories() -> Vec<Article> {
    let result = match reqwest::get("http://www.bbc.co.uk/news") {
        Ok(mut val) => val.text().unwrap(),
        Err(error) => {
            panic!("BBC News is unreachable. Are you connected to t'internet? Error: {error}.")
        }
    };

    let mut top_stories: Vec<Article> = Vec::new();

    // Use regex to extract the most read article rating (number), the link, and the summary
    // This site is good for building and testing regex: https://regexr.com/
    let re = Regex::new(r#"most-popular-read-(?P<number>\d{1,2}).*?href="(?P<link>.*?)">.*?gel-pica-bold">(?P<summary>.*?)</span>"#).unwrap();

    for cap in re.captures_iter(&result) {
        top_stories.push(Article::default(
            &cap["number"],
            &cap["link"],
            &cap["summary"],
        ));
    }

    top_stories
}

pub fn news_loop(top_stories: &mut Vec<Article>) {
    println!(
        r#"
    ____  ____   ____   _   _ _______        ______  
   | __ )| __ ) / ___| | \ | | ____\ \      / / ___|  
   |  _ \|  _ \| |     |  \| |  _|  \ \ /\ / /\___ \  
   | |_) | |_) | |___  | |\  | |___  \ V  V /  ___) | 
   |____/|____/ \____| |_| \_|_____|  \_/\_/  |____/ "#
    );

    println!("\n \n === WELCOME TO BBC NEWS === \n \n Here are the most read articles. \n");
    top_stories.iter().for_each(|a| a.display_them());

    println!("\n Input a number (1-10) to read the article, or type anything else to quit: \n");

    let input_no = get_user_input();

    let selected_story = top_stories[input_no - 1].read();
    let story_chunks: Vec<&str> = selected_story.split("[SEPARATE]").collect();

    for chunk in story_chunks {
        println!("{}[Press any key to continue]", chunk);
        get_any_user_input()
    }
    
}

pub fn get_user_input() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Keyboard input bad.");

    match input.trim().parse::<usize>() {
        Ok(val) if val < 11 && val > 0 => val,
        _ => {
            println!("~~ Goodbye ~~");
            std::process::abort();
        }
    }
}

// This function waits for any user input.
fn get_any_user_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Keyboard input bad.");
}