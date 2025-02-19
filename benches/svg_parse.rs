use divan::bench;
use vglang::svg::parse::ParseSvg;

fn main() {
    divan::main();
}

#[bench]
fn bench_parse_number() {
    "+3.14e-10".parse_svg::<f32>().unwrap();
}

#[bench]
fn bench_parse_number_rust() {
    "+3.14e-10".parse::<f32>().unwrap();
}
