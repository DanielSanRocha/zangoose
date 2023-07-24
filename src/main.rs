use druid::{AppLauncher, WindowDesc, PlatformError};

mod models;
mod ui;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui::ui_builder())
        .title("Zangoose")
        .with_min_size((600.0,400.0))
        .window_size((600.0,400.0));
    let data = models::AppData {master_volume: 1.0};

    println!("Starting application...");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}
