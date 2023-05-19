mod file_readers {
    pub mod csv_reader;
}
mod data_viz {
    pub mod scatter_plot;
    pub mod line_chart;
    pub mod area_chart;
}

// This is the entry point of a Rust program. It is where the program execution begins.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let house: (Vec<f64>, Vec<i32>) = match file_readers::csv_reader::read_csv("/home/arthur/workspace/rust-machine-learning/datasets/kc_house_data.csv") {
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

    data_viz::scatter_plot::scatter_plot(price_sqft_living.clone(), "Houses sales scatter plot", "/home/arthur/workspace/rust-machine-learning/rust_ml/house_sales_king_county_scatter_plot.svg")?;
    data_viz::line_chart::line_chart(sqft_living.clone(), "Houses sales line chart", "/home/arthur/workspace/rust-machine-learning/rust_ml/house_sales_king_county_line_chart.svg")?;
    data_viz::area_chart::area_chart(sqft_living.clone(), "Houses sales area chart", "/home/arthur/workspace/rust-machine-learning/rust_ml/house_sales_king_county_area_chart.svg")?;

    Ok(())
}
