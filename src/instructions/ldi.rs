use crate::vm::VM;

use super::sign_extend;


// {
//     /* destination register (DR) */
//     uint16_t r0 = (instr >> 9) & 0x7;
//     /* PCoffset 9*/
//     uint16_t pc_offset = sign_extend(instr & 0x1FF, 9);
//     /* add pc_offset to the current PC, look at that memory location to get the final address */
//     reg[r0] = mem_read(mem_read(reg[R_PC] + pc_offset));
//     update_flags(r0);
// }
impl VM {
    pub fn op_ldi(&mut self, instr: u16) {
        // Destination Register
        let r0: usize = ((instr >> 9) & 0x7).into();

        // Offset from PC (Program Counter)
        let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);

        let position: usize = (self.reg.pc + pc_offset).into();

        let value = self.mem.read(self.mem.read(position));

        self.reg.update(r0, value);
    }
}
