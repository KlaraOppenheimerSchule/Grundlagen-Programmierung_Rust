# OOP Test
Eine "Klasse" welche einen Nutzername & Passwort speichert und dieser abgefragt werden kann.  
Rust hat normalerweise keine implementierung für Klassen, das verhalten kann aber trotzdem "Emultiert" werden.  
Hierzu erstellt man eine Struktur und schreib implementierungen dafür
```Rust
struct MyStruct {
    my_field: MyType
}

impl MyStruct {
    fn my_method();
}
```

Wenn sichtbarkeit benötigt wird muss man die Struktur in einem "Modul" schreiben:
```Rust
mod my_module {
    struct MyStruct {

    }
}
```

oder es in einer seperaten Datei schreiben, was auch als "Modul" erkannt wird.  
Dann kann man entscheiden ob eine Methode oder eine Variable "public" oder "private" ist:
```Rust
pub struct MyStruct { // <-- Für alle Programme ausserhalb des "Moduls" sichtbar wenn das Modul importiert wird
    pub my_field: MyType, // <-- Für programme ausserhalb des "Moduls" sichtbar
    my_second_field: MyOtherType, // <-- Nur Funktionen innerhalb des Moduls haben zugriff auf die Variable
}
```
Ein Problem ist nun dass, anderst als z.B. bei C++, es bei Rust kein Constructor gibt, man kann aber einfach eine "Methode" selber schreiben: 
```Rust
impl MyStruct {
    fn new([parameter]) -> MyStruct { // <-- Funktion gibt instanz der Struktur zurück
        MyStruct {
            my_field: [parameter_my_field] // <-- Beispiel hier ist ein Konstruktor, die Funktion kann aber auch ohne Parameter funktionieren,
                                           // indem z.B. Standartwerte benutzt werden
        }
    }
}
```
Dieser "Contructor" wird aber nur benötigt wenn 1 oder mehrere Felder "private" sind, falls alle felder "public" sind kann man die Struktur normal initialisieren
```Rust
let my_struct = MyStruct {
    my_field: 5
}
```


Jetzt kann man mit "gettern" und "settern" private Felder verändern und lesen.
