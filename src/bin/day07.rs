extern crate aoc2024;
use aoc2024::read_file_to_string;
use std::{ path::Path};

#[derive(Debug)]
enum Ops {
    ADD,
    MUL,
    CONCAT,
}

fn generate_ops_combinations_part_1(eqn: &Vec<i64>) -> Vec<Vec<Ops>> {
    let num_ops = eqn.len() - 2;
    let num_combos: i32 = 2 << num_ops;
    // println!("Num ops: {}, num combos:  {}", num_ops, num_combos);
    let mut combos: Vec<Vec<Ops>> = Vec::new();
    for i in 0..num_combos {
        let bin = i as u64;
        let mut combo = Vec::new();
        for pos in 0..num_ops {
            let tmp = i >> pos;
            if tmp % 2 == 0 { // is odd
                combo.push(Ops::ADD);
            } else {
                combo.push(Ops::MUL);
            }
        }
        // println!("{:?}", combo);
        combos.push(combo);
    }
    return combos;
}

fn gen_perms(l: i64, num_ops: i64, v: Vec<Vec<i64>>, idx: i64) -> Vec<Vec<i64>>{
    if idx == l {
        return v.to_vec();
    } else {
        if idx == 0 { // initialize
            let mut vec = Vec::new();
            for i in 0..num_ops {
                let mut tmp: Vec<i64> = Vec::new();
                tmp.push(i);
                vec.push(tmp);
            }
            return gen_perms(l, num_ops, vec, idx + 1);
        } else {
            let mut new_vec = Vec::new();
            for perm in v {
                for i in 0..num_ops {
                    let mut new_perm = perm.clone();
                    // println!("New perm: {:?}", new_perm);
                    new_perm.push(i);
                    new_vec.push(new_perm);
                }
            }
            return gen_perms(l, num_ops, new_vec, idx + 1);
        }        
    }
}

fn populate_perm(vec: &Vec<Vec<i64>>) -> Vec<Vec<Ops>> {
    let mut perm_op: Vec<Vec<Ops>> = Vec::new();
    for i in 0..vec.len() {
        let mut perm: Vec<Ops> = Vec::new();
        for j in 0..vec[i].len() {
            let op = match vec[i][j] {
                0 => Ops::ADD,
                1 => Ops::MUL,
                2 => Ops::CONCAT,
                _ => Ops::ADD,
            };
            perm.push(op);
        }
        perm_op.push(perm);
    }
    return perm_op;
}

fn eval_op(x: i64, y: i64, op: &Ops) -> i64 {
    return match op {
        Ops::ADD => {x + y},
        Ops::MUL => {x * y},
        Ops::CONCAT => {
            (x.to_string() + &y.to_string()).parse::<i64>().unwrap()
        }
    }
}

fn check_combo(eqn: &Vec<i64>, combo: &Vec<Ops>) -> bool{
    let test_val = eqn[0];
    let mut val = 0;
    // print!("{} ? ", test_val);
    for i in 2..eqn.len() {
        if i == 2 {
            val = eval_op(eqn[i - 1], eqn[i], &combo[i - 2]);
            // print!("{} {:?} {}", eqn[i - 1], combo[i - 2],eqn[i]);
        } else {
            val = eval_op(val, eqn[i], &combo[i - 2]);
            // print!("{:?} {}", combo[i - 2], eqn[i]);
        }
    }
    // println!("");
    val == test_val
}

fn main() {
    let path: &Path = Path::new("inputs/day07.txt");
    let s = read_file_to_string(path);
    let lines: Vec<&str> = s.split("\n").collect();
    let mut all_eqns: Vec<Vec<i64>> = Vec::new();
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }
        let bc = line.split(":").collect::<Vec<&str>>()[0];
        let ac: &str = line.split(":").collect::<Vec<&str>>()[1];
        let mut eqn: Vec<i64> = Vec::new();
        let test_val = bc.parse::<i64>().unwrap();
        eqn.push(test_val);
        let vals = ac.split(' ').collect::<Vec<&str>>();
        // println!("Vals: {:?}", vals);
        for val in  vals{
            if val == "" {
                continue;
            }
            // println!("val: {}", val.trim());
            eqn.push(val.trim().parse::<i64>().unwrap());
        }
        all_eqns.push(eqn);
    }
    let mut sum: i64 = 0;
    for eqn in all_eqns.iter() {
        let combos = generate_ops_combinations_part_1(&eqn);
        for combo in combos {
            if check_combo(&eqn, &combo) {
                sum += eqn[0];
                break;
            }
        }
        // break;
    }
    println!("Sum: {}", sum);
    println!("PART 2");
    // part2
    let mut max_ops = 0;
    let mut min_ops = 100000; // big num
    let mut perms: Vec<Vec<Vec<i64>>> = Vec::new();
    for eqn in all_eqns.iter() {
        if eqn.len() - 2 > max_ops {
            max_ops = eqn.len() - 2;
        }
        if eqn.len() - 2 < min_ops {
            min_ops = eqn.len() - 2;
        }
    }
    for i in 0..max_ops+1 {
        let mut v: Vec<Vec<i64>> = Vec::new();
        v = gen_perms(i as i64, 3, v, 0);
        // println!("Ops: {} {:?}",i, v);
        perms.push(v);
    }
    let mut perm_ops:  Vec<Vec<Vec<Ops>>> = Vec::new();
    for i in 0..perms.len() {
        let perm_op = populate_perm(&perms[i]);
        perm_ops.push(perm_op);
    }
    sum = 0;
    for eqn in all_eqns.iter() {
        let num_ops = eqn.len() - 2;
        let perms = &perm_ops[num_ops];
        for combo in perms {
            if check_combo(&eqn, &combo) {
                sum += eqn[0];
                break;
            }
        }
        // break;
    }
    println!("Sum: {}", sum);
}