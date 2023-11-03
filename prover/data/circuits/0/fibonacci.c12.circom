pragma circom 2.1.0;
pragma custom_templates;

include "cmuladd.circom";
include "cinv.circom";
include "poseidon.circom";
include "bitify.circom";
include "fft.circom";
include "merklehash.circom";
include "evalpol.circom";
include "treeselector.circom";

template VerifyEvaluations() {
    signal input challenges[8][3];
    signal input evals[8][3];
    signal input publics[3];
    signal input enable;

    signal zMul[10][3];
    
    for (var i=0; i< 10; i++) {
        if (i==0) {
            zMul[i] <== CMul()(challenges[7], challenges[7]);
        } else {
            zMul[i] <== CMul()(zMul[i-1], zMul[i-1]);
        }
    }
        
    signal Z[3];

    Z[0] <== zMul[9][0] -1;
    Z[1] <== zMul[9][1];
    Z[2] <== zMul[9][2];
    signal tmp_0[3] <== [evals[0][0] - evals[1][0], evals[0][1] - evals[1][1], evals[0][2] - evals[1][2]];
    signal tmp_1[3] <== [1 - evals[2][0], -evals[2][1], -evals[2][2]];
    signal tmp_2[3] <== CMul()(tmp_0, tmp_1);
    signal tmp_19[3] <== [tmp_2[0] - 0, tmp_2[1], tmp_2[2]];
    signal tmp_3[3] <== [evals[3][0] - evals[4][0], evals[3][1] - evals[4][1], evals[3][2] - evals[4][2]];
    signal tmp_4[3] <== [1 - evals[2][0], -evals[2][1], -evals[2][2]];
    signal tmp_5[3] <== CMul()(tmp_3, tmp_4);
    signal tmp_20[3] <== [tmp_5[0] - 0, tmp_5[1], tmp_5[2]];
    signal tmp_6[3] <== [evals[5][0] - publics[0], evals[5][1], evals[5][2]];
    signal tmp_7[3] <== CMul()(evals[6], tmp_6);
    signal tmp_21[3] <== [tmp_7[0] - 0, tmp_7[1], tmp_7[2]];
    signal tmp_8[3] <== [evals[1][0] - publics[1], evals[1][1], evals[1][2]];
    signal tmp_9[3] <== CMul()(evals[6], tmp_8);
    signal tmp_22[3] <== [tmp_9[0] - 0, tmp_9[1], tmp_9[2]];
    signal tmp_10[3] <== [evals[1][0] - publics[2], evals[1][1], evals[1][2]];
    signal tmp_11[3] <== CMul()(evals[2], tmp_10);
    signal tmp_23[3] <== [tmp_11[0] - 0, tmp_11[1], tmp_11[2]];
    signal tmp_12[3] <== CMulAdd()(challenges[4], tmp_19, tmp_20);
    signal tmp_13[3] <== CMulAdd()(challenges[4], tmp_12, tmp_21);
    signal tmp_14[3] <== CMulAdd()(challenges[4], tmp_13, tmp_22);
    signal tmp_15[3] <== CMulAdd()(challenges[4], tmp_14, tmp_23);
    signal tmp_16[3] <== CMul()(evals[5], evals[5]);
    signal tmp_17[3] <== CMulAdd()(evals[1], evals[1], tmp_16);
    signal tmp_18[3] <== [tmp_17[0] - evals[4][0], tmp_17[1] - evals[4][1], tmp_17[2] - evals[4][2]];
    signal tmp_24[3] <== CMulAdd()(challenges[4], tmp_15, tmp_18);
    signal xN[3] <== zMul[9];

    signal xAcc[1][3];
    signal qStep[0][3];
    signal qAcc[1][3];
    for (var i=0; i< 1; i++) {
        if (i==0) {
            xAcc[0] <== [1, 0, 0];
            qAcc[0] <== evals[7+i];
        } else {
            xAcc[i] <== CMul()(xAcc[i-1], xN);
            qStep[i-1] <== CMul()(xAcc[i], evals[7+i]);

            qAcc[i][0] <== qAcc[i-1][0] + qStep[i-1][0];
            qAcc[i][1] <== qAcc[i-1][1] + qStep[i-1][1];
            qAcc[i][2] <== qAcc[i-1][2] + qStep[i-1][2];
        }
    }
    signal qZ[3] <== CMul()(qAcc[0], Z);

// Final Verification
    enable * (tmp_24[0] - qZ[0]) === 0;
    enable * (tmp_24[1] - qZ[1]) === 0;
    enable * (tmp_24[2] - qZ[2]) === 0;
}
        
