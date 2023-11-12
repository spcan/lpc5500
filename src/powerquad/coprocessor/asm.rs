//! Raw inline assembly to interact with coprocessors.



/// Internal function to create an inlined MCR instruction.
#[inline(always)]
pub(super) fn mcr<const CP: u32, const OP1: u32, const CRN: usize, const CRM: u32, const OP2: usize>(value: u32) {
    unsafe {
        core::arch::asm!(
            "MCR p{cp}, #{op1}, {0}, c{crn}, c{crm}, #{op2}",
            in(reg) value,
            cp  = const CP,
            op1 = const OP1,
            crn = const CRN,
            crm = const CRM,
            op2 = const OP2,
            options(nostack, nomem)
        )
    }
}



/// Internal function to create an inlined MRC instruction.
#[inline(always)]
pub(super) fn mrc<const CP: u32, const OP1: u32, const CRN: usize, const CRM: u32, const OP2: usize>() -> u32 {
    // Preallocate the value.
    let raw: u32;

    unsafe {
        core::arch::asm!(
            "MRC p{cp}, #{op1}, {0}, c{crn}, c{crm}, #{op2}",
            out(reg) raw,
            cp  = const CP,
            op1 = const OP1,
            crn = const CRN,
            crm = const CRM,
            op2 = const OP2,
            options(nostack, nomem)
        )
    }

    raw
}
