use clap::{App, Arg};
use serde::Deserialize;
use image::GenericImageView;

const UPPER_HALF_BLOCK: &str = "\u{2580}";
const LOWER_HALF_BLOCK: &str = "\u{2584}";
const POSTFIX: &str = "\x1B[0m";

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
    println!("weight: {:.1}kg", pokemon.weight as f32 / 10.0);
    println!("height: {:.1}m", pokemon.height as f32 / 10.0);

    let img_bytes = reqwest::blocking::get(pokemon.sprites.front_default)?.bytes()?;
    let img = image::load_from_memory(&img_bytes).unwrap();
    let (width, height) = img.dimensions();

    for y in (0..height).step_by(2)  {
        for x in 0..width {
            // Get top half block color
            let [r, g, b, alpha] = img.get_pixel(x, y).0;
            // Get bottom half block color
            let [r2, g2, b2, alpha2] = img.get_pixel(x, y + 1).0;

            // Both transparent
            if alpha == 0 && alpha2 == 0 {
                print!(" ")
            }
            // Top half transparent, set colored lower half block
            else if alpha == 0 {
                let prefix = format!("\x1B[38;2;{};{};{}m", r2, g2, b2);
                print!("{}{}{}", prefix, LOWER_HALF_BLOCK, POSTFIX);
            }
            // Bottom half transparent, set colored top half block
            else if alpha2 == 0 {
                let prefix = format!("\x1B[38;2;{};{};{}m", r, g, b);
                print!("{}{}{}", prefix, UPPER_HALF_BLOCK, POSTFIX);
            }
            // Both blocks are colored, set colored top half block with foreground color
            else {
                let prefix = format!("\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m", r, g, b, r2, g2, b2);
                print!("{}{}{}", prefix, UPPER_HALF_BLOCK, POSTFIX);
            }
        }
        println!()
    }

    Ok(())
}
