use crate::Node;
use plotters::prelude::*;

pub fn plot_route(route: &Vec<Node>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    let range_max = route
        .iter()
        .map(|x| (x.x as f32, x.y as f32))
        .fold((0.0 / 0.0, 0.0 / 0.0), |m, v| (v.0.max(m.0), v.1.max(m.1)));
    let range_min = route
        .iter()
        .map(|x| (x.x as f32, x.y as f32))
        .fold((0.0 / 0.0, 0.0 / 0.0), |m, v| (v.0.min(m.0), v.1.min(m.1)));
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_ranged(range_min.0..range_max.0, range_min.1..range_max.1)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        route.iter().map(|node| (node.x as f32, node.y as f32)),
        &RED,
    ))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        route.iter().map(|node| (node.x as f32, node.y as f32)),
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    ))?;
    Ok(())
}
