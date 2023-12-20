use itertools::Itertools;
use num::Integer;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone, Copy)]
struct Pulse {
    src: &'static str,
    high: bool,
    dest: &'static str,
}
impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('-')?;
        f.write_str(if self.high { "high" } else { "low" })?;
        f.write_str("-> ")?;
        f.write_str(self.dest)
    }
}
#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'static str, bool>),
}
impl ModuleType {
    fn is_conjunction(&self) -> bool {
        matches!(self, ModuleType::Conjunction(_))
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: &'static str,
    t: ModuleType,
    dests: Vec<&'static str>,
}
impl Module {
    fn on_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        // if self.name == "bb"{println!("{pulse}")};
        match &mut self.t {
            ModuleType::Broadcast => self
                .dests
                .iter()
                .map(|d| Pulse {
                    src: pulse.dest,
                    high: pulse.high,
                    dest: d,
                })
                .collect(),
            ModuleType::FlipFlop(on) => {
                if pulse.high {
                    vec![]
                } else {
                    *on = !*on;
                    self.dests
                        .iter()
                        .map(|d| Pulse {
                            src: pulse.dest,
                            high: *on,
                            dest: d,
                        })
                        .collect()
                }
            }
            ModuleType::Conjunction(inputs) => {
                inputs.insert(pulse.src, pulse.high);
                let high = !inputs.iter().all(|(_, high)| *high);
                self.dests
                    .iter()
                    .map(|d| Pulse {
                        src: pulse.dest,
                        high,
                        dest: d,
                    })
                    .collect()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Factory {
    modules: HashMap<&'static str, Module>,
    high_count: usize,
    low_count: usize,
}
fn read_input(input: &'static str) -> Factory {
    let mut modules: HashMap<&'static str, Module> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (module, dests) = l.split_once("->").unwrap();
            let dests: Vec<_> = dests.split(',').map(str::trim).collect();
            let module = module.trim();
            let (name, t) = match module.chars().next().unwrap() {
                '%' => (&module[1..], ModuleType::FlipFlop(false)),
                '&' => (&module[1..], ModuleType::Conjunction(HashMap::new())),
                _ => {
                    assert_eq!(module, "broadcaster");
                    ("broadcaster", ModuleType::Broadcast)
                }
            };
            (name, Module { name, t, dests })
        })
        .collect();

    // completings conjunctions
    let conjunctions: Vec<_> = modules
        .iter()
        .filter_map(|(name, m)| {
            if m.t.is_conjunction() {
                Some(*name)
            } else {
                None
            }
        })
        .collect();
    for conjunction in conjunctions {
        let sources: Vec<_> = modules
            .iter()
            .filter_map(|(name, m)| {
                if m.dests.contains(&conjunction) {
                    Some(*name)
                } else {
                    None
                }
            })
            .collect();
        modules.entry(conjunction).and_modify(|m| {
            if let ModuleType::Conjunction(inputs) = &mut m.t {
                for src in sources {
                    inputs.insert(src, false);
                }
            }
        });
    }

    Factory {
        modules,
        high_count: 0,
        low_count: 0,
    }
}

impl Factory {
    fn push_button(&mut self) {
        let mut pulses: VecDeque<Pulse> = VecDeque::new();
        pulses.push_back(Pulse {
            src: "button",
            high: false,
            dest: "broadcaster",
        });
        while let Some(p) = pulses.pop_front() {
            if p.high {
                self.high_count += 1;
            } else {
                self.low_count += 1;
            }
            if let Some(module) = self.modules.get_mut(p.dest) {
                for np in module.on_pulse(p) {
                    pulses.push_back(np);
                }
            }
        }
    }
    fn direct_predecessors(&self, module: &'static str) -> Vec<&'static str> {
        self.modules
            .iter()
            .filter_map(|(n, m)| {
                if m.dests.contains(&module) {
                    Some(*n)
                } else {
                    None
                }
            })
            .unique()
            .collect_vec()
    }

    fn get_period(&self, module: &'static str) -> usize {
        let period = match self.modules.get(module).unwrap().t {
            ModuleType::Broadcast => 1,
            ModuleType::Conjunction(_) => {
                // note as we saw that the conjunctions are always gathering otherwise disjoints graph, we can compute inner periods
                let predecessors = self.direct_predecessors(module);

                predecessors
                    .into_iter()
                    .map(|p| {
                        let predecessors = self.predecessors(p);
                        let modules = predecessors
                            .into_iter()
                            .filter_map(|p| self.modules.get(&p))
                            .cloned()
                            .map(|m| (m.name, m))
                            .collect();
                        let local_factory = Factory {
                            modules,
                            high_count: 0,
                            low_count: 0,
                        };
                        local_factory.get_period(p)
                    })
                    .reduce(|p1, p2| p1.lcm(&p2))
                    .unwrap_or(1)
            }
            ModuleType::FlipFlop(_) => {
                let mut factory_clone = self.clone();
                let mut i = 1usize;
                factory_clone.push_button();

                while !factory_clone.all_off() {
                    i += 1;
                    factory_clone.push_button();
                }
                i
            }
        };
        // println!("{module} period {period}");
        period
    }

    fn predecessors(&self, name: &'static str) -> HashSet<&'static str> {
        let mut result = HashSet::new();
        let mut current = vec![name];

        while !current.is_empty() {
            let mut new_current = HashSet::new();
            for c in current.iter() {
                result.insert(*c);
                for n in self.modules.iter().filter(|(_, m)| m.dests.contains(c)).map(|(n, _)| n) {
                    new_current.insert(n);
                }
            }
            current = new_current.into_iter().filter(|nc| !result.contains(*nc)).copied().collect();
        }
        result
    }
    fn start(self) -> usize {
        const END: &str = "rx";
        let predecessors = self.direct_predecessors(END);
        assert_eq!(1, predecessors.len());
        let predecessor = predecessors[0];

        // predecessor is a cunjunction, as well as its predecessor
        // thus we want all the predecessors predecessors to be off at the same time
        // ie. we want the period of each sub graph (which are disjoint, apart for the last conjunction)
        self.get_period(predecessor)
    }

    fn count_off(&self) -> usize {
        self.modules
            .values()
            .filter(|m| match &m.t {
                ModuleType::Broadcast => true,
                ModuleType::FlipFlop(on) => !*on,
                ModuleType::Conjunction(inputs) => inputs.values().all(|on| !*on),
            })
            .count()
    }
    fn all_off(&self) -> bool {
        self.modules.values().all(|m| match &m.t {
            ModuleType::Broadcast => true,
            ModuleType::FlipFlop(on) => !*on,
            ModuleType::Conjunction(inputs) => inputs.values().all(|on| !*on),
        })
    }
    fn warm(mut self) -> usize {
        let mut i = 1;
        self.push_button();
        while i < 1000 && !self.all_off() {
            self.push_button();
            i += 1;
        }
        if i < 1000 {
            let loops = 1000 / i;
            self.high_count *= loops;
            self.low_count *= loops;
            for _ in 0..1000 % i {
                self.push_button();
            }
        }

        self.high_count * self.low_count
    }
}

impl Display for Factory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (broadcast, flip, conjunct) =
            self.modules.iter().fold((0, 0, 0), |(broadcast, flip, conjunct), (_, m)| match m.t {
                ModuleType::Broadcast => (broadcast + 1, flip, conjunct),
                ModuleType::FlipFlop(_) => (broadcast, flip + 1, conjunct),
                ModuleType::Conjunction(_) => (broadcast, flip, conjunct + 1),
            });
        f.write_fmt(format_args!(
            "Factory[off : {}/{}, br{} , %{} &{}]",
            self.count_off(),
            self.modules.len(),
            broadcast,
            flip,
            conjunct
        ))
    }
}

pub fn warm_factory() {
    let factory = read_input(include_str!("../resources/day20_modules.txt"));
    let pulse_product = factory.warm();
    println!("pulse product {pulse_product}");
    let factory = read_input(include_str!("../resources/day20_modules.txt"));
    let necessary_pushes = factory.start();
    println!("necessary_pushes {necessary_pushes}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    #[test]
    fn aoc_example_works() {
        let input = indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "};
        let mut factory = read_input(input);
        println!("{factory:?}");
        assert!(factory.all_off());
        factory.push_button();
        assert!(factory.all_off());
        let mut factory = read_input(input);
        assert_eq!(32000000, factory.warm());

        let mut factory = read_input(indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "});
        assert_eq!(11687500, factory.warm());
    }
}
