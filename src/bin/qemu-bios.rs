use std::{
    env,
    process::{self, Command},
};

fn main() {
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive");
    qemu.arg(format!("format=raw,file={}", env!("BIOS_IMAGE")));
    qemu.arg("-device");
    qemu.arg("isa-debug-exit,iobase=0xf4,iosize=0x04");
    qemu.arg("-serial");
    qemu.arg("stdio");
    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}
