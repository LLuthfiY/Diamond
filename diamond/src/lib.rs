pub fn get_diamond(c: char) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    let alph:Vec<char> = String::from_utf8((b'A'..=b'Z').collect()).unwrap().chars().collect();
    for x in 0..(c as usize)-64{
        let mut temp = "".to_string();
        for y in 0..(((c as usize)-64)*2)-1{
            if y == ((c as usize)-65)+x || y == ((c as usize)-65)-x{
                temp = format!("{}{}",temp,alph[x]);
            }
            else{
                temp = format!("{}{}",temp," ");
            }
            
        }
        if x == (c as usize)-65{
            result.insert(result.len()/2,temp);
        }
        else{
            result.insert(result.len()/2,temp.clone());
            result.insert(result.len()/2,temp);
        }
        //println!("{} {}",x ,(c as usize)-65);
    }
    result
}
