pragma circom 2.2.0;

include "../include/mimcsponge.circom";
include "../include/mimc.circom";
include "../include/bitify.circom";
include "../include/escalarmulfix.circom";
include "../include/babyjub.circom";
include "../include/montgomery.circom";
include "../include/escalarmul.circom";

template PhiTMP() {
    signal input sk;
    signal output phi;
    
    component Gsk = BabyPbk();
    Gsk.in <== sk;

    component phiHash = MultiMiMC7(2, 91);
    phiHash.in[0] <== Gsk.Ax;
    phiHash.in[1] <== Gsk.Ay;
    phiHash.k <== 1;

    phi <== phiHash.out;
}
component main = PhiTMP();