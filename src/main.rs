//use tokio;
//use reqwest;
use news_reader::Article;
use regex::Regex;

#[tokio::main]
async fn main() {
    let result = match reqwest::get("http://www.bbc.co.uk/news") {
        Ok(mut val) => val.text().unwrap(),
        Err(error) => {
            panic!("BBC News is unreachable. Are you connected to t'internet? Error: {error}.")
        }
    };
 
    // Use regex to extract the most read article rating (number), the link, and the summary
    let re = Regex::new(r#"most-popular-read-(?P<number>\d{1,2}).*?href="/news/(?P<link>.*?)">.*?gel-pica-bold">(?P<summary>.*?)</span>"#).unwrap();

    let mut stories: Vec<Article> = Vec::new();

    for cap in re.captures_iter(&result) {
        //println!("Number: {:?}, href: {:?}, summary: {:?}", &cap["number"],&cap["link"],&cap["summary"]);
        stories.push(Article::default(
            &cap["number"],
            &cap["link"],
            &cap["summary"],
        ));
    }

    println!("\n \n === WELCOME TO BBC NEWS === \n \n Here are the most read articles. \n");
    stories.iter().for_each(|a| a.display_them());

    println!("\n Input a number (1-10) to read the article, or type anything else to quit: \n");

    // how to parse html formatting
    // grey out if been_read

}
