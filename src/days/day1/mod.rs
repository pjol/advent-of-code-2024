use crate::utils::get_input_path;

pub fn part1() -> i32 {
    let path = get_input_path(1);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();
    let mut r = reader.records();

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    while let Some(result) = r.next() {
        let record = result.unwrap();
        let nums: Vec<&str> = record[0].split("   ").collect();
        let val1: i32 = nums[0].parse().unwrap();
        let val2: i32 = nums[1].parse().unwrap();
        list1.push(val1);
        list2.push(val2);
    }

    list1.sort();
    list2.sort();


    let mut total_diff = 0;
    for (index, _) in list1.iter().enumerate() {
        let diff = (list1[index] - list2[index]).abs();
        total_diff = total_diff + diff;
    }

    return total_diff
}



pub fn part2() -> i32 {
    let path = get_input_path(1);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    let mut r = reader.records();
    while let Some(result) = r.next() {
        let record = result.unwrap();
        let nums: Vec<&str> = record[0].split("   ").collect();
        let val1: i32 = nums[0].parse().unwrap();
        let val2: i32 = nums[1].parse().unwrap();
        list1.push(val1);
        list2.push(val2);
    }

    list1.sort();
    list2.sort();


    let mut total_score = 0;
    for (_, val) in list1.iter().enumerate() {
        let pos = list2.iter().position(|&x| x == *val);
        if pos.is_none() {
            continue;
        }
        let i = pos.unwrap();
        let mut count= 0;
        for n in list2.clone().into_iter().skip(i) {
            if n != *val {
                break
            }
            count = count + 1;
        }
        total_score = total_score + (count * val);
    }

    return total_score
}
