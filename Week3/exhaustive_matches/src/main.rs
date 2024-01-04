enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
    Trollinger,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetFranc => println!("Cabernet Franc tastes like blackberry, raspberry, and plum"),
        WineGrapes::Tannat => println!("Tannat tastes like blackberry, black cherry, and black plum"),
        WineGrapes::Merlot => println!("Merlot tastes like black cherry, raspberry, and plum"),
        /* Commenting one out, will throw an error, this all options have to be covered.
        Alternatively one can implement a wildcard pattern _: */
        _ => println!("I don't know what this wine tastes like"),
    }
}

fn main() {
    let mut grapes = WineGrapes::CabernetFranc;
    taste_wine(grapes);
    grapes = WineGrapes::Trollinger;
    taste_wine(grapes);
}