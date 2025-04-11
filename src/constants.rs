// Information of LRS circuit for the CircDescriptor
pub const NUM_PUB_IO_LRS_SE: usize = 2;
pub const NUM_COMMIT_WITNESS_LRS_SE: usize = 1;
pub const IOPUTS_NAME_LRS_SE : [&str; 4] = [
    "main.sc",
    "main.L",
    "main.phi",
    "main.sk",
];
pub const PATH_PREFIX_LRS_SE: &str = "./circoms/lrs/";
pub const CIRCUIT_NAME_LRS_SE: &str = "lrs";

pub const NUM_PUB_IO_LRS_A: usize = 7;
pub const NUM_COMMIT_WITNESS_LRS_A: usize = 0;
pub const IOPUTS_NAME_LRS_A: [&str; 41] = [
    "main.root",
    "main.sc",
    "main.L",
    "main.msg",
    "main.R[0]",
    "main.R[1]",
    "main.s",
    "main.sk",
    "main.pk[0]",
    "main.pk[1]",
    "main.phi",
    "main.pathElements[0]",
    "main.pathElements[1]",
    "main.pathElements[2]",
    "main.pathElements[3]",
    "main.pathElements[4]",
    "main.pathElements[5]",
    "main.pathElements[6]",
    "main.pathElements[7]",
    "main.pathElements[8]",
    "main.pathElements[9]",
    "main.pathElements[10]",
    "main.pathElements[11]",
    "main.pathElements[12]",
    "main.pathElements[13]",
    "main.pathElements[14]",
    "main.pathIndices[0]",
    "main.pathIndices[1]",
    "main.pathIndices[2]",
    "main.pathIndices[3]",
    "main.pathIndices[4]",
    "main.pathIndices[5]",
    "main.pathIndices[6]",
    "main.pathIndices[7]",
    "main.pathIndices[8]",
    "main.pathIndices[9]",
    "main.pathIndices[10]",
    "main.pathIndices[11]",
    "main.pathIndices[12]",
    "main.pathIndices[13]",
    "main.pathIndices[14]",
];
pub const PATH_PREFIX_LRS_A: &str = "./circoms/lrs_a/";
pub const CIRCUIT_NAME_LRS_A: &str = "lrs_a";

// Information of Merkle circuit for the CircDescriptor
pub const NUM_PUB_IO_MERKLE: usize = 2;
pub const NUM_COMMIT_WITNESS_MERKLE: usize = 2;
pub const IOPUTS_NAME_MERKLE: [&str; 32] = [
    "main.root",
    "main.leaf",
    "main.pathElements[0]",
    "main.pathElements[1]",
    "main.pathElements[2]",
    "main.pathElements[3]",
    "main.pathElements[4]",
    "main.pathElements[5]",
    "main.pathElements[6]",
    "main.pathElements[7]",
    "main.pathElements[8]",
    "main.pathElements[9]",
    "main.pathElements[10]",
    "main.pathElements[11]",
    "main.pathElements[12]",
    "main.pathElements[13]",
    "main.pathElements[14]",
    "main.pathIndices[0]",
    "main.pathIndices[1]",
    "main.pathIndices[2]",
    "main.pathIndices[3]",
    "main.pathIndices[4]",
    "main.pathIndices[5]",
    "main.pathIndices[6]",
    "main.pathIndices[7]",
    "main.pathIndices[8]",
    "main.pathIndices[9]",
    "main.pathIndices[10]",
    "main.pathIndices[11]",
    "main.pathIndices[12]",
    "main.pathIndices[13]",
    "main.pathIndices[14]",
];
pub const PATH_PREFIX_MERKLE: &str = "./circoms/tests/merkle/";
pub const CIRCUIT_NAME_MERKLE: &str = "merkle";

// Information of SchnorrSign circuit for the CircDescriptor
pub const NUM_PUB_IO_SCHNORR_SIGN: usize = 4;
pub const NUM_COMMIT_WITNESS_SCHNORR_SIGN: usize = 1;
pub const IOPUTS_NAME_SCHNORR_SIGN: [&str; 8] = [
    "main.msg",
    "main.R[0]",
    "main.R[1]",
    "main.s",
    "main.sk",
    "main.pk[0]",
    "main.pk[1]",
    "main.k",
];
pub const PATH_PREFIX_SCHNORR_SIGN: &str = "./circoms/tests/schnorr_sign/";
pub const CIRCUIT_NAME_SCHNORR_SIGN: &str = "schnorr_sign";

// Information of SchnorrVerify circuit for the CircDescriptor
pub const NUM_PUB_IO_SCHNORR_VERIFY: usize = 6;
pub const NUM_COMMIT_WITNESS_SCHNORR_VERIFY: usize = 0;
pub const IOPUTS_NAME_SCHNORR_VERIFY: [&str; 7] = [
    "main.msg",
    "main.R[0]",
    "main.R[1]",
    "main.s",
    "main.pk[0]",
    "main.pk[1]",
    "main.sk",
];
pub const PATH_PREFIX_SCHNORR_VERIFY: &str = "./circoms/tests/schnorr_verify/";
pub const CIRCUIT_NAME_SCHNORR_VERIFY: &str = "schnorr_verify";