use clap::{App, Arg};
use rand::Rng;
use std::fs::File;
use std::io::Write;

fn generate_password(length: usize, use_special_chars: bool) -> String {
    let mut rng = rand::thread_rng();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let special_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?";
    let charset = if use_special_chars {
        format!("{}{}", chars, special_chars)
    } else {
        chars.to_string()
    };

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

fn main() {
    let matches = App::new("Password Generator")
        .version("1.0")
        .author("Tu Nombre <tu_email@example.com>")
        .about("Genera contraseñas seguras")
        .arg(
            Arg::with_name("length")
                .short('l')
                .long("length")
                .takes_value(true)
                .default_value("12")
                .help("Longitud de la contraseña"),
        )
        .arg(
            Arg::with_name("special")
                .short('s')
                .long("special")
                .takes_value(false)
                .help("Usar caracteres especiales"),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .help("Archivo de salida para guardar la contraseña"),
        )
        .get_matches();

    let length = matches
        .value_of("length")
        .unwrap()
        .parse::<usize>()
        .expect("La longitud debe ser un número entero");
    let use_special_chars = matches.is_present("special");

    let password = generate_password(length, use_special_chars);
    println!("Contraseña generada: {}", password);

    if let Some(output) = matches.value_of("output") {
        let mut file = File::create(output).expect("No se pudo crear el archivo");
        file.write_all(password.as_bytes())
            .expect("No se pudo escribir en el archivo");
        println!("Contraseña guardada en: {}", output);
    }
}