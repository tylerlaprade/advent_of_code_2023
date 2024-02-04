use std::collections::HashMap;

pub fn run() {
    let data = std::fs::read_to_string("inputs/19.txt").unwrap();
    let groups: Vec<&str> = data.split("\n\n").collect();
    println!("Groups: {:?}", groups);

    let mut first_group = HashMap::new();

    for line in groups[0].lines() {
        let parts: Vec<&str> = line.split('{').collect();
        let name = parts[0];
        let rest = parts[1].trim_end_matches('}');
        let mut vec = Vec::new();
        for part in rest.split(',') {
            let sub_parts: Vec<&str> = part.split(':').collect();
            let condition_string = sub_parts[0];
            if sub_parts.len() != 2 {
                vec.push((Condition::Always(Always {}), condition_string.to_string()));
                continue;
            }
            let field = condition_string.chars().next().unwrap().to_string();
            let comparator = condition_string.chars().nth(1).unwrap();
            let value: usize = condition_string[2..].parse().unwrap();
            let target = sub_parts[1];
            vec.push((
                match comparator {
                    '<' => Condition::LessThan(LessThan {
                        field: field,
                        value: value,
                    }),
                    '>' => Condition::GreaterThan(GreaterThan {
                        field: field,
                        value: value,
                    }),
                    _ => panic!("Unknown comparator: {}", comparator),
                },
                target.to_string(),
            ));
        }
        first_group.insert(name.to_string(), vec);
    }

    println!("Groups[1]: {:?}", groups[1]);
    let mut sum = 0;
    for line in groups[1].lines() {
        let parts: Vec<&str> = line.split('{').collect();
        let rest = parts[1].trim_end_matches('}');
        let mut widget = HashMap::new();
        for part in rest.split(',') {
            let parts: Vec<&str> = part.split('=').collect();
            widget.insert(parts[0].to_string(), parts[1].parse::<usize>().unwrap());
        }

        if is_accepted(&first_group, &widget) {
            sum += widget.get("x").unwrap()
                + widget.get("m").unwrap()
                + widget.get("a").unwrap()
                + widget.get("s").unwrap();
        }
    }
    println!("Sum: {}", sum);
}

fn is_accepted(
    first_group: &HashMap<String, Vec<(Condition, String)>>,
    widget: &HashMap<String, usize>,
) -> bool {
    let mut target = "in";
    loop {
        println!("Target: {}", target);
        for instruction in first_group.get(target).unwrap() {
            let condition = &instruction.0;
            target = &instruction.1;
            println!("Target: {}", target);
            match condition {
                Condition::Always(_always) => {}
                Condition::LessThan(less_than) => {
                    let field = &less_than.field;
                    let value = widget.get(field).unwrap();
                    if !less_than.is_valid(*value) {
                        continue;
                    }
                }
                Condition::GreaterThan(greater_than) => {
                    let field = &greater_than.field;
                    let value = widget.get(field).unwrap();
                    if !greater_than.is_valid(*value) {
                        println!("Field: {}, Value: {}", field, value);
                        continue;
                    }
                }
            }
            match target {
                "A" => {
                    return true;
                }
                "R" => {
                    return false;
                }
                _ => {
                    break;
                }
            }
        }
    }
}

enum Condition {
    Always(Always),
    LessThan(LessThan),
    GreaterThan(GreaterThan),
}

struct Always {}

impl Always {
    fn is_valid(&self, value: usize) -> bool {
        true
    }
}

struct LessThan {
    field: String,
    value: usize,
}

impl LessThan {
    fn is_valid(&self, value: usize) -> bool {
        value < self.value
    }
}

struct GreaterThan {
    field: String,
    value: usize,
}

impl GreaterThan {
    fn is_valid(&self, value: usize) -> bool {
        value > self.value
    }
}
