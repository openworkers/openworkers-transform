//! What lowering a bundle costs. The parser is the larger half of a cold start
//! on every backend without a code cache, so a version bump that slows it down
//! is worth seeing here rather than in a request.

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;

use openworkers_transform::CodeLanguage;
use openworkers_transform::parse_worker_code;

/// A bundle shaped like what an adapter emits: many small modules behind one
/// default export.
fn bundle(modules: usize, language: CodeLanguage) -> Vec<u8> {
    let typed = matches!(language, CodeLanguage::TypeScript);
    let mut source = String::with_capacity(modules * 220);

    for index in 0..modules {
        if typed {
            source.push_str(&format!(
                "interface Shape{index} {{ id: number; name: string }}\n\
                 const build{index} = (input: Shape{index}): string => \
                 `${{input.id}}:${{input.name}}`;\n\
                 function pick{index}(values: Shape{index}[]): Shape{index} | undefined {{\n\
                 return values.find((value) => value.id === {index});\n}}\n"
            ));
        } else {
            source.push_str(&format!(
                "const build{index} = (input) => `${{input.id}}:${{input.name}}`;\n\
                 function pick{index}(values) {{\n\
                 return values.find((value) => value.id === {index});\n}}\n"
            ));
        }
    }

    source.push_str("export default { fetch(request) { return new Response(build0({ id: 1, name: 'x' })); } };\n");

    source.into_bytes()
}

fn lower(c: &mut Criterion) {
    let mut group = c.benchmark_group("lower");

    for modules in [64, 1024, 4096] {
        for language in [CodeLanguage::JavaScript, CodeLanguage::TypeScript] {
            let source = bundle(modules, language);
            let name = match language {
                CodeLanguage::JavaScript => "javascript",
                CodeLanguage::TypeScript => "typescript",
            };

            group.throughput(Throughput::Bytes(source.len() as u64));
            group.bench_with_input(
                BenchmarkId::new(name, source.len()),
                &source,
                |b, source| {
                    b.iter(|| parse_worker_code(black_box(source), language).expect("should lower"))
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, lower);
criterion_main!(benches);
