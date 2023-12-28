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