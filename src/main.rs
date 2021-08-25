use clap::{App, Arg};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Pokemon {
    id: u16,
    name: String,
    weight: u16,
    height: u16,
    sprites: Sprites,
}

#[derive(Deserialize, Debug)]
struct Sprites {
    front_default: String
}

fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Pokedex")
        .about("Get pokemon info from your terminal")
        .arg(
            Arg::with_name("search")
                .help("Pokemon name or id")
                .required(true)
                .index(1),
        )
        .get_matches();

    let search = matches.value_of("search").unwrap().to_lowercase();
    let uri = format!("https://pokeapi.co/api/v2/pokemon/{}", search);
    let pokemon: Pokemon = reqwest::blocking::get(uri)?.json()?;

    println!("id: {}", pokemon.id);
    println!("name: {}", pokemon.name);
    println!("weight: {}kg", pokemon.weight as f64 / 10.0);
    println!("height: {}m", pokemon.height as f64 / 10.0);
    println!("sprite url: {}", pokemon.sprites.front_default);

    Ok(())
}
