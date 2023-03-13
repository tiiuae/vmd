mod kvm_util;

#[macro_use] extern crate rocket;

use rocket::mtls::Certificate;

#[get("/")]
fn mutual(cert: Certificate<'_>) -> String {
    format!("Hello! Here's what we know: [{}] {}", cert.serial(), cert.subject())
}

#[get("/", rank = 2)]
fn kvm_api_version() -> String {
    match kvm_util::check_kvm_api_version() {
        Ok(version) => format!("KVM API version: {}", version),
        Err(e) => format!("Error: {:?}", e),
    }
}

#[launch]
fn rocket() -> _ {
    // See `Rocket.toml` and `Cargo.toml` for TLS configuration.
    // Run `./private/gen_certs.sh` to generate a CA and key pairs.
    rocket::build().mount("/", routes![
        kvm_api_version,
        mutual
    ])
}
// fn main() {
//     match kvm_util::check_kvm_api_version() {
//         Ok(version) => println!("KVM API version: {}", version),
//         Err(e) => eprintln!("Error: {:?}", e),
//     }
//     match kvm_util::check_kvm_extensions() {
//         Ok(()) => println!("KVM extensions OK"),
//         Err(e) => eprintln!("Error: {:?}", e),
//     }
// }
