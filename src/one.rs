pub fn run(input: Vec<i64>) {
    let mut sum = 0;
    for num in input {
        sum += fuel_for_mass_and_fuel(num);
    }
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
