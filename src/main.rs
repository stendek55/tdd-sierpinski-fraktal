// ============================================================================
// ===============================  TYPEN  ====================================
// ============================================================================

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CanvasPixel {
    Hintergrund,
    Fraktal,
    Ausserhalb,
}

pub struct SierpinskiCanvas {
    pub grid: Vec<Vec<CanvasPixel>>,
}

// ============================================================================
// =============================  FUNKTIONEN  =================================
// ============================================================================

impl SierpinskiCanvas {
    pub fn new(groesse: usize, x_versatz: usize, y_versatz: usize) -> Self {
        //falsche groessen abfangen
        if groesse == 0 {
            panic!("Groesse muss groesser als 0 sein!");
        }
        if !groesse.is_power_of_two() {
            panic!("Groesse muss zweierpotenz sein!");
        }

        //mindestmaße fürs dreieck
        let breite_3eck = (groesse * 2) - 1;
        let hoehe_3eck = groesse;

        //gesamtgröße des grids inklusive versatz
        let breite_gesamt = breite_3eck + 2 * (x_versatz);
        let hoehe_gesamt = hoehe_3eck + 2 * (y_versatz);

        //2-D spielfeld Erstellen
        //inneres vec! -> erstellt eine zeile mit richtiger breite
        //äußeres vec! -> kopiert diese zeile genauso oft wie die höhe bzw groesse ist
        let tabelle = vec![vec![CanvasPixel::Ausserhalb; breite_gesamt]; hoehe_gesamt];
        SierpinskiCanvas { grid: tabelle }
    }

    pub fn zeichne_rekursiv(&mut self, x: usize, y: usize, groesse: usize) {
        //wenn dreieck auf kleinstmögliche einheit (1 pixel) geschrumpt ist
        //wird fraktal gesetzt und beendet
        if groesse == 1 {
            self.grid[y][x] = CanvasPixel::Fraktal;
            return;
        }

        //dimension halbieren um dreieck in drei kleinere unterdreiecke zu zerlegen
        let halb = groesse / 2;

        //zeichne rekursiv die 3 teil-dreiecke
        self.zeichne_rekursiv(x, y, halb); //oben
        self.zeichne_rekursiv(x - halb, y + halb, halb); //unten links
        self.zeichne_rekursiv(x + halb, y + halb, halb); //unten rechts

        //das negativ loch in der mitte ausstanzen
        //das loch beginnt genau in vertikaler mitte (y + halb) und ist halb hoch
        for i in 0..halb {
            let zeile_y = y + halb + i;

            //das umgekehrte dreieck ist oben (bei i = 0) am breitesten
            //läuft nach unten spitz zu -> i zieht mit jeder zeile breite ab
            let breite = halb - 1 - i;
            let start_x = x - breite; //linke grenze loch
            let end_x = x + breite; //rechte grenze loch

            //zeile für zeile den bereich füllen
            for zeile_x in start_x..=end_x {
                self.grid[zeile_y][zeile_x] = CanvasPixel::Hintergrund;
            }
        }
    }

    pub fn darstellen(&self) {
        //vim liste sonderzeichen :digraphs
        //einfügen -> Strg+k -> vim-kürzel
        //し ぱ ₈ ∴ ⅔ 2 ◎ ●
        for zeile in &self.grid {
            for pixel in zeile {
                let zeichen = match pixel {
                    CanvasPixel::Fraktal => "▲",
                    CanvasPixel::Hintergrund => ".",
                    CanvasPixel::Ausserhalb => "☆",
                };
                print!("{}", zeichen);
            }
            println!();
        }
    }
}

// ============================================================================
// ============================  HAUPTPROGRAMM  ===============================
// ============================================================================
fn main() {
    println!("TDDprojekt - SIERPINSKI-FRAKTAL");
    //grösse mus zweierpotenz sein
    let groesse = 8;
    let x_offset = 4;
    let y_offset = 2;
    let iks = groesse - 1;
    let yps = 0;
    let mut canvas = SierpinskiCanvas::new(groesse, x_offset, y_offset);
    canvas.zeichne_rekursiv(iks + x_offset, yps + y_offset, groesse);
    canvas.darstellen();
}

