use eframe::{
    egui::{self, CentralPanel, Color32, FontDefinitions, Label, ScrollArea},
    run_native, App, NativeOptions,
};
use egui::{FontFamily, FontId, RichText, TextStyle};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

struct Headlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl Headlines {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

    fn render_news_cards(&self, ui: &mut egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);

            // render title
            let title = format!("▶️  {}", a.title);
            ui.colored_label(WHITE, title);

            // render desc
            ui.add_space(PADDING);
            let desc = RichText::new(&a.desc).text_style(eframe::egui::TextStyle::Button);

            ui.label(&a.desc);
            ui.label(&a.url);
        }
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

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_right(true)
                .show(ui, |ui| {
                    self.render_news_cards(ui);
                });
        });
    }
}

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([540.0, 960.0])
            .with_transparent(true),
        ..Default::default()
    };
    let _ = run_native(
        "Headlines",
        options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}
