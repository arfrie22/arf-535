use std::{cell::{RefCell, RefMut}, io::{self, Write}, rc::Rc};

use simulator::memory::{ClockedMemory, DirectCache, FrontMemory, InnerMemory, Memory};

fn main() {
    println!("535 Memory Test");
    let raw_memory = Rc::new(RefCell::new(Memory::new()));
    let m = Rc::new(RefCell::new(ClockedMemory::<4, _>::new(raw_memory.clone(), None)));

    let raw_cache = Rc::new(RefCell::new(DirectCache::<2>::new()));
    let c = Rc::new(RefCell::new(ClockedMemory::<2, _>::new(raw_cache.clone(), Some(m.clone()))));
    
    
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        let input = input.trim_ascii().to_lowercase().to_owned();
        if input.starts_with(";") {
            continue;
        }

        let v: Vec<&str> = input.split_ascii_whitespace().collect();
        if v.len() > 0 {
            match v[0] {
                "f" => {
                    if v.len() < 4 {
                        println!("Usage: f <c | m> <requester id> <address>");
                    } else {
                        let mut selected: RefMut<'_, dyn FrontMemory> = match v[1] {
                            "c" => c.borrow_mut(),
                            "m" => m.borrow_mut(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        let requester_id = match v[2].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Requester ID");
                                continue
                            }
                        };

                        let address = match v[3].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Address");
                                continue
                            }
                        };

                        println!("Result: {:?}", selected.fetch(requester_id, address));
                    }
                },
                "s" => {
                    if v.len() < 5 {
                        println!("Usage: s <c | m> <requester id> <address> <value>");
                    } else {
                        let mut selected: RefMut<'_, dyn FrontMemory> = match v[1] {
                            "c" => c.borrow_mut(),
                            "m" => m.borrow_mut(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        let requester_id = match v[2].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Requester ID");
                                continue
                            }
                        };

                        let address = match v[3].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Address");
                                continue
                            }
                        };

                        let value = match v[4].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Value");
                                continue
                            }
                        };

                        println!("Result: {:?}", selected.store(requester_id, address, value));
                    }
                },
                "c" => {
                    if v.len() < 2 {
                        println!("Usage: c <c | m> <requester id>");
                    } else {
                        let mut selected: RefMut<'_, dyn FrontMemory> = match v[1] {
                            "c" => c.borrow_mut(),
                            "m" => m.borrow_mut(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        let requester_id = match v[2].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Requester ID");
                                continue
                            }
                        };

                        println!("Result: {:?}", selected.cancel(requester_id));
                    }
                }
                "a" => {
                    if v.len() < 2 {
                        println!("Usage: a <c | m>");
                    } else {
                        let res = match v[1] {
                            "c" => c.borrow_mut().current_request(),
                            "m" => m.borrow_mut().current_request(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        println!("Result: {:?}", res);
                    }
                }
                "r" => {
                    if v.len() < 2 {
                        println!("Usage: r <c | m> <address>");
                    } else {
                        let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
                            "c" => c.borrow_mut().inner(),
                            "m" => m.borrow_mut().inner(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        let address = match v[2].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Address");
                                continue
                            }
                        };

                        println!("Result: {:?}", selected.borrow_mut().read(address));
                    }
                }
                "rl" => {
                    if v.len() < 3 {
                        println!("Usage: rl <c | m> <line>");
                    } else {
                        let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
                            "c" => c.borrow_mut().inner(),
                            "m" => m.borrow_mut().inner(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        let line = match v[2].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Line");
                                continue
                            }
                        };

                        println!("Result: {:?}", selected.borrow_mut().read_line(line));
                    }
                }
                "cl" => {
                    if v.len() < 2 {
                        println!("Usage: cl <line>");
                    } else {
                        let line = match v[1].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Line");
                                continue
                            }
                        };

                        let v = raw_cache.borrow_mut();
                        println!("Result: {:?}  -  {:?}", v.raw_line(line), v.line_metadata(line));
                    }
                }
                "w" => {
                    if v.len() < 3 {
                        println!("Usage: w <c | m> <address> <value>");
                    } else {
                        let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
                            "c" => c.borrow_mut().inner(),
                            "m" => m.borrow_mut().inner(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        let address = match v[2].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Address");
                                continue
                            }
                        };

                        let value = match v[3].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Value");
                                continue
                            }
                        };

                        println!("Result: {:?}", selected.borrow_mut().write(address, value));
                    }
                }
                "h" => {
                    if v.len() < 2 {
                        println!("Usage: h <c | m> <address>");
                    } else {
                        let selected: Rc<RefCell<dyn InnerMemory>> = match v[1] {
                            "c" => c.borrow_mut().inner(),
                            "m" => m.borrow_mut().inner(),
                            c => {
                                println!("Unknown memory type: {}", c);
                                continue;
                            },
                        };

                        let address = match v[2].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Invalid Address");
                                continue
                            }
                        };

                        println!("Result: {:?}", selected.borrow_mut().has(address));
                    }
                }
                c => println!("Unknown command: {}", c),
            }
        }
    }
}