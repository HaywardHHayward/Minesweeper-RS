use std::num::{NonZeroU8, NonZeroU16};

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

const BEGINNER_DIMENSIONS: (u8, u8) = (9, 9);
const INTERMEDIATE_DIMENSIONS: (u8, u8) = (16, 16);
const EXPERT_DIMENSIONS: (u8, u8) = (30, 16);

const BOARD_SIZES: [(u8, u8); 10] = [
    (4, 4),
    (5, 5),
    BEGINNER_DIMENSIONS,
    (12, 12),
    INTERMEDIATE_DIMENSIONS,
    (25, 13),
    (20, 20),
    EXPERT_DIMENSIONS,
    (30, 30),
    (50, 50),
];

/// Worst case scenario for opening a cell. Since there's only one mine, pretty
/// much every cell in the board is going to be opened
pub fn open_cell_one_mine(c: &mut Criterion) {
    let mut group = c.benchmark_group("Opening cells, one mine");
    for (width, height) in BOARD_SIZES.iter() {
        group.throughput(Throughput::Elements(u64::from(
            (*width as u16) * (*height as u16) - 1,
        )));
        let input_string = format!("{width}x{height}");
        let input = (*width, *height);
        group.bench_with_input(
            BenchmarkId::from_parameter(input_string),
            &input,
            |b, &(width, height)| {
                b.iter_custom(|iterations| {
                    let mut timed_duration = std::time::Duration::ZERO;
                    for _ in 0..iterations {
                        let mut board = minesweeper_rs::Board::create_custom(
                            NonZeroU8::new(width).unwrap(),
                            NonZeroU8::new(height).unwrap(),
                            NonZeroU16::new(1).unwrap(),
                        )
                        .unwrap();
                        let (center_x, center_y) = (width / 2, height / 2);
                        let random_seed = rand::random::<u64>();
                        board.generate_mines_with_seed(center_x, center_y, random_seed);
                        let start = std::time::Instant::now();
                        #[allow(clippy::unit_arg)]
                        std::hint::black_box(board.open_cell(
                            std::hint::black_box(center_x),
                            std::hint::black_box(center_y),
                        ));
                        timed_duration += start.elapsed();
                    }
                    timed_duration
                })
            },
        );
    }
}

pub fn open_cell_defaults(c: &mut Criterion) {
    let mut group = c.benchmark_group("Opening cells, default mines");
    let choices = [
        (BEGINNER_DIMENSIONS, 10),
        (INTERMEDIATE_DIMENSIONS, 40),
        (EXPERT_DIMENSIONS, 99),
    ];
    for ((width, height), mines) in choices.iter() {
        group.throughput(Throughput::Elements(u64::from(
            (*width as u16) * (*height as u16) - mines,
        )));
        let input_string = format!("{width}x{height}");
        let input = (*width, *height);
        group.bench_with_input(
            BenchmarkId::from_parameter(input_string),
            &input,
            |b, &(width, height)| {
                b.iter_custom(|iterations| {
                    let mut timed_duration = std::time::Duration::ZERO;
                    for _ in 0..iterations {
                        let mut board = minesweeper_rs::Board::create_custom(
                            NonZeroU8::new(width).unwrap(),
                            NonZeroU8::new(height).unwrap(),
                            NonZeroU16::new(*mines).unwrap(),
                        )
                        .unwrap();
                        let (center_x, center_y) = (width / 2, height / 2);
                        let random_seed = rand::random::<u64>();
                        board.generate_mines_with_seed(center_x, center_y, random_seed);
                        let start = std::time::Instant::now();
                        #[allow(clippy::unit_arg)]
                        std::hint::black_box(board.open_cell(
                            std::hint::black_box(center_x),
                            std::hint::black_box(center_y),
                        ));
                        timed_duration += start.elapsed();
                    }
                    timed_duration
                })
            },
        );
    }
}

pub fn open_cell_almost_all_mines(c: &mut Criterion) {
    let mut group = c.benchmark_group("Opening cells, (almost) all mines");
    for (width, height) in BOARD_SIZES.iter() {
        let input_string = format!("{width}x{height}");
        let input = (*width, *height);
        group.bench_with_input(
            BenchmarkId::from_parameter(input_string),
            &input,
            |b, &(width, height)| {
                let mines = width as u16 * height as u16 - 9;
                b.iter_custom(|iterations| {
                    let mut timed_duration = std::time::Duration::ZERO;
                    for _ in 0..iterations {
                        let mut board = minesweeper_rs::Board::create_custom(
                            NonZeroU8::new(width).unwrap(),
                            NonZeroU8::new(height).unwrap(),
                            NonZeroU16::new(mines).unwrap(),
                        )
                        .unwrap();
                        let (center_x, center_y) = (width / 2, height / 2);
                        let random_seed = rand::random::<u64>();
                        board.generate_mines_with_seed(center_x, center_y, random_seed);
                        let start = std::time::Instant::now();
                        #[allow(clippy::unit_arg)]
                        std::hint::black_box(board.open_cell(
                            std::hint::black_box(center_x),
                            std::hint::black_box(center_y),
                        ));
                        timed_duration += start.elapsed();
                    }
                    timed_duration
                })
            },
        );
    }
}

