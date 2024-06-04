use crate::city_interface::*;
use crate::distance::distance_km;
use eframe::egui;

// Visual Interface
pub struct MyApp {
    input1: String,
    input2: String,
    method: CityType,
    output1: String,
    output2: String,
    distance: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input1: String::new(),
            input2: String::new(),
            method: CityType::API,
            output1: String::new(),
            output2: String::new(),
            distance: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("First City:");
            ui.text_edit_singleline(&mut self.input1);

            ui.label("Second City:");
            ui.text_edit_singleline(&mut self.input2);

            ui.label("Method:");
            egui::ComboBox::from_label("Select a method")
                .selected_text(format!(
                    "{}",
                    match self.method {
                        CityType::API => "API",
                        CityType::CSV => "CSV",
                        CityType::Mock => "Mock",
                    }
                ))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.method, CityType::API, "API");
                    ui.selectable_value(&mut self.method, CityType::CSV, "CSV");
                    ui.selectable_value(&mut self.method, CityType::Mock, "Mock");
                });

            if ui.button("Calculate distance").clicked() {
                self.run_function();
            }

            ui.separator();

            ui.label(format!("First City Coordinates:     {}", self.output1));
            ui.label(format!("Second City Coordinates: {}", self.output2));
            ui.label(format!("Distance: {}", self.distance));
        });
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn run_function(&mut self) {
        let city1: CityInterface = city_from_type(&self.method, self.input1.as_str());
        let city2: CityInterface = city_from_type(&self.method, self.input2.as_str());

        let coords1 = get_coordinates(&city1);
        let coords2 = get_coordinates(&city2);

        match (coords1, coords2) {
            (Ok(coords1), Ok(coords2)) => {
                self.output1 = format!("{:.5}, {:.5}", coords1.latitude, coords1.longitude);
                self.output2 = format!("{:.5}, {:.5}", coords2.latitude, coords2.longitude);
                self.distance = format!("{:.1} km", distance_km(&coords1, &coords2));
            }
            (Err(e), Ok(_)) => self.output1 = format!("Error getting coordinates: {}", e),
            (Ok(_), Err(e)) => self.output2 = format!("Error getting coordinates: {}", e),
            (Err(e1), Err(e2)) => {
                self.output1 = format!("Error getting coordinates: {}", e1);
                self.output2 = format!("Error getting coordinates: {}", e2);
            }
        }
    }
}
