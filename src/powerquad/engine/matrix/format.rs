//! Internal formatting functions.



#[cfg(feature = "defmt")]
#[doc(hidden)]
#[inline(never)]
pub(super) fn mtxformat(data: &[f32], c: usize, r: usize, typed: bool, f: defmt::Formatter) {
    if typed {
        defmt::write!(f, "Const Sized Matrix [{=usize}x{=usize}]\n", c, r);
    } else {
        defmt::write!(f, "Dyn Sized Matrix [{=usize}x{=usize}]\n", c, r);
    }

    // Iterate over the rows.
    for i in 0..r {
        // Get the row slice.
        let slice = &data[(i * c)..((i+1) * c)];

        // Write the slice.
        defmt::write!(f, "{=[?]}\n", slice);
    }
}

#[cfg(feature = "defmt")]
#[doc(hidden)]
#[inline(never)]
pub(super) fn rowformat(data: &[f32], c: usize, typed: bool, f: defmt::Formatter) {
    if typed {
        defmt::write!(f, "Const Sized Matrix Row [{=usize}]\n", c);
    } else {
        defmt::write!(f, "Dyn Sized Matrix Row [{=usize}]\n", c);
    }

    // Write the slice.
    defmt::write!(f, "{=[?]}\n", data);
}
