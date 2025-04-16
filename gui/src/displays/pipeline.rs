use std::{cell::RefCell, rc::Rc};

use eframe::egui::{self, Context, Id, Margin, UiBuilder, Vec2};
use simulator::SimulatorState;

const TOP_ROW_HEIGHT: f32 = 24.0;
const ROW_HEIGHT: f32 = 18.0;

pub struct PipelineDisplay {
    salt: String,
    state: Rc<RefCell<SimulatorState>>,
    prefetched: Vec<egui_table::PrefetchInfo>,
}

impl PipelineDisplay {
    pub fn new(state: Rc<RefCell<SimulatorState>>, salt: &str) -> Self {
        Self {
            salt: salt.to_owned(),
            state,
            prefetched: vec![],
        }
    }
}

impl PipelineDisplay {
    fn was_row_prefetched(&self, row_nr: u64) -> bool {
        self.prefetched
            .iter()
            .any(|info| info.visible_rows.contains(&row_nr))
    }

    fn cell_content_ui(&mut self, row_nr: u64, col_nr: usize, ui: &mut egui::Ui) {
        assert!(
            self.was_row_prefetched(row_nr),
            "Was asked to show row {row_nr} which was not prefetched! This is a bug in egui_table."
        );

        ui.vertical(|ui| {
            if col_nr == 0 {
                ui.horizontal(|ui| {
                    ui.label(match row_nr {
                        0 => "Fetch",
                        1 => "Decode",
                        2 => "Execute",
                        3 => "Memory",
                        4 => "Write Back",
                        _ => unreachable!()
                    });
                });
            } else if col_nr == 1 {
                ui.horizontal(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.label(match row_nr {
                            0 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.fetch_state.as_ref() {
                                    format!("{:#04X}", v)
                                } else {
                                    if let Some(v) = state.fetch_result.as_ref() {
                                        format!("{:#04X}", v.pc)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            1 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.decode_state.as_ref() {
                                    format!("{:#04X}: {:?}", v.pc, v.instruction)
                                } else {
                                    if let Some(v) = state.decode_result.as_ref() {
                                        format!("{:#04X}: {:?}", v.pc, v.instruction)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            2 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.execute_state.as_ref() {
                                    format!("{:#04X}: {:?}", v.pc, v.instruction)
                                } else {
                                    if let Some(v) = state.execute_result.as_ref() {
                                        format!("{:#04X}: {:?}", v.writeback.pc, v.writeback.instruction)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            3 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.memory_state.as_ref() {
                                    format!("{:#04X}: {:?}", v.writeback.pc, v.writeback.instruction)
                                } else {
                                    if let Some(v) = state.memory_result.as_ref() {
                                        format!("{:#04X}: {:?}", v.pc, v.instruction)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            4 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.writeback_state.as_ref() {
                                    format!("{:#04X}: {:?}", v.pc, v.instruction)
                                } else {
                                    "N/A".to_owned()
                                }
                            },
                            _ => unreachable!()
                        });
                    });
                });
            } else if col_nr == 2 {
                ui.horizontal(|ui| {
                    ui.label(match row_nr {
                        0 => {
                            let state = self.state.borrow();
                            if state.fetch_state.is_some() {
                                "false"
                            } else if state.fetch_result.is_some() {
                                "true"
                            } else {
                                "N/A"
                            }
                        },
                        1 => {
                            let state = self.state.borrow();
                            if state.decode_state.is_some() {
                                "false"
                            } else if state.decode_result.is_some() {
                                "true"
                            } else {
                                "N/A"
                            }
                        },
                        2 => {
                            let state = self.state.borrow();
                            if state.execute_state.is_some() {
                                "false"
                            } else if state.execute_result.is_some() {
                                "true"
                            } else {
                                "N/A"
                            }
                        },
                        3 => {
                            let state = self.state.borrow();
                            if state.memory_state.is_some() {
                                "false"
                            } else if state.memory_result.is_some() {
                                "true"
                            } else {
                                "N/A"
                            }
                        },
                        4 => {
                            let state = self.state.borrow();
                            if state.writeback_state.is_some() {
                                "false"
                            } else {
                                "N/A"
                            }
                        },
                        _ => unreachable!()
                    });
                });
            } else if col_nr == 3 {
                ui.horizontal(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        ui.label(match row_nr {
                            0 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.fetch_state.as_ref() {
                                    format!("Fetching {:#04X}", v)
                                } else {
                                    if let Some(v) = state.fetch_result.as_ref() {
                                        format!("Value: {:#08X}", v.value)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            1 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.decode_state.as_ref() {
                                    format!("Read: {:?}, Write: {:?}", v.read_registers, v.write_registers)
                                } else {
                                    if let Some(v) = state.decode_result.as_ref() {
                                        format!("Timer: {}, Registers: {:?}", v.timer, v.registers)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            2 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.execute_state.as_ref() {
                                    format!("Timer: {}, Registers: {:?}", v.timer, v.registers)
                                } else {
                                    if let Some(v) = state.execute_result.as_ref() {
                                        format!("{:?}", v.memory)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            3 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.memory_state.as_ref() {
                                    format!("{:?}", v.memory)
                                } else {
                                    if let Some(v) = state.memory_result.as_ref() {
                                        format!("Holds: {:?}, Registers: {:?}", v.holds, v.registers)
                                    } else {
                                        "N/A".to_owned()
                                    }
                                }
                            },
                            4 => {
                                let state = self.state.borrow();
                                if let Some(v) = state.writeback_state.as_ref() {
                                    format!("Holds: {:?}, Registers: {:?}", v.holds, v.registers)
                                } else {
                                    "N/A".to_owned()
                                }
                            },
                            _ => unreachable!()
                        });
                    });
                });
            } else {
                unreachable!()
            }
        });
    }
}

impl egui_table::TableDelegate for PipelineDisplay {
    fn prepare(&mut self, info: &egui_table::PrefetchInfo) {
        assert!(
            info.visible_rows.end <= 5 as u64,
            "Was asked to prefetch rows {:?}, but we only have {} rows. This is a bug in egui_table.",
            info.visible_rows,
            5
        );
        self.prefetched.push(info.clone());
    }

    fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell_inf: &egui_table::HeaderCellInfo) {
        let egui_table::HeaderCellInfo {
            col_range,
            ..
        } = cell_inf;

        let margin = 4;
                    
        egui::Frame::NONE
            .inner_margin(Margin::symmetric(margin, 0))
            .show(ui, |ui| {
                if col_range.start == 0 {
                    ui.heading("Stage");
                } else if col_range.start == 1 {
                    ui.heading("Address");
                } else if col_range.start == 2 {
                    ui.heading("Finished");
                } else if col_range.start == 3 {
                    ui.heading("Info");
                } else {
                    unreachable!()
                }
            });
    }

    fn cell_ui(&mut self, ui: &mut egui::Ui, cell_info: &egui_table::CellInfo) {
        let egui_table::CellInfo { row_nr, col_nr, .. } = *cell_info;

        if row_nr % 2 == 1 {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }

        egui::Frame::NONE
            .inner_margin(Margin::symmetric(4, 0))
            .show(ui, |ui| {
                self.cell_content_ui(row_nr, col_nr, ui);
            });
    }

    fn row_top_offset(&self, _ctx: &Context, _table_id: Id, row_nr: u64) -> f32 {
        row_nr as f32 * ROW_HEIGHT
    }
}

impl PipelineDisplay {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let id_salt = Id::new(&self.salt);
        ui.push_id(id_salt, |ui| {
            let estimated_height = 5.0 * ROW_HEIGHT + TOP_ROW_HEIGHT;
            let estimated_width = 600.0;

            let (_id, rect) = ui.allocate_space(
                Vec2::new(estimated_width, estimated_height),
            );

            let columns = vec![
                egui_table::Column::new(100.0).resizable(false),
                egui_table::Column::new(200.0).resizable(false),
                egui_table::Column::new(100.0).resizable(false),
                egui_table::Column::new(200.0).resizable(false),
            ];

            ui.allocate_new_ui(UiBuilder::new().sizing_pass().max_rect(rect), |ui| {
                let id_salt = Id::new("table");
                let table = egui_table::Table::new()
                    .id_salt(id_salt)
                    .num_rows(5)
                    .columns(columns)
                    .num_sticky_cols(1)
                    .headers([
                        egui_table::HeaderRow::new(TOP_ROW_HEIGHT),
                    ])
                    .auto_size_mode(egui_table::AutoSizeMode::Never);

                table.show(ui, self);
            });
        });
    }
}
