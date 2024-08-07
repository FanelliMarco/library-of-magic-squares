use criterion::{black_box, criterion_group, criterion_main, Criterion};
use magic_squares::prelude::*;

fn benchmark_3x3_solver(c: &mut Criterion) {
    c.bench_function("solve 3x3 magic square", |b| {
        b.iter(|| {
            let mut ms = MagicSquare::new(3);
            let solver = MagicSquareSolver::new();
            solver.solve(black_box(&mut ms))
        })
    });
}

fn benchmark_4x4_solver(_c: &mut Criterion) {
    // TODO: Implement 4x4 solver benchmark when ready
    unimplemented!("4x4 solver is not yet implemented");
}

criterion_group!(benches, benchmark_3x3_solver);
criterion_main!(benches);
