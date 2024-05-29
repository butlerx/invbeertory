mod beer;

use crate::components::charts::stacked_bar::{Data, Dataset};
pub use beer::{Beer, Stock};
use csv::ReaderBuilder;
use std::collections::HashMap;

const INVENTORY: &str = include_str!("inventory.csv");

pub fn load() -> Result<Stock, csv::Error> {
    let mut reader = ReaderBuilder::new().from_reader(INVENTORY.as_bytes());
    let mut beers = Vec::new();
    for result in reader.deserialize() {
        let beer: Beer = result?;
        beers.push(beer);
    }
    Ok(Stock { beers })
}

pub fn find_beer<'a>(beers: &'a [Beer], brewery: &str, year: &str, name: &str) -> Option<&'a Beer> {
    let year = year.parse::<i16>().unwrap() else {
        return None;
    };

    beers
        .iter()
        .find(|beer| beer.brewery == brewery && beer.year == year && beer.name == name)
}

fn field_by_field<F, G>(beers: &[Beer], field_a: F, field_b: G) -> HashMap<String, i64>
where
    F: Fn(&Beer) -> i64,
    G: Fn(&Beer) -> String,
{
    let mut res: HashMap<String, i64> = HashMap::new();
    for beer in beers {
        let field_a_value = field_a(beer);
        let field_b_value = field_b(beer);

        let entry = res.entry(field_b_value).or_insert(0);
        *entry += field_a_value;
    }

    let mut res_vec: Vec<(String, i64)> = res.into_iter().collect();
    res_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by value in descending order

    let mut largest: HashMap<String, i64> = HashMap::new();
    let mut other_sum = 0;

    for (i, (key, value)) in res_vec.into_iter().enumerate() {
        if i < 30 {
            largest.insert(key, value);
        } else {
            other_sum += value;
        }
    }

    if other_sum > 0 {
        largest.insert("Other".to_string(), other_sum);
    }

    largest
}

pub fn purchases_by_style(beers: &[Beer]) -> HashMap<String, i64> {
    field_by_field(beers, |beer| beer.purchased, |beer| beer.style.clone())
}

pub fn purchases_by_brewery(beers: &[Beer]) -> HashMap<String, i64> {
    field_by_field(beers, |beer| beer.purchased, |beer| beer.brewery.clone())
}

fn round_abv(abv: f64) -> i64 {
    let factor = 10.0_f64.powi(1);
    ((abv * factor).round() / factor) as i64
}

pub fn stock_by_abv(beers: &[Beer]) -> Vec<(String, i64)> {
    // Group by rounded ABV and sum stock quantities
    let mut grouped_stock: HashMap<String, i64> =
        beers.iter().fold(HashMap::new(), |mut acc, beer| {
            *acc.entry(beer.abv.to_string()).or_insert(0) += beer.stock;
            acc
        });

    grouped_stock.retain(|_abv, stock| *stock != 0);

    let mut stock_vec: Vec<(String, i64)> = grouped_stock.into_iter().collect();
    stock_vec.sort_by(|a, b| {
        let a_num: f64 = a.0.parse().unwrap_or(0.0);
        let b_num: f64 = b.0.parse().unwrap_or(0.0);
        a_num
            .partial_cmp(&b_num)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    stock_vec
}

pub fn brewery_by_style(beers: &[Beer]) -> Data {
    let mut brewery_counts: HashMap<String, i64> = HashMap::new();

    // Aggregate brewery counts
    for beer in beers {
        let count = brewery_counts.entry(beer.brewery.clone()).or_insert(0);
        *count += beer.purchased;
    }

    let mut sorted_breweries: Vec<_> = brewery_counts.into_iter().collect();
    sorted_breweries.sort_by(|a, b| b.1.cmp(&a.1));

    // Select most popular breweries
    let top_breweries: Vec<String> = sorted_breweries
        .iter()
        .take(20)
        .map(|(brewery, _)| brewery.clone())
        .collect();

    // Group the rest of the breweries under "Other"
    let other_count: i64 = sorted_breweries
        .iter()
        .skip(20)
        .map(|(_, count)| count)
        .sum();

    let mut breweries: HashMap<String, HashMap<String, i64>> = HashMap::new();

    for beer in beers {
        let brewery = if top_breweries.contains(&beer.brewery) {
            beer.brewery.clone()
        } else {
            "Other".to_string()
        };

        let brewery_entry = breweries.entry(brewery).or_insert_with(HashMap::new);

        if let Some(style_counts) = brewery_entry.get_mut(&beer.style) {
            *style_counts += beer.purchased;
        } else {
            brewery_entry.insert(beer.style.clone(), beer.purchased);
        }
    }

    let labels: Vec<String> = breweries.keys().cloned().collect();
    let mut datasets: Vec<Dataset> = Vec::new();

    for (brewery, styles) in &breweries {
        for (style, &count) in styles {
            let mut found = false;
            for dataset in &mut datasets {
                if dataset.label == *style {
                    let index = labels.iter().position(|l| l == brewery).unwrap_or(0);
                    dataset.data[index] = count;
                    found = true;
                    break;
                }
            }
            if !found {
                let mut data = vec![0; labels.len()];
                let index = labels.iter().position(|l| l == brewery).unwrap_or(0);
                data[index] = count;
                datasets.push(Dataset {
                    label: style.clone(),
                    data,
                });
            }
        }
    }

    Data { labels, datasets }
}
