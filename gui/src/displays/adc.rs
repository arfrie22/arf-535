use eframe::egui::{self, Checkbox, Context, Id, Margin, TextEdit, UiBuilder, Vec2, Widget};
use simulator::streams::{ConstantInput, InputStream, WavInput};

use crate::PaneInner;

const TOP_ROW_HEIGHT: f32 = 24.0;
const ROW_HEIGHT: f32 = 18.0;

#[derive(Debug, Default)]
pub struct ADCNameState {
    use_file: bool,
    name: String,
}

pub struct ADCName {
    salt: String,
    prefetched: Vec<egui_table::PrefetchInfo>,
    states: [ADCNameState; 4],
}

impl ADCName {
    pub fn new(salt: &str) -> Self {
        Self {
            salt: salt.to_owned(),
            prefetched: vec![],
            states: Default::default(),
        }
    }

    pub fn create_input_stream(&self, i: usize) -> Box<dyn InputStream> {
        if i >= 4 || !self.states[i].use_file{
            Box::new(ConstantInput::new(0))
        } else {
            Box::new(WavInput::new(&format!("wav/{}.wav", self.states[i].name)))
        }
    }
}

impl ADCName {
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
                    ui.label(format!("A{}", row_nr + 1));
                });
            } else if col_nr == 1 {
                ui.horizontal(|ui| {
                    Checkbox::new(&mut self.states[row_nr as usize].use_file, "").ui(ui);
                });
            } else if col_nr == 2 {
                ui.horizontal(|ui| {
                    TextEdit::singleline(&mut self.states[row_nr as usize].name).background_color(ui.visuals().code_bg_color).ui(ui);
                });
            } else {
                unreachable!()
            }
        });
    }
}

impl egui_table::TableDelegate for ADCName {
    fn prepare(&mut self, info: &egui_table::PrefetchInfo) {
        assert!(
            info.visible_rows.end <= 32 as u64,
            "Was asked to prefetch rows {:?}, but we only have {} rows. This is a bug in egui_table.",
            info.visible_rows,
            32
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
                    ui.heading("ADC");
                } else if col_range.start == 1 {
                    ui.heading("Use WAV");
                } else if col_range.start == 2 {
                    ui.heading("Wavfile");
                } else {
                    unreachable!()
                }
            });
    }

    fn cell_ui(&mut self, ui: &mut egui::Ui, cell_info: &egui_table::CellInfo) {
        let egui_table::CellInfo { row_nr, col_nr, .. } = *cell_info;

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

impl PaneInner for ADCName {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let id_salt = Id::new(&self.salt);
        ui.push_id(id_salt, |ui| {
            let estimated_height = 4.0 * ROW_HEIGHT + TOP_ROW_HEIGHT;
            let estimated_width = 300.0;

            let (_id, rect) = ui.allocate_space(
                Vec2::new(estimated_width, estimated_height),
            );

            let columns = vec![
                egui_table::Column::new(100.0).resizable(false),
                egui_table::Column::new(100.0).resizable(false),
                egui_table::Column::new(100.0).resizable(false),
            ];

            ui.allocate_new_ui(UiBuilder::new().sizing_pass().max_rect(rect), |ui| {
                let id_salt = Id::new("table");
                let table = egui_table::Table::new()
                    .id_salt(id_salt)
                    .num_rows(4)
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
