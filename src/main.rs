#![cfg_attr(not(debug_assertions), windows_subsystem="windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use eframe::egui::{
    self,
    DragValue,
    Event,
    Slider,
    Vec2
};

use egui_plot::{
    Legend,
    Line,
    PlotPoints,
};

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    
    eframe::run_native("Elliptic Curves", options, Box::new(|_cc| Ok(Box::<Graph>::default())))
}

struct EllipticConstants {
    a: i32,
    b: i32
}

impl Default for EllipticConstants {
    fn default() -> Self {
        Self { a: 1, b: 1 }
    }
}

struct Graph {
    lock_x: bool,
    lock_y: bool,
    ctrl_to_zoom: bool,
    shift_to_horizontal: bool,
    zoom_speed: f32,
    scroll_speed: f32,
    ellipse: EllipticConstants
}

impl Default for Graph {
    fn default() -> Self {
        Self { lock_x: false, lock_y: false, ctrl_to_zoom: false, shift_to_horizontal: false, zoom_speed: 1.0, scroll_speed: 1.0, ellipse: EllipticConstants::default() }
    }
}

impl eframe::App for Graph {
    fn update(
        &mut self, 
        ctx: &egui::Context, 
        _frame: &mut eframe::Frame
    ) {
        egui::SidePanel::left("options").show(ctx, |ui| {
            ui.checkbox(&mut self.lock_x, "Lock x axis").on_hover_text("Check to keep the X axis fixed");

            ui.checkbox(&mut self.lock_y, "Lock y axis").on_hover_text("Check to keep the Y axis fixed");

            ui.checkbox(&mut self.ctrl_to_zoom, "Ctrl to zoom").on_hover_text("If unchecked, the behavior of the Ctrl key is inverted");

            ui.checkbox(&mut self.shift_to_horizontal, "Shift to horizontal scroll").on_hover_text("If unchecked, the behaviour of the shift key is inverted to the default controls");

            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.zoom_speed)
                        .range(0.1..=2.0)
                        .speed(0.1),
                );
                ui.label("Zoom speed").on_hover_text("How fast to zoom in and out with the mouse wheel");
            });

            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.scroll_speed)
                        .range(0.1..=100.0)
                        .speed(0.1)
                );
                ui.label("Scroll speed").on_hover_text("How fast to pand with the mouse wheel");
            });

            ui.horizontal(|ui| {
                ui.label("a: ");
                ui.add(Slider::new(&mut self.ellipse.a, -100..=100))
            });

            ui.horizontal(|ui| {
                ui.label("b: ");
                ui.add(Slider::new(&mut self.ellipse.b, -100..=100))
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (scroll, pointer_down, modifiers) = ui.input(|i| {
                let scroll = i.events.iter().find_map(|e| match e {
                    Event::MouseWheel { 
                        unit: _, 
                        delta, 
                        modifiers: _, 
                    } => Some(*delta),
                    _ => None,
                });

                (scroll, i.pointer.primary_down(), i.modifiers)
            });

            ui.label("Input events for plot controls");

            egui_plot::Plot::new("plot")
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    if let Some(mut scroll) = scroll {
                        if modifiers.ctrl == self.ctrl_to_zoom {
                            scroll = Vec2::splat(scroll.x + scroll.y);
                            let mut zoom_factor = Vec2::from([
                                (scroll.x * self.zoom_speed / 10.0).exp(),
                                (scroll.y * self.zoom_speed / 10.0).exp(),
                            ]);
                            if self.lock_x {
                                zoom_factor.x = 1.0;
                            } 
                            if self.lock_y {
                                zoom_factor.y = 1.0;
                            }
                            plot_ui.zoom_bounds_around_hovered(zoom_factor);
                        } else {
                            if modifiers.shift == self.shift_to_horizontal {
                            scroll = Vec2::new(scroll.y, scroll.x);
                            }
                            if self.lock_x {
                                scroll.x = 0.0;
                            }
                            if self.lock_y {
                                scroll.y = 0.0;
                            }
                            let delta_pos = self.scroll_speed * scroll;
                            plot_ui.translate_bounds(delta_pos);
                        }
                    }
                    if plot_ui.response().hovered() && pointer_down {
                        let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
                        if self.lock_x {
                            pointer_translate.x = 0.0;
                        }
                        if self.lock_y {
                            pointer_translate.y = 0.0;
                        }
                        plot_ui.translate_bounds(pointer_translate);
                    }

                    let elliptic_points_positive = PlotPoints::from_explicit_callback(|x| f64::sqrt(x*x*x + self.ellipse.a as f64 * x + self.ellipse.b as f64), .., 5000);
                    let elliptic_points_negative = PlotPoints::from_explicit_callback(|x| -1.*f64::sqrt(x*x*x + self.ellipse.a as f64 * x + self.ellipse.b as f64), .., 5000);

                    plot_ui.line(Line::new("Positive elliptic curve", elliptic_points_positive));
                    plot_ui.line(Line::new("Negative elliptic curve", elliptic_points_negative));
                });
        });
    }
}