template parallel VerifyQuery() {
    signal input ys[11];
    signal input challenges[8][3];
    signal input evals[8][3];
    signal input tree1[2];
    
    signal input tree3[1];
            
    signal input tree4[3];
    signal input consts[2];
    signal output out[3];
        
    component mapValues = MapValues();

    for (var i=0; i< 2; i++ ) {
        mapValues.vals1[i] <== tree1[i];
    }
    for (var i=0; i< 1; i++ ) {
        mapValues.vals3[i] <== tree3[i];
    }
    for (var i=0; i< 3; i++ ) {
        mapValues.vals4[i] <== tree4[i];
    }
    signal xacc[11];
    xacc[0] <== ys[0]*(49 * roots(11)-49) + 49;
    for (var i=1; i<11; i++ ) {
        xacc[i] <== xacc[i-1] * ( ys[i]*(roots(11 - i) - 1) +1);
    }
    component den1inv = CInv();
    den1inv.in[0] <== xacc[10] - challenges[7][0];
    den1inv.in[1] <== -challenges[7][1];
    den1inv.in[2] <== -challenges[7][2];
    signal xDivXSubXi[3];
    xDivXSubXi[0] <== xacc[10] * den1inv.out[0];
    xDivXSubXi[1] <== xacc[10] * den1inv.out[1];
    xDivXSubXi[2] <== xacc[10] * den1inv.out[2];
    
    component den2inv = CInv();
    den2inv.in[0] <== xacc[10] - roots(10)*challenges[7][0];
    den2inv.in[1] <== -roots(10)*challenges[7][1];
    den2inv.in[2] <== -roots(10)*challenges[7][2];
    signal xDivXSubWXi[3];
    xDivXSubWXi[0] <== xacc[10] * den2inv.out[0];
    xDivXSubWXi[1] <== xacc[10] * den2inv.out[1];
    xDivXSubWXi[2] <== xacc[10] * den2inv.out[2];
    
    signal tmp_0[3] <== [challenges[5][0] * mapValues.tree1_0 + mapValues.tree1_1, challenges[5][1] * mapValues.tree1_0, challenges[5][2] * mapValues.tree1_0];
    signal tmp_1[3] <== CMulAdd()(challenges[5], tmp_0, [mapValues.tree3_0, 0, 0]);
    signal tmp_2[3] <== CMulAdd()(challenges[5], tmp_1, mapValues.tree4_0);
    signal tmp_3[3] <== [mapValues.tree1_0 - evals[1][0], -evals[1][1], -evals[1][2]];
    signal tmp_4[3] <== [consts[1] - evals[2][0], -evals[2][1], -evals[2][2]];
    signal tmp_5[3] <== CMulAdd()(tmp_3, challenges[6], tmp_4);
    signal tmp_6[3] <== [mapValues.tree3_0 - evals[4][0], -evals[4][1], -evals[4][2]];
    signal tmp_7[3] <== CMulAdd()(tmp_5, challenges[6], tmp_6);
    signal tmp_8[3] <== [mapValues.tree1_1 - evals[5][0], -evals[5][1], -evals[5][2]];
    signal tmp_9[3] <== CMulAdd()(tmp_7, challenges[6], tmp_8);
    signal tmp_10[3] <== [consts[0] - evals[6][0], -evals[6][1], -evals[6][2]];
    signal tmp_11[3] <== CMulAdd()(tmp_9, challenges[6], tmp_10);
    signal tmp_12[3] <== [mapValues.tree4_0[0] - evals[7][0], mapValues.tree4_0[1] - evals[7][1], mapValues.tree4_0[2] - evals[7][2]];
    signal tmp_13[3] <== CMulAdd()(tmp_11, challenges[6], tmp_12);
    signal tmp_14[3] <== CMul()(tmp_13, xDivXSubXi);
    signal tmp_15[3] <== CMulAdd()(challenges[5], tmp_2, tmp_14);
    signal tmp_16[3] <== [mapValues.tree1_1 - evals[0][0], -evals[0][1], -evals[0][2]];
    signal tmp_17[3] <== [mapValues.tree1_0 - evals[3][0], -evals[3][1], -evals[3][2]];
    signal tmp_18[3] <== CMulAdd()(tmp_16, challenges[6], tmp_17);
    signal tmp_19[3] <== CMul()(tmp_18, xDivXSubWXi);
    signal tmp_20[3] <== CMulAdd()(challenges[5], tmp_15, tmp_19);
    out[0] <== tmp_20[0];
    out[1] <== tmp_20[1];
    out[2] <== tmp_20[2];
}
    
