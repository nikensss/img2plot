pub fn distance_to_origin(progress: f64) -> Box<dyn Fn(f64, f64) -> f64> {
    Box::new(move |x: f64, y: f64| {
        const A: f64 = 10.0;
        let x = x as f64 / 10.0;
        let y = y as f64 / 10.0;
        ((progress - 0.5).abs() * 2.0) * (A * ((x.powi(2) + y.powi(2)).sqrt()).exp() - A)
    })
}
