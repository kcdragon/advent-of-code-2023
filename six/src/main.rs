fn main() {
    let times = [61, 70, 90, 66];
    let distances = [643, 1184, 1362, 1041];

    let mut answer = 1;

    for i in (0..times.len()) {
        let max_time = times[i];
        let distance_record = distances[i];

        let mut count = 0;

        for time in (0..max_time) { // no need to check max time since it will always travel a distance of 0
            let velocity = time; // in millimeters per millisecond
            let time_left = max_time - time;

            let distance_travelled = velocity * time_left;

            if (distance_travelled >= distance_record) {
                count = count + 1;
            }
        }

        answer = answer * count;
    }

    println!("answer is {}", answer);
}
