use std::{cell::{RefCell, RefMut}, io::{self, Write}, rc::Rc};

use simulator::{enums::{Condition, Register}, instruction::Instruction, memory::{ClockedMemory, DirectCache, FrontMemory, InnerMemory, Memory}, pipeline::PipelineOutter, Simulator, SimulatorState};

const DATA_M_CYCLES: usize = 2;
const PROG_M_CYCLES: usize = 2;

const DATA_C_CYCLES: usize = 1;
const PROG_C_CYCLES: usize = 1;

fn print_reg(state: &Rc<RefCell<SimulatorState>>) {
    println!("{:?}", state.borrow().registers);
}

fn print_states(state: &Rc<RefCell<SimulatorState>>) {
    let state_ref = state.borrow();
    print!("Fetch ");
    if state_ref.fetch_state.is_some() {
        println!("state: {:?}", state_ref.fetch_state);
    } else {
        println!("result: {:?}", state_ref.fetch_result);
    }

    print!("Decode ");
    if state_ref.decode_state.is_some() {
        println!("state: {:?}", state_ref.decode_state);
    } else {
        println!("result: {:?}", state_ref.decode_result);
    }

    print!("Execute ");
    if state_ref.execute_state.is_some() {
        println!("state: {:?}", state_ref.execute_state);
    } else {
        println!("result: {:?}", state_ref.execute_result);
    }

    print!("Memory ");
    if state_ref.memory_state.is_some() {
        println!("state: {:?}", state_ref.memory_state);
    } else {
        println!("result: {:?}", state_ref.memory_result);
    }

    println!("Writeback state: {:?}", state_ref.writeback_state);
    println!("--------")
}

