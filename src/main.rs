use std::process::exit;

use pagemap::*;
use clap::Parser;
use colored::Colorize;

// Compile with:
// CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --target=x86_64-unknown-linux-gnu

// Docker run:
// docker run --rm -it -v /Users/echostone/Documents/ICS2023/0xc/rpm/target/x86_64-unknown-linux-gnu/debug:/home --privileged --platform linux/amd64 ubuntu bin/bash

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "the PID of the process to dump from")]
    pid: u64,

    #[arg(help = "the virtual address of the target pagemap entry")]
    addr: Option<String>,

    #[arg(short, long, help = "dump the entire memory region")]
    all: bool,
}

macro_rules! ensure {
    ($result: expr) => {
        $result.unwrap_or_else(|err| {
            eprintln!("{}", err.to_string().red());
            exit(-1);
        })
    };
}

fn dump_maps_hdr() {
    println!(
        "{}",
        format!(
            "{:<64} {:<8} {:<10} {:<10} {:<10} {}",
            "Memory Region (size)",
            "Perm",
            "Offset",
            "Device",
            "Inode",
            "Path",
        ).green()
    );
}

fn dump_maps_entry(entry: &MapsEntry) {
    println!(
        "{:<64} {:<8} {:<10} {:<10} {:<10} {}", 
        // entry.memory_region().to_string(), 
        entry.vma().to_string(),
        entry.permissions().to_string(), 
        entry.offset().to_string(), 
        entry.device_numbers().to_string(), 
        entry.inode().to_string(), 
        entry.path().unwrap_or(""),
    )
}

fn dump_pmaps_hdr() {
    println!(
        "{}",
        format!(
            "{:<18}  {:<12} {:<8} {:<8} {:<8} {:<8} {:<10}",
            "VirtAddr",
            "PFN",
            "Present",
            "Swapped",
            "FileMap",
            "Shared",
            "SoftDirty",
        ).green()
    );
}

fn dump_pmaps_entry(addr: u64, entry: &PageMapEntry) {
    println!(
        "0x{:0>16x}  {:<12} {:<8} {:<8} {:<8} {:<8} {:<10}",
        addr,
        entry.pfn().map(|pfn| format!("0x{:<10x}", pfn)).unwrap_or("-".to_string()),
        if entry.present() { "[*]" } else { "[ ]" },
        if entry.swapped() { "[*]" } else { "[ ]" },
        if entry.file_mapped() { "[*]" } else { "[ ]" },
        if !entry.exclusively_mapped() { "[*]" } else { "[ ]" },
        if entry.soft_dirty() { "[*]" } else { "[ ]" },
    )
}

fn main() {
    let cli = Cli::parse();

    let pid = cli.pid;
    let entries = ensure!(pagemap(pid));
    let page_sz = ensure!(page_size());
    let addr = cli.addr
        .map(|addr| {
            let addr = if addr.starts_with("0x") { &addr[2..] } else { &addr };
            ensure!(u64::from_str_radix(addr, 16))
        });
    let entry = addr
        .as_ref()
        .and_then(|addr| {
            entries.iter().find(|(entry, _)| entry.vma().contains(*addr))
        });

    match (addr, entry) {
        (Some(addr), Some((maps_entry, pmap_entry))) => {
            // dump_maps_hdr();
            // dump_maps_entry(maps_entry);
            let region = maps_entry.vma();
            if cli.all {
                dump_pmaps_hdr();
                for idx in 0..pmap_entry.len() as u64 {
                    dump_pmaps_entry(region.start_address() + idx * page_sz, pmap_entry.get(idx as usize).unwrap());
                }
            } else {
                let idx = (addr - region.start_address()) / page_sz;
                dump_pmaps_hdr();
                dump_pmaps_entry(region.start_address() + idx * page_sz, pmap_entry.get(idx as usize).unwrap());
            }
        }
        _ => {
            if let Some(addr) = addr {
                println!("{}", format!("[Warning] Address 0x{:x} is not mapped..\n", addr).yellow());
            }
            // Simply dump `maps`
            dump_maps_hdr();
            for (maps_entry, _) in entries {
                dump_maps_entry(&maps_entry);
            }
        }
    }
}