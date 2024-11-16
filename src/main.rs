mod assembler;
mod vm;

use assembler::Assembler;
use vm::cpu::CPU;
use vm::VMConfig;

fn main()
{
    // Create VM configuration
    let config = VMConfig::new(256, true);

    // Create a new VM instance with config
    let mut vm = CPU::new(config);

    // Example program using assembly syntax
    let assembly_code = r#"
        ; Initialize values
        MOV r0, 5        ; Load 5 into r0
        MOV r1, 3        ; Load 3 into r1
        
        ; Basic arithmetic
        ADD r0, r1       ; r0 = r0 + r1 (8)
        OUT r0           ; Should print 8
        
        ; Memory operations
        STORE r0, 0x50   ; Store r0 at address 0x50
        LOAD r3, 0x50    ; Load from address 0x50 into r3
        OUT r3           ; Should print 8

        HALT            ; Stop program
    "#;

    println!("Running test program");
    println!("Expected outputs: 8, 8, 8\n");

    // Create assembler and assemble the code
    let mut assembler = Assembler::new();
    match assembler.assemble(assembly_code) {
        Ok(bytecode) => {
            println!("Generated bytecode: {:02X?}", bytecode);
            vm.load_program(&bytecode);

            // Run the program
            match vm.run() {
                Ok(_) => println!("\nProgram completed successfully"),
                Err(e) => {
                    eprintln!("\nProgram failed during execution: {}", e);
                    vm.dump_state();
                }
            }
        }
        Err(e) => {
            eprintln!("\nAssembly failed: {}", e);
            // Print the labels for debugging assembly errors
            eprintln!("\nLabel addresses:");
            for (label, addr) in assembler.labels() {
                eprintln!("{}: 0x{:02X}", label, addr);
            }
        }
    }
}
