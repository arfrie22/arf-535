use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use eframe::egui::{self, Context, Id, Margin, TextEdit, UiBuilder, Vec2, Widget};
use simulator::memory::InnerMemory;

use crate::PaneInner;

const NUM_ROWS: u64 = 0xFFFF >> 2;

const TOP_ROW_HEIGHT: f32 = 24.0;
const ROW_HEIGHT: f32 = 18.0;

pub struct MemoryDisplay {
    salt: String,
    memory_cell: Rc<RefCell<dyn InnerMemory>>,
    default_column: egui_table::Column,
    prefetched: Vec<egui_table::PrefetchInfo>,
}

impl MemoryDisplay {
    pub fn new(memory_cell: Rc<RefCell<dyn InnerMemory>>, salt: &str) -> Self {
        Self {
            salt: salt.to_owned(),
            memory_cell,
            default_column: egui_table::Column::new(100.0)
                .range(10.0..=500.0)
                .resizable(false),
            prefetched: vec![],
        }
    }
}

impl MemoryDisplay {
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
                    ui.label(format!("{:#06X}", row_nr << 2));
                });
            } else {

                ui.horizontal(|ui| {
                    ui.label(format!("{:08X}", self.memory_cell.borrow().read_line((row_nr << 2) as u32).unwrap()[col_nr-1]));
                });
            }
        });
    }
}

impl egui_table::TableDelegate for MemoryDisplay {
    fn prepare(&mut self, info: &egui_table::PrefetchInfo) {
        assert!(
            info.visible_rows.end <= NUM_ROWS,
            "Was asked to prefetch rows {:?}, but we only have {} rows. This is a bug in egui_table.",
            info.visible_rows,
            NUM_ROWS
        );
        self.prefetched.push(info.clone());
    }

    fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell_inf: &egui_table::HeaderCellInfo) {
        let egui_table::HeaderCellInfo {
            group_index,
            col_range,
            ..
        } = cell_inf;

        let margin = 4;
                    
        egui::Frame::NONE
            .inner_margin(Margin::symmetric(margin, 0))
            .show(ui, |ui| {
                if col_range.start == 0 {
                    ui.heading("Address");
                } else {
                    ui.label(format!("{:#03x}", group_index - 1));
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

impl PaneInner for MemoryDisplay {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let id_salt = Id::new(&self.salt);
        ui.push_id(id_salt, |ui| {
            let estimated_height = 16.0 * ROW_HEIGHT + TOP_ROW_HEIGHT;
            let estimated_width = 5.0 * self.default_column.current;

            let (_id, rect) = ui.allocate_space(
                Vec2::new(estimated_width, estimated_height),
            );

            ui.allocate_new_ui(UiBuilder::new().sizing_pass().max_rect(rect), |ui| {
                let id_salt = Id::new("table");
                let table = egui_table::Table::new()
                    .id_salt(id_salt)
                    .num_rows(NUM_ROWS)
                    .columns(vec![self.default_column; 5])
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
