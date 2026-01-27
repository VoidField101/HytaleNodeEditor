use egui::{Color32, Rect, Response, Sense, Ui, Vec2, Widget};

pub struct StripedButton {
    text: String,
    stripe_color: Color32,
}

impl StripedButton {
    pub fn new(text: impl Into<String>, color: Color32) -> Self {
        Self {
            text: text.into(),
            stripe_color: color,
        }
    }
}

impl Widget for StripedButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let button_padding = ui.spacing().button_padding;
        let galley = ui.painter().layout_no_wrap(
            self.text,
            egui::FontId::proportional(14.0),
            ui.visuals().widgets.active.fg_stroke.color,
        );
        let stripe_width = 4.0;
        let gap = 4.0;
        let desired_size = Vec2::new(
            stripe_width + gap + galley.size().x + button_padding.x * 2.0,
            galley.size().y + button_padding.y * 2.0,
        );

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let bg_fill = if response.hovered() {
                visuals.bg_fill
            } else {
                Color32::TRANSPARENT
            };

            ui.painter()
                .rect_filled(rect, visuals.corner_radius, bg_fill);
            let stripe_rect =
                Rect::from_min_max(rect.min, rect.min + Vec2::new(stripe_width, rect.height()));
            ui.painter()
                .rect_filled(stripe_rect, visuals.corner_radius, self.stripe_color);
            let text_pos =
                rect.min + Vec2::new(stripe_width + gap + button_padding.x, button_padding.y);
            ui.painter()
                .galley(text_pos, galley, visuals.fg_stroke.color);
        }

        response
    }
}
