#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, io::Read, path::Path, rc::Rc, time::Instant};

use assembler::{assemble, load_file};
use displays::{cache::CacheDisplay, condition::ConditionDisplay, f_register::FRegisterDisplay, memory::MemoryDisplay, pipeline::PipelineDisplay, register::RegisterDisplay, timer::TimerDisplay};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use log::{error, info};
use simulator::{
    memory::{ClockedMemory, DirectCache, FrontMemory, Memory}, streams::{InputStream, OutputStream, ConstantInput, NoOperationOutput}, Simulator
};

const DATA_M_CYCLES: usize = 4;
const PROG_M_CYCLES: usize = 10;

const DATA_C_CYCLES: usize = 1;
const PROG_C_CYCLES: usize = 1;

const PROGRAM_CACHE_LINES: usize = 32;
const DATA_CACHE_LINES: usize = 32;

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

    let adc_streams = core::array::from_fn(|_| Box::new(ConstantInput::new(0)) as Box<dyn InputStream>);
    let dac_streams = core::array::from_fn(|_| Box::new(NoOperationOutput::new()) as Box<dyn OutputStream>);

    Simulator::new(
        raw_program_memory,
        raw_data_memory,
        raw_program_cache,
        raw_data_cache,
        used_prog,
        used_data,
        adc_streams,
        dac_streams,
        1
    )
}

struct SimulatorGUI {
    simulator: Rc<RefCell<Simulator>>,
    pipeline_display: PipelineDisplay,
    register_display: RegisterDisplay,
    f_register_display: FRegisterDisplay,
    timer_display: TimerDisplay,
    condition_display: ConditionDisplay,
    program_memory_display: MemoryDisplay,
    data_memory_display: MemoryDisplay,
    program_cache_display: CacheDisplay<PROGRAM_CACHE_LINES>,
    data_cache_display: CacheDisplay<DATA_CACHE_LINES>,
    remaining_steps: usize,
    file_name: String,
    use_pipeline: bool,
    use_cache: bool,
}

impl Default for SimulatorGUI {
    fn default() -> Self {
        Self::new(true, true, "test")
    }
}

impl SimulatorGUI {
    fn new(use_pipeline: bool, use_cache: bool, file_name: &str) -> Self {
        let simulator = Rc::new(RefCell::new(create_simulator(use_cache)));
        let simulator_state = simulator.borrow().get_state();
        simulator_state.borrow_mut().single_instruction_pipeline = !use_pipeline;

        let program_memory = simulator.borrow().get_program_memory();
        let data_memory = simulator.borrow().get_data_memory();
        let program_cache = simulator.borrow().get_program_cache();
        let data_cache = simulator.borrow().get_data_cache();

        Self {
            simulator,
            pipeline_display: PipelineDisplay::new(simulator_state.clone(), "pipeline"),
            register_display: RegisterDisplay::new(simulator_state.clone(), "register"),
            f_register_display: FRegisterDisplay::new(simulator_state.clone(), "f_register"),
            timer_display: TimerDisplay::new(simulator_state.clone(), "timer"),
            condition_display: ConditionDisplay::new(simulator_state.clone(), "condition"),
            program_memory_display: MemoryDisplay::new(program_memory, "program_memory"),
            data_memory_display: MemoryDisplay::new(data_memory, "data_memory"),
            program_cache_display: CacheDisplay::new(program_cache, "program_cache"),
            data_cache_display: CacheDisplay::new(data_cache, "data_cache"),
            remaining_steps: 0,
            file_name: file_name.to_owned(),
            use_pipeline,
            use_cache,
        }
    }

    fn cycle(&mut self) {
        self.simulator.borrow_mut().cycle();
        self.program_memory_display.reload_inputs();
        self.data_memory_display.reload_inputs();
    }

    fn cycle_remaining(&mut self) {
        let start = Instant::now();
        let state = self.simulator.borrow().get_state();
        state.borrow_mut().running = true;
        loop {
            if !state.borrow().running {
                self.remaining_steps = 0;
            }

            if  (Instant::now() - start).as_millis() > 1000 || self.remaining_steps == 0 {
                break;
            }

            self.cycle();

            self.remaining_steps -= 1;
        }
    }

