use clap::{App, Arg};
use serde::Deserialize;
use image::{GenericImageView};

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
    println!("weight: {:.1}kg", pokemon.weight as f64 / 10.0);
    println!("height: {:.1}m", pokemon.height as f64 / 10.0);

    let img_bytes = reqwest::blocking::get(pokemon.sprites.front_default)?.bytes()?;
    let img = image::load_from_memory(&img_bytes).unwrap();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let [r, g, b, alpha] = img.get_pixel(x, y).0;
            if alpha == 0 {
                print!(" ")
            } else {
                let prefix = format!("\x1B[48;2;{};{};{}m", r, g, b);
                let postfix = "\x1B[0m";
                print!("{} {}", prefix, postfix);
            }
        }
        println!()
    }

    Ok(())
}
