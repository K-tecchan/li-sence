use license::License;

mod license;

use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license = License::new(
        String::from("The MIT License"),
        Some(String::from("MIT")),
        String::from("Popular / Strong Community"),
        String::from("https://opensource.org/licenses/MIT"),
        String::from("mit"),
    );

    static BASE_URL: &str = "https://opensource.org/license/";

    let response = reqwest::blocking::get(BASE_URL.to_string() + license.slug.as_str())?;

    println!("BASE_URL = {}", BASE_URL);

    println!("Status: {}", response.status());

    let body = response.text()?;
    println!("Body: {}", body);

    println!("{:?}", license);
    Ok(())
}
