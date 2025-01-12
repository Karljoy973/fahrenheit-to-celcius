
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
    io::stdin().read_line(&mut temperature).expect("Unable to read user input \n");
    let temperature = temperature.trim(); 
    println!("You entered {temperature} F");

    let temperature:f32 = temperature.parse().expect("Invalid Input, please provide a valid number \n"); 
    let temperature_in_celcius = fahrencius(temperature);
    println!("The corresponding temperature in degrees Celcius is: {temperature_in_celcius} °C");


}
