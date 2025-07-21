//Desarrollo de una calculadora con estructuras

use std::io;

struct Calculadora;

impl Calculadora{
    fn sumar(a: i32, b: i32) -> i32{
        a+b
    }

    fn restar(a: i32, b: i32) -> i32{
        a-b
    }

    fn multiplicar(a: i32, b: i32) -> i32{
        a*b
    }

    fn dividir(a: i32, b: i32) -> f32{
        a as f32 / b as f32
    }
}

fn leer() -> i32{
    let mut entrada: String= String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer el número");
    entrada.trim().parse().expect("Error al convertir el valor")
}

fn main(){
    println!("Calculadora Cli!");
    println!("Ingresa la opción a usar");
    println!("1. sumar\nb. restar \nc. multiplicar \nd.dividir");
    let opcion: i32= leer();

    println!("\nIngresa el primer valor: ");
    let valor1: i32= leer();
    println!("\nIngresa el segundo valor: ");
    let valor2: i32= leer();

    let resultado: f32;

    if  opcion == 1{
        resultado = Calculadora::sumar(valor1, valor2) as f32;
    } else if opcion == 2{
        resultado = Calculadora::restar(valor1, valor2) as f32;
    } else if opcion == 3{
        resultado = Calculadora::multiplicar(valor1, valor2) as f32;
    } else if opcion == 4{
        resultado = Calculadora::dividir(valor1, valor2);
    } else{
        println!("Opción ingresada no válida");
        return;
    }

    println!("El resultado es: {}", resultado);

}