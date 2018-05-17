pub mod proyectil {    
    use rand;

    use elementos::items::items::Posicion;

    pub struct Proyectil {
        pub posicion_anterior: Posicion,
        pub posicion_actual: Posicion,
        id: String,
    }
 
    impl Proyectil {
        pub fn new() -> Proyectil {        
            let random_number:u32 = rand::random();
            //let random_number: u8 = 20;
            let nuevo_id:String = random_number.to_string();
            Proyectil {
                posicion_anterior: Posicion{x:0, y:0, z:0},
                posicion_actual: Posicion{x:0, y:0, z:0},
                id: nuevo_id
            }

        }

        pub fn _get_id(&self) -> String {             
            let cadena = self.id.clone();
            cadena
        }

        pub fn set_posicion_actual(&mut self, posicion:Posicion) {
            self.posicion_actual = posicion;
        }

        pub fn get_posicion_anterior(&self) -> Posicion { 
            let mut pos = Posicion{x:0, y:0, z:0};

            pos.x = self.posicion_anterior.x;
            pos.y = self.posicion_anterior.y;
            pos.z = self.posicion_anterior.z;

            pos
        }

        pub fn get_posicion_actual(&self) -> Posicion {
            let mut pos = Posicion{x:0, y:0, z:0};

            pos.x = self.posicion_actual.x;
            pos.y = self.posicion_actual.y;
            pos.z = self.posicion_actual.z;
            
            pos            
        }
    }
}