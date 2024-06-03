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

impl Stock {
    pub fn load() -> Result<Self, csv::Error> {
        let inventory_str = include_str!("inventory.csv");
        let mut reader = ReaderBuilder::new().from_reader(inventory_str.as_bytes());
        let beers: Result<IArray<Beer>, _> = reader.deserialize().collect();
        Ok(Self { beers: beers? })
    }

    pub fn find_beer(self, brewery: &str, year: i16, name: &str) -> Option<Beer> {
        self.beers.iter().find(|beer| {
            beer.brewery.to_lowercase().replace(" ", "_") == brewery
                && beer.year == year
                && beer.name.to_lowercase().replace(" ", "_") == name
        })
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
            .map(|beer| (field_b(&beer), field_a(&beer)))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                *acc.entry(key).or_insert(0) += value;
                acc
            });

        let mut res_vec = res.into_iter().collect::<Vec<(IString, i64)>>();
        res_vec.sort_by(|a, b| b.1.cmp(&a.1));

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
        let brewery_counts: HashMap<String, i64> =
            self.beers.iter().fold(HashMap::new(), |mut acc, beer| {
                *acc.entry(beer.brewery.to_string()).or_insert(0) += beer.purchased;
                acc
            });

        let mut sorted_breweries: Vec<_> = brewery_counts.into_iter().collect();
        sorted_breweries.sort_by(|a, b| b.1.cmp(&a.1));

        // Select most popular breweries
        let top_breweries: Vec<String> = sorted_breweries
            .iter()
            .take(20)
            .map(|(brewery, _)| brewery.clone())
            .collect();

        let mut breweries: HashMap<String, HashMap<String, i64>> = HashMap::new();

        for beer in self.beers.iter() {
            let brewery = if top_breweries.contains(&beer.brewery.to_string()) {
                beer.brewery.to_string()
            } else {
                "Other".to_string()
            };

            let brewery_entry = breweries.entry(brewery).or_default();
            *brewery_entry.entry(beer.style.to_string()).or_insert(0) += beer.purchased;
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
        stacked_bar::Data {
            labels: labels.into_iter().map(Into::into).collect(),
            datasets: datasets
                .into_iter()
                .map(|d| stacked_bar::Dataset {
                    label: IString::from(d.label),
                    data: d.data.into(),
                })
                .collect(),
        }
    }

    pub fn search(&self, query: &str) -> IArray<Beer> {
        self.beers
            .iter()
            .filter(|beer| {
                beer.brewery.contains(query)
                    || beer.name.contains(query)
                    || beer.style.contains(query)
                    || beer.year.to_string().contains(query)
                    || beer.abv.to_string().contains(query)
                    || beer
                        .collaborators
                        .as_ref()
                        .map_or(false, |c| c.contains(query))
            })
            .collect()
    }

    pub fn current_stock(&self) -> IArray<Beer> {
        let beers = self
            .beers
            .iter()
            .filter(|beer| beer.stock > 0)
            .collect::<Vec<_>>();
        IArray::from(beers)
    }
}
