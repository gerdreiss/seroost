use std::collections::HashMap;
use std::path::PathBuf;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type TFI = HashMap<PathBuf, TF>;
pub(crate) type TF = HashMap<String, usize>;
