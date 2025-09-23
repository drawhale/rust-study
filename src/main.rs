use std::collections::HashMap;

fn main() {    
    let array = [1, 2, 3, 4, 5, 3, 3, 3];

    let mut vec = Vec::from(array);
    let mut hash_map = HashMap::new();

    // 한 번의 순회로 빈도 계산
    for &i in &vec {
        let count = hash_map.entry(i).or_insert(0);
        *count += 1;
    }

    // 정렬하여 중간값 구하기
    vec.sort();
    let middle_value = vec[vec.len() / 2];

    println!("중간값: {}, 빈도: {:?}", middle_value, hash_map);
}   

