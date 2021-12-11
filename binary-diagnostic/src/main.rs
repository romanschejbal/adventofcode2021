use std::io::{stdin, BufRead};

fn main() {
    let binary_numbers = stdin()
        .lock()
        .lines()
        .map(|line| {
            let s = line.unwrap();
            (s.clone(), i32::from_str_radix(&s, 2).unwrap())
        })
        .collect::<Vec<_>>();

    let gamma_rate_histogram = build_histogram(&binary_numbers);

    let gamma_rate = gamma_rate_histogram
        .iter()
        .rev()
        .map(|x| if *x > binary_numbers.len() / 2 { 1 } else { 0 })
        .collect::<Vec<_>>();
    let epsilon_rate = gamma_rate.iter().map(|x| x ^ 1).collect::<Vec<_>>();

    let gamma_rate_binary = gamma_rate.iter().map(i32::to_string).collect::<String>();
    let epsilon_rate_binary = epsilon_rate.iter().map(i32::to_string).collect::<String>();

    let gamma_rate = i32::from_str_radix(&gamma_rate_binary, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_binary, 2).unwrap();

    println!("Part 1 = {}", gamma_rate * epsilon_rate);

    let mut oxygen_generator_ratings = binary_numbers.clone();
    for i in (0..oxygen_generator_ratings[0].0.len()).rev() {
        if oxygen_generator_ratings.len() == 1 {
            break;
        }
        let histogram = build_histogram(&oxygen_generator_ratings);
        let bit = i32::pow(2, i as u32);
        if histogram[i] >= (oxygen_generator_ratings.len() as f32 / 2 as f32).ceil() as usize {
            oxygen_generator_ratings.retain(|(_, number)| number & bit == bit);
        } else {
            oxygen_generator_ratings.retain(|(_, number)| number & bit != bit);
        }
    }

    let mut co2_scrubber_rating = binary_numbers.clone();
    for i in (0..co2_scrubber_rating[0].0.len()).rev() {
        if co2_scrubber_rating.len() == 1 {
            break;
        }
        let histogram = build_histogram(&co2_scrubber_rating);
        let bit = i32::pow(2, i as u32);
        if histogram[i] >= (co2_scrubber_rating.len() as f32 / 2 as f32).ceil() as usize {
            co2_scrubber_rating.retain(|(_, number)| number & bit != bit);
        } else {
            co2_scrubber_rating.retain(|(_, number)| number & bit == bit);
        }
    }

    println!(
        "Part 2 = {}",
        oxygen_generator_ratings[0].1 * co2_scrubber_rating[0].1
    );
}

fn build_histogram(binary_numbers: &Vec<(String, i32)>) -> Vec<usize> {
    let mut histogram = vec![0; binary_numbers[0].0.len()];

    for (s, n) in binary_numbers {
        for j in (0..s.len()).rev() {
            let bit = i32::pow(2, j as u32);
            if n & (bit) == bit {
                histogram[j] += 1;
            }
        }
    }
    histogram
}
