use std::{collections::{HashMap, HashSet}, error::Error, fs, str::FromStr};

pub fn count_provinces() -> String {
    let districts = read_districts("district.json").unwrap();

    let mut result = String::new();
    let mut res = vec![0; districts.len()];
    for (batch, data) in districts {
        let num_provinces = count_connected_components(data);
        println!("批次{}：属于{}个省", batch, num_provinces);
        res[usize::from_str(&batch).unwrap() - 1] = num_provinces
    }

    for item in res {
        if 0 == result.len() {
            result.push_str(&item.to_string());
        } else {
            result.push_str(&format!(",{}", item));
        }
    }
    println!("{}", result);
    result
}

type DistrictData = HashMap<String, Vec<String>>;

fn read_districts(filename: &str) -> Result<HashMap<String, DistrictData>, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let data: HashMap<String, DistrictData> = serde_json::from_str(&content)?;
    Ok(data)
}

fn count_connected_components(mapping: DistrictData) -> usize {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for (node, neighbors) in mapping {
        graph.entry(node.clone()).or_insert_with(HashSet::new);
        for neighbor in neighbors {
            graph.entry(node.clone()).or_insert_with(HashSet::new).insert(neighbor.clone());
            graph.entry(neighbor.clone()).or_insert_with(HashSet::new).insert(node.clone());
        }
    }

    let mut visited = HashSet::new();
    let mut count = 0;
    for node in graph.keys() {
        if !visited.contains(node) {
            count += 1;
            dfs(node, &graph, &mut visited);
        }
    }

    count
}

fn dfs(node: &String, graph: &HashMap<String, HashSet<String>>, visited: &mut HashSet<String>) {
    let mut stack = vec![node.clone()];
    while let Some(curr) = stack.pop() {
        if !visited.insert(curr.clone()) {
            continue;
        }

        if let Some(neighbors) = graph.get(&curr) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    stack.push(neighbor.clone());
                }
            }
        }
    }
}