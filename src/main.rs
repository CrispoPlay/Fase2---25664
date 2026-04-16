use std::collections::HashMap;

fn main() {
    // Vector dinámico
    let mut numeros = Vec::new();
    for i in 0..30 {
        numeros.push(i);
    }

    println!("Vector:");
    for n in &numeros {
        print!("{} - ", n);
    }

    // HashMap (clave-valor)
    let mut edades = HashMap::new();
    edades.insert("Ana", 25);
    edades.insert("Luis", 30);

    println!("\nHashMap:");
    for (nombre, edad) in &edades {
        println!("{} tiene {} años", nombre, edad);
    }
}