fn main() {
    println!("535 Pipeline Test");
    let raw_program_memory = Rc::new(RefCell::new(Memory::new()));
    let program_memory = Rc::new(RefCell::new(ClockedMemory::<PROG_M_CYCLES, _>::new(raw_program_memory.clone(), None)));

    let raw_program_cache = Rc::new(RefCell::new(DirectCache::<1>::new()));
    let program_cache = Rc::new(RefCell::new(ClockedMemory::<PROG_C_CYCLES, _>::new(raw_program_cache.clone(), Some(program_memory.clone()))));

    let raw_data_memory = Rc::new(RefCell::new(Memory::new()));
    let data_memory = Rc::new(RefCell::new(ClockedMemory::<DATA_M_CYCLES, _>::new(raw_data_memory.clone(), None)));

    let raw_data_cache = Rc::new(RefCell::new(DirectCache::<2>::new()));
    let data_cache = Rc::new(RefCell::new(ClockedMemory::<DATA_C_CYCLES, _>::new(raw_data_cache.clone(), Some(data_memory.clone()))));

    raw_data_memory.borrow_mut().write(0, 42).unwrap();
    
    raw_program_memory.borrow_mut().write(0, Instruction::IntegerLoadData { rx: Register::R1, label: 0 }.into()).unwrap();
    raw_program_memory.borrow_mut().write(1, Instruction::IntegerStoreData { rx: Register::R1, label: 1 }.into()).unwrap();
    raw_program_memory.borrow_mut().write(2, Instruction::AddUnsignedInteger { c: false, rx: Register::R1, ry: Register::R1, rz: Register::R1 }.into()).unwrap();
    raw_program_memory.borrow_mut().write(3, Instruction::IntegerLoadLow { rx: Register::R3, value: 1 }.into()).unwrap();

    raw_program_memory.borrow_mut().write(4, Instruction::IntegerLoadHigh { rx: Register::R3, value: 0 }.into()).unwrap();


    raw_program_memory.borrow_mut().write(5, Instruction::IntegerLoadData { rx: Register::R2, label: 0 }.into()).unwrap();
    raw_program_memory.borrow_mut().write(6, Instruction::AddUnsignedInteger { c: false, rx: Register::R2, ry: Register::R2, rz: Register::R3 }.into()).unwrap();
    raw_program_memory.borrow_mut().write(7, Instruction::IntegerStoreData { rx: Register::R2, label: 0 }.into()).unwrap();
    
    raw_program_memory.borrow_mut().write(8, Instruction::ImmediateJump { l: false, condition: Condition::AlwaysTrue, label: 5 }.into()).unwrap();
    raw_program_memory.borrow_mut().write(9, Instruction::IntegerStoreData { rx: Register::R4, label: 0 }.into()).unwrap();

    let mut simulator = Simulator::new(raw_program_memory, raw_data_memory, raw_program_cache, raw_data_cache, program_cache, data_cache);
    // let simulator = Simulator::new(raw_program_memory, raw_data_memory, program_memory, data_memory);
    
    let state = simulator.get_state();
    // state.borrow_mut().single_instruction_pipeline = true;
    print_reg(&state);
    print_states(&state);

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        simulator.cycle();
        print_reg(&state);
        print_states(&state);
    }
    // loop {
    //     let mut input = String::new();
    //     print!("> ");
    //     io::stdout().flush().unwrap();

    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");


    //     let input = input.trim_ascii().to_lowercase().to_owned();
    //     if input.starts_with(";") {
    //         continue;
    //     }

    //     let v: Vec<&str> = input.split_ascii_whitespace().collect();
    //     if v.len() > 0 {
    //         match v[0] {
    //             "f" => {
    //                 if v.len() < 4 {
    //                     println!("Usage: f <c | m> <requester id> <address>");
    //                 } else {
    //                     let mut selected: RefMut<'_, dyn FrontMemory> = match v[1] {
    //                         "c" => c.borrow_mut(),
    //                         "m" => m.borrow_mut(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     let requester_id = match v[2].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Requester ID");
    //                             continue
    //                         }
    //                     };

    //                     let address = match v[3].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Address");
    //                             continue
    //                         }
    //                     };

    //                     println!("Result: {:?}", selected.fetch(requester_id, address));
    //                 }
    //             },
    //             "s" => {
    //                 if v.len() < 5 {
    //                     println!("Usage: s <c | m> <requester id> <address> <value>");
    //                 } else {
    //                     let mut selected: RefMut<'_, dyn FrontMemory> = match v[1] {
    //                         "c" => c.borrow_mut(),
    //                         "m" => m.borrow_mut(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     let requester_id = match v[2].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Requester ID");
    //                             continue
    //                         }
    //                     };

    //                     let address = match v[3].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Address");
    //                             continue
    //                         }
    //                     };

    //                     let value = match v[4].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Value");
    //                             continue
    //                         }
    //                     };

    //                     println!("Result: {:?}", selected.store(requester_id, address, value));
    //                 }
    //             },
    //             "c" => {
    //                 if v.len() < 2 {
    //                     println!("Usage: c <c | m> <requester id>");
    //                 } else {
    //                     let mut selected: RefMut<'_, dyn FrontMemory> = match v[1] {
    //                         "c" => c.borrow_mut(),
    //                         "m" => m.borrow_mut(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     let requester_id = match v[2].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Requester ID");
    //                             continue
    //                         }
    //                     };

    //                     println!("Result: {:?}", selected.cancel(requester_id));
    //                 }
    //             }
    //             "a" => {
    //                 if v.len() < 2 {
    //                     println!("Usage: a <c | m>");
    //                 } else {
    //                     let res = match v[1] {
    //                         "c" => c.borrow_mut().current_request(),
    //                         "m" => m.borrow_mut().current_request(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     println!("Result: {:?}", res);
    //                 }
    //             }
    //             "r" => {
    //                 if v.len() < 2 {
    //                     println!("Usage: r <c | m> <address>");
    //                 } else {
    //                     let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
    //                         "c" => c.borrow_mut().inner(),
    //                         "m" => m.borrow_mut().inner(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     let address = match v[2].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Address");
    //                             continue
    //                         }
    //                     };

    //                     println!("Result: {:?}", selected.borrow_mut().read(address));
    //                 }
    //             }
    //             "rl" => {
    //                 if v.len() < 3 {
    //                     println!("Usage: rl <c | m> <line>");
    //                 } else {
    //                     let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
    //                         "c" => c.borrow_mut().inner(),
    //                         "m" => m.borrow_mut().inner(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     let line = match v[2].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Line");
    //                             continue
    //                         }
    //                     };

    //                     println!("Result: {:?}", selected.borrow_mut().read_line(line));
    //                 }
    //             }
    //             "cl" => {
    //                 if v.len() < 2 {
    //                     println!("Usage: cl <line>");
    //                 } else {
    //                     let line = match v[1].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Line");
    //                             continue
    //                         }
    //                     };

    //                     let v = raw_cache.borrow_mut();
    //                     println!("Result: {:?}  -  {:?}", v.raw_line(line), v.line_metadata(line));
    //                 }
    //             }
    //             "w" => {
    //                 if v.len() < 3 {
    //                     println!("Usage: w <c | m> <address> <value>");
    //                 } else {
    //                     let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
    //                         "c" => c.borrow_mut().inner(),
    //                         "m" => m.borrow_mut().inner(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     let address = match v[2].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Address");
    //                             continue
    //                         }
    //                     };

    //                     let value = match v[3].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Value");
    //                             continue
    //                         }
    //                     };

    //                     println!("Result: {:?}", selected.borrow_mut().write(address, value));
    //                 }
    //             }
    //             "h" => {
    //                 if v.len() < 2 {
    //                     println!("Usage: h <c | m> <address>");
    //                 } else {
    //                     let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
    //                         "c" => c.borrow_mut().inner(),
    //                         "m" => m.borrow_mut().inner(),
    //                         c => {
    //                             println!("Unknown memory type: {}", c);
    //                             continue;
    //                         },
    //                     };

    //                     let address = match v[2].parse() {
    //                         Ok(v) => v,
    //                         Err(_) => {
    //                             println!("Invalid Address");
    //                             continue
    //                         }
    //                     };

    //                     println!("Result: {:?}", selected.borrow_mut().has(address));
    //                 }
    //             }
    //             c => println!("Unknown command: {}", c),
    //         }
    //     }
    // }
}