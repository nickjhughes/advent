use hashbrown::HashMap;
use std::{collections::VecDeque, fs};

pub fn part1() -> String {
    let input = get_input_file_contents();
    let mut modules = Modules::parse(&input);
    for _ in 0..1000 {
        modules.push_button();
    }
    modules.pulse_counts_product().to_string()
}

pub fn part2() -> String {
    let input = get_input_file_contents();
    buttons_presses_until_low_pulse_to_module(&input, "vr").to_string()
}

fn get_input_file_contents() -> String {
    fs::read_to_string("inputs/input20").expect("Failed to open input file")
}

fn buttons_presses_until_low_pulse_to_module(input: &str, module: &str) -> u64 {
    const SECOND_LAST_MODULES: [&str; 4] = ["pq", "fg", "dk", "fm"];
    let mut button_presses = Vec::new();
    for second_last_module in SECOND_LAST_MODULES {
        let mut modules = Modules::parse(input);
        modules.push_button_until_module_has_high_input(module, second_last_module);
        button_presses.push(modules.button_presses);
    }
    button_presses.iter().product::<u64>()
}

#[derive(Debug)]
struct Modules<'input> {
    modules: HashMap<&'input str, Module<'input>>,
    output: Option<&'input str>,
    button_presses: u64,
    pulse_counts: HashMap<Pulse, u64>,
    input_buffers: HashMap<&'input str, VecDeque<(&'input str, Pulse)>>,
}

#[derive(Debug, PartialEq)]
struct Module<'input> {
    ty: ModuleType<'input>,
    name: &'input str,
    outputs: Vec<&'input str>,
}

#[derive(Debug, PartialEq)]
enum ModuleType<'input> {
    Broadcast,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        last_inputs: HashMap<&'input str, Pulse>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Pulse {
    Low,
    High,
}

impl<'input> Modules<'input> {
    fn parse(input: &'input str) -> Self {
        let mut modules: HashMap<&str, Module> = input
            .lines()
            .map(|line| {
                let module = Module::parse(line);
                (module.name, module)
            })
            .collect();

        let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();
        for module in modules.values() {
            for output in module.outputs.iter() {
                inputs.entry(output).or_default().push(module.name);
            }
        }
        let mut output = None;
        for (module_name, inputs) in inputs.iter() {
            if !modules.contains_key(module_name) {
                output = Some(*module_name);
                continue;
            }
            let module = modules.get_mut(module_name).unwrap();
            if let ModuleType::Conjunction { last_inputs } = &mut module.ty {
                for input in inputs.iter() {
                    last_inputs.insert(input, Pulse::Low);
                }
            }
        }

        let mut input_buffers: HashMap<&str, VecDeque<(&str, Pulse)>> = HashMap::new();
        for module_name in modules.keys() {
            input_buffers.insert(module_name, VecDeque::new());
        }

        Modules {
            modules,
            output,
            button_presses: 0,
            pulse_counts: ([(Pulse::Low, 0), (Pulse::High, 0)])
                .into_iter()
                .collect::<HashMap<Pulse, u64>>(),
            input_buffers,
        }
    }

    fn pulse_counts_product(&self) -> u64 {
        self.pulse_counts.get(&Pulse::Low).unwrap() * self.pulse_counts.get(&Pulse::High).unwrap()
    }

    fn push_button_until_module_has_high_input(&mut self, conj_module: &str, input_module: &str) {
        loop {
            self.button_presses += 1;

            self.input_buffers
                .get_mut("broadcaster")
                .unwrap()
                .push_back(("button", Pulse::Low));
            *self.pulse_counts.get_mut(&Pulse::Low).unwrap() += 1;
            while !self.input_buffers.values().all(|pulses| pulses.is_empty()) {
                for module in self.modules.values_mut() {
                    if let Some((source, pulse)) =
                        self.input_buffers.get_mut(module.name).unwrap().pop_front()
                    {
                        let pulse_to_send = match &mut module.ty {
                            ModuleType::Broadcast => Some(pulse),
                            ModuleType::FlipFlop { on } => {
                                if pulse == Pulse::Low {
                                    if *on {
                                        *on = false;
                                        Some(Pulse::Low)
                                    } else {
                                        *on = true;
                                        Some(Pulse::High)
                                    }
                                } else {
                                    None
                                }
                            }
                            ModuleType::Conjunction { last_inputs } => {
                                if module.name == conj_module
                                    && *last_inputs.get(input_module).unwrap() == Pulse::High
                                {
                                    return;
                                }

                                last_inputs.insert(source, pulse);
                                if last_inputs.values().all(|p| *p == Pulse::High) {
                                    Some(Pulse::Low)
                                } else {
                                    Some(Pulse::High)
                                }
                            }
                        };
                        if let Some(pulse_to_send) = pulse_to_send {
                            *self.pulse_counts.get_mut(&pulse_to_send).unwrap() +=
                                module.outputs.len() as u64;
                            for output in module.outputs.iter() {
                                if self.output == Some(*output) {
                                    continue;
                                }
                                self.input_buffers
                                    .get_mut(*output)
                                    .unwrap()
                                    .push_back((module.name, pulse_to_send));
                            }
                        }
                    }
                }
            }
        }
    }

