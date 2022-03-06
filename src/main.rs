use pbr::ProgressBar;
use plotters::prelude::*;

mod utils;

const OUT_FILE_NAME: &'static str = "plots/3d-plot.gif";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 100)?.into_drawing_area();

    let frame_count = 100;
    let mut pb = ProgressBar::new(frame_count);

    for pitch in 0..frame_count {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("2D Guassian PDF", ("sans-serif", 20))
            .build_cartesian_3d(-3.0..3.0, 0.0..6.0, -3.0..3.0)?;

        chart.with_projection(|mut p| {
            p.pitch = 0.69;
            p.scale = 0.7;
            p.into_matrix()
        });

        chart.configure_axes().draw()?;

        chart.draw_series(
            SurfaceSeries::xoz(
                (-15..=15).map(|x| x as f64 / 5.0),
                (-15..=15).map(|x| x as f64 / 5.0),
                utils::distance_to_origin(pitch as f64 / frame_count as f64),
            )
            .style_func(&|&v| {
                (&HSLColor(240.0 / 360.0 - 240.0 / 360.0 * v / 5.0, 1.0, 0.7)).into()
            }),
        )?;

        root.present()?;
        pb.inc();
    }
    pb.finish_print("Rendering finished!");

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
