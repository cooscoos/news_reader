use tokio;
use reqwest;
use regex::Regex;



#[tokio::main]
async fn main() {


    let result = match reqwest::get("http://www.bbc.co.uk/news"){
        Ok(mut val) => val.text().unwrap(),
        Err(error) => panic!("BBC News is unreachable. Are you connected to t'internet? Error: {error}."),
    };

    
    println!("{:?}",result);

    // Use regex to search for a 1 or 2 digit number
    let re = Regex::new(r#"most-popular-read-(?P<number>\d{1,2}).*?href="/news/(?P<link>.*?)">.*?gel-pica-bold">(?P<summary>.*?)</span>"#).unwrap();

    for cap in re.captures_iter(&result) {
        println!("Number: {:?}, href: {:?}, summary: {:?}", &cap["number"],&cap["link"],&cap["summary"]);
    }

    
    

    

    
}
