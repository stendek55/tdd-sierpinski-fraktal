// ============================================================================
// ===============================  TYPEN  ====================================
// ============================================================================

#[derive(Debug, PartialEq, Clone)]
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
            self.grid[x][y] = CanvasPixel::Fraktal;
        }
    }
}

// ============================================================================
// ============================  HAUPTPROGRAMM  ===============================
// ============================================================================
fn main() {
    println!("TDDprojekt - SIERPINSKI-FRAKTAL");
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
        let canvas = SierpinskiCanvas::new(2);
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
        let canvas = SierpinskiCanvas::new(2);
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
}
