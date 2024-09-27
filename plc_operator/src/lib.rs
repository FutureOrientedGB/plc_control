use std::mem;

#[derive(Debug)]
pub enum DataType {
    Bool,
    U32,
    U64,
    F32,
    F64,
}

#[derive(Debug)]
pub struct DataRule {
    pub start_bit: usize,
    pub bit_length: usize,
}

impl DataType {
    pub fn parse_bool(u16_buf: &[u16], rule: &DataRule) -> Option<u32> {
        if rule.bit_length != 1 {
            return None;
        }
        let start_index = rule.start_bit / 16;
        let start_bit_offset = rule.start_bit % 16;
        if start_index >= u16_buf.len() {
            return None;
        }
        let value = (u16_buf[start_index] >> start_bit_offset) & 1;
        Some(value as u32)
    }

    pub fn parse_u32(u16_buf: &[u16], rule: &DataRule) -> Option<u32> {
        if rule.bit_length != 32 {
            return None;
        }
        let start_index = rule.start_bit / 16;
        let start_bit_offset = rule.start_bit % 16;
        if start_index + 1 >= u16_buf.len() {
            return None;
        }
        let value = (u16_buf[start_index] >> start_bit_offset) | (u16_buf[start_index + 1] << (16 - start_bit_offset));
        Some(value as u32)
    }

    pub fn parse_u64(u16_buf: &[u16], rule: &DataRule) -> Option<u64> {
        if rule.bit_length != 64 {
            return None;
        }
        let start_index = rule.start_bit / 16;
        let start_bit_offset = rule.start_bit % 16;
        if start_index + 3 >= u16_buf.len() {
            return None;
        }
        let value = (u16_buf[start_index] >> start_bit_offset) |
                    (u16_buf[start_index + 1] << (16 - start_bit_offset)) |
                    (u16_buf[start_index + 2] << (32 - start_bit_offset)) |
                    (u16_buf[start_index + 3] << (48 - start_bit_offset));
        Some(value as u64)
    }

    pub fn parse_f32(u16_buf: &[u16], rule: &DataRule) -> Option<f32> {
        if rule.bit_length != 32 {
            return None;
        }
        let value = Self::parse_u32(u16_buf, rule)?;
        Some(unsafe { mem::transmute::<u32, f32>(value) })
    }

    pub fn parse_f64(u16_buf: &[u16], rule: &DataRule) -> Option<f64> {
        if rule.bit_length != 64 {
            return None;
        }
        let value = Self::parse_u64(u16_buf, rule)?;
        Some(unsafe { mem::transmute::<u64, f64>(value) })
    }

    pub fn from_bool(value: bool, rule: &DataRule) -> Option<[u16; 1]> {
        if rule.bit_length != 1 {
            return None;
        }
        let mut result = [0; 1];
        result[0] |= (value as u16) << rule.start_bit % 16;
        Some(result)
    }

    pub fn from_u32(value: u32, rule: &DataRule) -> Option<[u16; 2]> {
        if rule.bit_length != 32 {
            return None;
        }
        let mut result = [0; 2];
        result[0] |= ((value & 0xFFFF) << rule.start_bit % 16) as u16;
        result[1] |= ((value >> 16) << rule.start_bit % 16) as u16;
        Some(result)
    }

    pub fn from_u64(value: u64, rule: &DataRule) -> Option<[u16; 4]> {
        if rule.bit_length != 64 {
            return None;
        }
        let mut result = [0; 4];
        result[0] |= ((value & 0xFFFF) << rule.start_bit % 16) as u16;
        result[1] |= (((value >> 16) & 0xFFFF) << rule.start_bit % 16) as u16;
        result[2] |= (((value >> 32) & 0xFFFF) << rule.start_bit % 16) as u16;
        result[3] |= (((value >> 48) & 0xFFFF) << rule.start_bit % 16) as u16;
        Some(result)
    }

    pub fn from_f32(value: f32, rule: &DataRule) -> Option<[u16; 2]> {
        if rule.bit_length != 32 {
            return None;
        }
        let value_bits = unsafe { mem::transmute::<f32, u32>(value) };
        Self::from_u32(value_bits, rule)
    }

    pub fn from_f64(value: f64, rule: &DataRule) -> Option<[u16; 4]> {
        if rule.bit_length != 64 {
            return None;
        }
        let value_bits = unsafe { mem::transmute::<f64, u64>(value) };
        Self::from_u64(value_bits, rule)
    }
}