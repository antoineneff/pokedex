use clap::{App, Arg};
use serde::Deserialize;
use image::GenericImageView;
use term_table::{Table, TableStyle};
use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};

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

    let mut table = Table::new();
    table.max_column_width = 96;
    table.style = TableStyle::extended();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(pokemon.name.to_uppercase(), 3, Alignment::Center),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(format!("# {}", pokemon.id), 1, Alignment::Center),
        TableCell::new_with_alignment(format!("{:.1}kg", pokemon.weight as f32 / 10.0), 1, Alignment::Center),
        TableCell::new_with_alignment(format!("{:.2}m", pokemon.height as f32 / 10.0), 1, Alignment::Center)
    ]));

    let img_bytes = reqwest::blocking::get(pokemon.sprites.front_default)?.bytes()?;
    let img = image::load_from_memory(&img_bytes).unwrap();
    let (width, height) = img.dimensions();

    let mut pokemon_str = String::new();

    for y in (0..height).step_by(2)  {
        for x in 0..width {
            // Get top half block color
            let [r, g, b, alpha] = img.get_pixel(x, y).0;
            // Get bottom half block color
            let [r2, g2, b2, alpha2] = img.get_pixel(x, y + 1).0;

            // Both transparent
            if alpha == 0 && alpha2 == 0 {
                pokemon_str.push_str(" ");
            }
            // Top half transparent, set colored lower half block
            else if alpha == 0 {
                let prefix = format!("\x1B[38;2;{};{};{}m", r2, g2, b2);
                pokemon_str.push_str(&format!("{}{}{}", prefix, LOWER_HALF_BLOCK, POSTFIX));
            }
            // Bottom half transparent, set colored top half block
            else if alpha2 == 0 {
                let prefix = format!("\x1B[38;2;{};{};{}m", r, g, b);
                pokemon_str.push_str(&format!("{}{}{}", prefix, UPPER_HALF_BLOCK, POSTFIX));
            }
            // Both blocks are colored, set colored top half block with foreground color
            else {
                let prefix = format!("\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m", r, g, b, r2, g2, b2);
                pokemon_str.push_str(&format!("{}{}{}", prefix, UPPER_HALF_BLOCK, POSTFIX));
            }
        }
        pokemon_str.push_str("\n");
    }

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(pokemon_str, 3, Alignment::Center)
    ]));

    println!("{}", table.render());

    Ok(())
}
