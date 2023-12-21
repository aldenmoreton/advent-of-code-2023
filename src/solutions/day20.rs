use std::collections::{HashMap, VecDeque};
use num::integer::lcm;

type ModuleMap<'a> = HashMap<String, (Module, Vec<String>)>;

#[derive(Clone, PartialEq, Debug)]
enum Pulse {
    Low,
    High
}

#[derive(Debug, PartialEq)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>)
}

fn input_generator(input: &str) -> ModuleMap {
    let mut map = input
        .lines()
        .map(|line| {
            let (module, destinations) = line.split_once(" -> ").unwrap();
            let (name, module) = match module.chars().nth(0).unwrap() {
                'b' => ("broadcaster", Module::Broadcaster),
                '%' => (&module[1..], Module::FlipFlop(false)),
                '&' => (&module[1..], Module::Conjunction(HashMap::new())),
                _ => panic!("This is not a valid module")
            };
            let destinations = destinations.split(", ").map(|dest| dest.to_string()).collect::<Vec<_>>();
            (name.to_string(), (module, destinations))
        })
        .collect::<HashMap<_,_>>();

    let conjunctions = map
        .iter()
        .filter(|(_, (module, _))| matches!(module, Module::Conjunction(_)))
        .map(|(name, _)| name)
        .cloned()
        .collect::<Vec<_>>();

    for conjunction in conjunctions {
        let inputs = map
            .iter()
            .filter(|(_, (_, outputs))| outputs.contains(&conjunction))
            .map(|(name, _)| (name.clone(), Pulse::Low))
            .collect::<HashMap<_, _>>();

        let insertion = map.get_mut(&conjunction).unwrap();
        insertion.0 = Module::Conjunction(inputs);
    }

    map
}

#[aoc(day20, part1)]
fn part_one(input: &str) -> usize {
    let mut module_map = input_generator(input);

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    for _ in 0..1_000 {
        let mut pulses = VecDeque::new();
        low_pulse_count += 1;
        pulses.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

        while let Some((from_module, curr_pulse, to_module)) = pulses.pop_front() {
            let Some((curr_module, destinations)) = module_map.get_mut(&to_module) else {
                continue
            };
            match curr_module {
                Module::Broadcaster => {
                    for next_module in destinations.iter() {
                        low_pulse_count += 1;
                        pulses.push_back((to_module.clone(), Pulse::Low, next_module.clone()))
                    }
                },
                Module::FlipFlop(state) => {
                    if curr_pulse == Pulse::High {
                        ()
                    } else {
                        *state = !*state;
                        let (sending_pulse, counter): (_, &mut usize) = state
                            .then_some((Pulse::High, &mut high_pulse_count))
                            .unwrap_or((Pulse::Low, &mut low_pulse_count));
                        for next_module in destinations.iter() {
                            *counter += 1;
                            pulses.push_back((to_module.clone(), sending_pulse.clone(), next_module.clone()))
                        }
                    }
                },
                Module::Conjunction(prev_states) => {
                    let prev_state = prev_states.get_mut(&from_module).unwrap();
                    *prev_state = curr_pulse;


                    let (sending_pulse, counter) = if prev_states.values().all(|prev_pulse| *prev_pulse == Pulse::High) {
                        (Pulse::Low, &mut low_pulse_count)
                    } else {
                        (Pulse::High, &mut high_pulse_count)
                    };
                    for next_module in destinations.iter() {
                        *counter += 1;
                        pulses.push_back((to_module.clone(), sending_pulse.clone(), next_module.clone()))
                    }
                }
            }
        }
    }

    low_pulse_count * high_pulse_count
}

fn find_lcms(mut lcm_modules: Vec<&str>, mut module_map: ModuleMap) -> Vec<usize> {
    let mut lcms = Vec::new();

    let mut button_press_count = 0;
    'button_pushes: loop {
        button_press_count += 1;
        let mut pulses = VecDeque::new();
        pulses.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

        while let Some((from_module, curr_pulse, to_module)) = pulses.pop_front() {
            lcm_modules.retain(|curr_module| {
                if from_module == *curr_module && curr_pulse == Pulse::High {
                    lcms.push(button_press_count);
                    false
                } else {
                    true
                }
            });
            if lcm_modules.len() == 0 { break 'button_pushes }

            let Some((curr_module, destinations)) = module_map.get_mut(&to_module) else {
                continue
            };
            match curr_module {
                Module::Broadcaster => {
                    for next_module in destinations.iter() {
                        pulses.push_back((to_module.clone(), Pulse::Low, next_module.clone()))
                    }
                },
                Module::FlipFlop(state) => {
                    if curr_pulse == Pulse::High {
                        ()
                    } else {
                        *state = !*state;
                        let sending_pulse = state
                            .then_some(Pulse::High)
                            .unwrap_or(Pulse::Low);
                        for next_module in destinations.iter() {
                            pulses.push_back((to_module.clone(), sending_pulse.clone(), next_module.clone()))
                        }
                    }
                },
                Module::Conjunction(prev_states) => {
                    let prev_state = prev_states.get_mut(&from_module).unwrap();
                    *prev_state = curr_pulse;

                    let sending_pulse = if prev_states.values().all(|prev_pulse| *prev_pulse == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for next_module in destinations.iter() {
                        pulses.push_back((to_module.clone(), sending_pulse.clone(), next_module.clone()))
                    }
                }
            }
        }
    }

    lcms
}

#[aoc(day20, part2)]
fn part_two(input: &str) -> usize {
    let module_map: ModuleMap = input_generator(input);

    let lcm_modules = vec!["pm", "mk", "pk", "hf"];
    let lcms = find_lcms(lcm_modules, module_map);

    lcms
        .into_iter()
        .fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "};
        let result = part_one(input);
        assert_eq!(result, 32_000_000);
    }

    #[test]
    fn part1_2() {
        let input = indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "};
        let result = part_one(input);
        assert_eq!(result, 11_687_500);
    }
}
