pub fn rot13(message: &str) -> String {
    let alphabit: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let alphabit_uppercase: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut res = String::new();
    for i in message.chars() {
        if i.is_uppercase() {
            if let Some(pos) = alphabit_uppercase.iter().position(|&x| x == i) {
                res.push(alphabit_uppercase[(pos+13) % alphabit_uppercase.len()]);
            } else {
                res.push_str(&i.to_string());
            }
        } else if i.is_lowercase() {
            if let Some(pos) = alphabit.iter().position(|&x| x == i) {
                res.push(alphabit[(pos+13) % alphabit.len()]);
            } else {
                res.push_str(&i.to_string());
            }
        } else {
            res.push_str(&i.to_string());
        }
    }
    return res;
}
