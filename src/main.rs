mod kvm_util;

use crate::kvm_util::get_kvm_api_version;

fn main() {
    let api_version: i32 =  get_kvm_api_version();
    println!("hello kvm_api {}", api_version)
}
