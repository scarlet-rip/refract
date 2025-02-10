use enigo::{Coordinate, Enigo, InputError, Mouse, Settings};
use std::thread;
use std::time::Duration;

pub(crate) struct Sweeper {
    enigo_client: Enigo,
}

impl Default for Sweeper {
    fn default() -> Self {
        Self {
            enigo_client: Enigo::new(&Settings::default()).expect("Failed to create enigo client"),
        }
    }
}

impl Sweeper {
    pub fn perform_horizontal_sweep(
        &mut self,
        pixels: i32,
        chunk_size: u16,
        delay_ms: u16,
    ) -> Result<(), InputError> {
        let chunk_size = chunk_size as i32;

        for _ in 0..(pixels / chunk_size) {
            self.enigo_client
                .move_mouse(chunk_size, 0, Coordinate::Rel)?;

            thread::sleep(Duration::from_millis(delay_ms.into()));
        }

        let remaining_pixels = pixels % chunk_size;

        if remaining_pixels > 0 {
            self.enigo_client
                .move_mouse(remaining_pixels, 0, Coordinate::Rel)?;
        }

        Ok(())
    }
}
