use scraper::{Html, Selector};

struct Circolare {
    id: String,
    title: String,
    date: String,
    link: String,
}

fn main() {
    let Ok(circolari) = scrape_circolari(1) else {
        println!("Error fetching circolari");
        return;
    };
    for c in circolari {
        println!("{}\t{}\t{}\t{}", c.id, c.title, c.date, c.link);
    }
}

fn scrape_circolari(page: u16) -> Result<Vec<Circolare>, reqwest::Error> {
    let url = format!("https://www.itispaleocapa.edu.it/circolari/page/{}/", page);
    let text = fetch_webpage(&url)?;
    let html = Html::parse_document(&text);
    let selector = Selector::parse("div.post-box-archive").unwrap();
    let mut circolari: Vec<Circolare> = Vec::new();
    for element in html.select(&selector) {
        let children_selector = Selector::parse("*").unwrap();
        let children: Vec<_> = element.select(&children_selector).collect();
        let circolare = Circolare {
            id: children[4].inner_html(),
            title: children[6].inner_html(),
            date: children[0].inner_html(),
            link: children[2].attr("href").unwrap().to_string(),
        };
        circolari.push(circolare);
    }
    Ok(circolari)
}

fn fetch_webpage(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::blocking::get(url)?;
    let text = res.text()?;
    Ok(text)
}
