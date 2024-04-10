use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Available endpoints:\n
    Addition:\t\t/add?first=<A>&second=<B>\n
    Substraction:\t/sub?first=<A>&second=<B>\n
    Multiplication:\t/mul?first=<A>&second=<B>\n
    Division:\t\t/div?first=<A>&second=<B>\n
    Integer Division:\t/div2?first=<A>&second=<B>\n
    Modulo:\t\t/mod?first=<A>&second=<B>\n
    Sine (rad):\t\t/sin/rad?arg=<A>\n
    Sine (deg):\t\t/sin/deg?arg=<A>\n
    Cosine (rad):\t/cos/rad?arg=<A>\n
    Cosine (deg):\t/cos/deg?arg=<A>\n
    Tangent (rad):\t/tan/rad?arg=<A>\n
    Tangent (deg):\t/tan/deg?arg=<A>\n
    Square root:\t/sqrt?arg=<A>\n
    Square:\t\t/sqr?arg=<A>"
}

#[get("/add?<first>&<second>")]
fn add(first: f64, second: f64) -> String {
    format!("{}", first + second)
}

#[get("/sub?<first>&<second>")]
fn sub(first: f64, second: f64) -> String {
    format!("{}", first - second)
}

#[get("/mul?<first>&<second>")]
fn mul(first: f64, second: f64) -> String {
    format!("{}", first * second)
}

#[get("/div?<first>&<second>")]
fn div(first: f64, second: f64) -> String {
    if second == 0.0 { 
        return String::from("Division by zero");
    }
    format!("{}", first / second)
}

#[get("/div2?<first>&<second>")]
fn div2(first: isize, second: isize) -> String {
    if second == 0 {
        return String::from("Division by zero");
    }
    format!("{}", first / second)
}

#[get("/mod?<first>&<second>")]
fn modulo(first: f64, second: f64) -> String {
    if second == 0.0 {
        return String::from("Division by zero")
    }
    format!("{}", first % second)
}

#[get("/sin/rad?<arg>")]
fn sin_rad(arg: f64) -> String {
    format!("{}", sin(arg, true))
}

#[get("/sin/deg?<arg>")]
fn sin_deg(arg: f64) -> String {
    format!("{}", sin(arg, false))
}

fn sin(arg: f64, rad: bool) -> f64 {
    if rad {
        return arg.sin();
    }
    arg.to_radians().sin()
}

#[get("/cos/rad?<arg>")]
fn cos_rad(arg: f64) -> String {
    format!("{}", cos(arg, true))
}

#[get("/cos/deg?<arg>")]
fn cos_deg(arg: f64) -> String {
    format!("{}", cos(arg, false))
}

fn cos(arg: f64, rad: bool) -> f64 {
    if rad {
        return arg.cos();
    }
    arg.to_radians().cos()
}

#[get("/tan/rad?<arg>")]
fn tan_rad(arg: f64) -> String {
    format!("{}", tan(arg, true))
}

#[get("/tan/deg?<arg>")]
fn tan_deg(arg: f64) -> String {
    if arg % 90.0 == 0.0 && arg % 180.0 != 0.0 {
        return format!("tan({}) does not exist", arg);
    }
    format!("{}", tan(arg, false))
}

fn tan(arg: f64, rad: bool) -> f64 {
    if rad {
        return arg.tan();
    }
        arg.to_radians().tan()
}

#[get("/sqrt?<arg>")]
fn sqrt(arg: f64) -> String {
    if arg < 0.0 {
        return String::from("Negative argument");
    }
    format!("{}", arg.powf(0.5))
}

#[get("/sqr?<arg>")]
fn sqr(arg: f64) -> String {
    format!("{}", arg.powf(2.0))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, add, sub, mul, div, div2, modulo, sin_rad, sin_deg, cos_rad, cos_deg, tan_rad, tan_deg, sqrt, sqr]);

    Ok(rocket.into())
}
