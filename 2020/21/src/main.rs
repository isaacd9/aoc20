use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn read_input(lines: Lines<StdinLock>) -> Vec<Food> {
    let allergen_re = Regex::new(r"(?P<ingredients>.*+) \(contains (?P<allergens>.*+)\)").unwrap();

    lines
        .map(|li| li.unwrap())
        .map(|li| {
            let caps = allergen_re.captures(&li).unwrap();
            let ingredients: Vec<String> = caps["ingredients"]
                .split(" ")
                .map(|k| k.trim().to_string())
                .collect();

            let allergens: Vec<String> = caps["allergens"]
                .split(",")
                .map(|k| k.trim().to_string())
                .collect();
            Food {
                ingredients: ingredients,
                allergens: allergens,
            }
        })
        .collect()
}

fn food_by_allergen(foods: &Vec<Food>) -> HashMap<String, HashSet<usize>> {
    let mut hm: HashMap<String, HashSet<usize>> = HashMap::new();
    for (food_i, food) in foods.iter().enumerate() {
        for allergen in &food.allergens {
            hm.entry(allergen.clone()).or_default().insert(food_i);
        }
    }
    hm
}

fn find_allergenic_ingredients(
    all_foods: &Vec<Food>,
    food_by_allergen: &HashMap<String, HashSet<usize>>,
) -> HashMap<String, String> {
    let mut hm: HashMap<String, String> = HashMap::new();

    while hm.len() < food_by_allergen.len() {
        for (allergen, foods) in food_by_allergen {
            let mut food_ingredients = foods.iter().map(|food_i| &all_foods[*food_i].ingredients);
            let first_ing = HashSet::from_iter(
                food_ingredients
                    .by_ref()
                    .nth(0)
                    .unwrap()
                    .iter()
                    .filter(|it| !hm.contains_key(&it.to_string())),
            );
            let mut intersection =
                food_ingredients.fold(first_ing, |acc: HashSet<&String>, set: &Vec<String>| {
                    acc.intersection(&HashSet::from_iter(
                        set.iter().filter(|it| !hm.contains_key(&it.to_string())),
                    ))
                    .cloned()
                    .collect()
                });

            if intersection.len() == 1 {
                let allergen_ingredient = intersection.drain().nth(0).unwrap();
                hm.insert(allergen_ingredient.clone(), allergen.clone());
            }
        }
    }
    hm
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let foods = read_input(lines);

    let fba = food_by_allergen(&foods);
    let allergenic_ingredients = find_allergenic_ingredients(&foods, &fba);
    let no_allergens: HashMap<&String, u64> = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|ing| !allergenic_ingredients.contains_key(&ing.to_string()))
        .fold(HashMap::new(), |mut acc, ing| {
            *acc.entry(ing).or_default() += 1;
            acc
        });

    println!("{:?}", no_allergens.values().sum::<u64>());

    let mut allergen_list: Vec<(&String, &String)> = allergenic_ingredients.iter().collect();
    allergen_list.sort_by(|(_, b_allergen), (_, a_allergen)| b_allergen.cmp(a_allergen));
    println!(
        "{:?}",
        allergen_list
            .iter()
            .map(|(i, _)| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}
