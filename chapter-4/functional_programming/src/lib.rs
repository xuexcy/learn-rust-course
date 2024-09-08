#![feature(test)]
extern crate rand;
extern crate test;

fn sum_for(x: &[f64]) -> f64 {
    //let mut result: f64 = 0.0;
    //for i in 0..x.len() {
     //   result += x[i];
    //}
    //result
    x.iter().sum::<f64>()
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[cfg(test)]
mod bench {
    use test::{bench::BenchSamples, Bencher};
    use rand::{Rng, thread_rng};
    use super::*;

    const LEN: usize = 1024 * 1024;
    fn rand_array(cnt: u32) -> Vec<f64> {
        let mut rng = thread_rng();
        (0..cnt).map(|_| rng.gen::<f64>()).collect()
    }
    #[bench]
    fn bench_fo(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_for(&samples);
        })
        // let samples = rand_array(LEN as u32);
        // b.iter(|| {
        //     sum_iter(&samples);
        // })
    }
    #[bench]
    fn bench_ite(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_iter(&samples);
        })
    }
}
