use scraper::{Html, Selector};
use std::env;

struct Circolare {
    id: String,
    title: String,
    date: String,
    link: String,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let res = match args[1].as_str() {
        "page" => {
            if args.len() == 3 {
                if let Ok(p) = args[2].parse() {
                    page(p)
                } else {
                    Err(format!(
                        "Argument must be an integer, found \"{}\"",
                        args[2]
                    ))
                }
            } else {
                Err("Bad arguments".to_string())
            }
        }
        "search" => Ok(()),
        "help" => Ok(()),
        cmd => Err(format!("Unknown sub-command \"{}\"", cmd)),
    };
    if let Err(err) = res {
        println!("Error: {}", err);
    }
}

fn page(page: u16) -> Result<(), String> {
    let circolari = scrape_circolari(page).map_err(|e| e.to_string())?;
    print_circolari(&circolari);
    Ok(())
}

fn print_circolari(circolari: &Vec<Circolare>) {
    for c in circolari {
        println!("# {} ({})\n{}\n{}\n", c.id, c.date, c.title, c.link);
    }
}

fn append_circolari(circolari: &mut Vec<Circolare>, page: u16) -> Result<(), reqwest::Error> {
    let mut new_circolari = scrape_circolari(page)?;
    circolari.append(&mut new_circolari);
    Ok(())
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
