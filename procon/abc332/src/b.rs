use proconio::input;

fn main() {
    input! {
        k: u32,
        g: u32,
        m: u32,
    }
    let mut cg = g;
    let mut cm = m;
    for _ in 0..k {
        if cg == g {
            cg = 0;
        } else if cm == 0 {
            cm = m;
        } else {
            while cg < g && cm > 0 {
                cg += 1;
                cm -= 1;
            }
        }
    }
    println!("{} {}", cg, cm);
}
