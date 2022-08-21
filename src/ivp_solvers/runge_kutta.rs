use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct ExplicitButcherTableau {
    pub time_weights: NDvector,
    pub coeff_matrix: NDMatrix,
    pub sum_weights: NDvector
}

pub trait TableauLoader {
     fn load_from_file<P: AsRef<Path>>(filepath: &P) -> ExplicitButcherTableau {
        let mut handle = fs::File::open(filepath).unwrap();
        let mut buffer = String::new();
        handle.read_to_string(&mut buffer);
        println!("{}", buffer);
        serde_json::from_str(&buffer).unwrap()
    }
}

impl TableauLoader for ExplicitButcherTableau {}