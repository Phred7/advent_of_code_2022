use std::fs;

pub fn day_1_main() {
    let file_lines: String = read_file();
    let mut elf_total: i64 = 0;
    let mut largest_elf: i64 = 0;
    let mut mid_elf: i64 = 0;
    let mut small_elf: i64 = 0;
    for line in file_lines.lines() {
        if line.is_empty() {
            if elf_total > largest_elf {
                small_elf = mid_elf;
                mid_elf = largest_elf;
                largest_elf = elf_total;
            } else if elf_total > mid_elf {
                small_elf = mid_elf;
                mid_elf = elf_total;
            } else if elf_total > small_elf {
                small_elf = elf_total;
            }
            println!("Largest Total: {}", largest_elf);
            println!("2nd Largest Total: {}", mid_elf);
            println!("3rd Largest Total: {}\n", small_elf);
            elf_total = 0;
        } else {
            elf_total = elf_total + line.parse::<i64>().expect("Failed to parse string");
        }
    }
    println!("Largest Total: {}", largest_elf);
    println!("2nd Largest Total: {}", mid_elf);
    println!("3rd Largest Total: {}", small_elf);
    println!("Sum: {}", largest_elf+mid_elf+small_elf);
}

fn read_file() -> String {
    let data = fs::read_to_string("day_1_input.txt").expect("Unable to read file");
    return data;
}