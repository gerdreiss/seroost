use crate::lexer::Lexer;
use crate::types::Model;
use crate::types::DF;
use crate::types::TF;

use std::collections::HashMap;
use std::path::PathBuf;

fn compute_tf(term: &str, doc: &TF) -> f32 {
    let f = *doc.get(term).unwrap_or(&0) as f32;
    let s = doc.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    f / s
}

fn compute_idf(term: &str, n: usize, df: &DF) -> f32 {
    let n = n as f32;
    let m = df.get(term).copied().unwrap_or(1) as f32;
    (n / m).log10()
}

fn compute_score(phrase: &str, doc: &TF, model: &Model) -> f32 {
    let content = phrase.chars().collect::<Vec<_>>();
    Lexer::new(&content)
        .map(|token| {
            compute_tf(&token, doc) * compute_idf(&token, model.tf_index.len(), &model.df_index)
        })
        .sum()
}

pub(crate) fn compute_scores(phrase: String, model: &Model) -> HashMap<&PathBuf, f32> {
    let mut ranks = model
        .tf_index
        .iter()
        .map(|(path, tf)| (path, compute_score(&phrase, tf, model)))
        .filter(|(_, score)| *score > 0.0)
        .collect::<Vec<_>>();

    ranks.sort_by(|(_, l), (_, r)| r.total_cmp(l));
    ranks.clone().into_iter().take(10).for_each(|(p, r)| {
        println!("{} => {}", p.display(), r);
    });

    ranks.iter().copied().collect()
}
