#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, rc::Rc};

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use simulator::{memory::{ClockedMemory, DirectCache, FrontMemory, Memory}, Simulator};

const DATA_M_CYCLES: usize = 2;
const PROG_M_CYCLES: usize = 2;

const DATA_C_CYCLES: usize = 1;
const PROG_C_CYCLES: usize = 1;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<SimulatorGUI>::default())
        }),
    )
}

fn create_simulator(use_cache: bool) -> Simulator {
    let raw_program_memory = Rc::new(RefCell::new(Memory::new()));
    let program_memory = Rc::new(RefCell::new(ClockedMemory::<PROG_M_CYCLES, _>::new(raw_program_memory.clone(), None)));

    let raw_program_cache = Rc::new(RefCell::new(DirectCache::<1>::new()));
    let program_cache = Rc::new(RefCell::new(ClockedMemory::<PROG_C_CYCLES, _>::new(raw_program_cache.clone(), Some(program_memory.clone()))));

    let raw_data_memory = Rc::new(RefCell::new(Memory::new()));
    let data_memory = Rc::new(RefCell::new(ClockedMemory::<DATA_M_CYCLES, _>::new(raw_data_memory.clone(), None)));

    let raw_data_cache = Rc::new(RefCell::new(DirectCache::<2>::new()));
    let data_cache = Rc::new(RefCell::new(ClockedMemory::<DATA_C_CYCLES, _>::new(raw_data_cache.clone(), Some(data_memory.clone()))));

    let used_prog: Rc<RefCell<dyn FrontMemory>> = if use_cache {program_cache} else {program_memory};
    let used_data: Rc<RefCell<dyn FrontMemory>> = if use_cache {data_cache} else {data_memory};

    Simulator::new(raw_program_memory, raw_data_memory, raw_program_cache, raw_data_cache, used_prog, used_data)
}

struct SimulatorGUI {
    simulator: Rc<RefCell<Simulator>>,
}

impl Default for SimulatorGUI {
    fn default() -> Self {
        Self {
            simulator: Rc::new(RefCell::new(create_simulator(true)))
        }
    }
}

impl eframe::App for SimulatorGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));

            TableBuilder::new(ui)
            .column(Column::auto().resizable(false))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("First column");
                });
                header.col(|ui| {
                    ui.heading("Second column");
                });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Hello");
                    });
                    row.col(|ui| {
                        ui.button("world!");
                    });
                });
            });
        });
    }
}