use eframe::egui::{
    self, Button, Color32, Context, FontDefinitions, Hyperlink, Layout, RichText, Separator,
    TopBottomPanel,
};

use egui::{FontFamily, FontId, TextStyle};
pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl Headlines {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            desc: format!("desc {}", a),
            url: format!("https://example.com/{}", a),
        });
        Self {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn render_news_cards(&self, ui: &mut egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);

            // render title
            let title = format!("▶ {}", a.title);
            ui.colored_label(WHITE, title);

            // render desc
            ui.add_space(PADDING);
            // ui.label(&a.desc);
            ui.label(RichText::new(&a.desc).text_style(TextStyle::Body));

            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more⤴  ", &a.url));
            });

            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.add_space(15.);
                ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {
                    ui.label(RichText::new("📰").text_style(TextStyle::Body));
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                    let close_btn =
                        ui.add(Button::new(RichText::new("❌").text_style(TextStyle::Body)));

                    let refresh_btn =
                        ui.add(Button::new(RichText::new("🔄").text_style(TextStyle::Body)));

                    let theme_btn =
                        ui.add(Button::new(RichText::new("🌙").text_style(TextStyle::Body)));
                });
                ui.add_space(15.);
            });
        });
    }
}

fn configure_fonts(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "MesloLGS".to_owned(),
        egui::FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
    );

    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "MesloLGS".to_owned());

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(35.0, Proportional)),
        (TextStyle::Body, FontId::new(20.0, Proportional)),
    ]
    .into();

    ctx.set_fonts(fonts);
    ctx.set_style(style);
}
