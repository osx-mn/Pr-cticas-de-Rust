//lectura de números en rust
use std::io;

fn main(){
    println!("Ingresa un número");

    let mut entrada: String= String::new();

    io::stdin()
        .read_line(&mut entrada) //el simbolo & es para hacer referencias a variables sin reemplazarlas
        .expect("Error al leer la variable");

    let numero: i32 = entrada
        .trim()
        .parse()
        .expect("Número no válido");

    println!("El número ingresado es: {}", numero);    
}