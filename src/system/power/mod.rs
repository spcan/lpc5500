//! Power Control module.



pub mod voltages;

mod process;
mod profile;



use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};

use self::{
    process::ProcessNode,
    profile::PowerProfile,
};



pub struct PowerControl {
    /// The process node of the device.
    node: Option<ProcessNode>,
}

impl PowerControl {
    /// PMC base address.
    const PMC: u32 = 0x40020000;

    /// DCDC 0 register offset.
    const DCDC0: u32 = 0x10;

    /// DCDC 1 register offset.
    const DCDC1: u32 = 0x14;

    /// LDOPMU register offset.
    const LDOPMU: u32 = 0x1C;

    /// Initializes the power control system.
    pub(crate) fn init() -> Self {
        Self {
            node: None,
        }
    }

    /// Configures the power of the device for the given target frequency.
    pub fn target(&mut self, frequency: u32) {
        // Get the power profile for the target frequency.
        let profile = PowerProfile::from(frequency);

        // Get the NMPA DCDC values.
        let (dcdc0, dcdc1) = profile.dcdc();

        // Write the DCDC values to the PMC.
        unsafe {
            write( (Self::PMC + Self::DCDC0) as *mut u32, dcdc0 );
            write( (Self::PMC + Self::DCDC1) as *mut u32, dcdc1 );
        }

        // Get the process node.
        let process = match self.node {
            Some(node) => node,
            None => {
                // Read the process node.
                let node = ProcessNode::get();

                // Store the process node to avoid reading it again.
                self.node = Some(node);

                node
            },
        };

        // Get the core voltage for the given process and profile and set it.
        self.setvoltage( process.voltage( profile ) );
    }

    /// Sets the voltage of the device.
    fn setvoltage(&mut self, voltage: u32) {
        use voltages::*;

        // Get the core voltage.
        let core = CoreVoltage::from(voltage);

        // Get the LDO AO and LDO AO Boost voltages.
        let (ldoao, boost) = AOVoltage::from(core);

        // Setup Always-On domain LDO.
        unsafe {
            // Get the pointer to the register.
            let ptr = Self::PMC + Self::LDOPMU;

            // Read the register.
            let mut r = read(ptr as *const u32);

            // Clear the values and set the voltage.
            r &= !((0x1F << 10) | 0x1F);
            r |= ((boost as u32) << 10) | (ldoao as u32);

            // Write the modified value.
            write(ptr as *mut u32, r);
        }

        // Setup core voltage.
        unsafe {
            // Get the pointer to the register.
            let ptr = Self::PMC + Self::DCDC0;

            // Read the register.
            let mut r = read(ptr as *const u32);

            // Clear the values and set the voltage.
            r &= !(0xF << 17);
            r |= (core as u32) << 17;

            // Write the modified value.
            write(ptr as *mut u32, r);
        }
    }
}
