pub trait Fahrzeug {
    fn fortbewegen(&mut self);
    fn anzahl_sitzplaetze(&self) -> i32;
    fn hoechstgeschwindigkeit(&self) -> i32;
}

pub struct Landfahrzeug {
    anzahl_sitzplaetze: i32,
    hoechstgeschwindigkeit: i32,
}


pub struct Wasserfahrzeug {
    anzahl_sitzplaetze: i32,
    hoechstgeschwindigkeit: i32,
}


impl Landfahrzeug {
    pub fn new() -> Self {
        Self {
            anzahl_sitzplaetze: 5,
            hoechstgeschwindigkeit: 180,
        }
    }
}

impl Fahrzeug for Landfahrzeug {
    fn fortbewegen(&mut self) {
        println!("Das Landfahrzeug fährt auf dem Land!");
    }

    fn anzahl_sitzplaetze(&self) -> i32 {
        self.anzahl_sitzplaetze
    }

    fn hoechstgeschwindigkeit(&self) -> i32 {
        self.hoechstgeschwindigkeit
    }
}


impl Wasserfahrzeug {
    pub fn new() -> Self {
        Self {
            anzahl_sitzplaetze: 20,
            hoechstgeschwindigkeit: 60,
        }
    }
}

impl Fahrzeug for Wasserfahrzeug {
    fn fortbewegen(&mut self) {
        println!("Das Wasserfahrzeug fährt in dem Wasser!");
    }

    fn anzahl_sitzplaetze(&self) -> i32 {
        self.anzahl_sitzplaetze
    }

    fn hoechstgeschwindigkeit(&self) -> i32 {
        self.anzahl_sitzplaetze
    }
}


pub struct Amphibienfahrzeug {
    land_hoechstgeschwindigkeit: i32,
    wasser_hoechstgeschwindigkeit: i32,
    anzahl_sitzplaetze: i32,
}

impl Fahrzeug for Amphibienfahrzeug {
    fn fortbewegen(&mut self) {
        println!("Das Amphibienfahrzeug fährt auf dem Wasser und auf dem Land");
    }

    fn anzahl_sitzplaetze(&self) -> i32 {
        todo!()
    }

    fn hoechstgeschwindigkeit(&self) -> i32 {
        todo!()
    }
}
