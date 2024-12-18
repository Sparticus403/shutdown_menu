use std::os::unix::process::CommandExt;

use eframe::egui::{self, FontData, FontDefinitions, Vec2};

fn main() {
    draw_gui();
}

fn draw_gui() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Shutdown_Menu",
        native_options,
        Box::new(|cc| Box::new(ShutdownApp::new(cc))),
    );
}

#[derive(Default)]
struct ShutdownApp {}

impl ShutdownApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ShutdownApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut font = FontDefinitions::default();
        font.font_data.insert("nerd_font".to_owned(), FontData::from_static(include_bytes!("/usr/share/fonts/nerd-fonts-git/TTF/JetBrainsMonoNerdFont-Medium.ttf")));
        font.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "nerd_font".to_owned());
        font.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("nerd_font".to_owned());

        ctx.set_fonts(font);
        let button_size = Vec2::new(32., 32.);

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(5.0);
            ui.horizontal(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                ui.spacing_mut().item_spacing.x = 5.0;
                let shutdown_button = egui::Button::new("󰐥")
                    .min_size(button_size)
                    .fill(egui::Color32::RED);
                let sleep_button = egui::Button::new("󰤄").min_size(button_size);

                let lock_button = egui::Button::new("")
                    .min_size(button_size);

                let reboot_button = egui::Button::new("").min_size(button_size);

                let cancel_button = egui::Button::new("󰜺").min_size(button_size);

                // shutdown_button.min_size(Vec2::new(16.0, 16.0));

                if ui.add(shutdown_button).clicked() {
                    std::process::Command::new("/usr/bin/shutdown").arg("-P").arg("now").exec();
                    std::process::exit(0);
                }
                if ui.add(sleep_button).clicked() {
                    std::process::Command::new("/usr/bin/systemctl").arg("suspend").exec();
                    std::process::exit(0);
                }
                if ui.add(lock_button).clicked() {
                    let _lock = std::process::Command::new("/usr/bin/loginctl").arg("lock-session").output().expect("Could not lock current running session");
                    std::process::exit(0);
                }
                if ui.add(reboot_button).clicked() {
                    std::process::Command::new("/usr/bin/reboot").exec();
                }
                if ui.add(cancel_button).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    std::process::exit(0);
                }
            })
        });
    }
}
// 󰐥
// 
// 󰜺
// 󰤄
// 
