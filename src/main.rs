use std::{vec, fs};
use serde_json;
use rand::{Rng};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct House {
    pub id: i32,
    pub no_adults: i32,
    pub no_children: i32,
    pub house_size_m2: i32,
    pub no_electric_cars :i32
}
static mut AMOUNT_OF_HOUSES: i32 = 100;

fn main() {
    let args: Vec<String> = env::args().collect();

    unsafe { AMOUNT_OF_HOUSES = args[1].parse().expect(&*format!("Invalid value found in amount field; Please ensure first parameter is a number. Found {:?}", &args[0])); }

    let mut houses: Vec<House> = vec![];
    unsafe {
        for i in 1..=AMOUNT_OF_HOUSES {
            let house = generate_house(i);
            houses.push(house);
        }
    }

    let json = serde_json::to_string(&houses).expect("Could not serialize houses");

    fs::write("./houses.json", json).expect("Could not write data");
}

fn generate_house(id: i32) -> House {
    let mut rng = rand::thread_rng();

    let no_adults = rng.gen_range(1..=2);
    let no_children = rng.gen_range(0..=7);
    
    // House size will be dependent on amount of people; so a house housing 4 people will have a larger chance at being larger
    let house_size_m2 = 
            // Base size, this is the space used by kitchen, living room, etc
            rng.gen_range(70..=100) +
                // Each l
            (0..no_adults).reduce(|total, _| {total + rng.gen_range(10..=15)}).unwrap_or(0) +
            (0..no_children).reduce(|total, _| {total + rng.gen_range(15..=20)}).unwrap_or(0);

    let no_electric_cars = 
        (0..=no_adults)
        .reduce(|amount, _| 
            {amount + (rng.gen_ratio(1, 10) as i32)}
        ).expect("Could not calculate no. electric cars");

    House {
        id,
        no_adults,
        no_children,
        house_size_m2,
        no_electric_cars
    }
}