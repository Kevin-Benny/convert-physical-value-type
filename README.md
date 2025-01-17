# Convert Decimal to iso15118-2/20 Physical Value Type and vice versa

Eg:
```
pub struct PreChargeReq {
    pub response_code: ResponseCode,
    pub ev_target_voltage: PhysicalValueType,
}
pub struct PhysicalValueType {
    pub multiplier: i8,
    pub value: i16,
}

let pre_charge_res = PreChargeRes {
                        response_code: ResponseCode::Ok,
                        ev_target_voltage: PhysicalValueType {
                            multiplier: -2,
                            value: 25051,
                        },
                     };
let value: Decimal = get_value_from_physical_type(pre_charge_res.evse_present_voltage); // value = 250.51

                     
let present_module_voltage_mv: u32 = 500210;
let decimal_value: Decimal = milli_to_decimal(present_module_voltage_mv); // decimal_value = 500.21 where value = 50021 and multiplier = -2
let evse_present_voltage: PhysicalValueType = decimal_to_physical_value_type(decimal_value);                 
let pre_charge_res = PreChargeRes {
                        response_code: ResponseCode::Ok,
                        evse_present_voltage,
                     };
```
