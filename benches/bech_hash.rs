use ckb_hash::blake2b_256;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use golomb_coded_set::{GCSFilterWriter, Sip24Hasher, ToU64Hasher, M, P};
use std::convert::TryInto;

const SCRIPT_COUNT: usize = 6;
const SCRIPT_SIZE: usize = 69;

struct Blake2bHasher;

impl ToU64Hasher for Blake2bHasher {
    fn hash_to_u64(&self, element: &[u8]) -> u64 {
        u64::from_le_bytes(blake2b_256(element)[..8].try_into().unwrap())
    }
}

fn blake2b_sip24(scripts: &[Vec<u8>]) {
    let mut writer = std::io::Cursor::new(Vec::new());
    let mut filter = GCSFilterWriter::new(&mut writer, Sip24Hasher::new(0, 0), M, P);
    for script in scripts {
        filter.add_element(&blake2b_256(&script)[..]);
    }
    filter
        .finish()
        .expect("flush to memory writer should be OK");
}

fn blake2b(scripts: &[Vec<u8>]) {
    let mut writer = std::io::Cursor::new(Vec::new());
    let mut filter = GCSFilterWriter::new(&mut writer, Blake2bHasher, M, P);
    for script in scripts {
        filter.add_element(script);
    }
    filter
        .finish()
        .expect("flush to memory writer should be OK");
}

fn sip24(scripts: &[Vec<u8>]) {
    let mut writer = std::io::Cursor::new(Vec::new());
    let mut filter = GCSFilterWriter::new(&mut writer, Sip24Hasher::new(0, 0), M, P);
    for script in scripts {
        filter.add_element(script);
    }
    filter
        .finish()
        .expect("flush to memory writer should be OK");
}

fn gcs_bench(c: &mut Criterion) {
    let scripts = (0..SCRIPT_COUNT)
        .map(|count| count as u8)
        .map(|start| (start..start + SCRIPT_SIZE as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut group = c.benchmark_group("gcs");
    for count in 3..=SCRIPT_COUNT {
        group.bench_with_input(
            BenchmarkId::new("blake2b_sip24", count),
            &count,
            |b, count| b.iter(|| blake2b_sip24(&scripts[0..*count])),
        );
        group.bench_with_input(BenchmarkId::new("blake2b", count), &count, |b, count| {
            b.iter(|| blake2b(&scripts[0..*count]))
        });
        group.bench_with_input(BenchmarkId::new("sip24", count), &count, |b, count| {
            b.iter(|| sip24(&scripts[0..*count]))
        });
    }
    group.finish();
}

criterion_group!(benches, gcs_bench);
criterion_main!(benches);
