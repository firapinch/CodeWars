pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut n = numbers.to_vec();
    if let Some((pos, _)) = n.iter().enumerate().min_by_key(|&(_, x)| x) {
        n.remove(pos);
    }
    n
}

//         Or
// pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
//     let mut n = numbers.to_vec(); 
//     if let Some(min) = n.iter().min() {
//         if let Some(pos) = n.iter().position(|&x| x == *min) {
//             n.remove(pos); 
//         } 
//     } 
//     n 
// }

