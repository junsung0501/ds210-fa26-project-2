//! Draws `plot.png`, and prints the summary table underneath it.
//!
//! You won't edit this, but reading it may help you understand what the
//! plot is measuring for which one of the writeup questions.

use crate::measure::measure;
use crate::secret_keeper::SecretKeeper;
use crate::strategies::{Approach, binary, jump, linear, lucky, random};

use plotters::prelude::*;
use plotters::style::full_palette::{GREEN, ORANGE, PURPLE};

mod measure;
mod secret_keeper;
mod strategies;

/// The worst case for one strategy on `[min, max)`, over a handful of numbers
/// chosen to include the ends and the middle. Every keeper here is built fresh,
/// so nothing is remembered between calls.
fn worst_case(approach: &Approach, min: u32, max: u32) -> u32 {
    let numbers = [
        min,
        max - 1,
        (min + max) / 2,
        (min + max) / 2 + (min + max) / 3,
    ];
    let mut worst = 0;

    for number in numbers {
        let mut keeper = SecretKeeper::known(number);
        match approach {
            Approach::Linear => {
                linear(&mut keeper, min, max);
            }
            Approach::Binary => {
                binary(&mut keeper, min, max);
            }
            Approach::Jump => {
                jump(&mut keeper, min, max);
            }
            Approach::Lucky => {
                lucky(&mut keeper, min, max);
            }
            _ => {
                random(&mut keeper, min, max);
            }
        }
        if keeper.questions_asked() > worst {
            worst = keeper.questions_asked();
        }
    }

    worst
}

fn main() {
    let mut random = Vec::new();
    let mut linear = Vec::new();
    let mut binary = Vec::new();
    let mut jump = Vec::new();
    let mut lucky = Vec::new();

    for max in 1..=100 {
        random.push((max as f32, worst_case(&Approach::Random, 0, max) as f32));
        linear.push((max as f32, worst_case(&Approach::Linear, 0, max) as f32));
        binary.push((max as f32, worst_case(&Approach::Binary, 0, max) as f32));
        jump.push((max as f32, worst_case(&Approach::Jump, 0, max) as f32));
        lucky.push((max as f32, worst_case(&Approach::Lucky, 0, max) as f32));
    }

    let root = BitMapBackend::new("plot.png", (600, 600)).into_drawing_area();
    let root = root.margin(10, 10, 10, 10);
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Number of guesses to find solution",
            ("sans-serif", 30).into_font(),
        )
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(0f32..103f32, 0f32..103f32)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .x_label_style(("sans-serif", 25))
        .y_labels(20)
        .y_label_style(("sans-serif", 25))
        .draw()
        .unwrap();

    let style1 = ShapeStyle::from(&ORANGE).stroke_width(1).filled();
    let style2 = ShapeStyle::from(&BLUE).stroke_width(1).filled();
    let style3 = ShapeStyle::from(&BLACK).stroke_width(1).filled();
    let style4 = ShapeStyle::from(&GREEN).stroke_width(1).filled();
    let style5 = ShapeStyle::from(&PURPLE).stroke_width(1).filled();

    chart
        .draw_series(LineSeries::new(linear, style1.clone()).point_size(2))
        .unwrap()
        .label("Linear")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style1));
    chart
        .draw_series(LineSeries::new(binary, style2.clone()).point_size(2))
        .unwrap()
        .label("Binary")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style2));
    chart
        .draw_series(LineSeries::new(jump, style4.clone()).point_size(2))
        .unwrap()
        .label("Jump")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style4));
    chart
        .draw_series(LineSeries::new(lucky, style5.clone()).point_size(2))
        .unwrap()
        .label("Lucky")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style5));
    chart
        .draw_series(LineSeries::new(random, style3.clone()).point_size(2))
        .unwrap()
        .label("Random")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style3));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .label_font(("sans-serif", 30))
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    root.present().unwrap();
    println!("Wrote plot.png");

    // The plot above shows the worst case. The table below shows the distribution.
    println!();
    println!("Over 1000 rounds on the range [0, 100):");
    println!(
        "{:<10}{:>8}{:>10}{:>12}{:>9}",
        "strategy", "best", "mean", "worst case", "std dev"
    );
    let mut ranked = Vec::new();
    for (name, approach) in [
        ("linear", Approach::Linear),
        ("binary", Approach::Binary),
        ("jump", Approach::Jump),
        ("lucky", Approach::Lucky),
        ("clever", Approach::Clever),
        ("random", Approach::Random),
    ] {
        let r = measure(approach, 0, 100, 1000);
        println!(
            "{:<10}{:>8}{:>10.2}{:>12}{:>9.2}",
            name, r.best, r.mean, r.worst, r.std_dev
        );
        ranked.push((name, r));
    }

    // Put the same six in order using the rule the student wrote, so they can see
    // what their own definition of "better" actually claims. Selection sort, the
    // one from lecture 10, with is_better_than in place of `<`.
    for i in 0..ranked.len() {
        let mut best = i;
        for j in (i + 1)..ranked.len() {
            if ranked[j].1.is_better_than(&ranked[best].1) {
                best = j;
            }
        }
        ranked.swap(i, best);
    }

    println!();
    print!("Ranked by your own is_better_than, best first: ");
    for (position, (name, _)) in ranked.iter().enumerate() {
        if position > 0 {
            print!(", ");
        }
        print!("{}", name);
    }
    println!();
    println!("Does that order match what the plot shows? The writeup asks.");
}
