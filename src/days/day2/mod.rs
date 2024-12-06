use crate::utils::get_input_path;

pub fn part1() -> i32 {
    let path = get_input_path(2);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();

    let mut r = reader.records();

    let mut total = 0;
    while let Some(result) = r.next() {

        let record = result.unwrap();
        let row: Vec<&str> = record[0].split(" ").collect();
        let mut nums: Vec<i32> = Vec::new();
        for n in row {
            let num: i32 = n.parse().unwrap();
            nums.push(num)
        }

        if is_safe(nums) { total += 1 };
    }

    return total
}

pub fn part2() -> i32 {
    let path = get_input_path(2);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .unwrap();

    let mut r = reader.records();

    let mut total = 0;
    while let Some(result) = r.next() {

        let record = result.unwrap();
        let row: Vec<&str> = record[0].split(" ").collect();
        let mut nums: Vec<i32> = Vec::new();
        for n in row {
            let num: i32 = n.parse().unwrap();
            nums.push(num)
        }

        let mut safe = false;
        for (index, _) in nums.iter().enumerate() {
            let mut newnums = nums.clone();
            newnums.remove(index);
            if is_safe(newnums) {
                safe = true;
                break
            }
        }
        if safe { total += 1 }
    }

    return total
}

fn is_safe(nums: Vec<i32>) -> bool {
    let mut safe = true;
    let pos = nums[0] < nums[nums.len() - 1];
    for (index, num) in nums.iter().enumerate() {
        if index == 0 { continue }
        let prev = nums[index - 1];
        if pos != (prev < *num)
        || *num - prev == 0
        || (*num - prev).abs() > 3
        {
            safe = false;
            break;
        }
    }
    return safe
}