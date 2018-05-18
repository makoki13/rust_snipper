extern crate rand;

//use elementos::items::items::*;
//use elementos::proyectil::proyectil::*;

use std::mem;
use std::io;

extern crate separator;

use separator::Separatable;

pub mod elementos;

fn main() {

    let dimension = 600;
    let mundo: Vec<i32> = Vec::with_capacity(dimension * dimension * dimension);
    
    let tamanyo = mundo.capacity() * mem::size_of::<i32>();

    println!("{}",mem::size_of::<i32>().separated_string());
    println!("{} bytes reservados en memoria!!!",tamanyo.separated_string());

    let mut input = String::new();

    io::stdin().read_line(&mut input);


    /*

    let mut my_item1 = Item::new();
    let id = my_item1._get_id();
    println!("{}",id);

    let mut my_item2 = Item::new();
    let id = my_item2._get_id();
    println!("{}",id);


    //let mut posicion = Posicion {x:10, y:15, z: 5};
    let mut posicion = my_item1.get_posicion();
    posicion.x += 100;
    my_item1.set_posicion(posicion);

    let mut posicion = my_item2.get_posicion();
    posicion.x += 150;
    my_item2.set_posicion(posicion);

    //my_item1.print_posicion();
    my_item2.print_posicion();  

    let mut my_proyectil = Proyectil::new();
    let posicion_bala:Posicion = Posicion {x:100, y:100, z:100};
    my_proyectil.set_posicion_actual(posicion_bala);

    println!("{}", my_proyectil.get_posicion_actual().z);

    println!("{}", my_item2.es_alcanzado(my_proyectil));  
    */
}
