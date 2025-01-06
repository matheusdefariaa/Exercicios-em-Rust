struct Color(u8, u8, u8);
struct Hex(String, String, String);

fn main() {
    let n1 = Color(67,13, 8);
    converter_decimal_hexa(n1);
}

fn converter_decimal_hexa(t: Color) {
    let mut hex_1 = String::new();
    let mut hex_2 = String::new();
    let mut hex_3 = String::new();
    let mut cociente_1_str = String::new();
    let mut cociente_2_str = String::new();
    let mut cociente_3_str = String::new();
    let mut resto_1_str = String::new();
    let mut resto_2_str = String::new();
    let mut resto_3_str = String::new();


    let cociente_1 = t.0 / 16;
    let resto_1 = t.0 % 16;
    
    if cociente_1 > 10 {
        cociente_1_str = match cociente_1 {
            10 => "A".to_string(),
            11 => "B".to_string(),
            12 => "C".to_string(),
            13 => "D".to_string(),
            14 => "E".to_string(),
            15 => "F".to_string(),
            _ => "*".to_string(),
        };
        hex_1.push_str(&cociente_1_str);

    }
    else {
        let cociente_1 = cociente_1.to_string();
        hex_1.push_str(&cociente_1);
    }
    
    if resto_1 > 0 {
        
        if resto_1 > 10 {
            resto_1_str = match resto_1 {
                10 => "A".to_string(),
                11 => "B".to_string(),
                12 => "C".to_string(),
                13 => "D".to_string(),
                14 => "E".to_string(),
                15 => "F".to_string(),
                _ => "*".to_string(),
                };
                hex_1.push_str(&resto_1_str);
            }
            
        else {
            let resto_1: String = resto_1.to_string();
            hex_1.push_str(&resto_1);
        }
    }
    
    
    let cociente_2 = t.1 / 16;
    let resto_2 = t.1 % 16;
    
    if cociente_2 >= 10 {
        cociente_2_str = match cociente_2 {
            10 => "A".to_string(),
            11 => "B".to_string(),
            12 => "C".to_string(),
            13 => "D".to_string(),
            14 => "E".to_string(),
            15 => "F".to_string(),
            _ => "*".to_string(),
        };
        hex_2.push_str(&cociente_2_str);

    }
    else {
        let cociente_2 = cociente_2.to_string();
        hex_2.push_str(&cociente_2);
    }
    
    if resto_2 > 0 {
        
        if resto_2 >= 10 {
            resto_2_str = match resto_2 {
                10 => "A".to_string(),
                11 => "B".to_string(),
                12 => "C".to_string(),
                13 => "D".to_string(),
                14 => "E".to_string(),
                15 => "F".to_string(),
                _ => "*".to_string(),
                };
                hex_2.push_str(&resto_2_str);
            }
            
        else {
            let resto_2: String = resto_2.to_string();
            hex_2.push_str(&resto_2);
        }
    }
    
        let cociente_3 = t.2 / 16;
        let resto_3 = t.2 % 16;
    
    if cociente_3 >= 10 {
        cociente_3_str = match cociente_3 {
            10 => "A".to_string(),
            11 => "B".to_string(),
            12 => "C".to_string(),
            13 => "D".to_string(),
            14 => "E".to_string(),
            15 => "F".to_string(),
            _ => "*".to_string(),
        };
        hex_3.push_str(&cociente_3_str);

    }
    else {
        let cociente_3 = cociente_3.to_string();
        hex_3.push_str(&cociente_3);
    }
    
    if resto_3 > 0 {
        
        if resto_3 >= 10 {
            resto_3_str = match resto_3 {
                10 => "A".to_string(),
                11 => "B".to_string(),
                12 => "C".to_string(),
                13 => "D".to_string(),
                14 => "E".to_string(),
                15 => "F".to_string(),
                _ => "*".to_string(),
                };
                hex_3.push_str(&resto_3_str);
            }
            
        else {
            let resto_3: String = resto_3.to_string();
            hex_3.push_str(&resto_3);
        }
    }
    
    let res = Hex(hex_1, hex_2, hex_3);

    println!("{}{}{}", res.0, res.1, res.2);
}

