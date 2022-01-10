fn main() {
    let body: String = ureq::get("https://api.cloudscale.ch/v1")
        .call().unwrap().into_string().unwrap();

    println!("{}", body);
}
