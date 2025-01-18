use crate::lexer::Lexer;
use crate::types::TF;
use crate::types::TFI;

use std::collections::HashMap;
use std::path::PathBuf;

fn tf(term: &str, doc: &TF) -> f32 {
    let f = *doc.get(term).unwrap_or(&0) as f32;
    let s = doc.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    f / s
}

fn idf(term: &str, tfi: &TFI) -> f32 {
    let n = tfi.len() as f32;
    let m = tfi
        .values()
        .filter(|tf| tf.contains_key(term))
        .count()
        .max(1) as f32;
    (n / m).log10()
}

fn rank(phrase: &str, doc: &TF, tfi: &TFI) -> f32 {
    let content = phrase.chars().collect::<Vec<_>>();
    Lexer::new(&content)
        .map(|token| tf(&token, doc) * idf(&token, tfi))
        .sum()
}

pub(crate) fn compute_ranks(phrase: String, tf_index: &TFI) -> HashMap<&PathBuf, f32> {
    let mut ranks = tf_index
        .iter()
        .filter_map(|(path, tf)| {
            let score = rank(&phrase, tf, tf_index);
            if score > 0.0 {
                Some((path, score))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    ranks.sort_by(|(_, l), (_, r)| r.total_cmp(l));
    ranks.clone().into_iter().take(10).for_each(|(p, r)| {
        println!("{path} => {r}", path = p.display());
    });

    ranks.iter().copied().collect()
}
