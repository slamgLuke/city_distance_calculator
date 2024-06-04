use crate::city_interface::*;
use crate::three_point::three_point_distance;
use eframe::egui;

// Visual Interface
pub struct MyApp {
    input1: String,
    input2: String,
    input3: String,
    method: CityType,
    output: (String, String),
    distance: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input1: String::new(),
            input2: String::new(),
            input3: String::new(),
            method: CityType::API,
            output: (String::new(), String::new()),
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

            ui.label("Third City:");
            ui.text_edit_singleline(&mut self.input3);

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

            if ui.button("Calculate shortest distance").clicked() {
                self.run_function();
            }

            ui.separator();

            ui.label(format!(
                "Closest City Pair: {}, {}",
                self.output.0, self.output.1
            ));
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
        let city3: CityInterface = city_from_type(&self.method, self.input3.as_str());

        let coords1 = get_coordinates(&city1);
        let coords2 = get_coordinates(&city2);
        let coords3 = get_coordinates(&city3);

        if coords1.is_ok() && coords2.is_ok() && coords3.is_ok() {
            let coords1 = coords1.unwrap();
            let coords2 = coords2.unwrap();
            let coords3 = coords3.unwrap();

            let (shortest_1, shortest_2, shortest_dist) =
                three_point_distance((&city1, &coords1), (&city2, &coords2), (&city3, &coords3));

            self.output = (get_name(shortest_1), get_name(shortest_2));
            self.distance = format!("{:.5} km", shortest_dist);
        }
    }
}
