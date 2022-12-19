use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    time_left: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32
}

#[derive(Debug)]
struct Cost {
    ore_to_ore: u32,
    ore_to_clay: u32,
    ore_to_obsidian: u32,
    clay_to_obisidan: u32,
    ore_to_geode: u32,
    obsidian_to_geode: u32
}

fn parse_costs(line: &str) -> Cost {
    let split: Vec<&str> = line.trim().split(' ').collect();
    Cost {
        ore_to_ore: split[6].parse().unwrap(),
        ore_to_clay: split[12].parse().unwrap(),
        ore_to_obsidian: split[18].parse().unwrap(),
        clay_to_obisidan: split[21].parse().unwrap(),
        ore_to_geode: split[27].parse().unwrap(),
        obsidian_to_geode: split[30].parse().unwrap(),
    }
}

fn max_geode_helper(state: &State, costs: &Cost, dp: &mut HashMap<State, u32>, mut cur_max: u32) -> Option<u32> {
    if let Some(geode) = dp.get(state) {
        return if *geode > cur_max { Some(*geode) } else { None };
    }
    if state.time_left == 0 {
        return if state.geode > cur_max { Some(state.geode) } else { None };
    }
    if state.time_left * state.geode_robot +
       (state.time_left - 1) * state.time_left / 2 +
       state.geode < cur_max {
        return None;
    }
    let mut max_dp = 0u32;

    let do_nothing_state = State {
        time_left: state.time_left - 1,
        ore: state.ore + state.ore_robot,
        clay: state.clay + state.clay_robot,
        obsidian: state.obsidian + state.obsidian_robot,
        geode: state.geode + state.geode_robot,
        ..*state
    };

    if let Some(geode) = max_geode_helper(&do_nothing_state, costs, dp, cur_max) {
        max_dp = max_dp.max(geode);
        cur_max = cur_max.max(max_dp);
    }

    if state.ore >= costs.ore_to_ore {
        let build_ore_state = State {
            ore_robot: state.ore_robot + 1,
            ore: do_nothing_state.ore - costs.ore_to_ore,
            ..do_nothing_state
        };

        if let Some(geode) = max_geode_helper(&build_ore_state, costs, dp, cur_max) {
            max_dp = max_dp.max(geode);
            cur_max = cur_max.max(max_dp);
        }
    }

    if state.ore >= costs.ore_to_clay {
        let build_clay_state = State {
            clay_robot: state.clay_robot + 1,
            ore: do_nothing_state.ore - costs.ore_to_clay,
            ..do_nothing_state
        };

        if let Some(geode) = max_geode_helper(&build_clay_state, costs, dp, cur_max) {
            max_dp = max_dp.max(geode);
            cur_max = cur_max.max(max_dp);
        }
    }

    if state.ore >= costs.ore_to_obsidian && state.clay >= costs.clay_to_obisidan {
        let build_obsidian_state = State {
            obsidian_robot: state.obsidian_robot + 1,
            ore: do_nothing_state.ore - costs.ore_to_obsidian,
            clay: do_nothing_state.clay - costs.clay_to_obisidan,
            ..do_nothing_state
        };

        if let Some(geode) = max_geode_helper(&build_obsidian_state, costs, dp, cur_max) {
            max_dp = max_dp.max(geode);
            cur_max = cur_max.max(max_dp);
        }
    }

    if state.ore >= costs.ore_to_geode && state.obsidian >= costs.obsidian_to_geode {
        let build_geode_state = State {
            geode_robot: state.geode_robot + 1,
            ore: do_nothing_state.ore - costs.ore_to_geode,
            obsidian: do_nothing_state.obsidian - costs.obsidian_to_geode,
            ..do_nothing_state
        };

        if let Some(geode) = max_geode_helper(&build_geode_state, costs, dp, cur_max) {
            max_dp = max_dp.max(geode);
        }
    }

    dp.insert(*state, max_dp);
    return if max_dp > 0 { Some(max_dp) } else { None };
}

fn max_geode(costs: &Cost) -> u32 {
    let start_state = State {
        time_left: 32,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robot: 1,
        clay_robot: 0,
        obsidian_robot: 0,
        geode_robot: 0,
    };
    let mut dp: HashMap<State, u32> = HashMap::new();
    let cur_max = 0u32;
    match max_geode_helper(&start_state, costs, &mut dp, cur_max) {
        Some(geode) => geode,
        None => 0
    }
}

pub fn solve(input: &String) {
    let mut answer = 1u32;
    for (i, line) in input.lines().enumerate() {
        if i > 2 {
            break;
        }
        println!("Processing blueprint {i}");
        answer *= max_geode(&parse_costs(line));
    }
    println!("Answer: {answer}");
}