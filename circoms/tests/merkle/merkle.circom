pragma circom 2.2.0;

include "../include/mimcsponge.circom";
include "../include/mimc.circom";
include "../include/bitify.circom";
include "../include/escalarmulfix.circom";
include "../include/babyjub.circom";
include "../include/montgomery.circom";
include "../include/escalarmul.circom";

// if s == 0 returns [in[0], in[1]]
// if s == 1 returns [in[1], in[0]]
template DualMux() {
    signal input in[2];
    signal input s;
    signal output out[2];

    s * (1 - s) === 0;
    out[0] <== (in[1] - in[0])*s + in[0];
    out[1] <== (in[0] - in[1])*s + in[1];
}

// EdDSA/MiMC 签名验证 (基于 BabyJubjub)
template MerkleTreeChecker(levels) {
    signal input leaf;
    signal input pathElements[levels];
    signal input pathIndices[levels];
    signal output root;

    component selectors[levels];
    component hashers[levels];

    selectors[0] = DualMux();
    selectors[0].in[0] <== leaf;
    selectors[0].in[1] <== pathElements[0];
    selectors[0].s <== pathIndices[0];

    hashers[0] = MultiMiMC7(2, 91);
    hashers[0].k <== 1;
    hashers[0].in[0] <== selectors[0].out[0];
    hashers[0].in[1] <== selectors[0].out[1];

    for (var i = 1; i < levels; i++) {
        selectors[i] = DualMux();
        selectors[i].in[0] <== hashers[i - 1].out;
        selectors[i].in[1] <== pathElements[i];
        selectors[i].s <== pathIndices[i];

        hashers[i] = MultiMiMC7(2, 91);
        hashers[i].k <== 1;
        hashers[i].in[0] <== selectors[i].out[0];
        hashers[i].in[1] <== selectors[i].out[1];
    }

    root <== hashers[levels - 1].out;
}

component main = MerkleTreeChecker(15);