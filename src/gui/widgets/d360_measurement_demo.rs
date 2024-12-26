use crate::start;
use egui::{TextEdit, Ui};
use std::sync::mpsc::{Receiver, Sender};

pub(crate) struct D360MeasurementDemo {
    pixel_360_distance: i32,
    tracking_status: bool,
    tracking_status_receiver: Receiver<bool>,
    total_movement_receiver: Receiver<i32>,
    do_360_pixel_amount_sender: Sender<u32>,
    do_360_pixels: String,
}

impl Default for D360MeasurementDemo {
    fn default() -> Self {
        let (tracking_status_receiver, total_movement_receiver, do_360_pixel_amount_sender) =
            start();
        Self {
            pixel_360_distance: 0,
            tracking_status: false,
            tracking_status_receiver,
            total_movement_receiver,
            do_360_pixel_amount_sender,
            do_360_pixels: String::default(),
        }
    }
}

impl D360MeasurementDemo {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        ui.heading("360 Turn Distance Measurement");

        if let Ok(pixel_360_distance) = self.total_movement_receiver.try_recv() {
            self.pixel_360_distance = pixel_360_distance;
        }

        if let Ok(tracking_status) = self.tracking_status_receiver.try_recv() {
            self.tracking_status = tracking_status;
        }

        ui.toggle_value(
            &mut self.tracking_status,
            format!(
                "Horizontal 360° distance: {} pixels",
                self.pixel_360_distance
            ),
        );

        ui.vertical(|ui| {
            ui.label("Amount of pixels to move during do 360");
            if ui
                .add(TextEdit::singleline(&mut self.do_360_pixels))
                .changed()
            {
                if let Ok(amount) = self.do_360_pixels.parse::<u32>() {
                    self.do_360_pixel_amount_sender.send(amount).unwrap();
                }
            };
        });

        ui.separator();
    }
}
