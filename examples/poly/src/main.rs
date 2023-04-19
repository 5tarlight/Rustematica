use rustemetica::expr::{Dif, Poly, Vars};

fn main() {
    let poly1 = Poly::from(5f64, Vars::x, 3f64);
    let poly2 = Poly::from(3f64, Vars::x, 2f64);
    let poly3 = Poly::from(-7f64, Vars::x, 1f64);

    let origin = format!(
        "{} + {} + {}",
        poly1.to_string(),
        poly2.to_string(),
        poly3.to_string()
    );

    let diff = format!(
        "{} + {} + {}",
        poly1.differentiate().to_string(),
        poly2.differentiate().to_string(),
        poly3.differentiate().to_string()
    );

    println!("d/dx {} = {}", origin, diff)
}
