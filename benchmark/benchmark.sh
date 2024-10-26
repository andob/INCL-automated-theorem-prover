#!/bin/bash

# Generate a file random_formulas.txt with 300 (30x10) random formulas
# Generate 10 formulas with 1 operator, 10 with 2 operators, ..., 10 with 30 operators
cargo run -- --generate-random-formulas 30 10

# Benchmark and write results in data.csv
# For each problem, resulting proof tree size will be measured
cargo bench -- tests::generate_csv -- WithoutModality
cp data.csv data_propositional_theoretical.csv

# Calculate theoretical complexity
cargo run -- --headless

# For each problem, number of CPU instructions will be measured
cargo bench -- tests::generate_csv -- --cpu WithoutModality
cp data.csv data_propositional_cpu.csv

# Calculate CPU complexity
cargo run -- --headless

# For each problem, total allocated memory will be measured
cargo bench -- tests::generate_csv -- --ram WithoutModality
cp data.csv data_propositional_ram.csv

# Calculate RAM complexity
cargo run -- --headless

# Do the same for Intuitionistic logic
cargo bench -- tests::generate_csv -- IntuitionisticLogic
cp data.csv data_intuitionistic_theoretical.csv
cargo run -- --headless
cargo bench -- tests::generate_csv -- --cpu IntuitionisticLogic
cp data.csv data_intuitionistic_cpu.csv
cargo run -- --headless
cargo bench -- tests::generate_csv -- --ram IntuitionisticLogic
cp data.csv data_intuitionistic_ram.csv
cargo run -- --headless
