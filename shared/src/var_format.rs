#[derive(Default, Clone, Copy, Debug)]
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
            VarFormat::Signed => "i32",
            VarFormat::Unsigned => "u32",
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
        let ones_and_zeros = value
            .chars()
            .rev()
            .map(|char| char.to_digit(2).unwrap())
            .collect::<Vec<_>>();
        let mut base = convert_base::Convert::new(2, 16);
        let output = base.convert::<u32, u32>(&ones_and_zeros);
        let value: String = output
            .into_iter()
            .map(|number| char::from_digit(number, 16).unwrap())
            .collect();
        value
    }
}
