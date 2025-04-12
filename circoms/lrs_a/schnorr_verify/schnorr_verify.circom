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

    var base[2] = [3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507,
                1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569];

    
    component Pke = EscalarMul(253, base);
    Pke.inp[0] <== 0;
    Pke.inp[1] <== 1;

    component n2b = Num2Bits(253);
    n2b.in <== sk_e;
    for  (var i=0; i<253; i++) {
        Pke.in[i] <== n2b.out[i];
    }

    component Adder = BabyAdd();
    Adder.x1 <== Pke.out[0];
    Adder.y1 <== Pke.out[1];
    Adder.x2 <== R[0];
    Adder.y2 <== R[1];
    
    // 3. g^s
    component Gs = EscalarMul(253, base);
    Gs.inp[0] <== 0;
    Gs.inp[1] <== 1;
    component Sn2b = Num2Bits(253);
    Sn2b.in <== s;
    for (var i=0; i<253; i++) {
        Gs.in[i] <== Sn2b.out[i];
    }

    Adder.xout === Gs.out[0];
    Adder.yout === Gs.out[1];
}

component main = SchnorrVerify();