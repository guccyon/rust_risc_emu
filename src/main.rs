mod clike;
mod cpu_emu;

fn main() {
    clike::emulate();

    let rom = cpu_emu::Rom::new(vec![0b1111_000_000_00000]);
    if let Err(msg) = cpu_emu::CpuEmu::new(rom).run() {
        panic!(msg);
    }
}
