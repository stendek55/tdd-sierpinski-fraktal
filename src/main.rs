// ============================================================================
// ===============================  TYPEN  ====================================
// ============================================================================

#[derive(Debug, PartialEq)]
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
    pub fn new() -> Self {
        SierpinskiCanvas {
            grid: vec![vec![CanvasPixel::Hintergrund]],
        }
    }

    pub fn zeichne_rekursiv(&mut self, _x: i32, _y: i32, _groesse: i32) {}
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
        let mut canvas = SierpinskiCanvas::new();

        // Rufe die Zeichenfunktion auf
        canvas.zeichne_rekursiv(0, 0, 1);

        // Überprüfe, ob sich das Pixel an Position [0][0] verändert hat
        assert_eq!(
            canvas.grid[0][0],
            CanvasPixel::Fraktal,
            "Bei Größe 1 muss das Pixel zu Fraktal werden!"
        );
    }
}
