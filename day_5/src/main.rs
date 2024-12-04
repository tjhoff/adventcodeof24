use std::{cmp::Ordering, collections::HashMap, fs::read_to_string, usize};

struct Rules {
    // The key in here must be _after_ all of the items in the value.
    before: HashMap<usize, Vec<usize>>,
    // The key in here must be _before_ all the items in the value.
    after: HashMap<usize, Vec<usize>>,
}

fn parse_rules(rule_strings: Vec<&str>) -> Rules {
    let mut before = HashMap::new();
    let mut after = HashMap::new();

    let rule_numbers = rule_strings
        .into_iter()
        .map(|x| x.split("|").map(|c| c.parse::<usize>().unwrap()));

    for mut nums in rule_numbers {
        let first_num: usize = nums.next().unwrap();
        let second_num: usize = nums.next().unwrap();
        let before_nums = before.entry(second_num).or_insert(Vec::new());
        before_nums.push(first_num);

        let after_nums = after.entry(first_num).or_insert(Vec::new());
        after_nums.push(second_num);
    }

    // println!("{before:?}");
    // println!("{after:?}");

    return Rules {
        before: before,
        after: after,
    };
}

fn parse_page_updates(update_strings: Vec<&str>) -> Vec<Vec<usize>> {
    let update_list = update_strings
        .into_iter()
        .map(|x| x.split(",").map(|c| c.parse::<usize>().unwrap()).collect())
        .collect();
    return update_list;
}

fn element_is_in_correct_order(
    u: &usize,
    before: Vec<&usize>,
    after: Vec<&usize>,
    rules: &Rules,
) -> bool {
    let must_be_before_element = rules.before.get(&u);
    let must_be_after_element = rules.after.get(&u);

    let before_is_good = before
        .into_iter()
        .all(|e| !must_be_after_element.unwrap_or(&Vec::new()).contains(&e));
    let after_is_good = after
        .into_iter()
        .all(|e| !must_be_before_element.unwrap_or(&Vec::new()).contains(&e));
    return before_is_good && after_is_good;
}

fn check_if_update_good(update: &Vec<usize>, rules: &Rules) -> bool {
    for (index, page) in update.iter().enumerate() {
        let before: Vec<&usize> = update[..index].into_iter().collect();
        let after: Vec<&usize> = update[index..].into_iter().collect();
        if !element_is_in_correct_order(page, before, after, rules) {
            return false;
        };
    }
    return true;
}

fn fix_bad_update(update: &Vec<usize>, rules: &Rules) -> Vec<usize> {
    let order_update = |a: &usize, b: &usize| -> Ordering {
        if rules.after.get(a).unwrap_or(&Vec::new()).contains(b) {
            return Ordering::Less;
        }
        if rules.before.get(a).unwrap_or(&Vec::new()).contains(b) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    };
    let mut cloned = update.clone();
    cloned.sort_by(order_update);
    return cloned.to_vec();
}

fn main() {
    let filename = "data.txt";
    let file_text = read_to_string(filename).unwrap();
    let mut lines = file_text.lines();
    let rules_list: Vec<&str> = lines.by_ref().take_while(|l| l != &"").collect();
    let rules = parse_rules(rules_list);
    let rest: Vec<&str> = lines.filter(|line| line != &"").collect();
    let updates = parse_page_updates(rest);
    let (good_updates, bad_updates): (Vec<_>, Vec<_>) = updates
        .into_iter()
        .partition(|u| check_if_update_good(u, &rules));

    let get_middle_element = |up: Vec<usize>| {
        let len = up.len();
        let middle = len / 2;
        let result = up.get(middle).unwrap();
        return *result;
    };

    let result: usize = good_updates.into_iter().map(get_middle_element).sum();
    println!("{result}");

    let p2_result: usize = bad_updates
        .into_iter()
        .map(|up| fix_bad_update(&up, &rules))
        .map(get_middle_element)
        .sum();

    println!("{p2_result}");
}
