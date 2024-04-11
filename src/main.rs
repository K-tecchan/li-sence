use license::License;

mod license;

fn main() {
    let license = License::new(
        String::from("The MIT License"),
        Some(String::from("MIT")),
        String::from("Popular / Strong Community"),
        String::from("https://opensource.org/licenses/MIT"),
    );

    println!("{:?}", license);
}
