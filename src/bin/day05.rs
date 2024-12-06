extern crate aoc2024;
use aoc2024::read_file_to_string;
use std::path::Path;

struct UpdateRule {
    x: i32,
    y: i32,
}
#[derive(Clone)]
struct Update {
    pages: Vec<i32>,
}

impl Update {
    // returns true if updates contains pages in a rule
    fn update_rule_applies(&self, rule: &UpdateRule) -> bool {
        return self.pages.contains(&rule.x) && self.pages.contains(&rule.y);
    }
    
    // returns true if an update rule is followed within this update
    pub fn update_rule_is_valid(&self, rule: &UpdateRule) -> bool {
        if self.update_rule_applies(rule) {
            let mut indx_x = 0;
            let mut indx_y = 0;
            for i in 0..self.pages.len() {
                let page = self.pages[i];
                if page == rule.x {
                    indx_x = i;
                }
                if page == rule.y {
                    indx_y = i;
                }
            }
            return indx_x < indx_y;
        } else {
            return true; // return true because update rules that do not apply are valid
        }
    }

    // returns true if an update follows all rules
    pub fn validate_update(&self, rules: &Vec<UpdateRule>) -> bool {
        for rule in rules {
            if !self.update_rule_is_valid(rule) {
                return false;
            }
        }
        return true;
    }
}

fn switch_pages(update: &mut Update, rule: &UpdateRule) {
    let mut indx_x = 0;
    let mut indx_y = 0;
    if !update.update_rule_applies(rule) {
        return;
    }
    // println!("Fixing Rule: {}|{}", rule.x, rule.y);
    for i in 0..update.pages.len() {
        let page = update.pages[i];
        if page == rule.x {
            indx_x = i;
        }
        if page == rule.y {
            indx_y = i;
        }
    }
    // println!{"switching {} and {}", update.pages[indx_x], update.pages[indx_y]};
    let x_tmp = update.pages[indx_x];
    update.pages[indx_x] = update.pages[indx_y];
    update.pages[indx_y] = x_tmp;
}

fn correct_update(update: &mut Update, rules: &Vec<UpdateRule>) -> bool {
    if !update.validate_update(&rules) { // if update is not valid
        // println!("Update {:?} is invalid: ",update.pages);
        for i in 0..rules.len() {            
            if !update.update_rule_is_valid(&rules[i]) {
                // println!("Broken Rule: {}|{}", rules[i].x, rules[i].y);
                switch_pages(update, &rules[i]);
                // println!("After switch {:?}: ",update.pages);
                break;
            }
        }
        return correct_update(update, rules)
    } else {
        println!("Update {:?} is valid: ",update.pages);
        return true;
    }
}

fn main() {
    let path = Path::new("inputs/day05.txt");
    let s = read_file_to_string(path);
    let mut rules: Vec<UpdateRule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    // parse the string into update rules and updates
    let lines:Vec<&str> = s.split("\n").collect();
    for line in lines {
        if line.contains("|") { // is update rule
            let strx = line.split("|").collect::<Vec<&str>>()[0];
            let stry = line.split("|").collect::<Vec<&str>>()[1];
            rules.push(
                UpdateRule {
                    x: strx.trim().parse::<i32>().unwrap(),
                    y: stry.trim().parse::<i32>().unwrap(),
                }
            );
        } else if line.contains(",") { // is a list
            let mut v: Vec<i32> = Vec::new();
            let pages_str: Vec<&str> = line.split(",").collect();
            for page in pages_str {
                v.push(
                    page.trim().parse::<i32>().unwrap()
                );
            }
            updates.push(Update {pages: v});
        }
    }
    println!("Parsed {} rules and {} updates", rules.len(), updates.len());
    let mut valid_count = 0;
    let mut sum = 0;
    for update in updates.iter() {
        if update.validate_update(&rules) {
            println!("{:?}", update.pages);
            let middle_index = (update.pages.len() / 2);
            println!("middle value: {}", update.pages[middle_index]);
            sum += update.pages[middle_index];
            valid_count += 1;
        }
    }
    println!("Sum of middle pages from {} valid updates: {}",valid_count, sum);
    // part 2
    sum = 0;
    for update in updates.iter_mut() {
        if update.validate_update(&rules) {
            continue;
        }
        correct_update(update, &rules);
        let middle_index = (update.pages.len() / 2);
        println!("middle value: {}", update.pages[middle_index]);
        sum += update.pages[middle_index];
    }
    // for update in updates.iter() {
    //     if update.validate_update(&rules) {
    //         println!("{:?}", update.pages);
    //         let middle_index = (update.pages.len() / 2);
    //         println!("middle value: {}", update.pages[middle_index]);
    //         sum += update.pages[middle_index];
    //         valid_count += 1;
    //     }
    // }
    println!("Sum of middle pages from {} valid updates: {}",valid_count, sum);

}