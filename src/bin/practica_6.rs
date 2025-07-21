//Uso de impl con struct

use std::io;

fn leer() -> i32{
    let mut entrada: String= String::new();

    io::stdin().read_line(&mut entrada).expect("Error al leer la entrada");
    entrada.trim().parse().expect("Error al convertir el valor")
}

struct Cuadrado{
    ancho: i32,
    alto: i32,
}

impl Cuadrado{
    fn imprimir_dimensiones(&self){
        println!("El cuadrado mide {} de ancho por {} de alto", self.ancho, self.alto);
    }
}

fn main(){
    println!("Ingresa el ancho del cuadrado");
    let ancho= leer();
    println!("Ingresa el alto del cuadrado");
    let alto: i32= leer();

    let cuadrado_1= Cuadrado{
        ancho: ancho,
        alto: alto,
    };

    cuadrado_1.imprimir_dimensiones();
}