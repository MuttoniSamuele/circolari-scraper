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
            if args.len() == 4 {
                if let (Ok(p_min), Ok(p_max)) = (args[2].parse(), args[3].parse()) {
                    print_pages(p_min, p_max)
                } else {
                    Err(format!(
                        "Arguments must be integer, found \"{}\" and \"{}\"",
                        args[2], args[3]
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

/// Stampa tutte le circolari da page_min a page_max
fn print_pages(page_min: u16, page_max: u16) -> Result<(), String> {
    for page in page_min..=page_max {
        let circolari = scrape_circolari(page)?;
        println!("\n[Page {}]\n", page);
        print_circolari(&circolari);
    }
    Ok(())
}

/// Stampa le circolari specificate a schermo
fn print_circolari(circolari: &Vec<Circolare>) {
    for c in circolari {
        println!("# {} ({})\n{}\n{}\n", c.id, c.date, c.title, c.link);
    }
}

/// Aggiunge al vettore specificato le circolari nella pagina specificata
fn append_circolari(circolari: &mut Vec<Circolare>, page: u16) -> Result<(), String> {
    let mut new_circolari = scrape_circolari(page)?;
    circolari.append(&mut new_circolari);
    Ok(())
}

/// Fa lo scrape della pagina di circolari specificata e restituisce un vettore di circolari
fn scrape_circolari(page: u16) -> Result<Vec<Circolare>, String> {
    let url = format!("https://www.itispaleocapa.edu.it/circolari/page/{}/", page);
    let text = fetch_webpage(&url).map_err(|e| e.to_string())?;
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

/// Manda una richiesta GET all'URL specificato e restituisce il contenuto della pagina
fn fetch_webpage(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::blocking::get(url)?;
    let text = res.text()?;
    Ok(text)
}
