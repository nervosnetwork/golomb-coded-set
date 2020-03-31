use golomb_coded_set::GCSFilterWriter;
use rand::prelude::*;
use std::io;
use std::time::Instant;

#[allow(non_snake_case)]
fn main() {
    // M = 1.497137 2**P
    // bits/item = log2(eM) - log2(2πN)/(2N)
    println!("========");
    println!(">> Reference: https://gist.github.com/sipa/576d5f09c3b86c3b1b75598d799fc845");
    println!("--------");
    println!(" P : the bit parameter of the Golomb-Rice coding");
    println!(" M : 1.497137 * (2**P)");
    println!(" N : how many items in filter");
    println!(" bits/item : average bits cost for one item => log2(eM) - log2(2πN)/(2N)");
    println!(" bits/item(real) : real(random items [u8;32]) average bits cost for one item");
    println!(" false-positive-rate: 1.0/M");
    println!("========");

    let mut rng = rand::thread_rng();
    let mut items: Vec<[u8; 32]> = Vec::new();
    for _ in 0..100_000 {
        items.push(rng.gen());
    }

    let E = std::f64::consts::E;
    let PI = std::f64::consts::PI;
    for P in &[10u32, 15, 19, 20, 25] {
        let M = 1.497_137 * f64::from(2u32.pow(*P));
        let int_m = M.round() as u64;
        let false_positive_rate = 1.0 / M;
        println!("\n # P={}, M={:>8}, false-positive-rate={:.10}", P, int_m, false_positive_rate);
        for N in &[10.0_f64, 100.0, 1000.0, 10000.0, 100_000.0] {
            let now = Instant::now();
            let mut out = io::Cursor::new(Vec::new());
            let mut writer = GCSFilterWriter::new(&mut out, 0, 0, int_m, *P as u8);
            for item in items.iter().take(*N as usize) {
                writer.add_element(&item[..]);
            }
            writer.finish().unwrap();
            let bytes = out.into_inner();
            let build_filter_cost = now.elapsed();
            let real_bits_per_item = f64::from((bytes.len() * 8) as u32) / N;

            let bits_per_item = (E * M).log2() - (2f64 * PI * N).log2() / (2f64 * N);
            println!(
                "N={:<6}, bits/item={:.4}, bits/item(real)={:.4}, build-filter-cost={:?}",
                N, bits_per_item, real_bits_per_item,
                build_filter_cost,
            );
        }
    }
}
