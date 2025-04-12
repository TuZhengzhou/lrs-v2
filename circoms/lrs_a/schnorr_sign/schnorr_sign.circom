pragma circom 2.2.0;

include "../include/mimcsponge.circom";
include "../include/mimc.circom";
include "../include/bitify.circom";
include "../include/escalarmulfix.circom";
include "../include/babyjub.circom";
include "../include/montgomery.circom";
include "../include/escalarmul.circom";

// EdDSA/MiMC 签名验证 (基于 BabyJubjub)
template SchnorrSign(){
    signal input sk;
    signal input k;
    signal input msg;
    signal output pk[2];
    signal output R[2];
    signal output s;

    component PKComp = BabyPbk();
    PKComp.in <== sk;
    pk[0] <== PKComp.Ax;
    pk[1] <== PKComp.Ay;

    component RComp = BabyPbk();
    RComp.in <== k;
    R[0] <== RComp.Ax;
    R[1] <== RComp.Ay;

    // 1. 计算 e = H(msg || R || P)
    component eHash1 = MultiMiMC7(5, 91);
    eHash1.in[0] <== msg;
    eHash1.in[1] <== RComp.Ax;
    eHash1.in[2] <== RComp.Ay;
    eHash1.in[3] <== PKComp.Ax;
    eHash1.in[4] <== PKComp.Ay;
    eHash1.k <== 1;

    s <== k + eHash1.out * sk;

    signal sk_e;
    sk_e <== eHash1.out * sk;

    component PKEComp = BabyPbk();
    PKEComp.in <== sk_e;

    component check_Pke = BabyCheck();
    check_Pke.x <== PKEComp.Ax; 
    check_Pke.y <== PKEComp.Ay;

    component check_R = BabyCheck();
    check_R.x <== R[0]; 
    check_R.y <== R[1]; 

    component Adder = BabyAdd();
    Adder.x1 <== PKEComp.Ax;
    Adder.y1 <== PKEComp.Ay;
    Adder.x2 <== R[0];
    Adder.y2 <== R[1];

    // 3. g^s
    component Gs = BabyPbk();
    Gs.in <== s;

    Adder.xout === Gs.Ax;
    Adder.yout === Gs.Ay;
}

component main = SchnorrSign();