// ============================================================================
// ===============================  TYPEN  ====================================
// ============================================================================

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CanvasPixel {
    Hintergrund,
    Fraktal,
}

pub struct SierpinskiCanvas {
    pub grid: Vec<Vec<CanvasPixel>>,
}

// ============================================================================
// =============================  FUNKTIONEN  =================================
// ============================================================================

impl SierpinskiCanvas {
    /// Erstellt ein minimales 1x1 Feld, damit der erste Test funktioniert
    pub fn new(groesse: usize) -> Self {
        //falsche groessen abfangen
        if groesse == 0 {
            panic!("Groesse muss groesser als 0 sein!");
        }
        if !groesse.is_power_of_two() {
            panic!("Groesse muss zweierpotenz sein!");
        }

        //breite aus grösse berechnen
        let berechnete_breite = (groesse * 2) - 1;

        //2-D spielfeld Erstellen
        //inneres vec! -> erstellt eine zeile mit richtiger breite
        //äußeres vec! -> kopiert diese zeile genauso oft wie die höhe bzw groesse ist
        let tabelle = vec![vec![CanvasPixel::Hintergrund; berechnete_breite]; groesse];
        SierpinskiCanvas { grid: tabelle }
    }

    pub fn zeichne_rekursiv(&mut self, x: usize, y: usize, groesse: usize) {
        if groesse == 1 {
            self.grid[y][x] = CanvasPixel::Fraktal;
            return;
        }

        let halb = groesse / 2;

        self.zeichne_rekursiv(x, y, halb);
        self.zeichne_rekursiv(x - halb, y + halb, halb);
        self.zeichne_rekursiv(x + halb, y + halb, halb);
    }

    pub fn darstellen(&self) {
        //vim liste sonderzeichen :digraphs
        //einfügen -> Strg+k -> vim-kürzel
        //し ぱ ₈ ∴ ⅔ 2 ◎ ●
        for zeile in &self.grid {
            for pixel in zeile {
                let zeichen = match pixel {
                    CanvasPixel::Fraktal => "∴",
                    CanvasPixel::Hintergrund => " ",
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
    let groesse = 32;
    let iks = groesse - 1;
    let yps = 0;
    let mut canvas = SierpinskiCanvas::new(groesse);
    canvas.zeichne_rekursiv(iks, yps, groesse);
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
        let mut canvas = SierpinskiCanvas::new(1);

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
        let canvas = SierpinskiCanvas::new(2);

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
            CanvasPixel::Hintergrund,
            "Die Mitte sollte beim Erstellen noch leer (Hintergrund) sein"
        );
    }

    #[test]
    fn test_initialisierung_dimensionen_groesse_4() {
        let canvas = SierpinskiCanvas::new(4);
        assert_eq!(canvas.grid.len(), 4, "Höhe bei Größe 4 muss 4 sein");
        assert_eq!(canvas.grid[0].len(), 7, "Breite bei Größe 4 muss 7 sein");
    }

    #[test]
    fn test_groesse_2_obere_spitze() {
        let mut canvas = SierpinskiCanvas::new(2);
        canvas.zeichne_rekursiv(1, 0, 2);
        assert_eq!(
            canvas.grid[0][1],
            CanvasPixel::Fraktal,
            "Oben muss eine Spitze sein"
        );
        assert_eq!(
            canvas.grid[0][0],
            CanvasPixel::Hintergrund,
            "Neben Spitze muss leer sein"
        );
        assert_eq!(
            canvas.grid[0][2],
            CanvasPixel::Hintergrund,
            "Neben Spitze muss leer sein"
        );
    }

    #[test]
    fn test_groesse_2_untere_basis() {
        let mut canvas = SierpinskiCanvas::new(2);
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
        let _canvas = SierpinskiCanvas::new(7);
    }

    #[test]
    //test ohne asserts -> mit dieser flag wird ein panic-wurf erwartet zum bestehen
    //panic-wurf muss diese nachricht enthalten
    //(damit nicht ausversehen ein anderes panic den test bestehen lässt)
    #[should_panic(expected = "Groesse muss groesser als 0 sein!")]
    fn test_werfe_panic_bei_groesse_0() {
        let _canvas = SierpinskiCanvas::new(0);
    }

    #[test]
    fn test_anzahl_von_gesetzten_fraktalen_bei_groesse_4_und_64() {
        //ein echtes sierpinski-fraktal hat immer die selbe anzahl von gesetzten minifraktalen
        //bei größe 4 sind es 9 und bei größe 64 sind es 729
        //MATHEMATIK -> größe muss zweierpotenz sein und minis ergeben sich aus dreierpotenz bei
        //gleichem exponent
        //größe =  4=2² -> minifraktale = 3²=9
        //größe = 64=2⁶ -> minifraktale = 3⁶=729
        let mut canvas_4 = SierpinskiCanvas::new(4);
        let mut canvas_64 = SierpinskiCanvas::new(64);
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
}