template MapValues() {
    signal input vals1[2];
    signal input vals3[1];
    signal input vals4[3];
    signal output tree1_0;
    signal output tree1_1;
    signal output tree3_0;
    signal output tree4_0[3];
    tree1_0 <== vals1[0];
    tree1_1 <== vals1[1];
    tree3_0 <== vals3[0];
    tree4_0[0] <== vals4[0];
    tree4_0[1] <== vals4[1];
    tree4_0[2] <== vals4[2];
}
template StarkVerifier() {
    signal input publics[3];
    signal input root1[4];
    signal input root2[4];
    signal input root3[4];
    signal input root4[4];

    signal rootC[4];
    rootC[0] <== 8072859658275330050;
    rootC[1] <== 6129740704102247485;
    rootC[2] <== 16008196867495226449;
    rootC[3] <== 2863018592730207411;

    signal input evals[8][3];
    signal input s0_vals1[8][2];
    
    signal input s0_vals3[8][1];
        
    signal input s0_vals4[8][3];
    signal input s0_valsC[8][2];
    signal input s0_siblings1[8][11][4];

    signal input s0_siblings3[8][11][4];
        
    signal input s0_siblings4[8][11][4];
    signal input s0_siblingsC[8][11][4];
        
    signal input s1_root[4];
        
    signal input s2_root[4];
        
    signal input s1_vals[8][48];
    signal input s1_siblings[8][7][4];
        
    signal input s2_vals[8][48];
    signal input s2_siblings[8][3][4];
        
    signal input finalPol[8][3];
    
    signal enable;
    enable <== 1;
    
    signal challenges[8][3];
    
    signal s0_specialX[3];
    
    signal s1_specialX[3];
    
    signal s2_specialX[3];
    
    signal ys[8][11];
        signal tcHahs_0[12] <==  Poseidon(12)([publics[0],publics[1],publics[2],root1[0],root1[1],root1[2],root1[3],0], [0,0,0,0]);
    challenges[0][0] <== tcHahs_0[0];
    challenges[0][1] <== tcHahs_0[1];
    challenges[0][2] <== tcHahs_0[2];
    challenges[1][0] <== tcHahs_0[3];
    challenges[1][1] <== tcHahs_0[4];
    challenges[1][2] <== tcHahs_0[5];
    signal tcHahs_1[12] <==  Poseidon(12)([root2[0],root2[1],root2[2],root2[3],0,0,0,0], [tcHahs_0[0],tcHahs_0[1],tcHahs_0[2],tcHahs_0[3]]);
    challenges[2][0] <== tcHahs_1[0];
    challenges[2][1] <== tcHahs_1[1];
    challenges[2][2] <== tcHahs_1[2];
    challenges[3][0] <== tcHahs_1[3];
    challenges[3][1] <== tcHahs_1[4];
    challenges[3][2] <== tcHahs_1[5];
    signal tcHahs_2[12] <==  Poseidon(12)([root3[0],root3[1],root3[2],root3[3],0,0,0,0], [tcHahs_1[0],tcHahs_1[1],tcHahs_1[2],tcHahs_1[3]]);
    challenges[4][0] <== tcHahs_2[0];
    challenges[4][1] <== tcHahs_2[1];
    challenges[4][2] <== tcHahs_2[2];
    signal tcHahs_3[12] <==  Poseidon(12)([root4[0],root4[1],root4[2],root4[3],0,0,0,0], [tcHahs_2[0],tcHahs_2[1],tcHahs_2[2],tcHahs_2[3]]);
    challenges[7][0] <== tcHahs_3[0];
    challenges[7][1] <== tcHahs_3[1];
    challenges[7][2] <== tcHahs_3[2];
    signal tcHahs_4[12] <== Poseidon(12)([evals[0][0],evals[0][1],evals[0][2],evals[1][0],evals[1][1],evals[1][2],evals[2][0],evals[2][1]], [tcHahs_3[0],tcHahs_3[1],tcHahs_3[2],tcHahs_3[3]]);
    signal tcHahs_5[12] <== Poseidon(12)([evals[2][2],evals[3][0],evals[3][1],evals[3][2],evals[4][0],evals[4][1],evals[4][2],evals[5][0]], [tcHahs_4[0],tcHahs_4[1],tcHahs_4[2],tcHahs_4[3]]);
    signal tcHahs_6[12] <== Poseidon(12)([evals[5][1],evals[5][2],evals[6][0],evals[6][1],evals[6][2],evals[7][0],evals[7][1],evals[7][2]], [tcHahs_5[0],tcHahs_5[1],tcHahs_5[2],tcHahs_5[3]]);
    challenges[5][0] <== tcHahs_6[0];
    challenges[5][1] <== tcHahs_6[1];
    challenges[5][2] <== tcHahs_6[2];
    challenges[6][0] <== tcHahs_6[3];
    challenges[6][1] <== tcHahs_6[4];
    challenges[6][2] <== tcHahs_6[5];
    s0_specialX[0] <== tcHahs_6[6];
    s0_specialX[1] <== tcHahs_6[7];
    s0_specialX[2] <== tcHahs_6[8];
    signal tcHahs_7[12] <==  Poseidon(12)([s1_root[0],s1_root[1],s1_root[2],s1_root[3],0,0,0,0], [tcHahs_6[0],tcHahs_6[1],tcHahs_6[2],tcHahs_6[3]]);
    s1_specialX[0] <== tcHahs_7[0];
    s1_specialX[1] <== tcHahs_7[1];
    s1_specialX[2] <== tcHahs_7[2];
    signal tcHahs_8[12] <==  Poseidon(12)([s2_root[0],s2_root[1],s2_root[2],s2_root[3],0,0,0,0], [tcHahs_7[0],tcHahs_7[1],tcHahs_7[2],tcHahs_7[3]]);
    s2_specialX[0] <== tcHahs_8[0];
    s2_specialX[1] <== tcHahs_8[1];
    s2_specialX[2] <== tcHahs_8[2];
    signal tcHahs_9[12] <== Poseidon(12)([finalPol[0][0],finalPol[0][1],finalPol[0][2],finalPol[1][0],finalPol[1][1],finalPol[1][2],finalPol[2][0],finalPol[2][1]], [tcHahs_8[0],tcHahs_8[1],tcHahs_8[2],tcHahs_8[3]]);
    signal tcHahs_10[12] <== Poseidon(12)([finalPol[2][2],finalPol[3][0],finalPol[3][1],finalPol[3][2],finalPol[4][0],finalPol[4][1],finalPol[4][2],finalPol[5][0]], [tcHahs_9[0],tcHahs_9[1],tcHahs_9[2],tcHahs_9[3]]);
    signal tcHahs_11[12] <== Poseidon(12)([finalPol[5][1],finalPol[5][2],finalPol[6][0],finalPol[6][1],finalPol[6][2],finalPol[7][0],finalPol[7][1],finalPol[7][2]], [tcHahs_10[0],tcHahs_10[1],tcHahs_10[2],tcHahs_10[3]]);
    component tcN2b_0 = Num2Bits_strict();
    tcN2b_0.in <== tcHahs_11[0];
    component tcN2b_1 = Num2Bits_strict();
    tcN2b_1.in <== tcHahs_11[1];
    ys[0][0] <== tcN2b_0.out[0];
    ys[0][1] <== tcN2b_0.out[1];
    ys[0][2] <== tcN2b_0.out[2];
    ys[0][3] <== tcN2b_0.out[3];
    ys[0][4] <== tcN2b_0.out[4];
    ys[0][5] <== tcN2b_0.out[5];
    ys[0][6] <== tcN2b_0.out[6];
    ys[0][7] <== tcN2b_0.out[7];
    ys[0][8] <== tcN2b_0.out[8];
    ys[0][9] <== tcN2b_0.out[9];
    ys[0][10] <== tcN2b_0.out[10];
    ys[1][0] <== tcN2b_0.out[11];
    ys[1][1] <== tcN2b_0.out[12];
    ys[1][2] <== tcN2b_0.out[13];
    ys[1][3] <== tcN2b_0.out[14];
    ys[1][4] <== tcN2b_0.out[15];
    ys[1][5] <== tcN2b_0.out[16];
    ys[1][6] <== tcN2b_0.out[17];
    ys[1][7] <== tcN2b_0.out[18];
    ys[1][8] <== tcN2b_0.out[19];
    ys[1][9] <== tcN2b_0.out[20];
    ys[1][10] <== tcN2b_0.out[21];
    ys[2][0] <== tcN2b_0.out[22];
    ys[2][1] <== tcN2b_0.out[23];
    ys[2][2] <== tcN2b_0.out[24];
    ys[2][3] <== tcN2b_0.out[25];
    ys[2][4] <== tcN2b_0.out[26];
    ys[2][5] <== tcN2b_0.out[27];
    ys[2][6] <== tcN2b_0.out[28];
    ys[2][7] <== tcN2b_0.out[29];
    ys[2][8] <== tcN2b_0.out[30];
    ys[2][9] <== tcN2b_0.out[31];
    ys[2][10] <== tcN2b_0.out[32];
    ys[3][0] <== tcN2b_0.out[33];
    ys[3][1] <== tcN2b_0.out[34];
    ys[3][2] <== tcN2b_0.out[35];
    ys[3][3] <== tcN2b_0.out[36];
    ys[3][4] <== tcN2b_0.out[37];
    ys[3][5] <== tcN2b_0.out[38];
    ys[3][6] <== tcN2b_0.out[39];
    ys[3][7] <== tcN2b_0.out[40];
    ys[3][8] <== tcN2b_0.out[41];
    ys[3][9] <== tcN2b_0.out[42];
    ys[3][10] <== tcN2b_0.out[43];
    ys[4][0] <== tcN2b_0.out[44];
    ys[4][1] <== tcN2b_0.out[45];
    ys[4][2] <== tcN2b_0.out[46];
    ys[4][3] <== tcN2b_0.out[47];
    ys[4][4] <== tcN2b_0.out[48];
    ys[4][5] <== tcN2b_0.out[49];
    ys[4][6] <== tcN2b_0.out[50];
    ys[4][7] <== tcN2b_0.out[51];
    ys[4][8] <== tcN2b_0.out[52];
    ys[4][9] <== tcN2b_0.out[53];
    ys[4][10] <== tcN2b_0.out[54];
    ys[5][0] <== tcN2b_0.out[55];
    ys[5][1] <== tcN2b_0.out[56];
    ys[5][2] <== tcN2b_0.out[57];
    ys[5][3] <== tcN2b_0.out[58];
    ys[5][4] <== tcN2b_0.out[59];
    ys[5][5] <== tcN2b_0.out[60];
    ys[5][6] <== tcN2b_0.out[61];
    ys[5][7] <== tcN2b_0.out[62];
    ys[5][8] <== tcN2b_1.out[0];
    ys[5][9] <== tcN2b_1.out[1];
    ys[5][10] <== tcN2b_1.out[2];
    ys[6][0] <== tcN2b_1.out[3];
    ys[6][1] <== tcN2b_1.out[4];
    ys[6][2] <== tcN2b_1.out[5];
    ys[6][3] <== tcN2b_1.out[6];
    ys[6][4] <== tcN2b_1.out[7];
    ys[6][5] <== tcN2b_1.out[8];
    ys[6][6] <== tcN2b_1.out[9];
    ys[6][7] <== tcN2b_1.out[10];
    ys[6][8] <== tcN2b_1.out[11];
    ys[6][9] <== tcN2b_1.out[12];
    ys[6][10] <== tcN2b_1.out[13];
    ys[7][0] <== tcN2b_1.out[14];
    ys[7][1] <== tcN2b_1.out[15];
    ys[7][2] <== tcN2b_1.out[16];
    ys[7][3] <== tcN2b_1.out[17];
    ys[7][4] <== tcN2b_1.out[18];
    ys[7][5] <== tcN2b_1.out[19];
    ys[7][6] <== tcN2b_1.out[20];
    ys[7][7] <== tcN2b_1.out[21];
    ys[7][8] <== tcN2b_1.out[22];
    ys[7][9] <== tcN2b_1.out[23];
    ys[7][10] <== tcN2b_1.out[24];
    component verifyEvaluations = VerifyEvaluations();
    verifyEvaluations.enable <== enable;
    for (var i=0; i<8; i++) {
        for (var k=0; k<3; k++) {
            verifyEvaluations.challenges[i][k] <== challenges[i][k];
        }
    }
    for (var i=0; i<3; i++) {
        verifyEvaluations.publics[i] <== publics[i];
    }
    for (var i=0; i<8; i++) {
        for (var k=0; k<3; k++) {
            verifyEvaluations.evals[i][k] <== evals[i][k];
        }
    }
    
    component verifyQueries[8];
    component s0_merkle1[8];
    
    component s0_merkle3[8];
    
    component s0_merkle4[8];
    component s0_merkleC[8];
    component s0_lowValues[8];
    
    for (var q=0; q<8; q++) {
        verifyQueries[q] = VerifyQuery();
        s0_merkle1[q] = MerkleHash(1, 2, 2048);
    
        s0_merkle3[q] = MerkleHash(1, 1, 2048);
    
        s0_merkle4[q] = MerkleHash(1, 3, 2048);
        s0_merkleC[q] = MerkleHash(1, 2, 2048);
        s0_lowValues[q] = TreeSelector(4, 3) ;
    
        for (var i=0; i<11; i++ ) {
            verifyQueries[q].ys[i] <== ys[q][i];
            s0_merkle1[q].key[i] <== ys[q][i];
    
            s0_merkle3[q].key[i] <== ys[q][i];
    
            s0_merkle4[q].key[i] <== ys[q][i];
            s0_merkleC[q].key[i] <== ys[q][i];
        }
        for (var i=0; i<2; i++ ) {
            verifyQueries[q].tree1[i] <== s0_vals1[q][i];
            s0_merkle1[q].values[i][0] <== s0_vals1[q][i];
        }
    
        for (var i=0; i<1; i++ ) {
            verifyQueries[q].tree3[i] <== s0_vals3[q][i];
            s0_merkle3[q].values[i][0] <== s0_vals3[q][i];
        }
    
        for (var i=0; i<3; i++ ) {
            verifyQueries[q].tree4[i] <== s0_vals4[q][i];
            s0_merkle4[q].values[i][0] <== s0_vals4[q][i];
        }
        for (var i=0; i<2; i++ ) {
            verifyQueries[q].consts[i] <== s0_valsC[q][i];
            s0_merkleC[q].values[i][0] <== s0_valsC[q][i];
        }
        for (var i=0; i<8; i++) {
            for (var e=0; e<3; e++) {
                verifyQueries[q].challenges[i][e] <== challenges[i][e];
            }
        }
        for (var i=0; i<8; i++) {
            for (var e=0; e<3; e++) {
                verifyQueries[q].evals[i][e] <== evals[i][e];
            }
        }
        for (var i=0; i<11;i++) {
            for (var j=0; j<4; j++) {
                s0_merkle1[q].siblings[i][j] <== s0_siblings1[q][i][j];
    
                s0_merkle3[q].siblings[i][j] <== s0_siblings3[q][i][j];
        
                s0_merkle4[q].siblings[i][j] <== s0_siblings4[q][i][j];
                s0_merkleC[q].siblings[i][j] <== s0_siblingsC[q][i][j];
            }
        }
        
        for (var i=0; i<16; i++) {
            for (var e=0; e<3; e++) {
                s0_lowValues[q].values[i][e] <== s1_vals[q][i*3+e];
            }
        }
        for (var i=0; i<4; i++) {
            s0_lowValues[q].key[i] <== ys[q][i + 7];
        }
        
    }
        
    component s1_merkle[8];
    component s1_fft[8];
    component s1_evalPol[8];
    component s1_lowValues[8];
    signal s1_sx[8][7];
        
    for (var q=0; q<8; q++) {
        s1_merkle[q] = MerkleHash(3, 16, 128);
        s1_fft[q] = FFT(4, 3, 1);
        s1_evalPol[q] = EvalPol(16);
        s1_lowValues[q] = TreeSelector(4, 3) ;
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s1_merkle[q].values[i][e] <== s1_vals[q][i*3+e];
                s1_fft[q].in[i][e] <== s1_vals[q][i*3+e];
            }
        }
        
        for (var i=0; i<7; i++) {
            for (var j=0; j<4; j++) {
                s1_merkle[q].siblings[i][j] <== s1_siblings[q][i][j];
            }
            s1_merkle[q].key[i] <== ys[q][i];
        }
        s1_sx[q][0] <==  5646962470228954384 *  ( ys[q][0] * 8548973421900915980 +1);
        for (var i=1; i<7; i++) {
            s1_sx[q][i] <== s1_sx[q][i-1] *  ( ys[q][i] * ((1/roots(11 -i)) -1) +1);
        }
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s1_evalPol[q].pol[i][e] <== s1_fft[q].out[i][e];
            }
        }
        for (var e=0; e<3; e++) {
            s1_evalPol[q].x[e] <== s1_specialX[e] *  s1_sx[q][6];
        }
        
        for (var i=0; i<16; i++) {
            for (var e=0; e<3; e++) {
                s1_lowValues[q].values[i][e] <== s2_vals[q][i*3+e];
            }
        }
        for (var i=0; i<4; i++) {
            s1_lowValues[q].key[i] <== ys[q][i + 3];
        }
        
        for(var q = 0; q < 8; q ++) {
            for(var j = 0; j < 4; j ++) {
                enable * (s0_merkle1[q].root[j] - root1[j]) === 0;
                
                enable * (s0_merkle3[q].root[j] - root3[j]) === 0;
                enable * (s0_merkle4[q].root[j] - root4[j]) === 0;
                enable * (s0_merkleC[q].root[j] - rootC[j]) === 0;
            }
            for (var e = 0; e < 3; e ++) {
                enable * (s0_lowValues[q].out[e] - verifyQueries[q].out[e]) === 0;
            }
        }
        
        for (var e=0; e<3; e++) {
            enable * (s1_lowValues[q].out[e] - s1_evalPol[q].out[e]) === 0;
        }

        enable * (s1_merkle[q].root[0] - s1_root[0]) === 0;
        enable * (s1_merkle[q].root[1] - s1_root[1]) === 0;
        enable * (s1_merkle[q].root[2] - s1_root[2]) === 0;
        enable * (s1_merkle[q].root[3] - s1_root[3]) === 0;
    }
        
    component s2_merkle[8];
    component s2_fft[8];
    component s2_evalPol[8];
    component s2_lowValues[8];
    signal s2_sx[8][3];
        
    for (var q=0; q<8; q++) {
        s2_merkle[q] = MerkleHash(3, 16, 8);
        s2_fft[q] = FFT(4, 3, 1);
        s2_evalPol[q] = EvalPol(16);
        s2_lowValues[q] = TreeSelector(3, 3) ;
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s2_merkle[q].values[i][e] <== s2_vals[q][i*3+e];
                s2_fft[q].in[i][e] <== s2_vals[q][i*3+e];
            }
        }
        
        for (var i=0; i<3; i++) {
            for (var j=0; j<4; j++) {
                s2_merkle[q].siblings[i][j] <== s2_siblings[q][i][j];
            }
            s2_merkle[q].key[i] <== ys[q][i];
        }
        s2_sx[q][0] <==  2058312433970512828 *  ( ys[q][0] * 18442240469787213840 +1);
        for (var i=1; i<3; i++) {
            s2_sx[q][i] <== s2_sx[q][i-1] *  ( ys[q][i] * ((1/roots(7 -i)) -1) +1);
        }
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s2_evalPol[q].pol[i][e] <== s2_fft[q].out[i][e];
            }
        }
        for (var e=0; e<3; e++) {
            s2_evalPol[q].x[e] <== s2_specialX[e] *  s2_sx[q][2];
        }
        
        for (var i=0; i<8; i++) {
            for (var e=0; e<3; e++) {
                s2_lowValues[q].values[i][e] <== finalPol[i][e];
            }
        }
        for (var i=0; i<3; i++) {
            s2_lowValues[q].key[i] <== ys[q][i];
        }
        
        for(var q = 0; q < 8; q ++) {
            for(var j = 0; j < 4; j ++) {
                enable * (s0_merkle1[q].root[j] - root1[j]) === 0;
                
                enable * (s0_merkle3[q].root[j] - root3[j]) === 0;
                enable * (s0_merkle4[q].root[j] - root4[j]) === 0;
                enable * (s0_merkleC[q].root[j] - rootC[j]) === 0;
            }
            for (var e = 0; e < 3; e ++) {
                enable * (s0_lowValues[q].out[e] - verifyQueries[q].out[e]) === 0;
            }
        }
        
        for (var e=0; e<3; e++) {
            enable * (s2_lowValues[q].out[e] - s2_evalPol[q].out[e]) === 0;
        }

        enable * (s2_merkle[q].root[0] - s2_root[0]) === 0;
        enable * (s2_merkle[q].root[1] - s2_root[1]) === 0;
        enable * (s2_merkle[q].root[2] - s2_root[2]) === 0;
        enable * (s2_merkle[q].root[3] - s2_root[3]) === 0;
    }
        
    component lastIFFT = FFT(3, 3, 1);

    for (var k=0; k< 8; k++ ){
        for (var e=0; e<3; e++) {
            lastIFFT.in[k][e] <== finalPol[k][e];
        }
    }

    for (var k= 4; k< 8; k++ ) {
        for (var e=0; e<3; e++) {
            enable * lastIFFT.out[k][e] === 0;
        }
    }
}


component main {public [publics]}= StarkVerifier();
    