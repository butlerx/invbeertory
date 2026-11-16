use super::Beer;
use crate::components::charts::stacked_bar;
use csv::ReaderBuilder;
use implicit_clone::unsync::{IArray, IString};
use std::{collections::HashMap, convert::Into};

#[derive(Debug, PartialEq, Clone)]
pub struct Stock {
    pub beers: IArray<Beer>,
}

struct Dataset {
    pub data: Vec<i64>,
    pub label: String,
}

static INVENTORY_STR: &str = include_str!("../../inventory.csv");

/// Bars shown in the "Brewery by Style" chart before grouping into "Other".
const TOP_BREWERIES: usize = 12;
/// Stacked segments (and legend entries) before grouping into "Other".
const TOP_STYLES: usize = 10;
const OTHER: &str = "Other";

impl Stock {
    pub fn load() -> Result<Self, csv::Error> {
        let mut reader = ReaderBuilder::new().from_reader(INVENTORY_STR.as_bytes());
        let beers: Result<IArray<Beer>, _> = reader.deserialize().collect();
        Ok(Self { beers: beers? })
    }

    pub fn find_beer(self, brewery: &str, year: i16, name: &str) -> Option<Beer> {
        self.beers
            .iter()
            .find(|beer| {
                beer.brewery.to_lowercase().replace(' ', "_") == brewery
                    && beer.year == year
                    && beer.name.to_lowercase().replace(' ', "_") == name
            })
            .cloned()
    }

    pub fn purchases_by_style(&self) -> HashMap<IString, i64> {
        self.field_by_field(|beer| beer.purchased, |beer| beer.style.clone())
    }

    pub fn purchases_by_brewery(&self) -> HashMap<IString, i64> {
        self.field_by_field(|beer| beer.purchased, |beer| beer.brewery.clone())
    }

    fn field_by_field<F, G>(&self, field_a: F, field_b: G) -> HashMap<IString, i64>
    where
        F: Fn(&Beer) -> i64,
        G: Fn(&Beer) -> IString,
    {
        let res = self
            .beers
            .iter()
            .map(|beer| (field_b(beer), field_a(beer)))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                *acc.entry(key).or_insert(0) += value;
                acc
            });

        let mut res_vec = res.into_iter().collect::<Vec<(IString, i64)>>();
        res_vec.sort_by_key(|a| std::cmp::Reverse(a.1));

        let mut largest: HashMap<IString, i64> = HashMap::new();
        let mut other_sum = 0;

        for (i, (key, value)) in res_vec.into_iter().enumerate() {
            if i < 30 {
                largest.insert(key, value);
            } else {
                other_sum += value;
            }
        }

        if other_sum > 0 {
            largest.insert(IString::from("Other"), other_sum);
        }

        largest
    }

    pub fn stock_by_abv(&self) -> IArray<(IString, i64)> {
        let grouped_stock: HashMap<IString, i64> = self
            .beers
            .iter()
            .filter(|beer| beer.stock != 0)
            .map(|beer| (format!("{:.1}", beer.abv), beer.stock))
            .fold(HashMap::new(), |mut acc, (abv, stock)| {
                *acc.entry(abv.into()).or_insert(0) += stock;
                acc
            });

        let mut stock_vec: Vec<(IString, i64)> = grouped_stock.into_iter().collect();
        stock_vec.sort_by(|a, b| {
            let a_num: f64 = a.0.parse().unwrap_or(0.0);
            let b_num: f64 = b.0.parse().unwrap_or(0.0);
            a_num
                .partial_cmp(&b_num)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        IArray::from(stock_vec)
    }

    pub fn brewery_by_style(&self) -> stacked_bar::Data {
        let breweries = self.top_keys(|beer| beer.brewery.to_string(), TOP_BREWERIES);
        let styles = self.top_keys(|beer| beer.style.to_string(), TOP_STYLES);
        let bucket = |keys: &[String], key: String| {
            keys.iter()
                .position(|k| *k == key)
                .unwrap_or(keys.len() - 1)
        };

        let mut datasets: Vec<Dataset> = styles
            .iter()
            .map(|style| Dataset {
                label: style.clone(),
                data: vec![0; breweries.len()],
            })
            .collect();

        for beer in self.beers.iter() {
            let brewery = bucket(&breweries, beer.brewery.to_string());
            let style = bucket(&styles, beer.style.to_string());
            datasets[style].data[brewery] += beer.purchased;
        }

        let used_labels: Vec<usize> = (0..breweries.len())
            .filter(|&i| datasets.iter().any(|d| d.data[i] > 0))
            .collect();
        datasets.retain(|d| d.data.iter().any(|&v| v > 0));

        stacked_bar::Data {
            labels: used_labels
                .iter()
                .map(|&i| IString::from(breweries[i].clone()))
                .collect(),
            datasets: datasets
                .into_iter()
                .map(|d| stacked_bar::Dataset {
                    label: IString::from(d.label),
                    data: used_labels.iter().map(|&i| d.data[i]).collect(),
                })
                .collect(),
        }
    }

    /// The `limit` most-purchased values of `key`, most popular first,
    /// followed by an "Other" bucket for everything else.
    fn top_keys<F>(&self, key: F, limit: usize) -> Vec<String>
    where
        F: Fn(&Beer) -> String,
    {
        let counts = self.beers.iter().fold(HashMap::new(), |mut acc, beer| {
            *acc.entry(key(beer)).or_insert(0) += beer.purchased;
            acc
        });
        let mut sorted: Vec<(String, i64)> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        sorted
            .into_iter()
            .map(|(k, _)| k)
            .filter(|k| k != OTHER)
            .take(limit)
            .chain(std::iter::once(OTHER.to_string()))
            .collect()
    }

    pub fn search(&self, query: &str) -> IArray<Beer> {
        self.beers
            .iter()
            .filter(|beer| {
                beer.filter_check("name", query)
                    || beer.filter_check("brewery", query)
                    || beer.filter_check("style", query)
                    || beer.filter_check("year", query)
                    || beer.filter_check("abv", query)
                    || beer.filter_check("collaborators", query)
            })
            .cloned()
            .collect()
    }

    pub fn current_stock(&self) -> IArray<Beer> {
        let beers = self
            .beers
            .iter()
            .filter(|beer| beer.stock > 0)
            .cloned()
            .collect::<Vec<_>>();
        IArray::from(beers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brewery_by_style_is_capped() {
        let stock = Stock::load().unwrap();
        let data = stock.brewery_by_style();
        assert!(data.labels.len() <= TOP_BREWERIES + 1);
        assert!(data.datasets.len() <= TOP_STYLES + 1);
        for dataset in data.datasets.iter() {
            assert_eq!(dataset.data.len(), data.labels.len());
        }
    }

    #[test]
    fn brewery_by_style_keeps_every_purchase() {
        let stock = Stock::load().unwrap();
        let total: i64 = stock.beers.iter().map(|b| b.purchased).sum();
        let charted: i64 = stock
            .brewery_by_style()
            .datasets
            .iter()
            .flat_map(|d| d.data.iter().collect::<Vec<_>>())
            .sum();
        assert_eq!(total, charted);
    }
}
