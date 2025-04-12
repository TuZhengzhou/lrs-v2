pragma circom 2.2.0;

include "./include/escalarmul.circom";
include "./include/mimc.circom";



template Main() {
    signal input sk;
    signal input phi;
    signal input sc;
    signal input L;

    var base[2] = [
        5299619240641551281634865583518297030282874472190772894086521144482721001553,
        16950150798460657717958625567821834550301663161624707787222815936182638968203
    ];

    component n2b = Num2Bits(253);
    component PKComp = EscalarMul(253, base);

    PKComp.inp[0] <== 0;
    PKComp.inp[1] <== 1;

    var i;

    sk ==> n2b.in;

    for  (i=0; i<253; i++) {
        n2b.out[i] ==> PKComp.in[i];
    }

    component h_phi = MultiMiMC7(2, 91);
    h_phi.in[0] <== PKComp.out[0];
    h_phi.in[1] <== PKComp.out[1];
    h_phi.k <== 1;
    phi === h_phi.out;

    component h_L = MultiMiMC7(2, 91);
    h_L.in[0] <== sk;
    h_L.in[1] <== sc;
    h_L.k <== 1;
    L === h_L.out;
}

component main = Main();