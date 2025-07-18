use plotters::{prelude::*, style::full_palette::BLUE_200};

// This is the entry point of a Rust program. It is where the program execution begins.
pub fn line_chart(data_to_display: Vec<i32>, title: &str, path_to_save: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root_area: DrawingArea<SVGBackend, plotters::coord::Shift> = 
        SVGBackend::new(path_to_save, (1200, 800)).into_drawing_area();

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
        .caption(title, ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..8000.0, 0..10000)?;

    ctx.configure_mesh().draw()?;

    // Draw Line Chart
    ctx.draw_series(
        LineSeries::new(
            (0..).map(|x| x as f64)
            .zip(
                data_to_display[..10].iter()
            )
            .map(
                |(idx, y)| {(idx, *y)}
            ),
            &BLUE_200
        )
    )?;

    ctx
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root_area.present()?;

    Ok(())
}
