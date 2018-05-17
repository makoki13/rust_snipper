extern crate rand;

pub mod items {

    use elementos::proyectil::proyectil::Proyectil;

    use rand;

    pub struct Posicion {
        pub x: u32,
        pub y: u32,
        pub z: u32,
    }

    pub struct Item {
        posicion: Posicion,
        id: String,
    }

    impl Item {
        
        pub fn new() -> Item {        
            let random_number:u32 = rand::random();
            //let random_number: u8 = 20;
            let nuevo_id:String = random_number.to_string();
            Item {
                posicion: Posicion{x:0, y:0, z:0},
                id: nuevo_id
            }

        }

        pub fn _get_id(&self) -> String {             
            let cadena = self.id.clone();
            cadena
        }

        pub fn get_posicion(&self) -> Posicion {
            let mut pos = Posicion{x:0, y:0, z:0};
            pos.x = self.posicion.x;
            pos.y = self.posicion.y;
            pos.z = self.posicion.z;
            pos.z = self.posicion.z;
            pos
        }

        pub fn set_posicion(&mut self, nueva_posicion: Posicion) {
            self.posicion = nueva_posicion;
        }

        pub fn print_posicion(&mut self) {
            println!("{},{},{}", self.posicion.x, self.posicion.y, self.posicion.z);
        }

        /*
         * Dado una trayectoria cada item debe de determinar si esa trayectoria se cruzó con él en ese momento
         */ 
        pub fn es_alcanzado(&mut self, proyectil: Proyectil) -> bool {
            let resultado:bool = proyectil.get_posicion_actual().z > self.posicion.z;
            resultado
        }
    }
    
    

}