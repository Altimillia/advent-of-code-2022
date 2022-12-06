use std::collections::HashSet;

pub fn part_one(input: String) -> usize { 
    return get_first_unique_index(input, 4);
}

pub fn part_two(input: String) -> usize {
    return get_first_unique_index(input, 14);
}

fn get_first_unique_index(input: String, window_size: usize ) -> usize {
    let char_vector:Vec<char> = input.chars().into_iter().collect();
    return char_vector.windows(window_size)
        .map(|packet_marker| {
            return is_unique_marker(packet_marker.into_iter().collect(), window_size);
    }).position(|unique| unique == true)
    .unwrap() + window_size;
}

fn is_unique_marker(packet_marker: String, window_size: usize) -> bool {
    let unique_set:HashSet<char> = packet_marker.chars().into_iter().collect();
    return unique_set.len() == window_size;
}