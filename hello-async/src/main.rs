use trpl::{Either, Html};

fn main() {
    println!("Hello, Little Web Scrapper!");

    let args: Vec<String> = std::env::args().collect();
    
    trpl::block_on(async { 
        let title_future_1 = page_title(&args[1]);
        let title_future_2 = page_title(&args[2]);
        
        let (url, maybe_title) = match trpl::select(title_future_1, title_future_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} retunerd first");
        match maybe_title {
            Some(title) => println!("Page title was: '{title}'"),
            None => println!("Iy has no title"),
        }
    })
}

////
////
////

async fn page_title(url: &str) -> (&str, Option<String>) {
    /*
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    */
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
