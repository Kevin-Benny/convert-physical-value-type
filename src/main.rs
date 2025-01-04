use convert_physical_value_type::{decimal_to_physical_value_type};
use rust_decimal_macros::dec;

fn main() {
    let value = dec!(32767);
    let result = decimal_to_physical_value_type(value);
    println!("Physical value type= \n Value: {} \n Multiplier: {}", result.get_value(), result.get_multiplier());
    println!("Decimal: {:?}", result.get_decimal_from_physical_type());
}
