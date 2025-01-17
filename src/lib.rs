use rust_decimal::Decimal;
pub struct PhysicalValueType {
    multiplier: i8,
    // unit: UnitSymbolType, // Not useded for this program
    value: i16,
}
impl PhysicalValueType {
    pub fn new(multiplier: i8, value: i16) -> Self {
        PhysicalValueType {
            multiplier,
            value,
        }
    }
    pub fn get_multiplier(&self) -> i8 {
        self.multiplier
    }
    pub fn get_value(&self) -> i16 {
        self.value
    }
    pub fn get_decimal_from_physical_type(&self) -> Decimal {
        match self.multiplier {
            i8::MIN..=-1 => {
                let decimal = Decimal::from_i128_with_scale(self.value as i128, self.multiplier.abs() as u32);
                decimal
            }
            1..=i8::MAX => {
                let multi = 10_i32.pow(self.multiplier as u32);
                let value = self.value as i128 * multi as i128;
                value.into()
            }
            0 => self.value.into(),
        }
    }    
}
fn milli_to_decimal(value: u32) -> Decimal {
    let num = Decimal::new(value.into(), 3);
    num.normalize()
}
pub fn decimal_to_physical_value_type(decimal_value: Decimal) -> PhysicalValueType {
    // Strips any trailing zero’s after decimal point
    let mut decimal_value = decimal_value.normalize();
    // deconstruct the Decimal

    let mut mantissa = decimal_value.mantissa();
    let mut orig_scale = - (decimal_value.scale() as i16);

    let phy_value = if mantissa > i16::MAX as i128{
        while mantissa > i16::MAX as i128 {
            let new_mantissa= mantissa / 10;
            orig_scale += 1;
            mantissa = new_mantissa;
        };
        let value = mantissa as i16;
        let scale = orig_scale as i16;
        PhysicalValueType {
            multiplier: scale as i8,
            value,
        }
     
    } else {
        let value = decimal_value.mantissa() as i16;
        let scale = - (decimal_value.scale() as i16);
        PhysicalValueType {
            multiplier: scale as i8,
            value,
        }
    };

    phy_value
}

mod test {
    use rust_decimal_macros::dec;

    use crate::{decimal_to_physical_value_type};

    #[test]
    fn physical_value_test1 (){

        let value = dec!(32767);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(32767));

        let value = dec!(3276.7);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(3276.7));

        let value = dec!(327.67);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(327.67));

        let value = dec!(32.767);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(32.767));

        let value = dec!(32768);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(32760));

        let value = dec!(3276.8);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(3276.0));

        let value = dec!(327.68);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(327.60));

        let value = dec!(32.768);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(32.760));

        let value = dec!(327671);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(327670));

        let value = dec!(32767100);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(32767000));

        let value = dec!(33767);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(result.get_decimal_from_physical_type(), dec!(33760));

        let value = dec!(3.2767);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(get_value_from_physical_type(&result), dec!(3.276));

        let value = dec!(33760000);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(get_value_from_physical_type(&result), dec!(33760000));

        let value = dec!(680000);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(get_value_from_physical_type(&result), dec!(680000));

        let value = dec!(33768000);
        let result = decimal_to_physical_value_type(value);
        assert_eq!(get_value_from_physical_type(&result), dec!(33760000));
    }
}
