use num::complex::Complex;

fn calculate_mandelbrot(
    max_iter: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = Vec::with_capacity(height);

    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(width);
        for img_x in 0..width {
            let x_percent = (img_x as f64) / (width as f64);
            let y_percent = (img_y as f64) / (height as f64);
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;

            let escaped_at = mandelbrot_at(cx, cy, max_iter);
            row.push(escaped_at);
        }
        rows.push(row);
    }

    rows
}

fn mandelbrot_at(cx: f64, cy: f64, max_iter: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iter {
        if z.norm() > 2.0 {
            // Escape condition met
            return i;
        }
        z = z * z + c;
    }
    max_iter
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for escape_val in row {
            let c = match escape_val {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(c);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 2.0, -1.0, 1.0, 100, 24);
    render_mandelbrot(mandelbrot);
}
