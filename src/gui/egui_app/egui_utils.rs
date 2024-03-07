use egui::{Align2, Color32, ComboBox, InnerResponse, Ui, Vec2, Window};
use strum::IntoEnumIterator;
const OFFSET_X: f32 = 2.0;
const OFFSET_Y: f32 = 1.0;
pub fn conf_window(title: &'static str, pivot: Align2) -> Window<'static> {
    let offset = match pivot {
        Align2::CENTER_TOP => Vec2::new(0.0, OFFSET_Y),
        Align2::LEFT_TOP => Vec2::new(OFFSET_X, OFFSET_Y),
        Align2::LEFT_BOTTOM => Vec2::new(OFFSET_X, -OFFSET_Y),
        Align2::RIGHT_TOP => Vec2::new(-OFFSET_X, OFFSET_Y),
        Align2::RIGHT_BOTTOM => Vec2::new(-OFFSET_X, -OFFSET_Y),
        _ => Vec2::ZERO,
    };
    Window::new(title)
        .collapsible(true)
        .anchor(pivot, offset)
        .default_open(true)
        .resizable(true)
}
pub fn combo_box_from_string<E: Copy + std::cmp::PartialEq>(
    label: &str,
    (current_value, current_label): (&mut E, String),
    ui: &mut Ui,
    variants: Vec<(E, String)>,
    tooltip: &str,
) -> bool {
    let mut changed = false;
    ui.monospace(format!("{}:", label)).on_hover_text(tooltip);
    ComboBox::from_id_source(label)
        .selected_text(current_label)
        .show_ui(ui, |ui| {
            variants.into_iter().for_each(|(var, var_label)| {
                let selection = ui.selectable_value(current_value, var, var_label);
                if selection.changed() {
                    changed = true;
                }
            })
        });
    changed
}
pub fn combo_box<E: Copy + Into<&'static str> + IntoEnumIterator + std::cmp::PartialEq>(
    label: &str,
    current_value: &mut E,
    ui: &mut Ui,
    tooltip: &str,
) -> bool {
    let all_variants: Vec<E> = E::iter().collect();
    let mut changed = false;
    ui.monospace(format!("{}:", label)).on_hover_text(tooltip);
    let current_label: &'static str = (*current_value).into();
    ComboBox::from_id_source(label)
        .selected_text(current_label)
        .show_ui(ui, |ui| {
            variants_selection(&all_variants, ui, current_value, &mut changed)
        });
    changed
}

pub fn select_group_filtered<E: Copy + Into<&'static str> + std::cmp::PartialEq>(
    current_value: &mut E,
    ui: &mut Ui,
    filter_variants: &[E],
    tooltip: &str,
) -> bool {
    let mut changed = false;
    ui.group(|ui| {
        variants_selection(filter_variants, ui, current_value, &mut changed);
    })
    .response
    .on_hover_text(tooltip);
    changed
}

fn variants_selection<E: Copy + Into<&'static str> + std::cmp::PartialEq>(
    variants: &[E],
    ui: &mut Ui,
    current_value: &mut E,
    changed: &mut bool,
) {
    variants.iter().cloned().for_each(|var| {
        let var_label: &'static str = var.into();
        let selection = ui.selectable_value(current_value, var, var_label);
        if selection.changed() {
            *changed = true;
        }
    })
}
pub fn select_group<E: Copy + Into<&'static str> + IntoEnumIterator + std::cmp::PartialEq>(
    current_value: &mut E,
    ui: &mut Ui,
    tooltip: &str,
) -> bool {
    let all_variants: Vec<E> = E::iter().collect();
    select_group_filtered(current_value, ui, &all_variants, tooltip)
}

pub fn integer_slider(
    label: &str,
    current_value: &mut usize,
    upper_limit: usize,
    ui: &mut Ui,
    tooltip: &str,
) -> bool {
    ui.add(
        egui::Slider::new(current_value, 1..=upper_limit)
            .clamp_to_range(true)
            .step_by(1.0)
            .integer()
            .suffix(format!(" {}", label)),
    )
    .on_hover_text(tooltip)
    .changed()
}
pub fn float_slider(
    label: &str,
    current_value: &mut f64,
    upper_limit: f64,
    ui: &mut Ui,
    tooltip: &str,
) -> bool {
    ui.add(
        egui::Slider::new(current_value, (0.0)..=upper_limit)
            .clamp_to_range(true)
            .step_by(0.1)
            .suffix(format!(" {}", label)),
    )
    .on_hover_text(tooltip)
    .changed()
}
pub fn clickable_button(
    label: &str,
    selected: bool,
    enabled: bool,
    ui: &mut Ui,
    tooltip: &str,
) -> bool {
    let (bg_color, text_color) = if ui.visuals().dark_mode {
        (Color32::DARK_GRAY, Color32::RED)
    } else {
        (Color32::GOLD, Color32::BLACK)
    };
    let widget = egui::Button::new(label).fill(bg_color).selected(selected);
    ui.add_enabled_ui(enabled, |ui| {
        ui.visuals_mut().override_text_color = Some(text_color);
        ui.add(widget)
    })
    .inner
    .on_hover_text(tooltip)
    .clicked()
}
pub fn add_hyperlink(label: &str, link: &str, ui: &mut Ui, tooltip: &str) {
    ui.hyperlink_to(label, link).on_hover_ui(|ui| {
        ui.vertical(|ui| {
            ui.label(tooltip);
            ui.horizontal(|ui| {
                ui.label("Reference: ");
                ui.hyperlink(link);
            });
        });
    });
}
pub fn add_label(label: &str, ui: &mut Ui, tooltip: &str) {
    ui.monospace(label).on_hover_text(tooltip);
}
pub fn add_checkbox(label: &str, value: &mut bool, ui: &mut Ui, tooltip: &str) -> bool {
    let widget = egui::Checkbox::new(value, label);
    ui.add(widget).on_hover_text(tooltip).clicked()
}

pub fn group_horizontal<R>(
    ui: &mut Ui,
    f: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<InnerResponse<R>> {
    ui.group(|ui| ui.horizontal(|ui| f(ui)))
}

pub fn group_vertical<R>(
    ui: &mut Ui,
    f: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<InnerResponse<R>> {
    ui.group(|ui| ui.vertical(|ui| f(ui)))
}
