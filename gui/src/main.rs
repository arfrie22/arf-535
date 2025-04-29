#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, io::Read, path::Path, rc::Rc, time::Instant};

use assembler::{assemble, load_file};
use displays::{
    cache::CacheDisplay, condition::ConditionDisplay, f_register::FRegisterDisplay,
    memory::MemoryDisplay, pipeline::PipelineDisplay, register::RegisterDisplay,
    timer::TimerDisplay,
};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, Label, RichText, Widget};
use log::{error, info};
use simulator::{
    memory::{ClockedMemory, DirectCache, FrontMemory, Memory},
    streams::{ConstantInput, InputStream, NoOperationOutput, OutputStream, WavInput},
    Simulator,
};

const DATA_M_CYCLES: usize = 4;
const PROG_M_CYCLES: usize = 10;

const DATA_C_CYCLES: usize = 1;
const PROG_C_CYCLES: usize = 1;

const PROGRAM_CACHE_LINES: usize = 64;
const DATA_CACHE_LINES: usize = 64;

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

    let adc_streams =
        core::array::from_fn(|_| Box::new(WavInput::new("wav/dtmf.wav")) as Box<dyn InputStream>);
    
    let dac_streams =
        core::array::from_fn(|_| Box::new(NoOperationOutput::new()) as Box<dyn OutputStream>);

    Simulator::new(
        raw_program_memory,
        raw_data_memory,
        raw_program_cache,
        raw_data_cache,
        used_prog,
        used_data,
        adc_streams,
        dac_streams,
        88_200_000,
    )
}

pub struct SimulatorGUI {
    tree: egui_tiles::Tree<Pane>,
    simulator: Rc<RefCell<Simulator>>,
    pipeline_display: Rc<RefCell<PipelineDisplay>>,
    register_display: Rc<RefCell<RegisterDisplay>>,
    f_register_display: Rc<RefCell<FRegisterDisplay>>,
    timer_display: Rc<RefCell<TimerDisplay>>,
    condition_display: Rc<RefCell<ConditionDisplay>>,
    program_memory_display: Rc<RefCell<MemoryDisplay>>,
    data_memory_display: Rc<RefCell<MemoryDisplay>>,
    program_cache_display: Rc<RefCell<CacheDisplay<PROGRAM_CACHE_LINES>>>,
    data_cache_display: Rc<RefCell<CacheDisplay<DATA_CACHE_LINES>>>,
    file_name: String,
    use_pipeline: bool,
    use_cache: bool,
}

impl Default for SimulatorGUI {
    fn default() -> Self {
        // Self::new(true, true, "test")

        let simulator = Rc::new(RefCell::new(create_simulator(true)));
        let simulator_state = simulator.borrow().get_state();
        simulator_state.borrow_mut().single_instruction_pipeline = false;

        let program_memory = simulator.borrow().get_program_memory();
        let data_memory = simulator.borrow().get_data_memory();
        let program_cache = simulator.borrow().get_program_cache();
        let data_cache = simulator.borrow().get_data_cache();

        let pipeline_display = Rc::new(RefCell::new(PipelineDisplay::new(
            simulator_state.clone(),
            "pipeline",
        )));
        let register_display = Rc::new(RefCell::new(RegisterDisplay::new(
            simulator_state.clone(),
            "register",
        )));
        let f_register_display = Rc::new(RefCell::new(FRegisterDisplay::new(
            simulator_state.clone(),
            "f_register",
        )));
        let timer_display = Rc::new(RefCell::new(TimerDisplay::new(
            simulator_state.clone(),
            "timer",
        )));
        let condition_display = Rc::new(RefCell::new(ConditionDisplay::new(
            simulator_state.clone(),
            "condition",
        )));
        let program_memory_display = Rc::new(RefCell::new(MemoryDisplay::new(
            program_memory,
            "program_memory",
        )));
        let data_memory_display =
            Rc::new(RefCell::new(MemoryDisplay::new(data_memory, "data_memory")));
        let program_cache_display = Rc::new(RefCell::new(CacheDisplay::new(
            program_cache,
            "program_cache",
        )));
        let data_cache_display = Rc::new(RefCell::new(CacheDisplay::new(data_cache, "data_cache")));

        let mut tiles = egui_tiles::Tiles::default();

        let memories = vec![
            tiles.insert_pane(Pane::new("Program Memory", program_memory_display.clone())),
            tiles.insert_pane(Pane::new("Data Memory", data_memory_display.clone())),
        ];
        let memory_tab = tiles.insert_tab_tile(memories);

        let caches = vec![
            tiles.insert_pane(Pane::new("Program Cache", program_cache_display.clone())),
            tiles.insert_pane(Pane::new("Data Cache", data_cache_display.clone())),
        ];

        let cache_tab = tiles.insert_tab_tile(caches);

        let pipeline_tile = tiles.insert_pane(Pane::new("Pipeline", pipeline_display.clone()));

        let storage_tile = tiles.insert_vertical_tile(vec![memory_tab, cache_tab, pipeline_tile]);

        let registers = vec![
            tiles.insert_pane(Pane::new("Register", register_display.clone())),
            tiles.insert_pane(Pane::new("FPRegister", f_register_display.clone())),
            tiles.insert_pane(Pane::new("Timer", timer_display.clone())),
        ];

        let register_tile = tiles.insert_vertical_tile(registers);

        let root = tiles.insert_horizontal_tile(vec![storage_tile, register_tile]);

        let tree = egui_tiles::Tree::new("my_tree", root, tiles);

        Self {
            simulator,
            tree,
            pipeline_display,
            register_display,
            f_register_display,
            timer_display,
            condition_display,
            program_memory_display,
            data_memory_display,
            program_cache_display,
            data_cache_display,
            file_name: "test".to_owned(),
            use_pipeline: true,
            use_cache: true,
        }
    }
}

