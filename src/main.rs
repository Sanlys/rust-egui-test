use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 300.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    let mut url = "https://google.com".to_owned();
    let mut port = "80".to_owned();

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            let button = ui.button("Click each year");
            if button.clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));

            ui.heading("Network connection");
            ui.horizontal(|ui| {
                let url_label = ui.label("Enter a URL");
                ui.text_edit_singleline(&mut url)
                    .labelled_by(url_label.id);
            });
            ui.horizontal(|ui| {
                let port_label = ui.label("Enter a port");
                ui.text_edit_singleline(&mut port)
                    .labelled_by(port_label.id);
            });
        });
    })
}