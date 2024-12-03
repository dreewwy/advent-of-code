// Part 1
pub fn solvep1(input: &str)-> u64{
    let mut l1: Vec<u64> = Vec::new();
    let mut l2: Vec<u64> = Vec::new();
    for line in input.trim().split('\n') {
        let (val1, val2) = line.trim().split_once( "   ").unwrap();
        l1.push(val1.parse().unwrap());
        l2.push(val2.parse().unwrap());
    }
    l1.sort();
    l2.sort();
    let mut result: u64 = 0;
    for i in 0..l1.len() {
        result += l1[i].abs_diff(l2[i]);
    }
    result
    
}
// Part 2
pub fn solvep2(input: &str)-> u64 {
    let mut l1: Vec<u64> = Vec::new();
    let mut l2: Vec<u64> = Vec::new();
    for line in input.trim().split('\n') {
        let (val1, val2) = line.trim().split_once( "   ").unwrap();
        l1.push(val1.parse().unwrap());
        l2.push(val2.parse().unwrap());
    }

    let mut result:u64 = 0;
    for i in 0..l1.len(){
        for j in 0..l2.len(){
            if l1[i] == l2[j] {
                result += l1[i];
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn p1_complete() {
        let input: String = fs::read_to_string("src/inputs.txt").unwrap();
        let result: u64 = solvep1(input.as_str());
        assert_eq!(result,2192892);
    }
    #[test]
    fn p2_complete(){
        let input: String = fs::read_to_string("src/inputs.txt").unwrap();
        let result: u64 = solvep2(input.as_str());
        assert_eq!(result,22962826);
    }
}