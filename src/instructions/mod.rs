pub mod add;
pub mod and;
pub mod br;
pub mod jmp;
pub mod jsr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
pub mod not;
pub mod st;
pub mod sti;
pub mod str;
pub mod trap;

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_extend_positive_number() {
        // Positive number, should not change with sign extension
        // Example: 0b00101 (5 in 5-bit) -> should stay the same
        let x: u16 = 0b00101;
        let result = sign_extend(x, 5);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_sign_extend_negative_number() {
        // Negative number, sign should be extended
        // Example: 0b11110 (in 5-bit) -> should become 0xFFFE (-2 in 16-bit signed)
        let x: u16 = 0b11110;
        let result = sign_extend(x, 5);
        assert_eq!(result as i16, -2);
    }

    #[test]
    fn test_sign_extend_zero() {
        // Zero should remain zero no matter the bit count
        let x: u16 = 0b00000;
        let result = sign_extend(x, 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sign_extend_max_value() {
        // Maximum value for 5 bits: 0b11111 (should extend to -1)
        let x: u16 = 0b11111;
        let result = sign_extend(x, 5);

        println!("Result in binary: {:b}", result as i16);

        assert_eq!(result as i16, -1);
    }
}
