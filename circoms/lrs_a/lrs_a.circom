pragma circom 2.2.0;

include "include/mimcsponge.circom";
include "include/mimc.circom";
include "include/bitify.circom";
include "include/escalarmulfix.circom";
include "include/babyjub.circom";
include "include/montgomery.circom";
include "include/escalarmul.circom";

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

// Verifies that merkle proof is correct for given merkle root and a leaf
// pathIndices input is an array of 0/1 selectors telling whether given pathElement is on the left or right side of merkle path

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

// EdDSA/MiMC 签名验证 (基于 BabyJubjub)
template SchnorrVerify(){
    signal input sk;
    signal input pk[2];
    signal input R[2];
    signal input s;
    signal input msg;

    //signal output verified;

    // 1. 计算 e = H(msg || R || P)
    component eHash1 = MultiMiMC7(5, 91);
    eHash1.in[0] <== msg;
    eHash1.in[1] <== R[0];
    eHash1.in[2] <== R[1];
    eHash1.in[3] <== pk[0];
    eHash1.in[4] <== pk[1];
    eHash1.k <== 1;

    // 能不能直接乘 ? 打个问号
    signal sk_e <== eHash1.out * sk;

    component PKEComp = BabyPbk();
    PKEComp.in <== sk_e;

    component Adder = BabyAdd();
    Adder.x1 <== PKEComp.Ax;
    Adder.y1 <== PKEComp.Ay;
    Adder.x2 <== R[0];
    Adder.y2 <== R[1];
    
    // 3. g^s
    component GsComp = BabyPbk();
    GsComp.in <== s;

    Adder.xout === GsComp.Ax;
    Adder.yout === GsComp.Ay;
}

template SNARKCircuitA(levels){
    signal input msg;
    signal input sk;
    signal input phi;
    signal input sc;
    signal input L;
    signal input root;
    signal input pathElements[levels];
    signal input pathIndices[levels];
    signal input R[2];
    signal input s;

    component RCheck = BabyCheck();
    RCheck.x <== R[0];
    RCheck.y <== R[1];
    
    // 1. 计算 pk = g^sk
    component PKComp = BabyPbk();
    PKComp.in <== sk;
    signal pk[2];
    pk[0] <== PKComp.Ax;
    pk[1] <== PKComp.Ay;

    // 2. 计算 ϕ = MiMC(g^sk)
    component phiHash = MultiMiMC7(2, 91);
    phiHash.in[0] <== PKComp.Ax;
    phiHash.in[1] <== PKComp.Ay;
    phiHash.k <== 1;

    phi === phiHash.out;

    // 3. 计算 L = MiMC(sk, sc)
    component LHash = MultiMiMC7(2, 91);
    LHash.in[0] <== sk;
    LHash.in[1] <== sc;
    LHash.k <== 1;

    L === LHash.out;

    // // 4. Merkle证明验证
    component merkleProof = MerkleTreeChecker(levels);
    merkleProof.leaf <== phi;
    for (var i = 0; i < levels; i++) {
        merkleProof.pathElements[i] <== pathElements[i];
        merkleProof.pathIndices[i] <== pathIndices[i];
    }
    root === merkleProof.root;

    // 5. Schnorr签名验证
    component schnorr = SchnorrVerify();
    schnorr.sk <== sk;
    schnorr.pk <== pk;
    schnorr.R <== R;
    schnorr.s <== s;
    schnorr.msg <== msg;
}
