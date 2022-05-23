pub extern "C" fn abs(const x: f32) -> f32
{
    if x >= 0 {x} else {-x}
}

pub extern "C" fn pow(base: f64, exp: f64) -> f64
{
    let mut sum = base;

    for i in 0..exp {
        sum = sum * base;
    }

    sum
}

pub extern "C" fn sqrt(n: f64) -> f64
{
    let mut x: f64 = n;
    let mut y: f64 = 1;
    let e = 0.000001;

    while (x - y) > e {
        x = (x + y) / 2;
        y = n / x;
    }

    x
}