    fn push_button(&mut self) {
        self.button_presses += 1;

        self.input_buffers
            .get_mut("broadcaster")
            .unwrap()
            .push_back(("button", Pulse::Low));
        *self.pulse_counts.get_mut(&Pulse::Low).unwrap() += 1;
        while !self.input_buffers.values().all(|pulses| pulses.is_empty()) {
            for module in self.modules.values_mut() {
                if let Some((source, pulse)) =
                    self.input_buffers.get_mut(module.name).unwrap().pop_front()
                {
                    let pulse_to_send = match &mut module.ty {
                        ModuleType::Broadcast => Some(pulse),
                        ModuleType::FlipFlop { on } => {
                            if pulse == Pulse::Low {
                                if *on {
                                    *on = false;
                                    Some(Pulse::Low)
                                } else {
                                    *on = true;
                                    Some(Pulse::High)
                                }
                            } else {
                                None
                            }
                        }
                        ModuleType::Conjunction { last_inputs } => {
                            last_inputs.insert(source, pulse);
                            if last_inputs.values().all(|p| *p == Pulse::High) {
                                Some(Pulse::Low)
                            } else {
                                Some(Pulse::High)
                            }
                        }
                    };
                    if let Some(pulse_to_send) = pulse_to_send {
                        *self.pulse_counts.get_mut(&pulse_to_send).unwrap() +=
                            module.outputs.len() as u64;
                        for output in module.outputs.iter() {
                            if self.output == Some(*output) {
                                continue;
                            }
                            self.input_buffers
                                .get_mut(*output)
                                .unwrap()
                                .push_back((module.name, pulse_to_send));
                        }
                    }
                }
            }
        }
    }
}

impl<'input> Module<'input> {
    fn parse(input: &'input str) -> Self {
        let (type_and_name, outputs) = input.split_once(" -> ").unwrap();
        let outputs = outputs.split(',').map(|s| s.trim()).collect();

        if type_and_name.starts_with('%') {
            // Flip-flop
            Module {
                ty: ModuleType::FlipFlop { on: false },
                name: type_and_name.trim_start_matches('%'),
                outputs,
            }
        } else if type_and_name.starts_with('&') {
            // Conjuction
            Module {
                ty: ModuleType::Conjunction {
                    last_inputs: HashMap::new(),
                },
                name: type_and_name.trim_start_matches('&'),
                outputs,
            }
        } else {
            // Broadcast
            assert_eq!(type_and_name, "broadcaster");
            Module {
                ty: ModuleType::Broadcast,
                name: type_and_name,
                outputs,
            }
        }
    }
}

#[test]
fn test_parse_modules() {
    let input = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a\n";
    let modules = Modules::parse(input);

    assert_eq!(modules.modules.len(), 5);
    assert_eq!(
        modules.modules.get("broadcaster"),
        Some(&Module {
            ty: ModuleType::Broadcast,
            name: "broadcaster",
            outputs: vec!["a", "b", "c"]
        })
    );
    assert_eq!(
        modules.modules.get("a"),
        Some(&Module {
            ty: ModuleType::FlipFlop { on: false },
            name: "a",
            outputs: vec!["b"]
        })
    );
    assert_eq!(
        modules.modules.get("b"),
        Some(&Module {
            ty: ModuleType::FlipFlop { on: false },
            name: "b",
            outputs: vec!["c"]
        })
    );
    assert_eq!(
        modules.modules.get("c"),
        Some(&Module {
            ty: ModuleType::FlipFlop { on: false },
            name: "c",
            outputs: vec!["inv"]
        })
    );
    assert_eq!(
        modules.modules.get("inv"),
        Some(&Module {
            ty: ModuleType::Conjunction {
                last_inputs: {
                    let mut map = HashMap::new();
                    map.insert("c", Pulse::Low);
                    map
                }
            },
            name: "inv",
            outputs: vec!["a"]
        })
    );
}

#[test]
fn test_push_button() {
    {
        let input = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a\n";
        let mut modules = Modules::parse(input);

        modules.push_button();
        assert_eq!(
            modules.pulse_counts,
            ([(Pulse::Low, 8), (Pulse::High, 4)])
                .into_iter()
                .collect::<HashMap<Pulse, u64>>()
        );

        for _ in 0..999 {
            modules.push_button();
        }
        assert_eq!(
            modules.pulse_counts,
            ([(Pulse::Low, 8000), (Pulse::High, 4000)])
                .into_iter()
                .collect::<HashMap<Pulse, u64>>()
        );

        assert_eq!(modules.pulse_counts_product(), 32000000);
    }

    {
        let input = "broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output\n";
        let mut modules = Modules::parse(input);

        for _ in 0..1000 {
            modules.push_button();
        }
        assert_eq!(
            modules.pulse_counts,
            ([(Pulse::Low, 4250), (Pulse::High, 2750)])
                .into_iter()
                .collect::<HashMap<Pulse, u64>>()
        );
        assert_eq!(modules.pulse_counts_product(), 11687500);
    }
}
