use plotters::prelude::*;

mod file_readers {
    pub mod csv_reader;
}

// This is the entry point of a Rust program. It is where the program execution begins.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let house: (Vec<f64>, Vec<i32>) = match file_readers::csv_reader::read_csv() {
        Ok(t) => t,
        _ => (Vec::new(), Vec::new()),
    };
    let price: Vec<f64> = house.0;
    let sqft_living: Vec<i32> = house.1;

    let price_sqft_living: Vec<(f64, i32)> = price
        .iter()
        .cloned()
        .zip(sqft_living.iter().cloned())
        .collect();

    let root_area: DrawingArea<SVGBackend, plotters::coord::Shift> = 
        SVGBackend::new("/home/arthur/workspace/ia/rust-machine-learning/house_sales_king_county.svg", (1200, 800)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let mut ctx: 
        ChartContext<
            SVGBackend, 
            Cartesian2d<plotters::coord::types::RangedCoordf64, 
            plotters::coord::types::RangedCoordi32>
        > = 
        ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("House Sales in King County", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..8000.0, 0..10000)?;

    ctx.configure_mesh().draw()?;

    // Draw Scatter Plot
    ctx.draw_series(
        price_sqft_living
            .iter()
            .map(|point| Circle::new(*point, 4.0_f64, &BLUE)),
    )?;

    ctx
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root_area.present()?;

    // Draw Area Plot
    // ctx.draw_series(
    //     AreaSeries::new((0..).zip(sqft_living[..10].iter().map(|x| *x)), 0 ,&RED.mix(0.2)).border_style(&RED)
    // ).unwrap();

    // Draw Bar Plot
    // ctx.draw_series((0..).zip(sqft_living[..10].iter()).map(|(x, y)| {
    //     let x0 = SegmentValue::Exact(x);
    //     let x1 = SegmentValue::Exact(x + 1);
    //     let mut bar = Rectangle::new([(x0, 0), (x1, *y)], BLUE.filled());
    //     bar.set_margin(0, 0, 5, 5);
    //     bar
    // })).unwrap();

    // Draw Line Chart
    // ctx.draw_series(
    //     LineSeries::new((0..).zip(sqft_living[..10].iter()).map(|(idx, y)| {(idx, *y)}),&BLUE)
    // ).unwrap();

    println!("Should output plot");

    Ok(())
}
