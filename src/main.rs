use news_reader::*;

#[tokio::main]
async fn main() {
    let mut stories = get_top_stories();

    loop {
        news_loop(&mut stories);
    }
}
