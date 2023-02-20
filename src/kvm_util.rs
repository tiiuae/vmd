use kvm_ioctls::Kvm;

/// ```
/// use kvm_util::get_kvm_api_version;
/// //"Applications should refuse to run if KVM_GET_API_VERSION
/// // returns a value other than 12."
/// // https://www.kernel.org/doc/Documentation/virtual/kvm/api.txt
/// let api_version = get_kvm_api_version();
/// assert_eq!(api_version, 12);
/// ```
pub fn get_kvm_api_version() -> i32 {
    let kvm = Kvm::new().unwrap(); //TODO
    let kvm_api_version: i32 = kvm.get_api_version();
    assert_eq!(kvm_api_version, 12); // clients must check to continue
    kvm_api_version
}


/*fn main() {


    println!("Recommended number of VCPUs per VM:         {}", kvm.get_nr_vcpus());
    println!("Recommended maximum number of VCPUs per VM: {}", kvm.get_max_vcpus());
    println!("Maximum allowed memory slots per VM:        {}", kvm.get_nr_memslots());
    println!("KVM capabilities available:");
    println!("  IOMMU {}", kvm.check_extension(Cap::Iommu));
    println!("  UserMemory {}", kvm.check_extension(Cap::UserMemory));

}*/
