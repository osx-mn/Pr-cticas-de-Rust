//lectura de números con funciones
use std::io;

fn leer() -> i32 {
    let mut entrada: String = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer el número");

    entrada
        .trim()
        .parse()
        .expect("Error al convertir el valor")
}

fn main(){
    println!("Ingrese un número");
    let numero: i32= leer();
    println!("El número ingresado fue: {}", numero);
}