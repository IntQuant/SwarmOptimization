#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui::{self, Painter, WidgetText, Ui, Widget}, epaint::Color32, emath};
use particles::{ParticleWorld, himmelblau, StepSettings, Vector};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Particle swarm optimization", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    optimizer: Option<ParticleWorld<2>>,
    particle_amount: usize,
    settings: StepSettings,
    size_modifier: f32,
    distribution: f32,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let style = egui::Style::default();
        if false {
            cc.egui_ctx.set_style(style);
        }
        //cc.egui_ctx.set_pixels_per_point(2.0);
        Self {
            particle_amount: 128,
            size_modifier: 100.0,
            distribution: 5.0,
            ..Self::default()
        }
    }
}

impl MyEguiApp {
    /*
    fn *_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("*").show(ctx, |ui| {});
    }
    */
    fn labeled_drag_value<Num: emath::Numeric>(ui: &mut Ui, label: impl Into<WidgetText>, value: &mut Num, speed: impl Into<f64>) {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.add(egui::DragValue::new(value).speed(speed));
        });
    }

    fn labeled_control(ui: &mut Ui, label: impl Into<WidgetText>, widget: impl Widget) {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.add(widget);
        });
    }

    fn create_optimizer_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Создать оптимизатор").show(ctx, |ui| {
            Self::labeled_drag_value(ui, "Количество частиц:", &mut self.particle_amount, 1.0);
            Self::labeled_drag_value(ui, "Коэфф. инерции:", &mut self.settings.inertia_factor, 0.1);
            Self::labeled_drag_value(ui, "Коэфф. своего лучшего ответа:", &mut self.settings.my_position_factor, 0.1);
            Self::labeled_drag_value(ui, "Коэфф. общего лучшего ответа:", &mut self.settings.swarm_position_factor, 0.1);
            Self::labeled_drag_value(ui, "Начальное распределение частиц:", &mut self.distribution, 0.1);
            if ui.button("Создать оптимизатор").clicked() {
                self.optimizer = Some(ParticleWorld::new(self.particle_amount, self.distribution));
            }
        });
    }

    fn optimizer_control_window(&mut self, ctx: &egui::Context) {
        let mut opt = self.optimizer.take().expect("Should be called when optimizer exists");
        egui::Window::new("Оптимизатор").show(ctx, |ui| {
            let (score, position) = opt.best_solution();
            ui.label(format!("Лучшее решение: {:} (x: {:}, y: {:})", score, position.x, position.y));
            if ui.button("Шаг").clicked() {
                opt.step(himmelblau, &self.settings);
            }
            Self::labeled_control(ui, "Масштаб:", egui::Slider::new(&mut self.size_modifier, 1.0..=1000.0));
            if !ui.button("Сброс").clicked() {
                self.optimizer = Some(opt);
            }
        });
    }

    fn draw_optimizer_state(opt: &ParticleWorld<2>, painter: &Painter, size_modifier: f32) {
        let mut center = opt.particles.iter().map(|x| {
            x.position
        }).reduce(|acc, el| {
            acc + el
        }).unwrap_or_default() / opt.particles.len() as f32 * size_modifier;
        
        let paint_center = painter.clip_rect().center().to_vec2();
        center -= Vector::<2>::new(paint_center.x, paint_center.y);

        for particle in &opt.particles {
            let pos = particle.position * size_modifier;
            painter.circle_filled((pos-center).data.0[0].into() , 3.0, Color32::BLACK);
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //ui.heading("Hello World!");
        match &self.optimizer {
            Some(opt) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let painter = ui.painter();
                    Self::draw_optimizer_state(opt, painter, self.size_modifier);
                });
                self.optimizer_control_window(ctx);
            },
            None => {
                egui::CentralPanel::default().show(ctx, |_ui| {});
                self.create_optimizer_window(ctx);
            }
        }
    }
}
