use std::f32::consts::PI;

use csl::plot::{figure::FigureProperties, window::PlotWindowProperties};

extern crate csl;

fn main() {
    // Idea:
    /*
    let mut window = csl::plot::PlotWindow::new(300, 300, "Test Plot");
    let figure = window.create_figure(Figure {
        some_parameters_we_want_to_configure: 123123,
        like_grid: 1,
        or_labels: 1,
        etc: 1
        ..Default::default()
    });

    figure.plot(x, y, PlotPropperties {
        more_properties: "Plot title",
        ..Default::default()
    })

    window.show();

    */

    // Initialize data points
    let x = (0..=100).map(|x| (x as f32) / 100.0);

    let y = x.clone().map(|x| f32::sin(2.0 * PI * x));

    let mut window = csl::plot::window::PlotWindow::new(PlotWindowProperties {
        width: 300,
        height: 300,
        title: "Test plot".to_string(),
        ..Default::default()
    });

    window.create_figure(FigureProperties::default());

    /*figure.plot(x, y, PlotPropperties {
        more_properties: "Plot title",
        ..Default::default()
    })*/

    window.run(x, y);
}
