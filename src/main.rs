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
    let re = Regex::new(r#"data-entityid="most-popular-read-\d{1,2}"#).unwrap();

    println!("{:?}",re.is_match(&result));
    

    for cap in re.captures_iter(&result) {
        println!("{:?}",cap);
    }




    // this works, but you need to sit down and understand regex better.
    // the bit we're looking for is "data-entityid="most-popular-read-"
    // then look for promo: <span class="gs-c-promo-heading__title gel-pica-bold">
    
    //Our string
    
    // then </span></a></div></span></li>

    // apparently we can use Serde for this

    
    


    
    

    
}
