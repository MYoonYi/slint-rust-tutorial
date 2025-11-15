// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Tiles aus dem Modell abrufen
    let mut tiles = main_window
        .get_memory_tiles()
        .iter()
        .collect::<Vec<TileData>>();

    // Duplizieren,um sicherzustellen, dass Paarevorhanden sind.
    tiles.extend(tiles.clone());

    // Tiles zufällig anordnen
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // den gemischten Vec zur Property des Modells zuweisen.
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    main_window.run().unwrap();
}
