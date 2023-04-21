use rustemetica::expr::{AtomicExpr, Dif, Poly, Vars};

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
    print!("d/dx {} = ", origin);

    if let AtomicExpr::Poly(poly) = poly1.differentiate() {
        print!("{} ", poly.to_string());
    }
    if let AtomicExpr::Poly(poly) = poly2.differentiate() {
        print!("{} ", poly.to_string());
    }
    if let AtomicExpr::Constant(con) = poly3.differentiate() {
        println!("{}", con);
    }
}
