use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::thread::sleep;

fn main() {
    println!("Please provide the IP address of Awair Element.");

    let mut sensor_ip = String::new();

    io::stdin()
        .read_line(&mut sensor_ip)
        .expect("Failed to read line");

    loop {
        let request_url = format!("http://{}/air-data/latest", sensor_ip.trim());

        let resp = reqwest::blocking::get(request_url).unwrap().text();

        match resp {
            Ok(data) => {
                let data: HashMap<String, Value> = serde_json::from_str(&data).unwrap();

                // @TODO send this data to a database
                println!(
                    "Time: {}, Score: {}, Temperature: {}, VOC: {}, PM2.5: {}, CO2: {}, Humidity: {}", 
                    data["timestamp"],
                    data["score"],
                    data["temp"],
                    data["voc"],
                    data["pm25"],
                    data["co2"],
                    data["humid"],
                );
            }
            Err(_) => {
                println!("Failed to read data from Awair Element");
            }
        }

        sleep(std::time::Duration::from_secs(10));
    }
}
