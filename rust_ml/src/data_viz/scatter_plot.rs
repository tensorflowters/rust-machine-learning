use plotters::{prelude::*, style::full_palette::BLUE_200};

// This is the entry point of a Rust program. It is where the program execution begins.
pub fn scatter_plot(data_to_display: Vec<(f64, i32)>, title: &str, path_to_save: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    // Draw Scatter Plot
    ctx.draw_series(
        data_to_display
            .iter()
            .map(|point| Circle::new(*point, 4.0_f64, &BLUE_200)),
    )?;

    ctx
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root_area.present()?;

    Ok(())
}
