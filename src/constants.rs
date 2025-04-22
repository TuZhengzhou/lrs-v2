// Information of LRS circuit for the CircDescriptor
pub const NUM_PUB_IO_LRS_SE: usize = 2;
pub const NUM_COMMIT_WITNESS_LRS_SE: usize = 1;
pub const IOPUTS_NAME_LRS_SE: [&str; 4] = ["main.sc", "main.L", "main.phi", "main.sk"];
pub const PATH_PREFIX_LRS_SE: &str = "./circoms/lrs/";
pub const CIRCUIT_NAME_LRS_SE: &str = "lrs";

pub const CIRCUIT_NAME_SCHNORR_VERIFY: &str = "schnorr_verify";

macro_rules! define_input_and_circuit_names_lrs_a {
    ($name1:ident, $name2:ident, $depth:tt, [$( $i:tt ),*]) => {
        pub const $name1: [&str; {11 + 2 * $depth}] = [
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
            $(
                concat!("main.pathElements[", stringify!($i), "]"),
            )*
            $(
                concat!("main.pathIndices[", stringify!($i), "]"),
            )*
        ];
        pub const $name2: &str = concat!("lrs_a_", stringify!($depth));
    };
}

pub const NUM_PUB_IO_LRS_A: usize = 7;
pub const NUM_COMMIT_WITNESS_LRS_A: usize = 0;
pub const PATH_PREFIX_LRS_A: &str = "./circoms/lrs_a/lrs_a/lrs_a_circoms/";
define_input_and_circuit_names_lrs_a!(IOPUTS_NAME_LRS_A_3, CIRCUIT_NAME_LRS_A_3, 3, [0, 1, 2]);
define_input_and_circuit_names_lrs_a!(IOPUTS_NAME_LRS_A_4, CIRCUIT_NAME_LRS_A_4, 4, [0, 1, 2, 3]);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_5,
    CIRCUIT_NAME_LRS_A_5,
    5,
    [0, 1, 2, 3, 4]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_6,
    CIRCUIT_NAME_LRS_A_6,
    6,
    [0, 1, 2, 3, 4, 5]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_7,
    CIRCUIT_NAME_LRS_A_7,
    7,
    [0, 1, 2, 3, 4, 5, 6]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_8,
    CIRCUIT_NAME_LRS_A_8,
    8,
    [0, 1, 2, 3, 4, 5, 6, 7]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_9,
    CIRCUIT_NAME_LRS_A_9,
    9,
    [0, 1, 2, 3, 4, 5, 6, 7, 8]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_10,
    CIRCUIT_NAME_LRS_A_10,
    10,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_11,
    CIRCUIT_NAME_LRS_A_11,
    11,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_12,
    CIRCUIT_NAME_LRS_A_12,
    12,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_13,
    CIRCUIT_NAME_LRS_A_13,
    13,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_14,
    CIRCUIT_NAME_LRS_A_14,
    14,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_15,
    CIRCUIT_NAME_LRS_A_15,
    15,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_16,
    CIRCUIT_NAME_LRS_A_16,
    16,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_17,
    CIRCUIT_NAME_LRS_A_17,
    17,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_18,
    CIRCUIT_NAME_LRS_A_18,
    18,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_19,
    CIRCUIT_NAME_LRS_A_19,
    19,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
);
define_input_and_circuit_names_lrs_a!(
    IOPUTS_NAME_LRS_A_20,
    CIRCUIT_NAME_LRS_A_20,
    20,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
);

pub const NUM_PUB_IO_LRS_B: usize = 14;
pub const NUM_COMMIT_WITNESS_LRS_B: usize = 1;
pub const IOPUTS_NAME_LRS_B: [&str; 19] = [
    "main.sc",
    "main.L",
    "main.c[0][0]",
    "main.c[0][1]",
    "main.c[0][2]",
    "main.c[0][3]",
    "main.c[0][4]",
    "main.c[0][5]",
    "main.c[1][0]",
    "main.c[1][1]",
    "main.c[1][2]",
    "main.c[1][3]",
    "main.c[1][4]",
    "main.c[1][5]",
    "main.phi",
    "main.sk",
    "main.pk[0]",
    "main.pk[1]",
    "main.r",
];
pub const PATH_PREFIX_LRS_B: &str = "./circoms/lrs_b/";
pub const CIRCUIT_NAME_LRS_B: &str = "lrs_b";
