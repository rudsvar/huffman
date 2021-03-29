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

fn encoding_benchmark(c: &mut Criterion) {
    let random_string = random_string(INPUT_SIZE);
    c.bench_function("encode", |b| b.iter(|| huffman::encode(&random_string)));
}

fn decoding_benchmark(c: &mut Criterion) {
    let random_string = random_string(INPUT_SIZE);
    let encoded = huffman::encode(&random_string).unwrap();
    c.bench_function("decode", |b| b.iter(|| huffman::decode(&encoded)));
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
    encoding_benchmark,
    decoding_benchmark,
    encoding_decoding_benchmark
}

criterion_main!(benches);
