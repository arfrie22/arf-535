#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, rc::Rc};

use assembler::{assemble, load_file};
use displays::{cache::CacheDisplay, memory::MemoryDisplay, pipeline::PipelineDisplay};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use simulator::{
    memory::{ClockedMemory, DirectCache, FrontMemory, Memory}, Simulator
};

const DATA_M_CYCLES: usize = 2;
const PROG_M_CYCLES: usize = 2;

const DATA_C_CYCLES: usize = 1;
const PROG_C_CYCLES: usize = 1;

const PROGRAM_CACHE_LINES: usize = 4;
const DATA_CACHE_LINES: usize = 4;

pub mod displays;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Analog/RF ISA Simulator",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "monaspace".to_owned(),
                std::sync::Arc::new(
                    // .ttf and .otf supported
                    FontData::from_static(include_bytes!("MonaspaceNeon-Light.otf")),
                ),
            );

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "monaspace".to_owned());

            // Put my font as last fallback for monospace:
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .push("monaspace".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MACCHIATO);

            Ok(Box::<SimulatorGUI>::default())
        }),
    )
}

fn create_simulator(use_cache: bool) -> Simulator {
    let raw_program_memory = Rc::new(RefCell::new(Memory::new()));
    let program_memory = Rc::new(RefCell::new(ClockedMemory::<PROG_M_CYCLES, _>::new(
        raw_program_memory.clone(),
        None,
    )));

    let raw_program_cache = Rc::new(RefCell::new(DirectCache::<PROGRAM_CACHE_LINES>::new()));
    let program_cache = Rc::new(RefCell::new(ClockedMemory::<PROG_C_CYCLES, _>::new(
        raw_program_cache.clone(),
        Some(program_memory.clone()),
    )));

    let raw_data_memory = Rc::new(RefCell::new(Memory::new()));
    let data_memory = Rc::new(RefCell::new(ClockedMemory::<DATA_M_CYCLES, _>::new(
        raw_data_memory.clone(),
        None,
    )));

    let raw_data_cache = Rc::new(RefCell::new(DirectCache::<DATA_CACHE_LINES>::new()));
    let data_cache = Rc::new(RefCell::new(ClockedMemory::<DATA_C_CYCLES, _>::new(
        raw_data_cache.clone(),
        Some(data_memory.clone()),
    )));

    let used_prog: Rc<RefCell<dyn FrontMemory>> = if use_cache {
        program_cache
    } else {
        program_memory
    };
    let used_data: Rc<RefCell<dyn FrontMemory>> = if use_cache { data_cache } else { data_memory };

    Simulator::new(
        raw_program_memory,
        raw_data_memory,
        raw_program_cache,
        raw_data_cache,
        used_prog,
        used_data,
    )
}

struct SimulatorGUI {
    simulator: Rc<RefCell<Simulator>>,
    pipeline_display: PipelineDisplay,
    program_memory_display: MemoryDisplay,
    data_memory_display: MemoryDisplay,
    program_cache_display: CacheDisplay<PROGRAM_CACHE_LINES>,
    data_cache_display: CacheDisplay<DATA_CACHE_LINES>,
    use_pipeline: bool,
    use_cache: bool,
}

impl Default for SimulatorGUI {
    fn default() -> Self {
        let res = Self::new(true, true);
        
        let input = "
            .prog
            ldr r1 d:v1
            str r1 d:v2
            add r1 r1 r1

            ldl r3 1
            ldh r3 0

            loop:
            ldr r2 d:v1
            add r2 r2 r3
            str r2 d:v1

            b p:loop
            str r4 d:v1

            .data
            v1 42#1
            v2 0#1
        ";

        let a = assemble(input).unwrap();
        let mut v = Vec::new();
        a.to_file(&mut v);

        load_file(&v[..], &mut res.simulator.borrow_mut());

        res
    }
}

impl SimulatorGUI {
    fn new(use_pipeline: bool, use_cache: bool) -> Self {
        let simulator = Rc::new(RefCell::new(create_simulator(use_cache)));
        let simulator_state = simulator.borrow().get_state();
        simulator_state.borrow_mut().single_instruction_pipeline = !use_pipeline;

        let program_memory = simulator.borrow().get_program_memory();
        let data_memory = simulator.borrow().get_data_memory();
        let program_cache = simulator.borrow().get_program_cache();
        let data_cache = simulator.borrow().get_data_cache();

        Self {
            simulator,
            pipeline_display: PipelineDisplay::new(simulator_state, "pipeline"),
            program_memory_display: MemoryDisplay::new(program_memory, "program_memory"),
            data_memory_display: MemoryDisplay::new(data_memory, "data_memory"),
            program_cache_display: CacheDisplay::new(program_cache, "program_cache"),
            data_cache_display: CacheDisplay::new(data_cache, "data_cache"),
            use_pipeline,
            use_cache,
        }
    }

    fn cycle(&mut self) {
        self.simulator.borrow().cycle();
        self.program_memory_display.reload_inputs();
        self.data_memory_display.reload_inputs();
    }

    fn cycle_many(&mut self, count: usize) {
        let state = self.simulator.borrow().get_state();
        state.borrow_mut().running = true;
        for _ in 0..count {
            if !state.borrow().running {
                break
            }

            self.cycle();
        }
    }

    fn reload(&mut self) {
        *self = Self::new(self.use_pipeline, self.use_cache);

        let input = "
            .prog
            ldr r1 d:v1
            str r1 d:v2
            add r1 r1 r1

            ldl r3 1
            ldh r3 0

            loop:
            ldr r2 d:v1
            add r2 r2 r3
            str r2 d:v1

            b p:loop
            str r4 d:v1

            .data
            v1 42#1
            v2 0#1
        ";

        let a = assemble(input).unwrap();
        let mut v = Vec::new();
        a.to_file(&mut v);

        load_file(&v[..], &mut self.simulator.borrow_mut());
    }
}

// TODO: Simple program, more sophisticated (no pipe/cache, pipe only, cache only, both)
// TODO: See memory access information

impl eframe::App for SimulatorGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let button = ui.button("Load");
                ui.checkbox(&mut self.use_pipeline, "Use Pipeline");
                ui.checkbox(&mut self.use_cache, "Use Cache");
                if button.clicked() {
                    self.reload();
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Single Step").clicked() {
                    self.cycle();
                }

                if ui.button("1000 Steps").clicked() {
                    self.cycle_many(1000);
                }
            });

            self.pipeline_display.ui(ui);
            ui.horizontal_top(|ui| {
                self.program_memory_display.ui(ui);
                self.program_cache_display.ui(ui);
            });
            ui.horizontal(|ui| {
                self.data_memory_display.ui(ui);
                self.data_cache_display.ui(ui);
            });
        });
    }
}
