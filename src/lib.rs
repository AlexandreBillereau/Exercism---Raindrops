pub fn raindrops(n: u32) -> String {
    
    let raindrops  = [pling, plang, plong];
    let mut result = "".to_string();
    
    for func in raindrops{
        result.push_str(&func(n));
    }

    if result.len() == 0 {
        return n.to_string();
    }

    return result;
    
}

fn pling(n: u32) -> String {
    generic(n, 3, "Pling".to_string())
}

fn plang(n: u32) -> String {
    generic(n, 5, "Plang".to_string())
}

fn plong(n: u32) -> String {
    generic(n, 7, "Plong".to_string())
}

fn generic(n: u32, factor: u32, string: String) -> String {
    if n % factor == 0 { return string;}
    "".to_string()
}