// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ComponentHandle, Timer};
use std::time::Duration;

slint::include_modules!();

// Webseite-Version: auf Console "python -m http.server" eingeben
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Tiles aus dem Modell abrufen
    let mut tiles = main_window
        .get_memory_tiles()
        .iter()
        .collect::<Vec<TileData>>();

    // Duplizieren,um sicherzustellen, dass Paare vorhanden sind.
    tiles.extend(tiles.clone());

    // Tiles zufällig anordnen
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Den gemischten Vec zur Property des Modells zuweisen.
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    // Jetzt die Game-Logik implementieren.
    // - Weak-Pointer erstellen, um Zyklen der Referenzen zu vermeiden.
    let main_window_weak = main_window.as_weak();
    // - Die Callback-Funktion definieren, die aufgerufen wird, wenn ein Kachel angeklickt wird.
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (
            Some((tile_number1_index, mut tile_number1)),
            Some((tile_number2_index, mut tile_number2)),
        ) = (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = tile_number1 == tile_number2;
            if is_pair_solved {
                tile_number1.solved = true;
                tiles_model.set_row_data(tile_number1_index, tile_number1);
                tile_number2.solved = true;
                tiles_model.set_row_data(tile_number2_index, tile_number2);
            } else {
                // Den Steuern über das Fenster zu haben, um das Klick eines Kachels temporär bzw. für 1 Sekunde zu sperren.
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);

                // Jetzt die Interaktion mit den Kacheln wieder (nach 1 Sekunde) freigeben.
                let tiles_model = tiles_model.clone();
                Timer::single_shot(Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    tile_number1.image_visible = false;
                    tiles_model.set_row_data(tile_number1_index, tile_number1);
                    tile_number2.image_visible = false;
                    tiles_model.set_row_data(tile_number2_index, tile_number2);
                });
            }
        }
    });

    main_window.run().unwrap();
}
