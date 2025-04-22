use std::{cell::RefCell, rc::Rc};

use eframe::egui::{self, Context, Id, Margin, UiBuilder, Vec2};
use simulator::memory::Cache;

use crate::PaneInner;

const TOP_ROW_HEIGHT: f32 = 24.0;
const ROW_HEIGHT: f32 = 18.0;

pub struct CacheDisplay<const C: usize> {
    salt: String,
    cache_cell: Rc<RefCell<dyn Cache>>,
    default_column: egui_table::Column,
    prefetched: Vec<egui_table::PrefetchInfo>,
}

impl<const C: usize> CacheDisplay<C> {
    pub fn new(cache_cell: Rc<RefCell<dyn Cache>>, salt: &str) -> Self {
        Self {
            salt: salt.to_owned(),
            cache_cell,
            default_column: egui_table::Column::new(100.0)
                .range(10.0..=500.0)
                .resizable(false),
            prefetched: vec![],
        }
    }
}

impl<const C: usize> CacheDisplay<C> {
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
                    ui.label(format!("{}", row_nr));
                });
            } else if col_nr == 1 {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", self.cache_cell.borrow().line_metadata(row_nr as usize).valid));
                });
            } else if col_nr == 2 {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", self.cache_cell.borrow().line_metadata(row_nr as usize).dirty));
                });
            } else if col_nr == 3 {
                ui.horizontal(|ui| {
                    ui.label(format!("{:04X}", self.cache_cell.borrow().line_metadata(row_nr as usize).tag));
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(format!("{:08X}", self.cache_cell.borrow().raw_line(row_nr as usize)[col_nr-4]));
                });
            }
        });
    }
}

impl<const C: usize> egui_table::TableDelegate for CacheDisplay<C> {
    fn prepare(&mut self, info: &egui_table::PrefetchInfo) {
        assert!(
            info.visible_rows.end <= C as u64,
            "Was asked to prefetch rows {:?}, but we only have {} rows. This is a bug in egui_table.",
            info.visible_rows,
            C
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
                    ui.heading("Line");
                } else if col_range.start == 1 {
                    ui.heading("Valid");
                } else if col_range.start == 2 {
                    ui.heading("Dirty");
                } else if col_range.start == 3 {
                    ui.heading("Tag");
                } else {
                    ui.label(format!("{:#03x}", group_index - 4));
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

impl<const C: usize> PaneInner for CacheDisplay<C> {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let id_salt = Id::new(&self.salt);
        ui.push_id(id_salt, |ui| {
            let estimated_height = (C.min(8) as f32) * ROW_HEIGHT + TOP_ROW_HEIGHT;
            let estimated_width = 8.0 * self.default_column.current;

            let (_id, rect) = ui.allocate_space(
                Vec2::new(estimated_width, estimated_height),
            );

            ui.allocate_new_ui(UiBuilder::new().sizing_pass().max_rect(rect), |ui| {
                let id_salt = Id::new("table");
                let table = egui_table::Table::new()
                    .id_salt(id_salt)
                    .num_rows(C as u64)
                    .columns(vec![self.default_column; 8])
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
