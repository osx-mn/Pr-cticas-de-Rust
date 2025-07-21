//lista de productos con estructuras

struct Producto{
    nombre_producto: String,
    precio_producto: f32,
    cantidad_producto: i32,
    tipo_producto: String,
}

fn main(){
    let escritorio= Producto{
        nombre_producto: String::from("Escritorio de oficina"),
        precio_producto: 65.00,
        cantidad_producto: 7,
        tipo_producto: String::from("Mueble"),
    };

    let manzana= Producto{
        nombre_producto: String::from("Manzana roja"),
        precio_producto: 1.20,
        cantidad_producto: 40,
        tipo_producto: String::from("fruta")
    };

    println!("listado de productos!");
    println!("Productos disponibles:\n {} por {} dólares,\n {} por {} dólares", escritorio.nombre_producto, escritorio.precio_producto as f32, manzana.nombre_producto, manzana.precio_producto as f32);
}