// ============================================================================
// ===============================  TESTS  ====================================
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groesse_1_zeichnet_ein_einzelnes_zeichen() {
        // Erstelle das leere Feld
        let mut canvas = SierpinskiCanvas::new(1, 0, 0);

        // Rufe die Zeichenfunktion auf
        canvas.zeichne_rekursiv(0, 0, 1);

        // Überprüfe, ob sich das Pixel an Position [0][0] verändert hat
        assert_eq!(
            canvas.grid[0][0],
            CanvasPixel::Fraktal,
            "Bei Größe 1 muss das Pixel zu Fraktal werden!"
        );
    }
    #[test]
    fn test_initialisierung_groesse_2() {
        let canvas = SierpinskiCanvas::new(2, 0, 0);

        assert_eq!(
            canvas.grid.len(),
            2,
            "Das Grid muss exakt 2 Zeilen hoch sein"
        );
        assert_eq!(
            canvas.grid[0].len(),
            3,
            "Das Grid muss exakt 3 Spalten breit sein"
        );

        assert_eq!(
            canvas.grid[0][1],
            CanvasPixel::Ausserhalb,
            "Die Mitte sollte beim Erstellen noch leer (Ausserhalb) sein"
        );
    }

    #[test]
    fn test_initialisierung_dimensionen_groesse_4() {
        let canvas = SierpinskiCanvas::new(4, 0, 0);
        assert_eq!(canvas.grid.len(), 4, "Höhe bei Größe 4 muss 4 sein");
        assert_eq!(canvas.grid[0].len(), 7, "Breite bei Größe 4 muss 7 sein");
    }

    #[test]
    fn test_groesse_2_obere_spitze() {
        let mut canvas = SierpinskiCanvas::new(2, 0, 0);
        canvas.zeichne_rekursiv(1, 0, 2);
        assert_eq!(
            canvas.grid[0][1],
            CanvasPixel::Fraktal,
            "Oben muss eine Spitze sein"
        );
        assert_eq!(
            canvas.grid[0][0],
            CanvasPixel::Ausserhalb,
            "Neben Spitze muss leer sein"
        );
        assert_eq!(
            canvas.grid[0][2],
            CanvasPixel::Ausserhalb,
            "Neben Spitze muss leer sein"
        );
    }

    #[test]
    fn test_groesse_2_untere_basis() {
        let mut canvas = SierpinskiCanvas::new(2, 0, 0);
        canvas.zeichne_rekursiv(1, 0, 2);
        assert_eq!(
            canvas.grid[1][1],
            CanvasPixel::Hintergrund,
            "Die Mitte der Basis muss leer sein"
        );
        assert_eq!(
            canvas.grid[1][0],
            CanvasPixel::Fraktal,
            "Untere linke Ecke muss besetzt sein"
        );
        assert_eq!(
            canvas.grid[1][2],
            CanvasPixel::Fraktal,
            "Untere rechte Ecke muss besetzt sein"
        );
    }

    #[test]
    //test ohne asserts -> mit dieser flag wird ein panic-wurf erwartet zum bestehen
    //panic-wurf muss diese nachricht enthalten
    //(damit nicht ausversehen ein anderes panic den test bestehen lässt)
    #[should_panic(expected = "Groesse muss zweierpotenz sein!")]
    fn test_werfe_panic_bei_ungueltiger_groesse_keine_2erpotenz() {
        let _canvas = SierpinskiCanvas::new(7, 0, 0);
    }

    #[test]
    //test ohne asserts -> mit dieser flag wird ein panic-wurf erwartet zum bestehen
    //panic-wurf muss diese nachricht enthalten
    //(damit nicht ausversehen ein anderes panic den test bestehen lässt)
    #[should_panic(expected = "Groesse muss groesser als 0 sein!")]
    fn test_werfe_panic_bei_groesse_0() {
        let _canvas = SierpinskiCanvas::new(0, 0, 0);
    }

    #[test]
    fn test_anzahl_von_gesetzten_fraktalen_bei_groesse_4_und_64() {
        //ein echtes sierpinski-fraktal hat immer die selbe anzahl von gesetzten minifraktalen
        //bei größe 4 sind es 9 und bei größe 64 sind es 729
        //MATHEMATIK -> größe muss zweierpotenz sein und minis ergeben sich aus dreierpotenz bei
        //gleichem exponent
        //größe =  4=2² -> minifraktale = 3²=9
        //größe = 64=2⁶ -> minifraktale = 3⁶=729
        let mut canvas_4 = SierpinskiCanvas::new(4, 0, 0);
        let mut canvas_64 = SierpinskiCanvas::new(64, 0, 0);
        canvas_4.zeichne_rekursiv(3, 0, 4);
        canvas_64.zeichne_rekursiv(63, 0, 64);

        let minis_4_anzahl = canvas_4
            .grid //nimm das grid
            .iter() //gehe über die elemenete
            .flatten() //macht aus verschachtelung eine ebene
            .filter(|&&mf| mf == CanvasPixel::Fraktal) //filtert nur gesetzte fraktale
            .count(); //zählt das ergebnis

        let minis_64_anzahl = canvas_64
            .grid
            .iter()
            .flatten()
            .filter(|&&mf| mf == CanvasPixel::Fraktal)
            .count();

        assert_eq!(
            minis_4_anzahl, 9,
            "Bei größe 4 müssen 9 Minifraktale entstehen"
        );
        assert_eq!(
            minis_64_anzahl, 729,
            "Bei größe 64 müssen 729 Minifraktale entstehen"
        );
    }

    #[test]
    fn test_totale_anzahl_von_elementen_bei_groesse_128() {
        let mut canvas = SierpinskiCanvas::new(128, 0, 0);
        canvas.zeichne_rekursiv(127, 0, 128);

        //visuelle darstellung eines dreiecks mit grösse = 3 = höhe im grid(array)
        //...0... -> bau aus dreieck ein quadrat -> ..408..
        //..123.. ->                             -> ..123..
        //.45678. ->                             -> ..567..
        //summe aller elemente vom dreieck ist somit das quadrat der höhe
        let elemente_3eck = 128 * 128;
        //filtert und zählt die erstellten dreieckselemente
        let sum_alles_3eck = canvas
            .grid
            .iter()
            .flatten()
            .filter(|&&el| el == CanvasPixel::Hintergrund || el == CanvasPixel::Fraktal)
            .count();

        assert_eq!(
            sum_alles_3eck, elemente_3eck,
            "Bei größe 128 muss das Dreieck aus insgesamt {} Elementen bestehen (lebende und tote)",
            elemente_3eck
        );
    }

    #[test]
    fn test_offset_berechnung_und_zeichnung() {
        //dreieck soll um 5 nach rechts und 2 nach unten verschoben werden
        let offset_x = 5;
        let offset_y = 2;
        let groesse = 4;

        let mut canvas = SierpinskiCanvas::new(groesse, offset_x, offset_y);

        //berechnung des verschobenen startpunkts
        let start_x = (groesse - 1) + offset_x;
        let start_y = offset_y;
        canvas.zeichne_rekursiv(start_x, start_y, groesse);

        //befindet sich die spitze wirklich an neuer verschobener koordinate?
        assert_eq!(
            canvas.grid[start_y][start_x],
            CanvasPixel::Fraktal,
            "Spitze (Fraktal) muss an offset-Koordinate sein!"
        );

        //ist im offsetraum vor dem sierpinskidreieck unberührte aussenwelt
        assert_eq!(
            canvas.grid[0][0],
            CanvasPixel::Ausserhalb,
            "Der Bereich vor dem Offset darf nicht als Fraktal bzw Hintergrund gesetzt sein!"
        );
    }
}
