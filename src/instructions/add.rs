use crate::vm::VM;

use super::sign_extend;

// {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* first operand (SR1) */
//     uint16_t r1 = (instr >> 6) & 0x7;
//     /* whether we are in immediate mode */
//     uint16_t imm_flag = (instr >> 5) & 0x1;

//     if (imm_flag)
//     {
//         uint16_t imm5 = sign_extend(instr & 0x1F, 5);
//         reg[r0] = reg[r1] + imm5;
//     }
//     else
//     {
//         uint16_t r2 = instr & 0x7;
//         reg[r0] = reg[r1] + reg[r2];
//     }

//     update_flags(r0);
// }

impl VM {
    pub fn op_add(&mut self, instr: u16){
        // Destination Register (DR)
        let r0: usize = ((instr >> 9) & 0x7).into(); // Register number

        // First Operand (SR1)
        let r1: usize = ((instr >> 6) & 0x7).into();

        // Flag which indicates mode
        let imm_flag = (instr >> 5) & 0x1;

        if imm_flag == 1 {
            let imm5 = sign_extend(instr & 0x1F, 5);
            self.reg.general[r0] = self.reg.general[r1] + imm5;
        }
        else {
            let r2: usize = (instr & 0x7).into();
            self.reg.general[r0] = self.reg.general[r1] + self.reg.general[r2];

            // Other alternative
            let value = self.reg.general[r1] + self.reg.general[r2];
            self.reg.update(r0, value);
        }


    }
}
