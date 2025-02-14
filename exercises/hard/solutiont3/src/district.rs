use std::{
    collections::HashMap,
    fs::{self, File},
};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug)]
struct CityData(HashMap<String, Vec<String>>);
// 手动实现反序列化，为了合并相同的城市名称。
impl<'de> Deserialize<'de> for CityData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CityDataVisitor;

        impl<'de> serde::de::Visitor<'de> for CityDataVisitor {
            type Value = CityData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map with city names and their associated cities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut data = HashMap::new();

                while let Some((key, value)) = map.next_entry::<String, Vec<String>>()? {
                    data.entry(key)
                        .and_modify(|e: &mut Vec<String>| e.extend(value.iter().cloned()))
                        .or_insert_with(|| value);
                }

                Ok(CityData(data))
            }
        }

        deserializer.deserialize_map(CityDataVisitor)
    }
}

#[derive(Deserialize, Debug)]
struct Data {
    #[serde(flatten)]
    levels: HashMap<String, CityData>,
}

pub fn count_provinces() -> String {
    let path = File::open("district.json").expect("Failed to open file");

    // 解析 JSON 数据
    let data: Data = serde_json::from_reader(&path).expect("JSON parsing failed");
    let mut sum: Vec<HashMap<String, bool>> = Vec::new();
    // 根据索引有顺序额的存储我们结果，方便拼接。
    let mut my_str = vec![String::new(); data.levels.len()];
    // 把相关练的城市群的数据放进一个hash中，每次添加时，需要便利我们的sum，进行hash查询，要便利查询所有关联的城市，能够查到就把同一个城市群的新内容拼接进去
    // 如果没有查到证明是新省份，构造一个hash存入sum中，最后sum的长度就是当前数据的省份数量
    data.levels.iter().for_each(|(i, city)| {
        city.0.iter().for_each(|(name, city)| {
            match sum
                .iter_mut()
                .find(|map| map.contains_key(name) || city.iter().any(|tmp| map.contains_key(tmp)))
            {
                Some(map) => {
                    city.iter().for_each(|name| {
                        map.insert(name.to_string(), true);
                    });
                    map.insert(name.to_string(), true);
                }
                None => {
                    let mut map: HashMap<String, bool> =
                        city.iter().map(|name| (name.clone(), true)).collect();
                    map.insert(name.to_string(), true);
                    sum.push(map);
                }
            }
        });
        my_str[i.parse::<usize>().unwrap() - 1] = format!("{}", sum.len());
        sum.clear();
    });
    my_str.join(",")
}
