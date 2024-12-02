fn main() {
    let mut reader = csv::Reader::from_path("input.csv").unwrap();
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    let mut r = reader.records();
    while let Some(result) = r.next() {
        let record = result.unwrap();
        println!("{:?}", record);
        let nums: Vec<&str> = record[0].split("   ").collect();
        let val1: i32 = nums[0].parse().unwrap();
        let val2: i32 = nums[1].parse().unwrap();
        list1.push(val1);
        list2.push(val2);
    }

    list1.sort();
    list2.sort();


    let mut total_diff = 0;
    for (index, val) in list1.iter().enumerate() {
        let diff = (list1[index] - list2[index]).abs();
        total_diff = total_diff + diff;
    }

    println!("{:?}", list1.len());
    println!("{:?}", list2.len());
    println!("{:?}", total_diff);

    return
}
