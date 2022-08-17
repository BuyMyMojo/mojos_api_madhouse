use criterion::{black_box, criterion_group, criterion_main, Criterion};
use core::time;
use mojos_api_madhouse::anime::*;
use mojos_api_madhouse::structs::*;

fn anime_list_animechan_benchmark(c: &mut Criterion) {
    let mut anime_list_bench = c.benchmark_group("AnimeChan");
    
    // This is over half of your allowed requests per hour, be careful!
    anime_list_bench.sample_size(50).warm_up_time(time::Duration::from_secs(5));
    anime_list_bench.bench_function("Request list off all anime", |b| b.iter(|| animechan(black_box(AnimechanRout::ListAllAvailableAnime), None, None)));
    anime_list_bench.bench_function("Get 10 random quotes", |b| b.iter(|| animechan(black_box(AnimechanRout::Quotes), None, None)));
}

criterion_group!(benches, anime_list_animechan_benchmark);
criterion_main!(benches);