impl SimulatorGUI {
    fn reset_simulator(&mut self) {
        let simulator = Rc::new(RefCell::new(create_simulator(self.use_cache)));
        let simulator_state = simulator.borrow().get_state();
        simulator_state.borrow_mut().single_instruction_pipeline = !self.use_pipeline;

        let program_memory = simulator.borrow().get_program_memory();
        let data_memory = simulator.borrow().get_data_memory();
        let program_cache = simulator.borrow().get_program_cache();
        let data_cache = simulator.borrow().get_data_cache();

        self.simulator = simulator;
        *self.pipeline_display.borrow_mut() =
            PipelineDisplay::new(simulator_state.clone(), "pipeline");
        *self.register_display.borrow_mut() =
            RegisterDisplay::new(simulator_state.clone(), "register");
        *self.f_register_display.borrow_mut() =
            FRegisterDisplay::new(simulator_state.clone(), "f_register");
        *self.timer_display.borrow_mut() = TimerDisplay::new(simulator_state.clone(), "timer");
        *self.condition_display.borrow_mut() =
            ConditionDisplay::new(simulator_state.clone(), "condition");
        *self.program_memory_display.borrow_mut() =
            MemoryDisplay::new(program_memory, "program_memory");
        *self.data_memory_display.borrow_mut() = MemoryDisplay::new(data_memory, "data_memory");
        *self.program_cache_display.borrow_mut() =
            CacheDisplay::new(program_cache, "program_cache");
        *self.data_cache_display.borrow_mut() = CacheDisplay::new(data_cache, "data_cache");
    }

    fn cycle(&mut self) {
        self.simulator.borrow_mut().cycle();
    }

    fn cycle_remaining(&mut self) {
        let start = Instant::now();
        let state = self.simulator.borrow().get_state();
        state.borrow_mut().running = true;
        loop {
            if (Instant::now() - start).as_millis() > 1000 || !state.borrow().running {
                break;
            }

            self.cycle();
        }
    }

    fn load_file(&mut self) {
        self.reset_simulator();

        let input_path = Path::new("compiled").join(format!("{}.o", self.file_name));
        if input_path.exists() {
            match std::fs::File::open(&input_path) {
                Ok(mut input_file) => {
                    match load_file(&mut input_file, &mut self.simulator.borrow_mut()) {
                        Ok(_) => {
                            info!("Loaded {}:", input_path.to_string_lossy());
                        }
                        Err(e) => {
                            error!("Error loading {}:", input_path.to_string_lossy());
                            error!("{:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error opening {}:", input_path.to_string_lossy());
                    error!("{:?}", e);
                }
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
                            Ok(data) => match std::fs::File::create(&output_path) {
                                Ok(mut output_file) => match data.to_file(&mut output_file) {
                                    Ok(_) => {
                                        info!("Wrote {}:", output_path.to_string_lossy());
                                    }
                                    Err(e) => {
                                        error!("Error writing {}:", output_path.to_string_lossy());
                                        error!("{:?}", e);
                                    }
                                },
                                Err(e) => {
                                    error!("Error creating {}:", output_path.to_string_lossy());
                                    error!("{:?}", e);
                                }
                            },
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
                }
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

                let state = self.simulator.borrow().get_state();
                let mut state = state.borrow_mut();

                if state.running {
                    if ui.button("Cancel").clicked() {
                        state.running = false;
                    }
                } else {
                    if ui.button("Run").clicked() {
                        state.running = true;
                    }
                }

                ui.label(format!(
                    "Cycle: {}",
                    self.simulator.borrow().get_cycle_number()
                ));
            });

            ui.add_space(10.0);

            let mut behavior = TreeBehavior {};
            self.tree.ui(&mut behavior, ui);
        });

        if self.simulator.borrow().get_state().borrow().running {
            self.cycle_remaining();
            ctx.request_repaint();
        }
    }
}

struct TreeBehavior {}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        (&pane.name).into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        Label::new(RichText::from(&pane.name).heading())
            .selectable(false)
            .ui(ui);

        pane.inner.borrow_mut().ui(ui);

        let dragged = ui
            .allocate_rect(ui.max_rect(), egui::Sense::click_and_drag())
            .on_hover_cursor(egui::CursorIcon::Grab)
            .dragged();
        if dragged {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }
}

pub trait PaneInner {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct Pane {
    name: String,
    inner: Rc<RefCell<dyn PaneInner>>,
}

impl Pane {
    pub fn new(name: &str, inner: Rc<RefCell<dyn PaneInner>>) -> Self {
        Self {
            name: name.to_owned(),
            inner,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.inner.borrow_mut().ui(ui);
    }
}
