use std::collections::BTreeSet;
use std::collections::LinkedList;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let map = read_input()?;
    println!("Input = {:?}", &map);
    let mut visited = BTreeSet::new();

    let routes = explore(&map, &mut visited, false, "start");
    println!("part 1... {} routes", routes.len());

    let routes = explore(&map, &mut visited, true, "start");
    println!("part 2... {} routes", routes.len());
    Ok(())
}

fn explore<'a>(
    map: &'a Map,
    small_caves_visited: &mut BTreeSet<String>,
    can_visit_small_cave_again: bool,
    explore_from: &'a str,
) -> Vec<LinkedList<&'a str>> {
    if explore_from == "end" {
        return vec![LinkedList::from(["end"])];
    }
    match cave_size(explore_from) {
        CaveSize::Big => (),
        CaveSize::Small => {
            small_caves_visited.insert(explore_from.into());
        }
    };
    let paths_to = map.iter().filter_map(|(from, to)| {
        if from == explore_from {
            Some(to)
        } else if to == explore_from {
            Some(from)
        } else {
            None
        }
    });
    let mut routes_reaching_end = vec![];
    for destination in paths_to {
        if destination == explore_from {
            // prevent infinite loop
            continue;
        }
        let mut can_visit_small_cave_again = can_visit_small_cave_again;
        if small_caves_visited.contains(destination) {
            if destination == "start" || !can_visit_small_cave_again {
                continue;
            } else {
                can_visit_small_cave_again = false;
            }
        }
        for mut route in explore(
            map,
            &mut small_caves_visited.clone(),
            can_visit_small_cave_again,
            destination,
        ) {
            route.push_front(explore_from);
            routes_reaching_end.push(route);
        }
    }
    return routes_reaching_end;
}

type Map = Vec<Path>;
type Path = (String, String);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CaveSize {
    Small,
    Big,
}

fn cave_size(s: &str) -> CaveSize {
    let c = s.chars().next().unwrap();
    if c.is_ascii_uppercase() {
        CaveSize::Big
    } else {
        CaveSize::Small
    }
}

fn read_input<'a>() -> Result<Map> {
    let mut paths = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let (from, to) = line.split_once('-').ok_or("no split char")?;
        paths.push((from.into(), to.into()));
    }
    Ok(paths)
}
