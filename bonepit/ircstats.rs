type Nick = String;

#[deriving(Show)]
pub enum IrcEvent {
    Movement,
    Message(String, Vec<String>), // words of the message go here
    Unknown
}

pub fn parse_msg(msg: &str) -> IrcEvent {
    let fixed_parts = rm_dws(msg.clone()).as_slice().clone();
    let parts: Vec<&str> = fixed_parts.split_str(" ").collect();
    let join_part = String::from_str("-->".as_slice());

    fn collect<T: Clone>(parts: &[T]) -> Vec<T> {
        let mut l_vec: Vec<T> = Vec::new();
        l_vec.push_all(parts);
        return l_vec;
    }

    fn rm_dws(msg: &str) -> String{
        let mut last_char: char;
        let mut col_str: Vec<char> = vec![];

        last_char = ' ';
        
        for x in msg.chars() {
            if(x == last_char) {
                continue;
            }
            last_char = x;
            col_str.push(x.clone());
        }

        println!("FIXED: {}", String::from_chars(col_str.as_slice()));
        
        return String::from_chars(col_str.as_slice());
    }
    
    fn sanitize_parts(dirty: Vec<&str>) -> Vec<String> {
        let mut local_vec: Vec<String> = vec![];
        
        for x in dirty.iter() {
            local_vec.push(String::from_str(x.clone()).clone());
        }
        let lv_pt2 = local_vec[1].clone();
        local_vec[1] = rm_dws(lv_pt2.as_slice());
        return local_vec;
    }

    let s_parts = sanitize_parts(parts);
    println!("DEBUG: {}", s_parts.clone());
    match s_parts.as_slice() {
        [_, _, ref join_part] => IrcEvent::Movement,
        [_, ref nick, x..] => IrcEvent::Message(nick.clone(), collect(x)),
        _ => IrcEvent::Movement
    }
}

fn main() {
    let result: IrcEvent = parse_msg("2014-10-06 17:34:25	+tso	oh man this was on my youtube recommendations: https://www.youtube.com/watch?v=2HQaBWziYvY");
    println!("{}", result);
}
