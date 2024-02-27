mod headlines;
use crate::headlines::{Headlines, PADDING};

use eframe::{
    egui::{self, CentralPanel, Context, Hyperlink, ScrollArea, Separator, TopBottomPanel, Ui},
    run_native, App, NativeOptions,
};

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_right(true)
                .show(ui, |ui| {
                    self.render_news_cards(ui);
                });
            render_footer(ctx);
        });
    }
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Hyperlink::from_label_and_url(
                "Source code",
                "https://www.github.com/Yadav106/headlines",
            ));
        });
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([540.0, 760.0])
            .with_transparent(true),
        ..Default::default()
    };
    let _ = run_native(
        "Headlines",
        options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}
