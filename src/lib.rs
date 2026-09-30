use rust_embed::RustEmbed;
use serde::Deserialize;

use std::collections::HashMap;
use std::str;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Poor,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Deserialize)]
struct Entry {
    idx: u16,
    slug: String,
    gen: u8,
    name: HashMap<String, String>,
    desc: HashMap<String, String>,
    rarity: Rarity,
    types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Species {
    pub number: u16,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
    pub generation: u8,
    pub types: Vec<String>,
}

fn english(texts: &HashMap<String, String>) -> String {
    texts.get("en").cloned().unwrap_or_default()
}

/// Every species in national dex order, with English names and descriptions.
pub fn all_species() -> Vec<Species> {
    let file = Asset::get("pokemon.json").expect("pokemon.json is embedded");
    let entries: Vec<Entry> = serde_json::from_slice(&file.data).expect("pokemon.json is valid");
    entries
        .into_iter()
        .map(|entry| Species {
            number: entry.idx,
            name: english(&entry.name),
            description: english(&entry.desc),
            slug: entry.slug,
            rarity: entry.rarity,
            generation: entry.gen,
            types: entry.types,
        })
        .collect()
}

/// The ANSI colorscript for a species' regular form.
pub fn art(slug: &str, shiny: bool) -> Option<String> {
    let variant = if shiny { "shiny" } else { "regular" };
    let file = Asset::get(&format!("colorscripts/{variant}/{slug}"))?;
    str::from_utf8(&file.data).ok().map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_species_has_regular_and_shiny_art() {
        let species = all_species();
        assert!(species.len() > 1000);
        for entry in species {
            assert!(art(&entry.slug, false).is_some(), "{}", entry.slug);
            assert!(art(&entry.slug, true).is_some(), "{}", entry.slug);
        }
    }
}
