use crate::beal::run_beal_analysis;
use plotters::prelude::*;


#[allow(dead_code)]
pub fn plot(data: Vec<u64>, output_path: String, title: String) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&output_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(&title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..(data.len() + 1) as f32, 0f32..*(data.iter().max().unwrap()) as f32)?;
        chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(data.iter().enumerate().map(|(i, v)| (i as f32, *v as f32)), &RED))?
        .label(&title);
        // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}


/// for some reason at a limit of 100T, an additional 100M (0.0001% increase)
/// increases the runtime from 1.5s to ~10s, (a 660% increase!).
/// Run the analysis for many smaller cases and graph it to look for patterns.
#[allow(dead_code)]
pub fn inspect_100t_limit_anomaly() {
    let mut limit_breaks: Vec<u64> = vec![];
    let mut continue_counts: Vec<u64> = vec![];
    let mut inner_loop_counts: Vec<u64> = vec![];
    for i in 1..101 {
        let local_limit = i * 1_000_000_000_000;
        println!("\nRunning with limit: {local_limit}");
        let solution_set = run_beal_analysis(local_limit);
        limit_breaks.push(solution_set.limit_breaks);
        continue_counts.push(solution_set.continue_count);
        inner_loop_counts.push(solution_set.inner_loop_count);
    }
    plot(limit_breaks, "limit_breaks.png".to_string(), "limit_breaks".to_string()).unwrap();
    plot(continue_counts, "continue_counts.png".to_string(), "continue_counts".to_string()).unwrap();
    plot(inner_loop_counts, "inner_loop_counts.png".to_string(), "inner_loop_counts".to_string()).unwrap();
}
