use std::ops;

#[macro_use]
extern crate lazy_static;

use aoc2022::input::read_lines;
use regex::Regex;

fn main() {
    let blueprints = match read_lines("inputs/day19.txt")
        .map(|l| parse_blueprint(&l))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(blueprints) => blueprints,
        Err(err) => {
            println!("Can't parse blueprints: {}", err);
            return;
        }
    };

    println!(
        "Part 1: {}",
        blueprints
            .iter()
            .map(|b| b.quality_level(24))
            .sum::<isize>()
    );
    println!(
        "Part 2: {}",
        blueprints
            .iter()
            .take(3)
            .map(|b| b.max_geodes(32))
            .product::<isize>()
    );
}

fn parse_blueprint(str: &str) -> Result<Blueprint, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    }

    RE.captures(str)
        .and_then(|captures| -> Option<Blueprint> {
            match (
                captures.get(1).unwrap().as_str().parse(),
                captures.get(2).unwrap().as_str().parse(),
                captures.get(3).unwrap().as_str().parse(),
                captures.get(4).unwrap().as_str().parse(),
                captures.get(5).unwrap().as_str().parse(),
                captures.get(6).unwrap().as_str().parse(),
                captures.get(7).unwrap().as_str().parse(),
            ) {
                (
                    Ok(id),
                    Ok(ore_robot_ore_cost),
                    Ok(clay_robot_ore_cost),
                    Ok(obsidian_robot_ore_cost),
                    Ok(obsidian_robot_clay_cost),
                    Ok(geode_robot_ore_cost),
                    Ok(geode_robot_obsidian_cost),
                ) => Some(Blueprint::new(
                    id,
                    Resources {
                        ore: ore_robot_ore_cost,
                        clay: 0,
                        obsidian: 0,
                    },
                    Resources {
                        ore: clay_robot_ore_cost,
                        clay: 0,
                        obsidian: 0,
                    },
                    Resources {
                        ore: obsidian_robot_ore_cost,
                        clay: obsidian_robot_clay_cost,
                        obsidian: 0,
                    },
                    Resources {
                        ore: geode_robot_ore_cost,
                        clay: 0,
                        obsidian: geode_robot_obsidian_cost,
                    },
                )),
                _ => None,
            }
        })
        .ok_or(format!("Couldn't parse blueprint out of '{}'", str))
}

#[derive(Clone, Debug)]
struct Resources {
    ore: isize,
    clay: isize,
    obsidian: isize,
}

impl Resources {
    fn produced_by(robot: Robot) -> Resources {
        match robot {
            Robot::Ore => Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
            },
            Robot::Clay => Resources {
                ore: 0,
                clay: 1,
                obsidian: 0,
            },
            Robot::Obsidian => Resources {
                ore: 0,
                clay: 0,
                obsidian: 1,
            },
            Robot::Geode => Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
            },
        }
    }
}

impl ops::Add<&Resources> for &Resources {
    type Output = Resources;

