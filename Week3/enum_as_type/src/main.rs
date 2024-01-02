#[derive(Debug)]

enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Loire,
    Alsace,
    Rhone,
    Languedoc,
    Provence,
    Corsica,
    Rioja,
    Tuscany
}

struct Wine {
    name: String,
    region: WineRegions,
    year: u16,
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is a region in Spain and supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
        year: 2015,
    };
    
    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
        year: 2010,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);

    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);
}