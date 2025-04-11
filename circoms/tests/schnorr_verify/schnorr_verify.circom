pragma circom 2.2.0;

include "../include/mimcsponge.circom";
include "../include/mimc.circom";
include "../include/bitify.circom";
include "../include/escalarmulfix.circom";
include "../include/babyjub.circom";
include "../include/montgomery.circom";
include "../include/escalarmul.circom";

// EdDSA/MiMC 签名验证 (基于 BabyJubjub)
template SchnorrVerify(){
    signal input sk;
    signal input pk[2];
    signal input R[2];
    signal input s;
    signal input msg;

    component check_R = BabyCheck();
    check_R.x <== R[0]; 
    check_R.y <== R[1]; 

    component check_pk = BabyCheck();
    check_pk.x <== pk[0]; 
    check_pk.y <== pk[1]; 

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

    component Pke = BabyPbk();
    Pke.in <== sk_e;
    
    component Adder = BabyAdd();
    Adder.x1 <== Pke.Ax;
    Adder.y1 <== Pke.Ay;
    Adder.x2 <== R[0];
    Adder.y2 <== R[1];
    
    // 3. g^s
    component Gs = BabyPbk();
    Gs.in <== s;

    Adder.xout === Gs.Ax;
    Adder.yout === Gs.Ay;
}

component main = SchnorrVerify();