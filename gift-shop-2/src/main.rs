fn part_1() {
    let input = "52-75,71615244-71792700,89451761-89562523,594077-672686,31503-39016,733-976,1-20,400309-479672,458-635,836793365-836858811,3395595155-3395672258,290-391,5168-7482,4545413413-4545538932,65590172-65702074,25-42,221412-256187,873499-1078482,118-154,68597355-68768392,102907-146478,4251706-4487069,64895-87330,8664371543-8664413195,4091-5065,537300-565631,77-115,83892238-83982935,6631446-6694349,1112-1649,7725-9776,1453397-1493799,10240-12328,15873-20410,1925-2744,4362535948-4362554186,3078725-3256936,710512-853550,279817-346202,45515-60928,3240-3952";
    // let input = "1-20";
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let range: Vec<&str> = input.split(",").collect();

    let mut count = 0;

    // Iterate through each range
    for r in range {
        let lr: Vec<&str> = r.split("-").collect();

        let l = lr.first().unwrap().parse::<u64>().unwrap();
        let r = lr.last().unwrap().parse::<u64>().unwrap();

        for i in l..(r + 1) {
            let is = i.to_string();
            if is.len() % 2 != 0 {
                continue;
            }
            let (left, right) = is.split_at(is.len() / 2);
            if left == right {
                count += i;
            }
        }
    }

    println!("Sum: {}", count);
}

fn is_invalid(input: &str) -> bool {
    let len = input.len();
    // Only consider substrings that divide the string evenly and are not the whole string
    for sub_len in 1..=(len / 2) {
        if len % sub_len != 0 {
            continue;
        }
        let candidate = &input[0..sub_len];
        if input
            .chars()
            .collect::<Vec<_>>()
            .chunks(sub_len)
            .all(|chunk| chunk.iter().collect::<String>() == candidate)
        {
            return true;
        }
    }
    false
}

fn main() {
    let input = "52-75,71615244-71792700,89451761-89562523,594077-672686,31503-39016,733-976,1-20,400309-479672,458-635,836793365-836858811,3395595155-3395672258,290-391,5168-7482,4545413413-4545538932,65590172-65702074,25-42,221412-256187,873499-1078482,118-154,68597355-68768392,102907-146478,4251706-4487069,64895-87330,8664371543-8664413195,4091-5065,537300-565631,77-115,83892238-83982935,6631446-6694349,1112-1649,7725-9776,1453397-1493799,10240-12328,15873-20410,1925-2744,4362535948-4362554186,3078725-3256936,710512-853550,279817-346202,45515-60928,3240-3952";
    // // let input = "1-20";
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    // // let input = "1188511880-1188511890";
    // //
    let range: Vec<&str> = input.split(",").collect();

    let mut count = 0;

    // Iterate through each range
    for r in range {
        let lr: Vec<&str> = r.split("-").collect();

        let l = lr.first().unwrap().parse::<u64>().unwrap();
        let r = lr.last().unwrap().parse::<u64>().unwrap();

        for i in l..(r + 1) {
            let result = is_invalid(&i.to_string());
            // println!("Result for {} is {}", i, result);
            if result {
                count += i;
            }
        }
    }

    println!("Sum is {}", count);
}