    fn add(self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

impl ops::Sub<&Resources> for &Resources {
    type Output = Resources;

    fn sub(self, other: &Resources) -> Resources {
        Resources {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
        }
    }
}

impl ops::Mul<isize> for &Resources {
    type Output = Resources;

    fn mul(self, multiplicand: isize) -> Resources {
        Resources {
            ore: self.ore * multiplicand,
            clay: self.clay * multiplicand,
            obsidian: self.obsidian * multiplicand,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Blueprint {
    id: isize,
    ore_cost: Resources,
    clay_cost: Resources,
    obsidian_cost: Resources,
    geode_cost: Resources,
    max_ore_needed: isize,
    max_clay_needed: isize,
    max_obsidian_needed: isize,
}

impl Blueprint {
    fn new(
        id: isize,
        ore_cost: Resources,
        clay_cost: Resources,
        obsidian_cost: Resources,
        geode_cost: Resources,
    ) -> Blueprint {
        let all_costs = [&ore_cost, &clay_cost, &obsidian_cost, &geode_cost];
        let max_ore_needed = all_costs.iter().map(|c| c.ore).max().unwrap();
        let max_clay_needed = all_costs.iter().map(|c| c.clay).max().unwrap();
        let max_obsidian_needed = all_costs.iter().map(|c| c.obsidian).max().unwrap();

        Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
            max_ore_needed,
            max_clay_needed,
            max_obsidian_needed,
        }
    }

    fn quality_level(&self, minutes: isize) -> isize {
        self.id * self.max_geodes(minutes)
    }

    fn max_geodes(&self, minutes: isize) -> isize {
        let mut stack = Vec::new();
        let mut max = 0;

        stack.push(State::initial(minutes));

        while let Some(state) = stack.pop() {
            if state.minutes_left == 0 {
                if state.geodes > max {
                    max = state.geodes
                }
            } else {
                if state.theoretical_max_geodes() > max {
                    for next_state in self.next_states(&state) {
                        stack.push(next_state);
                    }
                }
            }
        }

        max
    }

    fn next_states(&self, state: &State) -> Vec<State> {
        lazy_static! {
            static ref ALL_ROBOTS: [Robot; 4] =
                [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];
        }
        let mut states = vec![];

        states.extend(
            ALL_ROBOTS
                .iter()
                .filter_map(|robot| self.try_build_if_needed(state, *robot)),
        );
        if states.is_empty() {
            states.push(state.wait_until_end());
        }

        states
    }

    fn try_build_if_needed(&self, state: &State, robot: Robot) -> Option<State> {
        let robot_needed = match robot {
            Robot::Ore => self.max_ore_needed > state.robots.ore,
            Robot::Clay => self.max_clay_needed > state.robots.clay,
            Robot::Obsidian => self.max_obsidian_needed > state.robots.obsidian,
            Robot::Geode => true,
        };
        if !robot_needed {
            return None;
        }
        let (cost, keep_at_least_minutes_left_after) = match robot {
            Robot::Ore => (&self.ore_cost, 2),
            Robot::Clay => (&self.clay_cost, 3),
            Robot::Obsidian => (&self.obsidian_cost, 2),
            Robot::Geode => (&self.geode_cost, 1),
        };
        state.try_build(robot, cost, keep_at_least_minutes_left_after)
    }
}

#[derive(Debug)]
struct State {
    minutes_left: isize,
    robots: Resources,
    resources: Resources,
    geode_robots: isize,
    geodes: isize,
}

impl State {
    fn initial(minutes: isize) -> State {
        State {
            minutes_left: minutes,
            robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
            },
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
            },
            geode_robots: 0,
            geodes: 0,
        }
    }

    fn try_build(
        &self,
        robot: Robot,
        robot_cost: &Resources,
        keep_at_least_minutes_left_after: isize,
    ) -> Option<State> {
        let minutes_needed = match self.minutes_needed_to_build(robot_cost) {
            Some(minutes_needed) => minutes_needed,
            _ => return None,
        };

        if self.minutes_left - keep_at_least_minutes_left_after < minutes_needed {
            return None;
        }

        let built_resources_robots = Resources::produced_by(robot);
        let built_geode_robots = match robot {
            Robot::Geode => 1,
            _ => 0,
        };

        Some(State {
            minutes_left: self.minutes_left - minutes_needed,
            robots: &self.robots + &built_resources_robots,
            resources: &(&self.resources + &(&self.robots * minutes_needed)) - robot_cost,
            geode_robots: self.geode_robots + built_geode_robots,
            geodes: self.geodes + self.geode_robots * minutes_needed,
        })
    }

    fn wait_until_end(&self) -> State {
        State {
            minutes_left: 0,
            robots: self.robots.clone(),
            resources: &self.resources + &(&self.robots * self.minutes_left),
            geode_robots: self.geode_robots,
            geodes: self.geodes + self.geode_robots * self.minutes_left,
        }
    }

    fn theoretical_max_geodes(&self) -> isize {
        let mut max_geodes = self.geodes;
        for minute in 0..self.minutes_left {
            max_geodes += self.geode_robots + minute;
        }
        max_geodes
    }

    fn minutes_needed_to_build(&self, robot_cost: &Resources) -> Option<isize> {
        if robot_cost.ore != 0 && self.robots.ore == 0
            || robot_cost.clay != 0 && self.robots.clay == 0
            || robot_cost.obsidian != 0 && self.robots.obsidian == 0
        {
            return None;
        }

        let minutes_to_collect_resources = [
            (robot_cost.ore - self.resources.ore, self.robots.ore),
            (robot_cost.clay - self.resources.clay, self.robots.clay),
            (
                robot_cost.obsidian - self.resources.obsidian,
                self.robots.obsidian,
            ),
        ]
        .iter()
        .filter(|(resources_needed, _)| *resources_needed > 0)
        .map(|(resources_needed, robots)| (resources_needed + robots - 1) / robots)
        .max()
        .unwrap_or(0);

        Some(minutes_to_collect_resources + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Blueprint, Resources};

    #[test]
    fn examples_from_description() {
        let blueprint = Blueprint::new(
            1,
            Resources {
                ore: 4,
                clay: 0,
                obsidian: 0,
            },
            Resources {
                ore: 2,
                clay: 0,
                obsidian: 0,
            },
            Resources {
                ore: 3,
                clay: 14,
                obsidian: 0,
            },
            Resources {
                ore: 2,
                clay: 0,
                obsidian: 7,
            },
        );

        assert_eq!(9, blueprint.quality_level(24));

        let blueprint = Blueprint::new(
            2,
            Resources {
                ore: 2,
                clay: 0,
                obsidian: 0,
            },
            Resources {
                ore: 3,
                clay: 0,
                obsidian: 0,
            },
            Resources {
                ore: 3,
                clay: 8,
                obsidian: 0,
            },
            Resources {
                ore: 3,
                clay: 0,
                obsidian: 12,
            },
        );

        assert_eq!(24, blueprint.quality_level(24));
    }
}
