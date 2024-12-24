use crate::start;
use egui::Ui;
use std::sync::mpsc::Receiver;

pub(crate) struct Pixel360Measurement {
    pixel_360_distance: i32,
    tracking_status: bool,
    tracking_status_receiver: Receiver<bool>,
    total_movement_receiver: Receiver<i32>,
}

impl Default for Pixel360Measurement {
    fn default() -> Self {
        let (tracking_status_receiver, total_movement_receiver) = start();
        Self {
            pixel_360_distance: 0,
            tracking_status: false,
            tracking_status_receiver,
            total_movement_receiver,
        }
    }
}

impl Pixel360Measurement {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
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
    }
}