    fn load_file(&mut self) {
        *self = Self::new(self.use_pipeline, self.use_cache, &self.file_name);

        let input_path = Path::new("compiled").join(format!("{}.o", self.file_name));
        if input_path.exists() {
            match std::fs::File::open(&input_path) {
                Ok(mut input_file) => {
                    match load_file(&mut input_file, &mut self.simulator.borrow_mut()) {
                        Ok(_) => {
                            info!("Loaded {}:", input_path.to_string_lossy());
                        },
                        Err(e) => {
                            error!("Error loading {}:", input_path.to_string_lossy());
                            error!("{:?}", e);
                        },
                    }
                }
                Err(e) => {
                    error!("Error opening {}:", input_path.to_string_lossy());
                    error!("{:?}", e);
                },
            }
        } else {
            error!("File {} does not exists.", input_path.to_string_lossy());
        }
    }

    fn assemble_file(&mut self) {
        let input_path = Path::new("asm").join(format!("{}.asm", self.file_name));
        let output_path = Path::new("compiled").join(format!("{}.o", self.file_name));
        if input_path.exists() {
            match std::fs::File::open(&input_path) {
                Ok(mut input_file) => {
                    let mut buf = String::new();
                    match input_file.read_to_string(&mut buf) {
                        Ok(_) => match assemble(&buf) {
                            Ok(data) => {
                                match std::fs::File::create(&output_path) {
                                    Ok(mut output_file) => {
                                        match data.to_file(&mut output_file) {
                                            Ok(_) => {
                                                info!("Wrote {}:", output_path.to_string_lossy());
                                            },
                                            Err(e) => {
                                                error!("Error writing {}:", output_path.to_string_lossy());
                                                error!("{:?}", e);
                                            },
                                        }
                                    },
                                    Err(e) => {
                                        error!("Error creating {}:", output_path.to_string_lossy());
                                        error!("{:?}", e);
                                    },
                                }
                            }
                            Err(e) => {
                                error!("Failed to assemble {}:", input_path.to_string_lossy());
                                error!("{:?}", e);
                            }
                        },
                        Err(e) => {
                            error!("Error reading {}:", input_path.to_string_lossy());
                            error!("{:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error opening {}:", input_path.to_string_lossy());
                    error!("{:?}", e);
                },
            }
        } else {
            error!("File {} does not exists.", input_path.to_string_lossy());
        }
    }
}

// TODO: Simple program, more sophisticated (no pipe/cache, pipe only, cache only, both)
// TODO: See memory access information

impl eframe::App for SimulatorGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.file_name);
            ui.horizontal(|ui| {
                if ui.button("Assemble").clicked() {
                    self.assemble_file();
                }
                if ui.button("Load").clicked() {
                    self.load_file();
                }
                ui.checkbox(&mut self.use_pipeline, "Use Pipeline");
                ui.checkbox(&mut self.use_cache, "Use Cache");
            });

            ui.horizontal(|ui| {
                if ui.button("Single Step").clicked() {
                    self.cycle();
                }

                if self.remaining_steps > 0 {
                    if ui.button("Cancel").clicked() {
                        self.remaining_steps = 0;
                    }
                } else {
                    if ui.button("1B Steps").clicked() {
                        self.remaining_steps = 1_000_000_000;
                    }
                }

                ui.label(format!("Cycle: {}", self.simulator.borrow().get_cycle_number()));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Program Memory");
                            self.program_memory_display.ui(ui);
                        });
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            ui.heading("Data Memory");
                            self.data_memory_display.ui(ui);
                        });
                    });
                    ui.vertical(|ui| {
                        ui.heading("Program Cache");
                        self.program_cache_display.ui(ui);
                        ui.add_space(10.0);
                        ui.heading("Data Cache");
                        self.data_cache_display.ui(ui);
                    });

                    ui.add_space(10.0);
                    ui.heading("Pipeline");
                    self.pipeline_display.ui(ui);
                });
                
                ui.add_space(10.0);

                ui.vertical(|ui| {
                    self.register_display.ui(ui);
                    ui.add_space(10.0);
                    self.f_register_display.ui(ui);
                    ui.add_space(10.0);
                    self.timer_display.ui(ui);
                    ui.add_space(10.0);
                    self.condition_display.ui(ui);
                });
            });
        });

        if self.remaining_steps > 0 {
            self.cycle_remaining();
        }
    }
}
