
use std::io;
fn main() {

/// fahrencius 
/// Goal : Convert Fahrenheit temperatures in Celcius temperatures 
/// 
    fn fahrencius(t: f32) -> f32{
       return  (t-32.0)*5./9.; 
    }
    println!("Which temperature is it in Fahrenheit ? ");
    let mut temperature = String::new();
    println!("You entered {temperature} F");
    io::stdin().read_line(&mut temperature).expect("Not nice");
    let temperature:f32 = match temperature.trim().parse() {
        Ok(value) => value,
        Err(_) => -800.,
    };
    let temperature_in_celcius = fahrencius(temperature);

    if temperature_in_celcius == fahrencius(-800.){
        println!("There was probably an error when entering the temperature but here is the default Celcius value: {temperature_in_celcius} °C")
    }

    else {
        println!("The corresponding temperature in degrees Celcius is: {temperature_in_celcius} °C");
    }


}