pub fn open_cell_25_percent(c: &mut Criterion) {
    let mut group = c.benchmark_group("Opening cells, 25 percent of cells mines");
    for (width, height) in BOARD_SIZES.iter() {
        let input_string = format!("{width}x{height}");
        let input = (*width, *height);
        let board_area = (*width as u16) * (*height as u16);
        let mines = u16::clamp(board_area / 4, 1, board_area - 1);
        group.throughput(Throughput::Elements(u64::from(
            (*width as u16) * (*height as u16) - mines,
        )));
        group.bench_with_input(
            BenchmarkId::from_parameter(input_string),
            &input,
            |b, &(width, height)| {
                b.iter_custom(|iterations| {
                    let mut timed_duration = std::time::Duration::ZERO;
                    for _ in 0..iterations {
                        let mut board = minesweeper_rs::Board::create_custom(
                            NonZeroU8::new(width).unwrap(),
                            NonZeroU8::new(height).unwrap(),
                            NonZeroU16::new(mines).unwrap(),
                        )
                        .unwrap();
                        let (center_x, center_y) = (width / 2, height / 2);
                        let random_seed = rand::random::<u64>();
                        board.generate_mines_with_seed(center_x, center_y, random_seed);
                        let start = std::time::Instant::now();
                        #[allow(clippy::unit_arg)]
                        std::hint::black_box(board.open_cell(
                            std::hint::black_box(center_x),
                            std::hint::black_box(center_y),
                        ));
                        timed_duration += start.elapsed();
                    }
                    timed_duration
                })
            },
        );
    }
}

pub fn open_cell_50_percent(c: &mut Criterion) {
    let mut group = c.benchmark_group("Opening cells, 50 percent of cells mines");
    for (width, height) in BOARD_SIZES.iter() {
        let input_string = format!("{width}x{height}");
        let input = (*width, *height);
        let board_area = (*width as u16) * (*height as u16);
        let mines = u16::clamp(board_area / 2, 1, board_area - 1);
        group.throughput(Throughput::Elements(u64::from(
            (*width as u16) * (*height as u16) - mines,
        )));
        group.bench_with_input(
            BenchmarkId::from_parameter(input_string),
            &input,
            |b, &(width, height)| {
                b.iter_custom(|iterations| {
                    let mut timed_duration = std::time::Duration::ZERO;
                    for _ in 0..iterations {
                        let mut board = minesweeper_rs::Board::create_custom(
                            NonZeroU8::new(width).unwrap(),
                            NonZeroU8::new(height).unwrap(),
                            NonZeroU16::new(mines).unwrap(),
                        )
                        .unwrap();
                        let (center_x, center_y) = (width / 2, height / 2);
                        let random_seed = rand::random::<u64>();
                        board.generate_mines_with_seed(center_x, center_y, random_seed);
                        let start = std::time::Instant::now();
                        #[allow(clippy::unit_arg)]
                        std::hint::black_box(board.open_cell(
                            std::hint::black_box(center_x),
                            std::hint::black_box(center_y),
                        ));
                        timed_duration += start.elapsed();
                    }
                    timed_duration
                })
            },
        );
    }
}

pub fn open_cell_75_percent(c: &mut Criterion) {
    let mut group = c.benchmark_group("Opening cells, 75 percent of cells mines");
    for (width, height) in BOARD_SIZES.iter() {
        let input_string = format!("{width}x{height}");
        let input = (*width, *height);
        let board_area = (*width as u16) * (*height as u16);
        let mines = u16::clamp(board_area * 3 / 4, 1, board_area - 1);
        group.throughput(Throughput::Elements(u64::from(
            (*width as u16) * (*height as u16) - mines,
        )));
        group.bench_with_input(
            BenchmarkId::from_parameter(input_string),
            &input,
            |b, &(width, height)| {
                b.iter_custom(|iterations| {
                    let mut timed_duration = std::time::Duration::ZERO;
                    for _ in 0..iterations {
                        let mut board = minesweeper_rs::Board::create_custom(
                            NonZeroU8::new(width).unwrap(),
                            NonZeroU8::new(height).unwrap(),
                            NonZeroU16::new(mines).unwrap(),
                        )
                        .unwrap();
                        let (center_x, center_y) = (width / 2, height / 2);
                        let random_seed = rand::random::<u64>();
                        board.generate_mines_with_seed(center_x, center_y, random_seed);
                        let start = std::time::Instant::now();
                        #[allow(clippy::unit_arg)]
                        std::hint::black_box(board.open_cell(
                            std::hint::black_box(center_x),
                            std::hint::black_box(center_y),
                        ));
                        timed_duration += start.elapsed();
                    }
                    timed_duration
                })
            },
        );
    }
}

criterion_group!(
    benches,
    open_cell_one_mine,
    open_cell_defaults,
    open_cell_almost_all_mines,
    open_cell_25_percent,
    open_cell_50_percent,
    open_cell_75_percent,
);
criterion_main!(benches);
