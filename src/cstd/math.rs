pub extern "C" fn abs(x: f32) -> f32
{
    x.abs() 
}

pub extern "C" fn pow(base: f64, exp: f64) -> f64
{
    base.pow(exp)
}

pub extern "C" fn sqrt(n: f64) -> f64
{
    n.sqrt()
}

pub extern "C" fn sin(n: f64) -> f64
{
    n.sin()
}

pub extern "C" fn cos(n: f64) -> f64
{
    n.cos()
}

pub extern "C" fn tan(n: f64) -> f64
{
    n.tan()
}
