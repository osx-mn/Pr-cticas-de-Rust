//Proyecto 1: Desarrollo de Una calculadora por CLI

use std::io;

fn leer() -> i32{
    let mut entrada: String= String::new();

    io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");
    entrada.trim().parse().expect("Error al convertir el valor")
}

fn sumar(numero1: i32, numero2: i32) -> i32{
    numero1+numero2
}

fn restar(numero1: i32, numero2: i32) -> i32{
    numero1-numero2
}

fn multiplicar(numero1: i32, numero2: i32) -> i32{
    numero1*numero2
}

fn dividir(numero1: i32, numero2: i32) -> f32{
    numero1 as f32/numero2 as f32
}

fn main(){
    println!("----- Calculadora Cli -----");
    println!("Selecciona la opción de operación");
    println!("1. Suma");
    println!("2. Resta");
    println!("3. Multiplicación");
    println!("4. División");

    let seleccion: i32= leer();

    if seleccion == 1 {
        println!("Has seleccionado suma");
        println!("Ingresa el primer número: ");
        let seleccion1: i32= leer();
        println!("\nIngresa el segundo número: ");
        let seleccion2: i32= leer();
        println!("\nEl resultado es: {}", sumar(seleccion1, seleccion2));
    } else if seleccion == 2{

        println!("Has seleccionado resta");
        println!("Ingresa el primer número: ");
        let seleccion1: i32= leer();
        println!("\nIngresa el segundo número: ");
        let seleccion2: i32= leer();
        println!("\nEl resultado es : {}", restar(seleccion1, seleccion2));
    } else if seleccion == 3{

        println!("Has seleccionado multiplicación");
        println!("Ingresa el primer número: ");
        let seleccion1: i32= leer();
        println!("\nIngresa el segundo número: ");
        let seleccion2: i32= leer();
        println!("\nEl resultado es: {}", multiplicar(seleccion1, seleccion2));
    } else if seleccion == 4{

        println!("Has seleccionado división");
        println!("Ingresa el primer número: ");
        let seleccion1: i32= leer();
        println!("\nIngresa el segundo número: ");
        let seleccion2: i32= leer();
        println!("\nEl resultado es: {}", dividir(seleccion1, seleccion2) as f32);
    } else{
        println!("Opción no válida");
    }
}