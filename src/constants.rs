pub(crate) const NUM_CHALLENGE_BITS: usize = 128;
pub(crate) const NUM_HASH_BITS: usize = 250;
pub(crate) const BN_LIMB_WIDTH: usize = 64;
pub(crate) const BN_N_LIMBS: usize = 4;
// pub(crate) const NUM_FE_WITHOUT_IO_FOR_CRHF: usize = 17;
// pub(crate) const NUM_FE_FOR_RO: usize = 24;
// Arasu: I think this needs to be changed? 
// pub(crate) const NUM_FE_WITHOUT_IO_FOR_CRHF: usize = 20;
// pub(crate) const NUM_FE_FOR_RO: usize = 30;

// After adding C_star to state: 
pub(crate) const NUM_FE_WITHOUT_IO_FOR_CRHF: usize = 21;
pub(crate) const NUM_FE_FOR_RO: usize = 30;