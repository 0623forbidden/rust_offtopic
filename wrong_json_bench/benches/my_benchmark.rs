use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wrong_json_bench::parse_body;

const INPUT: &str = 
r#"{
	"_item1": {
		"param1": "param1",
        "param2": false,
        "param3": "param3"
	},
	"_item2": {"param1": "param1", "param2": "param2"},
	"_item3": {
		"param1": {
            "item1": {
                "param1": 88
            }
        }
	},
	"item4":true
}"#;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_body", |b| b.iter(|| parse_body(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
