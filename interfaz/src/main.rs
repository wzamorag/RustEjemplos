use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculadora Cuadrática",
        options,
        Box::new(|_cc| Box::new(QuadraticApp::default())),
    )
}

struct QuadraticApp {
    a: String,
    b: String,
    c: String,
    resultado: String,
}

impl Default for QuadraticApp {
    fn default() -> Self {
        Self {
            a: "0".to_owned(),
            b: "0".to_owned(),
            c: "0".to_owned(),
            resultado: "Ingresa los valores y presiona calcular".to_owned(),
        }
    }
}

impl eframe::App for QuadraticApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Formulario de Ecuación Cuadrática");

            // Campos de entrada
            ui.horizontal(|ui| {
                ui.label("Valor a:");
                ui.text_edit_singleline(&mut self.a);
            });
            ui.horizontal(|ui| {
                ui.label("Valor b:");
                ui.text_edit_singleline(&mut self.b);
            });
            ui.horizontal(|ui| {
                ui.label("Valor c:");
                ui.text_edit_singleline(&mut self.c);
            });

            // Botón para procesar
            if ui.button("Calcular").clicked() {
                self.resultado = self.calcular_cuadratica();
            }

            ui.separator();
            ui.label(format!("Resultado: {}", self.resultado));
        });
    }
}

impl QuadraticApp {
    fn calcular_cuadratica(&self) -> String {
        // Parsear los strings a f64
        let a: f64 = self.a.parse().unwrap_or(0.0);
        let b: f64 = self.b.parse().unwrap_or(0.0);
        let c: f64 = self.c.parse().unwrap_or(0.0);

        let discriminante = b * b - 4.0 * a * c;

        if discriminante < 0.0 {
            "Raíces complejas (no reales)".to_string()
        } else {
            let x1 = (-b + discriminante.sqrt()) / (2.0 * a);
            let x2 = (-b - discriminante.sqrt()) / (2.0 * a);
            format!("x1 = {:.2}, x2 = {:.2}", x1, x2)
        }
    }
}
