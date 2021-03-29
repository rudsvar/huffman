use criterion::{criterion_group, criterion_main, Criterion};

fn random_string(length: usize) -> String {
    let mut buf = String::with_capacity(length);
    for _ in 0..length {
        let c: char = rand::random();
        buf.push(c);
    }
    buf
}

const INPUT_SIZE: usize = 100_000;

fn random_string_benchmark(c: &mut Criterion) {
    c.bench_function("random_string", |b| b.iter(|| random_string(INPUT_SIZE)));
}

fn encoding_benchmark(c: &mut Criterion) {
    c.bench_function("encode", |b| {
        b.iter(|| {
            let random_string = random_string(INPUT_SIZE);
            huffman::encode(&random_string)
        })
    });
}

fn encoding_decoding_benchmark(c: &mut Criterion) {
    let random_string = random_string(INPUT_SIZE);
    c.bench_function("encode-decode", |b| {
        b.iter(|| {
            let encoded = huffman::encode(&random_string)?;
            huffman::decode(&encoded)
        })
    });
}

criterion_group! {
    benches,
    random_string_benchmark,
    encoding_benchmark,
    encoding_decoding_benchmark
}

criterion_main!(benches);
