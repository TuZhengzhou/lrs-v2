pragma circom 2.2.0;

include "escalarmul.circom";
include "mimc.circom";
include "babyjub.circom";


template Main() {
    signal input sk;
    signal input phi;
    signal input sc;
    signal input L;

    var base[2] = [3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507,
            1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569];
    
    //component check = BabyCheck();
    //check.x <== base[0];
    //check.y <== base[1];


    component n2b = Num2Bits(253);
    component escalarMul = EscalarMul(253, base);

    escalarMul.inp[0] <== 0;
    escalarMul.inp[1] <== 1;

    var i;

    sk ==> n2b.in;

    for  (i=0; i<253; i++) {
        n2b.out[i] ==> escalarMul.in[i];
    }

   component h_phi = MultiMiMC7(2, 91);
   h_phi.in[0] <== escalarMul.out[0];
   h_phi.in[1] <== escalarMul.out[1];
   h_phi.k <== 1;
   phi === h_phi.out;

   component h_L = MultiMiMC7(2, 91);
   h_L.in[0] <== sk;
   h_L.in[1] <== sc;
   h_L.k <== 1;
   L === h_L.out;
}

component main = Main();