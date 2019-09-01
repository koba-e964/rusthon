extern crate rand;
extern crate rusthon;
use rand::Rng;
use rusthon::solver::*;

const TRIAL: usize = 20;

// テストケース生成。問題に合わせて変える。
fn gen<R: Rng>(rng: &mut R) -> Instance {
    Instance {}
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut sum = 0.0f64;
    let mut sqsum = 0.0;
    let mut count = 0.0;
    for _ in 0..TRIAL {
        let inst = gen(&mut rng);
        let score = 1.0; // インスタンスの答えに対するスコア。問題に合わせて変える。
        sum += score;
        sqsum += score * score;
        count += 1.0;
    }
    let avrg = sum / count;
    let var = sqsum / count - avrg * avrg;
    let uncertainty_sq = var / (count - 1.0);
    println!("Average: {} +- {}", avrg, uncertainty_sq.sqrt());
    println!("stdev: {}", var.sqrt());
}
