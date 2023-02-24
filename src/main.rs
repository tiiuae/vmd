mod kvm_util;

fn main() {
    match kvm_util::check_kvm_api_version() {
        Ok(version) => println!("KVM API version: {}", version),
        Err(e) => eprintln!("Error: {:?}", e),
    }
    match kvm_util::check_kvm_extensions() {
        Ok(()) => println!("KVM extensions OK"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
