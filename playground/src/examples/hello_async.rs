use trpl::Either;
use trpl::Html;

pub async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

pub fn run_async() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let fut1 = page_title(&args[1]);
        let fut2 = page_title(&args[2]);

        let (url, title) = match trpl::race(fut1, fut2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match title {
            Some(title) => println!("Its page title is: {title}"),
            None => println!("Its page title couldn't be parsed"),
        }
    });
}
