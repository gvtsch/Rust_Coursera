#[derive(Debug)]
#[derive(Clone)]
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
    Tuscany,
    Baden_Wurttemberg,
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

fn get_popularity(region: WineRegions) -> String {
    match region {
        WineRegions::Bordeaux => String::from("Highly Popular"),
        WineRegions::Burgundy => String::from("Moderately Popular"),
        WineRegions::Champagne => String::from("Highly Popular"),
        WineRegions::Loire => String::from("Moderately Popular"),
        WineRegions::Alsace => String::from("Moderately Popular"),
        WineRegions::Rhone => String::from("Moderately Popular"),
        WineRegions::Languedoc => String::from("Less Popular"),
        WineRegions::Provence => String::from("Less Popular"),
        WineRegions::Corsica => String::from("Less Popular"),
        WineRegions::Rioja => String::from("Highly Popular"),
        WineRegions::Tuscany => String::from("Highly Popular"),
        WineRegions::Baden_Wurttemberg => String::from("Less Popular"),
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

    let wine3 = Wine {
        name: String::from("Gundelsheimer Riesling"),
        region: WineRegions::Baden_Wurttemberg,
        year: 2015,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);

    supported_regions(wine1.region.clone());
    supported_regions(WineRegions::Rioja);

    let popularity1 = get_popularity(wine1.region);
    let popularity2 = get_popularity(wine2.region);
    let popularity3 = get_popularity(wine3.region);

    println!("Popularity of Wine 1: {}", popularity1);
    println!("Popularity of Wine 2: {}", popularity2);
    println!("Popularity of Wine 3: {}", popularity3);

    
}