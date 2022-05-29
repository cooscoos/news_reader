use news_reader::*;

#[tokio::main]
async fn main() {
    // last thing to do is parse html links (multi per line) better by using remove
    let mut stories = get_top_stories();

    loop {
        news_loop(&mut stories);
    }
}
