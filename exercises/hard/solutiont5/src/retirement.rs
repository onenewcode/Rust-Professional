pub fn retire_time(time: &str, tp: &str) -> String {
    let birth: Vec<f32> = time.split('-').map(|x| x.parse::<f32>().unwrap()).collect();
    let sex = tp.find("男").is_some();
    if sex {
        // 是男性
        if birth[0] < 1965.0 {
            return format!("{}-{:0>2},60,0", birth[0] + 60.0, birth[1]);
        } else {
            // 获取
            let difference = (((birth[0] - 1965.0) * 12.0 + birth[1]) / 4.0).ceil();
            if difference >= 36.0 {
                return format!("{}-{:0>2},63,36", birth[0] + 63.0, birth[1]);
            }
            let year = ((difference + birth[1]) / 12.0).floor();
            let month = ((difference + birth[1]) % 12.0).floor();
            return format!(
                "{}-{:0>2},{:<4.2},{}",
                birth[0] + 60.0 + year,
                month,
                60.0 + difference / 12.0,
                difference
            );
        }
    } else {
        let age = match tp.contains("50") {
            true => 50.0,
            false => 55.0,
        };
        if age == 55.0 {
            // 女性
            if birth[0] < 1970.0 {
                return format!("{}-{:0>2},55,0", birth[0] + 55.0, birth[1]);
            } else {
                // 获取
                let difference = (((birth[0] - 1970.0) * 12.0 + birth[1]) / 4.0).ceil();
                if difference >= 36.0 {
                    return format!("{}-{:0>2},58,36", birth[0] + 58.0, birth[1]);
                }
                let year = ((difference + birth[1]) / 12.0).floor();
                let month = ((difference + birth[1]) % 12.0).floor();
                return format!(
                    "{}-{:0>2},{:<4.2},{}",
                    birth[0] + 55.0 + year,
                    month,
                    55.0 + difference / 12.0,
                    difference
                );
            }
        } else {
            // 女性
            if birth[0] < 1975.0 {
                return format!("{}-{:0>2},50,0", birth[0] + 50.0, birth[1]);
            } else {
                // 获取
                let difference = (((birth[0] - 1975.0) * 12.0 + birth[1]) / 4.0).ceil();
                if difference >= 60.0 {
                    return format!("{}-{:0>2},55,60", birth[0] + 55.0, birth[1]);
                }
                let year = ((difference + birth[1]) / 12.0).floor();
                let month = ((difference + birth[1]) % 12.0).floor();
                return format!(
                    "{}-{:0>2},{:<4.2},{}",
                    birth[0] + 50.0 + year,
                    month,
                    50.0 + difference / 12.0,
                    difference
                );
            }
        }
    }
}