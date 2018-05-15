
pub mod items {
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
            let nuevo_id:String = String::from("Hello, world!");
            Item {
                posicion: Posicion{x:0, y:0, z:0},
                id: nuevo_id
            }

        }

        pub fn _get_id(&self) -> String { 
            let cadena:String = String::from("Hello, world!");
            // let cadena:String = self.id; inexplicable error
            return cadena;
        }

        pub fn get_posicion(&self) -> Posicion {
            let mut pos = Posicion{x:0, y:0, z:0};
            pos.x = self.posicion.x;
            pos.y = self.posicion.y;
            pos.z = self.posicion.z;
            pos.z = self.posicion.z;
            return pos;
        }

        pub fn set_posicion(&mut self, nueva_posicion: Posicion) {
            self.posicion = nueva_posicion;
        }

        pub fn print_posicion(&mut self) {
            println!("{},{},{}", self.posicion.x, self.posicion.y, self.posicion.z);
        } 
    }
    
    

}