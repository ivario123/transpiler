use transpiler::pseudo;

fn main() {
    pseudo!(
        ret.extend[

            let offset_addr = 0.local_into();
            if add {
                offset_addr = rn + imm;
            } else {
                offset_addr = rn - imm;
            }

            let address = 0.local_into();
            if index {
                address = offset_addr;
            } else {
                address = rn;
            }

            // TODO! Ensure that this is correct for writing only LS byte
            LocalAddress(address,8) = rt;

            if w {
                rn = offset_addr;
            }
            Jump(rn);
        ]
    );
}
