//suma de dos numeros
use std::io;

fn leer() -> i32{
    let mut entrada= String::new();

    io::stdin()
    .read_line(&mut entrada)
    .expect("Error al ingresar la entrada");

    entrada
        .trim()
        .parse()
        .expect("Error al convertir el valor")
}

fn main(){
    println!("Suma de números!");
    print!("Ingresa el primer dígito: ");
    let numero1: i32= leer();
    print!("\nIngresa el segundo número: ");
    let numero2: i32= leer();
    let resultado: i32= &numero1 + &numero2;
    println!("\n El resultado es: {}", resultado);
}