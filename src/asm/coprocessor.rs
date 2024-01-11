//! Coprocessor access assembly instructions.



/// Internal function to create an inlined MCR instruction.
/// This instruction moves one Register to a Coprocessor Register.
#[inline(always)]
pub(crate) fn mcr<const CP: u32, const OP1: u32, const CRN: usize, const CRM: u32, const OP2: usize>(value: u32) {
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
/// This instruction moves one Coprocessor Register to a Register.
#[inline(always)]
pub(crate) fn mrc<const CP: u32, const OP1: u32, const CRN: usize, const CRM: u32, const OP2: usize>() -> u32 {
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



/// Internal function to create an inlined MCRR instruction.
/// This instruction moves two Registers to Coprocessor Registers.
#[inline(always)]
pub(crate) fn mcrr<const CP: u32, const OP1: usize, const CRM: u32>(a: u32, b: u32) {
    unsafe {
        core::arch::asm!(
            "MCRR p{cp}, #{op1}, {0}, {1}, c{crm}",
            in(reg) a,
            in(reg) b,
            cp  = const CP,
            op1 = const OP1,
            crm = const CRM,
            options(nostack, nomem)
        )
    }
}
