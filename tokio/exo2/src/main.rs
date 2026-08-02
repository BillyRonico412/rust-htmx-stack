use futures::future::join_all;
use std::println;
use std::vec;

fn count_word_the_from_data(data: &str) -> usize {
    data.matches("the").count()
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://react.dev/",
        "https://rust-lang.org/",
        "https://four.htmx.org/",
    ];

    let s = join_all(urls.iter().map(|&url| {
        tokio::spawn(async move {
            let Ok(req) = reqwest::get(url).await else {
                println!("No data found for {}", url);
                return;
            };
            let Ok(body) = req.text().await else {
                println!("No data found for {}", url);
                return;
            };
            let count = count_word_the_from_data(&body);
            println!("Nb the word at {} is {}", url, count);
        })
    }));
    s.await;
}
