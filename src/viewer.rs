use eframe::egui::{self, ViewportBuilder};

pub fn show_svg(svg: String) {
    eframe::run_native(
        "Plotiron Viewer",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size(egui::Vec2::new(1200.0, 900.0)),
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Viewer::new(svg)))
        }),
    )
    .unwrap();
}

struct Viewer {
    // ratio: f32,
    hint: String,
    svg: String,
}

impl Viewer {
    pub fn new(svg: String) -> Self {
        let hint = svg_hint(&svg);
        Self {
            // ratio: 1.0,
            hint,
            svg,
        }
    }
}

fn svg_hint(svg: &str) -> String {
    use md5::Digest;

    let mut hasher = md5::Md5::new();
    hasher.update(svg.as_bytes());
    format!("{:x}", hasher.finalize())
}

impl eframe::App for Viewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::WHITE))
            .show(ctx, |ui| {
                // disable zoom
                // ui.input(|i| {
                //     for event in &i.events {
                //         if let Event::MouseWheel { delta, .. } = event {
                //             if delta.y > 0.0 {
                //                 self.ratio *= 1.02;
                //             } else if delta.y < 0.0 {
                //                 self.ratio /= 1.02;
                //             }
                //         }
                //     }
                // });

                ui.centered_and_justified(|ui| {
                    let bytes = self.svg.clone().into_bytes();
                    let img_src = eframe::egui::ImageSource::from((
                        format!("bytes://plotiron_view_{}.svg", self.hint),
                        bytes,
                    ));

                    let image = egui::Image::new(img_src);
                    ui.add(image);
                    // image.fit_to_original_size(self.ratio).ui(ui);
                });
            });

        // ctx.request_repaint_after_secs(0.1);
    }
}
