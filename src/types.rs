use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
pub(crate) type TFI = HashMap<PathBuf, TF>;
pub(crate) type TF = HashMap<String, usize>;
pub(crate) type DF = HashMap<String, usize>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Model {
    pub(crate) tf_index: TFI,
    pub(crate) df_index: DF,
}
