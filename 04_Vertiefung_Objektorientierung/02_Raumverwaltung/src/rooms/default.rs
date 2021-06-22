pub trait Raum {
    fn room_number(&self) -> i32;
    fn licht_anschalten(&mut self);
}


pub struct SeminarRaum {
    room_number: i32,
    flaeche_tafel: f32,
    beamer_vorhanden: bool,
    licht_an: bool,
}

impl Raum for SeminarRaum {
    fn room_number(&self) -> i32 {
        self.room_number
    }

    fn licht_anschalten(&mut self) {
        self.licht_an = true;
    }
}


pub struct Klassenzimmer {
    room_number: i32,
    flaeche_tafel: f32,
    anzahl_fenster: i32,
    licht_an: bool
}

impl Raum for Klassenzimmer {
    fn room_number(&self) -> i32 {
        self.room_number
    }

    fn licht_anschalten(&mut self) {
        self.licht_an = true;
    }
}

mod bueros {
    use super::Raum;

    pub trait Buero: Raum {
        fn etage(&self) -> i32;
    }

    pub struct LehrBuero {
        room_number: i32,
        etage: i32,
        licht_an: bool,
    }

    impl Raum for LehrBuero {
        fn room_number(&self) -> i32 {
            self.room_number
        }

        fn licht_anschalten(&mut self) {
            self.licht_an = true;
        }
    }

    impl Buero for LehrBuero {
        fn etage(&self) -> i32 {
            self.etage
        }
    }

    pub struct Rechnerraum {
        room_number: i32,
        etage: i32,
        datenuebertragungsrate: f32,
        licht_an: bool,
    }

    impl Raum for Rechnerraum {
        fn room_number(&self) -> i32 {
            self.room_number
        }

        fn licht_anschalten(&mut self) {
            self.licht_an = true;
        }
    }

    impl Buero for Rechnerraum {
        fn etage(&self) -> i32 {
            self.etage
        }
    }
}
