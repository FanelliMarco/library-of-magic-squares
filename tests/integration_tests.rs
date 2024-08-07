use magic_squares::prelude::*;

#[test]
fn test_create_and_solve_3x3() {
    let mut ms = MagicSquare::new(3);
    let solver = MagicSquareSolver::new();

    assert!(solver.solve(&mut ms));
    assert!(is_magic_square(&ms));
}

#[test]
fn test_create_and_solve_4x4() {
    let mut ms = MagicSquare::new(4);
    let solver = MagicSquareSolver::new();

    assert!(solver.solve(&mut ms));
    assert!(is_magic_square(&ms));
}

#[test]
fn test_invalid_size() {
    let ms = MagicSquare::new(2);
    assert_eq!(ms.order(), 2);
    assert!(ms.to_2d_vec().is_empty());
}

#[test]
fn test_magic_constant() {
    for n in 3..=5 {
        let ms = MagicSquare::new(n);
        assert_eq!(ms.magic_constant(), n * (n * n + 1) / 2);
    }
}

#[test]
fn test_display_format() {
    let mut ms = MagicSquare::new(3);
    let solver = MagicSquareSolver::new();
    solver.solve(&mut ms);

    let display_output = format!("{}", ms);
    assert!(display_output.lines().count() == 3);
    assert!(display_output
        .lines()
        .all(|line| line.split_whitespace().count() == 3));
}
