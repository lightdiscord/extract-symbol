use clap::Clap;
use xmas_elf::ElfFile;
use xmas_elf::sections::SectionData;
use xmas_elf::symbol_table::Entry;
use std::io::{self, Read, Write};
use std::error::Error;

#[derive(Clap)]
struct Opt {
    symbol: String
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the symbol requested by the user.
    let opt = Opt::parse();

    // Read all the content of the standard input.
    let mut file = io::stdin();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let file = ElfFile::new(&buffer)?;

    // Search the `.symtab` section for the requested symbol.
    let section = file.find_section_by_name(".symtab")
        .ok_or("Missing .symtab section")?;

    let table = match section.get_data(&file)? {
        SectionData::SymbolTable64(table) => table,
        _ => Err("Wrong section data type for .symtab")?
    };

    let symbol = table.into_iter()
        .find(|entry| entry.get_name(&file) == Ok(&opt.symbol))
        .ok_or("Symbol not found")?;

    // Slice the bytes corresponding to the requested symbol.
    let section = file.section_header(symbol.shndx())?;
    let start = symbol.value() - section.offset();
    let end = start + symbol.size();
    let data = section.raw_data(&file);
    let data = &data[(start as usize)..(end as usize)];

    io::stdout().write_all(data)?;

    Ok(())
}
