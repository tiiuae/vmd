#![allow(unused)]
mod kvm_util;
mod kvm_simple_vm;
use kvm_util::get_kvm_api_version;
use kvm_simple_vm::kvm_simple_vm;

use kvm_ioctls::VcpuExit;
use kvm_ioctls::{Kvm, VcpuFd, VmFd};
use std::io::Write;
use std::ptr::null_mut;
use std::slice;
use kvm_bindings::kvm_userspace_memory_region;
use kvm_bindings::KVM_MEM_LOG_DIRTY_PAGES;

fn main() {
    let api_version: i32 =  get_kvm_api_version();
    println!("hello kvm_api {}", api_version);

	kvm_simple_vm();
}
