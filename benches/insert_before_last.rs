use criterion::{black_box, criterion_group, criterion_main, Criterion};
use substring_replace::*;

/// A hand-written, std-only equivalent of `insert_before_last`, representing
/// what you'd write without this crate: find the last occurrence of `pat`
/// by byte search and splice `insert` in before it.
fn manual_insert_before_last(name: &str, insert: &str, pat: &str) -> String {
    match name.rfind(pat) {
        Some(byte_index) => {
            let mut result = String::with_capacity(name.len() + insert.len());
            result.push_str(&name[..byte_index]);
            result.push_str(insert);
            result.push_str(&name[byte_index..]);
            result
        }
        None => name.to_string(),
    }
}

/// Compares `insert_before_last` against the hand-rolled equivalent above,
/// deriving a resized image file name by inserting a suffix before the
/// last "." in the original name.
fn bench_insert_before_last(c: &mut Criterion) {
    let name = "my-cat-in-garden.jpg";

    // sanity check: both approaches must agree before we trust the timings
    let a = name.insert_before_last("--resized-640x640", ".");
    let b = manual_insert_before_last(name, "--resized-640x640", ".");
    assert_eq!(a, b);

    let mut group = c.benchmark_group("insert_before_last vs manual");

    group.bench_function("substring-replace", |b| {
        b.iter(|| black_box(black_box(name).insert_before_last("--resized-640x640", ".")))
    });

    group.bench_function("manual (std-only)", |b| {
        b.iter(|| manual_insert_before_last(black_box(name), "--resized-640x640", "."))
    });

    group.finish();
}

criterion_group!(benches, bench_insert_before_last);
criterion_main!(benches);
