#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, rc::Rc};

use displays::{cache::CacheDisplay, memory::MemoryDisplay, pipeline::PipelineDisplay};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use egui_extras::{Column, TableBuilder};
use simulator::{
    enums::{Condition, Register}, instruction::Instruction, memory::{ClockedMemory, DirectCache, FrontMemory, Memory}, Simulator
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
}

impl Default for SimulatorGUI {
    fn default() -> Self {
        let simulator = Rc::new(RefCell::new(create_simulator(true)));
        let simulator_state = simulator.borrow().get_state();
        let program_memory = simulator.borrow().get_program_memory();
        let data_memory = simulator.borrow().get_data_memory();
        let program_cache = simulator.borrow().get_program_cache();
        let data_cache = simulator.borrow().get_data_cache();


            data_memory.borrow_mut().write(0, 42).unwrap();
        
            program_memory.borrow_mut().write(0, Instruction::IntegerLoadData { rx: Register::R1, label: 0 }.into()).unwrap();
            program_memory.borrow_mut().write(1, Instruction::IntegerStoreData { rx: Register::R1, label: 1 }.into()).unwrap();
            program_memory.borrow_mut().write(2, Instruction::AddUnsignedInteger { c: false, rx: Register::R1, ry: Register::R1, rz: Register::R1 }.into()).unwrap();
            program_memory.borrow_mut().write(3, Instruction::IntegerLoadLow { rx: Register::R3, value: 1 }.into()).unwrap();

            program_memory.borrow_mut().write(4, Instruction::IntegerLoadHigh { rx: Register::R3, value: 0 }.into()).unwrap();


            program_memory.borrow_mut().write(5, Instruction::IntegerLoadData { rx: Register::R2, label: 0 }.into()).unwrap();
            program_memory.borrow_mut().write(6, Instruction::AddUnsignedInteger { c: false, rx: Register::R2, ry: Register::R2, rz: Register::R3 }.into()).unwrap();
            program_memory.borrow_mut().write(7, Instruction::IntegerStoreData { rx: Register::R2, label: 0 }.into()).unwrap();
            
            program_memory.borrow_mut().write(8, Instruction::ImmediateJump { l: false, condition: Condition::AlwaysTrue, label: 5 }.into()).unwrap();
            program_memory.borrow_mut().write(9, Instruction::IntegerStoreData { rx: Register::R4, label: 0 }.into()).unwrap();


        Self {
            simulator,
            pipeline_display: PipelineDisplay::new(simulator_state, "pipeline"),
            program_memory_display: MemoryDisplay::new(program_memory, "program_memory"),
            data_memory_display: MemoryDisplay::new(data_memory, "data_memory"),
            program_cache_display: CacheDisplay::new(program_cache, "program_cache"),
            data_cache_display: CacheDisplay::new(data_cache, "data_cache"),
        }
    }
}

impl SimulatorGUI {
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
}

// TODO: Simple program, more sophisticated (no pipe/cache, pipe only, cache only, both)
// TODO: See memory access information

impl eframe::App for SimulatorGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Single Step").clicked() {
                self.cycle();
            }

            if ui.button("100 Step").clicked() {
                self.cycle_many(100);
            }

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
