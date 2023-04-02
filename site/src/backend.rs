use std::{cell::RefCell, io::Cursor};

use rand::prelude::*;
use uiua::IoBackend;

pub struct WebBackend {
    pub stdout: RefCell<String>,
    rng: RefCell<SmallRng>,
    pub image_bytes: RefCell<Option<Vec<u8>>>,
}

impl Default for WebBackend {
    fn default() -> Self {
        Self {
            stdout: String::new().into(),
            rng: SmallRng::seed_from_u64(instant::now().to_bits()).into(),
            image_bytes: None.into(),
        }
    }
}

impl IoBackend for WebBackend {
    fn print_str(&self, s: &str) {
        self.stdout.borrow_mut().push_str(s);
    }
    fn rand(&self) -> f64 {
        self.rng.borrow_mut().gen()
    }
    fn show_image(&self, image: image::DynamicImage) -> Result<(), String> {
        let mut bytes = Cursor::new(Vec::new());
        image
            .write_to(&mut bytes, image::ImageOutputFormat::Png)
            .map_err(|e| format!("Failed to show image: {e}"))?;
        *self.image_bytes.borrow_mut() = Some(bytes.into_inner());
        Ok(())
    }
}