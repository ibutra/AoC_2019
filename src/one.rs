pub fn run(input: Vec<i64>) {
    let sum : i64 = input.iter().map(|x| fuel_for_mass_and_fuel(*x)).sum();
    println!("Needed Fuel: {}", sum);
}

fn fuel_for_mass(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn fuel_for_mass_and_fuel(mass: i64) -> i64 {
    let mut sum = 0;
    let mut fuel = mass;
    loop {
        fuel = fuel_for_mass(fuel);
        if fuel <= 0 {
            break;
        }
        sum += fuel;
    }
    sum
}
