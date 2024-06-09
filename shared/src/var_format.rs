use moonlight::*;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum VarFormat {
    ASCII,
    Binary,
    BinaryWithGroups,
    #[default]
    Hexadecimal,
    Octal,
    Signed,
    Unsigned,
}

impl VarFormat {
    pub fn as_static_str(&self) -> &'static str {
        match self {
            VarFormat::ASCII => "Text",
            VarFormat::Binary => "Bin",
            VarFormat::BinaryWithGroups => "Bins",
            VarFormat::Hexadecimal => "Hex",
            VarFormat::Octal => "Oct",
            VarFormat::Signed => "Int",
            VarFormat::Unsigned => "UInt",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            VarFormat::ASCII => VarFormat::Binary,
            VarFormat::Binary => VarFormat::BinaryWithGroups,
            VarFormat::BinaryWithGroups => VarFormat::Hexadecimal,
            VarFormat::Hexadecimal => VarFormat::Octal,
            VarFormat::Octal => VarFormat::Signed,
            VarFormat::Signed => VarFormat::Unsigned,
            VarFormat::Unsigned => VarFormat::ASCII,
        }
    }

    pub fn format(&self, value: wellen::SignalValue) -> String {
        // @TODO optimize it by not using `.to_string` if possible
        let value = value.to_string();
        if value.is_empty() {
            return value;
        }
        match self {
            VarFormat::ASCII => {
                let mut formatted_value = String::new();
                for group_index in 0..value.len() / 8 {
                    let offset = group_index * 8;
                    let group = &value[offset..offset + 8];
                    if let Ok(byte_char) = u8::from_str_radix(group, 2) {
                        formatted_value.push(byte_char as char);
                    }
                }
                formatted_value
            }
            VarFormat::Binary => value,
            VarFormat::BinaryWithGroups => {
                let char_count = value.len();
                value
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut value, (index, one_or_zero)| {
                        value.push(one_or_zero);
                        let is_last = index == char_count - 1;
                        if !is_last && (index + 1) % 4 == 0 {
                            value.push(' ');
                        }
                        value
                    })
            }
            VarFormat::Hexadecimal => {
                let ones_and_zeros = value
                    .chars()
                    .rev()
                    .map(|char| char.to_digit(2).unwrap())
                    .collect::<Vec<_>>();
                let mut base = convert_base::Convert::new(2, 16);
                let output = base.convert::<u32, u32>(&ones_and_zeros);
                let value: String = output
                    .into_iter()
                    .rev()
                    .map(|number| char::from_digit(number, 16).unwrap())
                    .collect();
                value
            }
            VarFormat::Octal => {
                let ones_and_zeros = value
                    .chars()
                    .rev()
                    .map(|char| char.to_digit(2).unwrap())
                    .collect::<Vec<_>>();
                let mut base = convert_base::Convert::new(2, 8);
                let output = base.convert::<u32, u32>(&ones_and_zeros);
                let value: String = output
                    .into_iter()
                    .rev()
                    .map(|number| char::from_digit(number, 8).unwrap())
                    .collect();
                value
            }
            VarFormat::Signed => {
                let mut ones_and_zeros = value
                    .chars()
                    .rev()
                    .map(|char| char.to_digit(2).unwrap())
                    .collect::<Vec<_>>();

                // https://builtin.com/articles/twos-complement
                let sign = if ones_and_zeros.last().unwrap() == &0 {
                    ""
                } else {
                    "-"
                };
                if sign == "-" {
                    let mut one_found = false;
                    for one_or_zero in &mut ones_and_zeros {
                        if one_found {
                            *one_or_zero = if one_or_zero == &0 { 1 } else { 0 }
                        } else if one_or_zero == &1 {
                            one_found = true;
                        }
                    }
                }

                let mut base = convert_base::Convert::new(2, 10);
                let output = base.convert::<u32, u32>(&ones_and_zeros);
                let value_without_sign: String = output
                    .into_iter()
                    .rev()
                    .map(|number| char::from_digit(number, 10).unwrap())
                    .collect();
                // @TODO chain `sign` before collecting?
                let value = sign.to_owned() + &value_without_sign;
                value
            }
            VarFormat::Unsigned => {
                let ones_and_zeros = value
                    .chars()
                    .rev()
                    .map(|char| char.to_digit(2).unwrap())
                    .collect::<Vec<_>>();
                let mut base = convert_base::Convert::new(2, 10);
                let output = base.convert::<u32, u32>(&ones_and_zeros);
                let value: String = output
                    .into_iter()
                    .rev()
                    .map(|number| char::from_digit(number, 10).unwrap())
                    .collect();
                value
            }
        }
    }
}
