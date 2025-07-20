//estructuras

struct Cuadrado{
    ancho: i32,
    alto: i32,
}

fn main(){
    let c = Cuadrado{
        ancho: 10,
        alto: 12,
    };

    println!("El cuadrado mide {} x {}", c.ancho, c.alto);
}