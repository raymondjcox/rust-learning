fn convert(celcius: f32) -> f32 {
    celcius * 1.8 + 32.0
}

fn main() {
    let celcius: f32 = 0.0;
    println!("{} celcius is {} farenheight", celcius, convert(celcius));
}
