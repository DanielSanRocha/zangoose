use std::{sync::mpsc, time::Duration};

use druid::{AppLauncher, WindowDesc, PlatformError};

mod models;
mod ui;
mod algorithms;

fn main() -> Result<(), PlatformError> {
    let data: models::AppData = models::AppData {volume: 0.5, denoise: false};
    let (tx_volume,rx_volume) = mpsc::channel::<f32>();
    let (tx_denoise,rx_denoise) = mpsc::channel::<bool>();

    let (client,_status) = jack::Client::new("Zangoose", jack::ClientOptions::NO_START_SERVER).unwrap();
    let in_port = client.register_port("in", jack::AudioIn::default()).expect("Error creating AudioIn port!");
    let mut out_port = client.register_port("out", jack::AudioOut::default()).expect("Error creating AudioOut port!");

    let mut volume = 0.5 as f32;
    let mut denoise_activated = false;

    let cback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        if let Ok(v) = rx_volume.recv_timeout(Duration::from_nanos(1)) {
            volume = v;
        }

        if let Ok(b) = rx_denoise.recv_timeout(Duration::from_nanos(1)) {
            denoise_activated = b;
        }

        let buffer = in_port.as_slice(ps).to_vec();
        let out_buffer = out_port.as_mut_slice(ps);

        let transformed = if denoise_activated { algorithms::denoise(buffer) } else { buffer };

        for (i, sample) in out_buffer.iter_mut().enumerate() {
            *sample = 2.0 * volume * transformed[i]
        }

        jack::Control::Continue
    };

    let main_window = WindowDesc::new(ui::ui_builder(tx_volume, tx_denoise))
        .title("Zangoose")
        .with_min_size((300.0,100.0))
        .window_size((300.0,100.0));

    println!("Activating jack client...");
    let activate_client = client.activate_async((), jack::ClosureProcessHandler::new(cback)).unwrap();

    println!("Starting application...");
    let result = AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data);

    activate_client.deactivate().unwrap();
    result
}
