pub fn string_literal_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1];        
        let bytes: Vec<u8> = inner
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .collect();
        
        return Some(bytes)
    } else {
        return None
    }
}

pub fn bytes_to_string_literal(bytes: Vec<u8>) -> String {
    let mut result = String::from("[");
    
    for (index, byte) in bytes.iter().enumerate() {
        result.push_str(&byte.to_string());
        
        if index < bytes.len() - 1 {
            result.push(',');
        }
    }
    result.push(']');
    
    return result
}