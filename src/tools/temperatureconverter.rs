use std::io;

#[derive(Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0 + 273.15
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 9.0 / 5.0 + 32.0
}

fn get_temperature_input() -> f64 {
    loop {
        println!("Entrez la température à convertir:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        match input.trim().parse::<f64>() {
            Ok(temp) => return temp,
            Err(_) => println!("Veuillez entrer un nombre valide."),
        }
    }
}

fn get_unit_input() -> TemperatureUnit {
    loop {
        println!("Entrez l'unité de la température (C pour Celsius, F pour Fahrenheit, K pour Kelvin):");
        let mut unit = String::new();
        io::stdin().read_line(&mut unit).expect("Erreur de lecture");
        let unit = unit.trim().to_uppercase();

        match unit.as_str() {
            "C" => return TemperatureUnit::Celsius,
            "F" => return TemperatureUnit::Fahrenheit,
            "K" => return TemperatureUnit::Kelvin,
            _ => println!("Unité inconnue. Veuillez entrer C, F ou K."),
        }
    }
}

fn convert_temperature(temp: f64, unit: TemperatureUnit) {
    match unit {
        TemperatureUnit::Celsius => {
            println!("{}°C en Fahrenheit: {}°F", temp, celsius_to_fahrenheit(temp));
            println!("{}°C en Kelvin: {}K", temp, celsius_to_kelvin(temp));
        }
        TemperatureUnit::Fahrenheit => {
            println!("{}°F en Celsius: {}°C", temp, fahrenheit_to_celsius(temp));
            println!("{}°F en Kelvin: {}K", temp, fahrenheit_to_kelvin(temp));
        }
        TemperatureUnit::Kelvin => {
            println!("{}K en Celsius: {}°C", temp, kelvin_to_celsius(temp));
            println!("{}K en Fahrenheit: {}°F", temp, kelvin_to_fahrenheit(temp));
        }
    }
}

pub fn main() {
    println!("Bienvenue dans le convertisseur de température!");

    loop {
        let temp = get_temperature_input();
        let unit = get_unit_input();
        convert_temperature(temp, unit);

        println!("Souhaitez-vous effectuer une autre conversion ? (Oui/Non)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let response = input.trim().to_lowercase();

        if response != "oui" && response != "o" {
            break;
        }
    }

    println!("Merci d'avoir utilisé le convertisseur de température!");
}

