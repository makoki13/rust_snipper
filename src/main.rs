
use elementos::items::items::*;
use elementos::proyectil::proyectil::*;

mod elementos { 
    pub mod items;
    pub mod proyectil;
    }

fn main() {
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

    my_item1.print_posicion();
    my_item2.print_posicion();    
}
