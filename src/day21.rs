use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[aoc_generator(day21)]
fn gen(input: &str) -> Vec<Recipe> {
    let re = Regex::new(r"([^(]*) \(contains ([^)]*)\)").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Recipe {
                ingredients: caps
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
                allergens: caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect(),
            }
        })
        .collect()
}

fn build_allergen_map(recipes: &Vec<Recipe>) -> HashMap<String, String> {
    let mut candidates = HashMap::<String, HashSet<String>>::new();
    for r in recipes {
        for a in &r.allergens {
            let c: &mut _ = candidates.entry(a.clone()).or_insert(r.ingredients.clone());
            *c = c.intersection(&r.ingredients).cloned().collect();
        }
    }
    let mut real = HashMap::new();
    while real.len() < candidates.len() {
        for (allergen, potential_ingredients) in &candidates {
            if potential_ingredients.len() == 1 {
                real.insert(
                    potential_ingredients.iter().next().unwrap().clone(),
                    allergen.clone(),
                );
            }
        }
        for ingredient in real.keys() {
            for c in candidates.values_mut() {
                c.remove(ingredient);
            }
        }
    }
    real
}

#[aoc(day21, part1)]
fn part1(recipes: &Vec<Recipe>) -> usize {
    let mut hypo_allergenic = recipes.iter().fold(HashSet::new(), |acc, r| {
        acc.union(&r.ingredients).cloned().collect()
    });
    for r in build_allergen_map(recipes).keys() {
        hypo_allergenic.remove(r);
    }
    recipes
        .iter()
        .map(|r| r.ingredients.intersection(&hypo_allergenic).count())
        .sum()
}

#[aoc(day21, part2)]
fn part2(recipes: &Vec<Recipe>) -> String {
    let real = build_allergen_map(recipes);

    let mut poisons: Vec<String> = real.keys().cloned().collect();
    poisons.sort_unstable_by_key(|p| real.get(p).unwrap());
    poisons.join(",")
}

#[test]
fn test_small() {
    // Each allergen is found in exactly one ingredient. Each ingredient contains zero or one
    // allergen. Allergens aren't always marked; when they're listed (as in (contains nuts,
    // shellfish) after an ingredients list), the ingredient that contains each listed allergen
    // will be somewhere in the corresponding ingredients list. However, even if an allergen isn't
    // listed, the ingredient that contains that allergen could still be present: maybe they forgot
    // to label it, or maybe it was labeled in a language you don't know.
    //
    // The first step is to determine which ingredients can't possibly contain any of the allergens
    // in any food in your list. In the above example, none of the ingredients kfcds, nhms, sbzzf,
    // or trh can contain an allergen. Counting the number of times any of these ingredients appear
    // in any ingredients list produces 5: they all appear once each except sbzzf, which appears
    // twice.
    let input = "
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"
    .trim();

    assert_eq!(part1(&gen(input)), 5);
    assert_eq!(part2(&gen(input)), "mxmxvkd,sqjhc,fvjkl");
}
