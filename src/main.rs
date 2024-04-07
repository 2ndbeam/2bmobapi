use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!\nHello Gryadki"
}

#[get("/add/<first>/<second>")]
fn add(first: f64, second: f64) -> String {
    format!("{} + {} = {}", first, second, first + second)
}

#[get("/sub/<first>/<second>")]
fn sub(first: f64, second: f64) -> String {
    format!("{} - {} = {}", first, second, first - second)
}

#[get("/mul/<first>/<second>")]
fn mul(first: f64, second: f64) -> String {
    format!("{} * {} = {}", first, second, first * second)
}

#[get("/div/<first>/<second>")]
fn div(first: f64, second: f64) -> String {
    if second == 0.0 { 
        return String::from("Division by zero");
    }
    format!("{} / {} = {}", first, second, first / second)
}

#[get("/sin/rad/<arg>")]
fn sin_rad(arg: f64) -> String {
    let res: f64 = sin(arg, true);
    format!("sin({}) = {}", arg, res)
}

#[get("/sin/deg/<arg>")]
fn sin_deg(arg: f64) -> String {
    let res: f64 = sin(arg, false);
    format!("sin({}) = {}", arg, res)
}

fn sin(arg: f64, rad: bool) -> f64 {
    if rad {
        return arg.sin();
    }
    arg.to_radians().sin()
}

#[get("/cos/rad/<arg>")]
fn cos_rad(arg: f64) -> String {
    let res: f64 = cos(arg, true);
    format!("cos({}) = {}", arg, res)
}

#[get("/cos/deg/<arg>")]
fn cos_deg(arg: f64) -> String {
    let res: f64 = cos(arg, false);
    format!("cos({}) = {}", arg, res)
}

fn cos(arg: f64, rad: bool) -> f64 {
    if rad {
        return arg.cos();
    }
    arg.to_radians().cos()
}

#[get("/tan/rad/<arg>")]
fn tan_rad(arg: f64) -> String {
    let res: f64 = tan(arg, true);
    format!("tan({}) = {}", arg, res)
}

#[get("/tan/deg/<arg>")]
fn tan_deg(arg: f64) -> String {
    if arg % 90.0 == 0.0 && arg % 180.0 != 0.0 {
        return format!("tan({}) does not exist", arg);
    }
    let res: f64 = tan(arg, false);
    format!("tan({}) = {}", arg, res)
}

fn tan(arg: f64, rad: bool) -> f64 {
    if rad {
        return arg.tan();
    }
        arg.to_radians().tan()
}


#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, add, sub, mul, div, sin_rad, sin_deg, cos_rad, cos_deg, tan_rad, tan_deg]);

    Ok(rocket.into())
}
