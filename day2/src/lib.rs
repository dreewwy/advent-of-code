pub fn solvep1(input: &str) -> i32 {
    let mut safe = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let numbers: Vec<i32> = line.split(' ').filter_map(|num| num.parse::<i32>().ok()).collect();
        let mut levels = true;
        for window in numbers.windows(2){
            if let [a,b] = window {
                let diff = b-a;
                if diff.abs() < 1 || diff.abs() > 3 {
                    levels = false;
                }
            }
        }
        let mut increasing = true;
        for i in 1..numbers.len() {
            if numbers[i] < numbers[i-1] {
                increasing = false;
            } 
        }
        let mut decreasing = true;
        for j in 1..numbers.len(){
            if numbers[j-1] < numbers[j] {
                decreasing = false;
            } 
        }
        if increasing && levels || decreasing && levels {
            println!("{:?} increasing: {:?}, decreasing: {:?}",numbers, increasing, decreasing);
            safe += 1;
        }
    }    
    println!("{:?}",safe);
    safe
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn it_works() {
        let input: String = fs::read_to_string("src/inputs.txt").unwrap();
        let result = solvep1(input.as_str());
        assert_eq!(result,534);
    }
}
