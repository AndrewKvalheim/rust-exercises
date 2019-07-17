#![allow(clippy::new_without_default)]

mod hat;
mod name;

use hat::Hat;
use lazy_static::lazy_static;
use name::Name;
use std::sync::Mutex;

//
// Global pool of available names
//

lazy_static! {
    static ref NAMES: Mutex<Hat<Name>> = Mutex::new(Name::all().collect());
}

fn allocate_name() -> Name {
    let name = NAMES.lock().unwrap().draw();

    name.expect("No names are available.")
}

fn free_name(name: Name) {
    NAMES.lock().unwrap().insert(name);
}

//
// Robot
//

pub struct Robot {
    name: Name,
    name_display: String,
}

impl Robot {
    pub fn new() -> Self {
        let name = allocate_name();

        Self {
            name,
            name_display: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name_display
    }

    pub fn reset_name(&mut self) {
        let previous = self.name;

        self.name = allocate_name();
        self.name_display = self.name.to_string();

        free_name(previous);
    }
}

impl Drop for Robot {
    fn drop(&mut self) {
        free_name(self.name);
    }
}
