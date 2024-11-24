use std::{f32::consts::PI, iter::zip};

use csl::plot::{
    figure::figure::FigureProperties,
    graph::{GraphProperties, Point},
    window::PlotWindowProperties,
};

extern crate csl;

fn main() {
    // Initialize data points
    let x = (0..=100).map(|x| (x as f32) / 100.0);
    let y = x.clone().map(|x| f32::sin(2.0 * PI * x));
    let points: Vec<Point> = zip(x, y).map(|(x, y)| [x, y]).collect();

    // Initialize Window
    let mut window = csl::plot::window::PlotWindow::new(PlotWindowProperties {
        width: 500,
        height: 500,
        title: "Test plot".to_string(),
        ..Default::default()
    });

    // Add a new Figure
    window.add_figure(FigureProperties {
        graphs: vec![GraphProperties {
            anim: Some(|_data| {
                // data.push([0.0, 0.0]);
            }),
            data: points.clone(),
            ..Default::default()
        }],
        ..Default::default()
    });

    window.run();
}
