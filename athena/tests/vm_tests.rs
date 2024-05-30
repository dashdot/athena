#[cfg(test)]
mod tests {
    extern crate sp1_core;

    use sp1_core::runtime::Program;
    use athena::vm::{VM, interpreter::Instruction};

    #[test]
    fn test_vm_creation() {
        let vm = VM::new(1024);
        assert_eq!(vm.memory.load(0), 0);
    }

    #[test]
    fn test_add_instruction() {
        let mut vm = VM::new(1024);
        vm.registers[1] = 2;
        vm.registers[2] = 3;
        vm.execute_instruction(Instruction::ADD { rd: 0, rs1: 1, rs2: 2 });
        assert_eq!(vm.registers[0], 5);
    }

    // #[test]
    // fn test_load_program() {
    //     let program = Program {
    //         code: vec![0x00, 0x00, 0x00, 0x00], // Dummy code
    //         entry_point: 0,
    //     };
    //     let mut vm = VM::new(1024);
    //     vm.load_program(program);
    //     assert_eq!(vm.pc, 0);
    // }

    #[test]
  fn test_load_and_run_elf() {
      let mut vm = VM::new(1024);

      // Load the program from an ELF file
      let program = Program::from_elf("../examples/hello_world/target/riscv32imac-unknown-none-elf/release/test_program");
      // let program = Program::from_elf("../../sp1/examples/fibonacci/program/elf/riscv32im-succinct-zkvm-elf");

      // Load the program into the VM
      // vm.load_program(program);

      // Run the VM
      vm.run();

      // Add assertions here to verify the expected behavior
  }
}
