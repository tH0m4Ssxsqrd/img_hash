#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion};

use image_hasher::{HashAlg, HasherConfig};

use image::{ImageBuffer, Rgba};

use rand::{rngs::SmallRng, SeedableRng};

type RgbaBuf = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn gen_test_img(width: u32, height: u32) -> RgbaBuf {
    let len = (width * height * 4) as usize;
    let mut rng = SmallRng::seed_from_u64(0x00c0_ffee);

    let buf: Vec<u8> = (0..len).map(|_| rand::Rng::gen(&mut rng)).collect();

    ImageBuffer::from_raw(width, height, buf).unwrap()
}

fn bench_functions(c: &mut Criterion) {
    const BENCH_HASH_SIZE: u32 = 8;
    const TEST_IMAGE_SIZE: u32 = 64;

    let mut group = c.benchmark_group("hash");

    let img = gen_test_img(TEST_IMAGE_SIZE, TEST_IMAGE_SIZE);

    for alg in [
        HashAlg::Mean,
        HashAlg::Gradient,
        HashAlg::DoubleGradient,
        HashAlg::VertGradient,
        HashAlg::Blockhash,
    ] {
        group.bench_with_input(
            BenchmarkId::new("hash", format!("{alg:?}")),
            &img,
            |b, img| {
                let hasher = HasherConfig::new()
                    .hash_size(BENCH_HASH_SIZE, BENCH_HASH_SIZE)
                    .hash_alg(alg)
                    .to_hasher();

                b.iter(|| {
                    hasher.hash_image(img);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_functions);
criterion_main!(benches);
