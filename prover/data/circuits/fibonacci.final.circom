pragma circom 2.0.6;

include "gl.circom";
include "poseidon.circom";
include "bitify.circom";
include "sha256/sha256.circom";
include "fft.circom";
include "merklehash.circom";
include "evalpol.circom";
include "treeselector.circom";
include "bn1togl3.circom";
include "compconstant64.circom";

template VerifyEvaluations() {
    signal input challenges[8][3];
    signal input evals[121][3];
    signal input publics[18];
    signal input enable;

    var p = 0xFFFFFFFF00000001;

    component zMul[16];
    
    for (var i=0; i< 16; i++) {
        zMul[i] = GLCMul();
        if (i==0) {
            zMul[i].ina[0] <== challenges[7][0];
            zMul[i].ina[1] <== challenges[7][1];
            zMul[i].ina[2] <== challenges[7][2];
            zMul[i].inb[0] <== challenges[7][0];
            zMul[i].inb[1] <== challenges[7][1];
            zMul[i].inb[2] <== challenges[7][2];
        } else {
            zMul[i].ina[0] <== zMul[i-1].out[0];
            zMul[i].ina[1] <== zMul[i-1].out[1];
            zMul[i].ina[2] <== zMul[i-1].out[2];
            zMul[i].inb[0] <== zMul[i-1].out[0];
            zMul[i].inb[1] <== zMul[i-1].out[1];
            zMul[i].inb[2] <== zMul[i-1].out[2];
        }
    }
        
    signal Z[3];

    Z[0] <== zMul[15].out[0] -1 + p;
    Z[1] <== zMul[15].out[1];
    Z[2] <== zMul[15].out[2];
    signal tmp_0[3] <== [evals[0][0] - publics[0] + p, evals[0][1], evals[0][2]];
    signal tmp_1[3] <== GLCMul()(evals[1], tmp_0);
    signal tmp_916[3] <== [tmp_1[0] - 0 + p, tmp_1[1], tmp_1[2]];
    signal tmp_2[3] <== [evals[2][0] - publics[1] + p, evals[2][1], evals[2][2]];
    signal tmp_3[3] <== GLCMul()(evals[1], tmp_2);
    signal tmp_917[3] <== [tmp_3[0] - 0 + p, tmp_3[1], tmp_3[2]];
    signal tmp_4[3] <== [evals[3][0] - publics[2] + p, evals[3][1], evals[3][2]];
    signal tmp_5[3] <== GLCMul()(evals[1], tmp_4);
    signal tmp_918[3] <== [tmp_5[0] - 0 + p, tmp_5[1], tmp_5[2]];
    signal tmp_6[3] <== [evals[4][0] - publics[3] + p, evals[4][1], evals[4][2]];
    signal tmp_7[3] <== GLCMul()(evals[1], tmp_6);
    signal tmp_919[3] <== [tmp_7[0] - 0 + p, tmp_7[1], tmp_7[2]];
    signal tmp_8[3] <== [evals[5][0] - publics[4] + p, evals[5][1], evals[5][2]];
    signal tmp_9[3] <== GLCMul()(evals[1], tmp_8);
    signal tmp_920[3] <== [tmp_9[0] - 0 + p, tmp_9[1], tmp_9[2]];
    signal tmp_10[3] <== [evals[6][0] - publics[5] + p, evals[6][1], evals[6][2]];
    signal tmp_11[3] <== GLCMul()(evals[1], tmp_10);
    signal tmp_921[3] <== [tmp_11[0] - 0 + p, tmp_11[1], tmp_11[2]];
    signal tmp_12[3] <== [evals[7][0] - publics[6] + p, evals[7][1], evals[7][2]];
    signal tmp_13[3] <== GLCMul()(evals[1], tmp_12);
    signal tmp_922[3] <== [tmp_13[0] - 0 + p, tmp_13[1], tmp_13[2]];
    signal tmp_14[3] <== [evals[8][0] - publics[7] + p, evals[8][1], evals[8][2]];
    signal tmp_15[3] <== GLCMul()(evals[1], tmp_14);
    signal tmp_923[3] <== [tmp_15[0] - 0 + p, tmp_15[1], tmp_15[2]];
    signal tmp_16[3] <== [evals[9][0] - publics[8] + p, evals[9][1], evals[9][2]];
    signal tmp_17[3] <== GLCMul()(evals[1], tmp_16);
    signal tmp_924[3] <== [tmp_17[0] - 0 + p, tmp_17[1], tmp_17[2]];
    signal tmp_18[3] <== [evals[10][0] - publics[9] + p, evals[10][1], evals[10][2]];
    signal tmp_19[3] <== GLCMul()(evals[1], tmp_18);
    signal tmp_925[3] <== [tmp_19[0] - 0 + p, tmp_19[1], tmp_19[2]];
    signal tmp_20[3] <== [evals[11][0] - publics[10] + p, evals[11][1], evals[11][2]];
    signal tmp_21[3] <== GLCMul()(evals[1], tmp_20);
    signal tmp_926[3] <== [tmp_21[0] - 0 + p, tmp_21[1], tmp_21[2]];
    signal tmp_22[3] <== [evals[12][0] - publics[11] + p, evals[12][1], evals[12][2]];
    signal tmp_23[3] <== GLCMul()(evals[1], tmp_22);
    signal tmp_927[3] <== [tmp_23[0] - 0 + p, tmp_23[1], tmp_23[2]];
    signal tmp_24[3] <== [evals[0][0] - publics[12] + p, evals[0][1], evals[0][2]];
    signal tmp_25[3] <== GLCMul()(evals[13], tmp_24);
    signal tmp_928[3] <== [tmp_25[0] - 0 + p, tmp_25[1], tmp_25[2]];
    signal tmp_26[3] <== [evals[2][0] - publics[13] + p, evals[2][1], evals[2][2]];
    signal tmp_27[3] <== GLCMul()(evals[13], tmp_26);
    signal tmp_929[3] <== [tmp_27[0] - 0 + p, tmp_27[1], tmp_27[2]];
    signal tmp_28[3] <== [evals[3][0] - publics[14] + p, evals[3][1], evals[3][2]];
    signal tmp_29[3] <== GLCMul()(evals[13], tmp_28);
    signal tmp_930[3] <== [tmp_29[0] - 0 + p, tmp_29[1], tmp_29[2]];
    signal tmp_30[3] <== [evals[4][0] - publics[15] + p, evals[4][1], evals[4][2]];
    signal tmp_31[3] <== GLCMul()(evals[13], tmp_30);
    signal tmp_931[3] <== [tmp_31[0] - 0 + p, tmp_31[1], tmp_31[2]];
    signal tmp_32[3] <== [evals[5][0] - publics[16] + p, evals[5][1], evals[5][2]];
    signal tmp_33[3] <== GLCMul()(evals[13], tmp_32);
    signal tmp_932[3] <== [tmp_33[0] - 0 + p, tmp_33[1], tmp_33[2]];
    signal tmp_34[3] <== [evals[6][0] - publics[17] + p, evals[6][1], evals[6][2]];
    signal tmp_35[3] <== GLCMul()(evals[13], tmp_34);
    signal tmp_933[3] <== [tmp_35[0] - 0 + p, tmp_35[1], tmp_35[2]];
    signal tmp_36[3] <== GLCMul()(evals[14], evals[15]);
    signal tmp_934[3] <== [tmp_36[0] - 0 + p, tmp_36[1], tmp_36[2]];
    signal tmp_37[3] <== GLCMul()(evals[16], evals[15]);
    signal tmp_935[3] <== [tmp_37[0] - 0 + p, tmp_37[1], tmp_37[2]];
    signal tmp_38[3] <== GLCMul()(evals[17], evals[15]);
    signal tmp_936[3] <== [tmp_38[0] - 0 + p, tmp_38[1], tmp_38[2]];
    signal tmp_39[3] <== GLCMul()(evals[18], evals[15]);
    signal tmp_937[3] <== [tmp_39[0] - 0 + p, tmp_39[1], tmp_39[2]];
    signal tmp_938[3] <== [evals[0][0] + evals[19][0], evals[0][1] + evals[19][1], evals[0][2] + evals[19][2]];
    signal tmp_939[3] <== GLCMul()(evals[20], tmp_938);
    signal tmp_940[3] <== tmp_939;
    signal tmp_40[3] <== GLCMul()([25, 0, 0], tmp_940);
    signal tmp_41[3] <== GLCMul()([15, 0, 0], evals[21]);
    signal tmp_42[3] <== [tmp_40[0] + tmp_41[0], tmp_40[1] + tmp_41[1], tmp_40[2] + tmp_41[2]];
    signal tmp_43[3] <== GLCMul()([41, 0, 0], evals[22]);
    signal tmp_44[3] <== [tmp_42[0] + tmp_43[0], tmp_42[1] + tmp_43[1], tmp_42[2] + tmp_43[2]];
    signal tmp_45[3] <== GLCMul()([16, 0, 0], evals[23]);
    signal tmp_46[3] <== [tmp_44[0] + tmp_45[0], tmp_44[1] + tmp_45[1], tmp_44[2] + tmp_45[2]];
    signal tmp_47[3] <== GLCMul()([2, 0, 0], evals[24]);
    signal tmp_48[3] <== [tmp_46[0] + tmp_47[0], tmp_46[1] + tmp_47[1], tmp_46[2] + tmp_47[2]];
    signal tmp_49[3] <== GLCMul()([28, 0, 0], evals[25]);
    signal tmp_50[3] <== [tmp_48[0] + tmp_49[0], tmp_48[1] + tmp_49[1], tmp_48[2] + tmp_49[2]];
    signal tmp_51[3] <== GLCMul()([13, 0, 0], evals[26]);
    signal tmp_52[3] <== [tmp_50[0] + tmp_51[0], tmp_50[1] + tmp_51[1], tmp_50[2] + tmp_51[2]];
    signal tmp_53[3] <== GLCMul()([13, 0, 0], evals[27]);
    signal tmp_54[3] <== [tmp_52[0] + tmp_53[0], tmp_52[1] + tmp_53[1], tmp_52[2] + tmp_53[2]];
    signal tmp_55[3] <== GLCMul()([39, 0, 0], evals[28]);
    signal tmp_56[3] <== [tmp_54[0] + tmp_55[0], tmp_54[1] + tmp_55[1], tmp_54[2] + tmp_55[2]];
    signal tmp_57[3] <== GLCMul()([18, 0, 0], evals[29]);
    signal tmp_58[3] <== [tmp_56[0] + tmp_57[0], tmp_56[1] + tmp_57[1], tmp_56[2] + tmp_57[2]];
    signal tmp_59[3] <== GLCMul()([34, 0, 0], evals[30]);
    signal tmp_60[3] <== [tmp_58[0] + tmp_59[0], tmp_58[1] + tmp_59[1], tmp_58[2] + tmp_59[2]];
    signal tmp_61[3] <== GLCMul()([20, 0, 0], evals[31]);
    signal tmp_62[3] <== [tmp_60[0] + tmp_61[0], tmp_60[1] + tmp_61[1], tmp_60[2] + tmp_61[2]];
    signal tmp_63[3] <== [evals[32][0] - tmp_62[0] + p, evals[32][1] - tmp_62[1] + p, evals[32][2] - tmp_62[2] + p];
    signal tmp_64[3] <== GLCMul()(evals[33], tmp_63);
    signal tmp_941[3] <== [tmp_64[0] - 0 + p, tmp_64[1], tmp_64[2]];
    signal tmp_65[3] <== GLCMul()([20, 0, 0], tmp_940);
    signal tmp_66[3] <== GLCMul()([17, 0, 0], evals[21]);
    signal tmp_67[3] <== [tmp_65[0] + tmp_66[0], tmp_65[1] + tmp_66[1], tmp_65[2] + tmp_66[2]];
    signal tmp_68[3] <== GLCMul()([15, 0, 0], evals[22]);
    signal tmp_69[3] <== [tmp_67[0] + tmp_68[0], tmp_67[1] + tmp_68[1], tmp_67[2] + tmp_68[2]];
    signal tmp_70[3] <== GLCMul()([41, 0, 0], evals[23]);
    signal tmp_71[3] <== [tmp_69[0] + tmp_70[0], tmp_69[1] + tmp_70[1], tmp_69[2] + tmp_70[2]];
    signal tmp_72[3] <== GLCMul()([16, 0, 0], evals[24]);
    signal tmp_73[3] <== [tmp_71[0] + tmp_72[0], tmp_71[1] + tmp_72[1], tmp_71[2] + tmp_72[2]];
    signal tmp_74[3] <== GLCMul()([2, 0, 0], evals[25]);
    signal tmp_75[3] <== [tmp_73[0] + tmp_74[0], tmp_73[1] + tmp_74[1], tmp_73[2] + tmp_74[2]];
    signal tmp_76[3] <== GLCMul()([28, 0, 0], evals[26]);
    signal tmp_77[3] <== [tmp_75[0] + tmp_76[0], tmp_75[1] + tmp_76[1], tmp_75[2] + tmp_76[2]];
    signal tmp_78[3] <== GLCMul()([13, 0, 0], evals[27]);
    signal tmp_79[3] <== [tmp_77[0] + tmp_78[0], tmp_77[1] + tmp_78[1], tmp_77[2] + tmp_78[2]];
    signal tmp_80[3] <== GLCMul()([13, 0, 0], evals[28]);
    signal tmp_81[3] <== [tmp_79[0] + tmp_80[0], tmp_79[1] + tmp_80[1], tmp_79[2] + tmp_80[2]];
    signal tmp_82[3] <== GLCMul()([39, 0, 0], evals[29]);
    signal tmp_83[3] <== [tmp_81[0] + tmp_82[0], tmp_81[1] + tmp_82[1], tmp_81[2] + tmp_82[2]];
    signal tmp_84[3] <== GLCMul()([18, 0, 0], evals[30]);
    signal tmp_85[3] <== [tmp_83[0] + tmp_84[0], tmp_83[1] + tmp_84[1], tmp_83[2] + tmp_84[2]];
    signal tmp_86[3] <== GLCMul()([34, 0, 0], evals[31]);
    signal tmp_87[3] <== [tmp_85[0] + tmp_86[0], tmp_85[1] + tmp_86[1], tmp_85[2] + tmp_86[2]];
    signal tmp_88[3] <== [evals[34][0] - tmp_87[0] + p, evals[34][1] - tmp_87[1] + p, evals[34][2] - tmp_87[2] + p];
    signal tmp_89[3] <== GLCMul()(evals[33], tmp_88);
    signal tmp_942[3] <== [tmp_89[0] - 0 + p, tmp_89[1], tmp_89[2]];
    signal tmp_90[3] <== GLCMul()([34, 0, 0], tmp_940);
    signal tmp_91[3] <== GLCMul()([20, 0, 0], evals[21]);
    signal tmp_92[3] <== [tmp_90[0] + tmp_91[0], tmp_90[1] + tmp_91[1], tmp_90[2] + tmp_91[2]];
    signal tmp_93[3] <== GLCMul()([17, 0, 0], evals[22]);
    signal tmp_94[3] <== [tmp_92[0] + tmp_93[0], tmp_92[1] + tmp_93[1], tmp_92[2] + tmp_93[2]];
    signal tmp_95[3] <== GLCMul()([15, 0, 0], evals[23]);
    signal tmp_96[3] <== [tmp_94[0] + tmp_95[0], tmp_94[1] + tmp_95[1], tmp_94[2] + tmp_95[2]];
    signal tmp_97[3] <== GLCMul()([41, 0, 0], evals[24]);
    signal tmp_98[3] <== [tmp_96[0] + tmp_97[0], tmp_96[1] + tmp_97[1], tmp_96[2] + tmp_97[2]];
    signal tmp_99[3] <== GLCMul()([16, 0, 0], evals[25]);
    signal tmp_100[3] <== [tmp_98[0] + tmp_99[0], tmp_98[1] + tmp_99[1], tmp_98[2] + tmp_99[2]];
    signal tmp_101[3] <== GLCMul()([2, 0, 0], evals[26]);
    signal tmp_102[3] <== [tmp_100[0] + tmp_101[0], tmp_100[1] + tmp_101[1], tmp_100[2] + tmp_101[2]];
    signal tmp_103[3] <== GLCMul()([28, 0, 0], evals[27]);
    signal tmp_104[3] <== [tmp_102[0] + tmp_103[0], tmp_102[1] + tmp_103[1], tmp_102[2] + tmp_103[2]];
    signal tmp_105[3] <== GLCMul()([13, 0, 0], evals[28]);
    signal tmp_106[3] <== [tmp_104[0] + tmp_105[0], tmp_104[1] + tmp_105[1], tmp_104[2] + tmp_105[2]];
    signal tmp_107[3] <== GLCMul()([13, 0, 0], evals[29]);
    signal tmp_108[3] <== [tmp_106[0] + tmp_107[0], tmp_106[1] + tmp_107[1], tmp_106[2] + tmp_107[2]];
    signal tmp_109[3] <== GLCMul()([39, 0, 0], evals[30]);
    signal tmp_110[3] <== [tmp_108[0] + tmp_109[0], tmp_108[1] + tmp_109[1], tmp_108[2] + tmp_109[2]];
    signal tmp_111[3] <== GLCMul()([18, 0, 0], evals[31]);
    signal tmp_112[3] <== [tmp_110[0] + tmp_111[0], tmp_110[1] + tmp_111[1], tmp_110[2] + tmp_111[2]];
    signal tmp_113[3] <== [evals[35][0] - tmp_112[0] + p, evals[35][1] - tmp_112[1] + p, evals[35][2] - tmp_112[2] + p];
    signal tmp_114[3] <== GLCMul()(evals[33], tmp_113);
    signal tmp_943[3] <== [tmp_114[0] - 0 + p, tmp_114[1], tmp_114[2]];
    signal tmp_115[3] <== GLCMul()([18, 0, 0], tmp_940);
    signal tmp_116[3] <== GLCMul()([34, 0, 0], evals[21]);
    signal tmp_117[3] <== [tmp_115[0] + tmp_116[0], tmp_115[1] + tmp_116[1], tmp_115[2] + tmp_116[2]];
    signal tmp_118[3] <== GLCMul()([20, 0, 0], evals[22]);
    signal tmp_119[3] <== [tmp_117[0] + tmp_118[0], tmp_117[1] + tmp_118[1], tmp_117[2] + tmp_118[2]];
    signal tmp_120[3] <== GLCMul()([17, 0, 0], evals[23]);
    signal tmp_121[3] <== [tmp_119[0] + tmp_120[0], tmp_119[1] + tmp_120[1], tmp_119[2] + tmp_120[2]];
    signal tmp_122[3] <== GLCMul()([15, 0, 0], evals[24]);
    signal tmp_123[3] <== [tmp_121[0] + tmp_122[0], tmp_121[1] + tmp_122[1], tmp_121[2] + tmp_122[2]];
    signal tmp_124[3] <== GLCMul()([41, 0, 0], evals[25]);
    signal tmp_125[3] <== [tmp_123[0] + tmp_124[0], tmp_123[1] + tmp_124[1], tmp_123[2] + tmp_124[2]];
    signal tmp_126[3] <== GLCMul()([16, 0, 0], evals[26]);
    signal tmp_127[3] <== [tmp_125[0] + tmp_126[0], tmp_125[1] + tmp_126[1], tmp_125[2] + tmp_126[2]];
    signal tmp_128[3] <== GLCMul()([2, 0, 0], evals[27]);
    signal tmp_129[3] <== [tmp_127[0] + tmp_128[0], tmp_127[1] + tmp_128[1], tmp_127[2] + tmp_128[2]];
    signal tmp_130[3] <== GLCMul()([28, 0, 0], evals[28]);
    signal tmp_131[3] <== [tmp_129[0] + tmp_130[0], tmp_129[1] + tmp_130[1], tmp_129[2] + tmp_130[2]];
    signal tmp_132[3] <== GLCMul()([13, 0, 0], evals[29]);
    signal tmp_133[3] <== [tmp_131[0] + tmp_132[0], tmp_131[1] + tmp_132[1], tmp_131[2] + tmp_132[2]];
    signal tmp_134[3] <== GLCMul()([13, 0, 0], evals[30]);
    signal tmp_135[3] <== [tmp_133[0] + tmp_134[0], tmp_133[1] + tmp_134[1], tmp_133[2] + tmp_134[2]];
    signal tmp_136[3] <== GLCMul()([39, 0, 0], evals[31]);
    signal tmp_137[3] <== [tmp_135[0] + tmp_136[0], tmp_135[1] + tmp_136[1], tmp_135[2] + tmp_136[2]];
    signal tmp_138[3] <== [evals[36][0] - tmp_137[0] + p, evals[36][1] - tmp_137[1] + p, evals[36][2] - tmp_137[2] + p];
    signal tmp_139[3] <== GLCMul()(evals[33], tmp_138);
    signal tmp_944[3] <== [tmp_139[0] - 0 + p, tmp_139[1], tmp_139[2]];
    signal tmp_140[3] <== GLCMul()([39, 0, 0], tmp_940);
    signal tmp_141[3] <== GLCMul()([18, 0, 0], evals[21]);
    signal tmp_142[3] <== [tmp_140[0] + tmp_141[0], tmp_140[1] + tmp_141[1], tmp_140[2] + tmp_141[2]];
    signal tmp_143[3] <== GLCMul()([34, 0, 0], evals[22]);
    signal tmp_144[3] <== [tmp_142[0] + tmp_143[0], tmp_142[1] + tmp_143[1], tmp_142[2] + tmp_143[2]];
    signal tmp_145[3] <== GLCMul()([20, 0, 0], evals[23]);
    signal tmp_146[3] <== [tmp_144[0] + tmp_145[0], tmp_144[1] + tmp_145[1], tmp_144[2] + tmp_145[2]];
    signal tmp_147[3] <== GLCMul()([17, 0, 0], evals[24]);
    signal tmp_148[3] <== [tmp_146[0] + tmp_147[0], tmp_146[1] + tmp_147[1], tmp_146[2] + tmp_147[2]];
    signal tmp_149[3] <== GLCMul()([15, 0, 0], evals[25]);
    signal tmp_150[3] <== [tmp_148[0] + tmp_149[0], tmp_148[1] + tmp_149[1], tmp_148[2] + tmp_149[2]];
    signal tmp_151[3] <== GLCMul()([41, 0, 0], evals[26]);
    signal tmp_152[3] <== [tmp_150[0] + tmp_151[0], tmp_150[1] + tmp_151[1], tmp_150[2] + tmp_151[2]];
    signal tmp_153[3] <== GLCMul()([16, 0, 0], evals[27]);
    signal tmp_154[3] <== [tmp_152[0] + tmp_153[0], tmp_152[1] + tmp_153[1], tmp_152[2] + tmp_153[2]];
    signal tmp_155[3] <== GLCMul()([2, 0, 0], evals[28]);
    signal tmp_156[3] <== [tmp_154[0] + tmp_155[0], tmp_154[1] + tmp_155[1], tmp_154[2] + tmp_155[2]];
    signal tmp_157[3] <== GLCMul()([28, 0, 0], evals[29]);
    signal tmp_158[3] <== [tmp_156[0] + tmp_157[0], tmp_156[1] + tmp_157[1], tmp_156[2] + tmp_157[2]];
    signal tmp_159[3] <== GLCMul()([13, 0, 0], evals[30]);
    signal tmp_160[3] <== [tmp_158[0] + tmp_159[0], tmp_158[1] + tmp_159[1], tmp_158[2] + tmp_159[2]];
    signal tmp_161[3] <== GLCMul()([13, 0, 0], evals[31]);
    signal tmp_162[3] <== [tmp_160[0] + tmp_161[0], tmp_160[1] + tmp_161[1], tmp_160[2] + tmp_161[2]];
    signal tmp_163[3] <== [evals[37][0] - tmp_162[0] + p, evals[37][1] - tmp_162[1] + p, evals[37][2] - tmp_162[2] + p];
    signal tmp_164[3] <== GLCMul()(evals[33], tmp_163);
    signal tmp_945[3] <== [tmp_164[0] - 0 + p, tmp_164[1], tmp_164[2]];
    signal tmp_165[3] <== GLCMul()([13, 0, 0], tmp_940);
    signal tmp_166[3] <== GLCMul()([39, 0, 0], evals[21]);
    signal tmp_167[3] <== [tmp_165[0] + tmp_166[0], tmp_165[1] + tmp_166[1], tmp_165[2] + tmp_166[2]];
    signal tmp_168[3] <== GLCMul()([18, 0, 0], evals[22]);
    signal tmp_169[3] <== [tmp_167[0] + tmp_168[0], tmp_167[1] + tmp_168[1], tmp_167[2] + tmp_168[2]];
    signal tmp_170[3] <== GLCMul()([34, 0, 0], evals[23]);
    signal tmp_171[3] <== [tmp_169[0] + tmp_170[0], tmp_169[1] + tmp_170[1], tmp_169[2] + tmp_170[2]];
    signal tmp_172[3] <== GLCMul()([20, 0, 0], evals[24]);
    signal tmp_173[3] <== [tmp_171[0] + tmp_172[0], tmp_171[1] + tmp_172[1], tmp_171[2] + tmp_172[2]];
    signal tmp_174[3] <== GLCMul()([17, 0, 0], evals[25]);
    signal tmp_175[3] <== [tmp_173[0] + tmp_174[0], tmp_173[1] + tmp_174[1], tmp_173[2] + tmp_174[2]];
    signal tmp_176[3] <== GLCMul()([15, 0, 0], evals[26]);
    signal tmp_177[3] <== [tmp_175[0] + tmp_176[0], tmp_175[1] + tmp_176[1], tmp_175[2] + tmp_176[2]];
    signal tmp_178[3] <== GLCMul()([41, 0, 0], evals[27]);
    signal tmp_179[3] <== [tmp_177[0] + tmp_178[0], tmp_177[1] + tmp_178[1], tmp_177[2] + tmp_178[2]];
    signal tmp_180[3] <== GLCMul()([16, 0, 0], evals[28]);
    signal tmp_181[3] <== [tmp_179[0] + tmp_180[0], tmp_179[1] + tmp_180[1], tmp_179[2] + tmp_180[2]];
    signal tmp_182[3] <== GLCMul()([2, 0, 0], evals[29]);
    signal tmp_183[3] <== [tmp_181[0] + tmp_182[0], tmp_181[1] + tmp_182[1], tmp_181[2] + tmp_182[2]];
    signal tmp_184[3] <== GLCMul()([28, 0, 0], evals[30]);
    signal tmp_185[3] <== [tmp_183[0] + tmp_184[0], tmp_183[1] + tmp_184[1], tmp_183[2] + tmp_184[2]];
    signal tmp_186[3] <== GLCMul()([13, 0, 0], evals[31]);
    signal tmp_187[3] <== [tmp_185[0] + tmp_186[0], tmp_185[1] + tmp_186[1], tmp_185[2] + tmp_186[2]];
    signal tmp_188[3] <== [evals[38][0] - tmp_187[0] + p, evals[38][1] - tmp_187[1] + p, evals[38][2] - tmp_187[2] + p];
    signal tmp_189[3] <== GLCMul()(evals[33], tmp_188);
    signal tmp_946[3] <== [tmp_189[0] - 0 + p, tmp_189[1], tmp_189[2]];
    signal tmp_190[3] <== GLCMul()([13, 0, 0], tmp_940);
    signal tmp_191[3] <== GLCMul()([13, 0, 0], evals[21]);
    signal tmp_192[3] <== [tmp_190[0] + tmp_191[0], tmp_190[1] + tmp_191[1], tmp_190[2] + tmp_191[2]];
    signal tmp_193[3] <== GLCMul()([39, 0, 0], evals[22]);
    signal tmp_194[3] <== [tmp_192[0] + tmp_193[0], tmp_192[1] + tmp_193[1], tmp_192[2] + tmp_193[2]];
    signal tmp_195[3] <== GLCMul()([18, 0, 0], evals[23]);
    signal tmp_196[3] <== [tmp_194[0] + tmp_195[0], tmp_194[1] + tmp_195[1], tmp_194[2] + tmp_195[2]];
    signal tmp_197[3] <== GLCMul()([34, 0, 0], evals[24]);
    signal tmp_198[3] <== [tmp_196[0] + tmp_197[0], tmp_196[1] + tmp_197[1], tmp_196[2] + tmp_197[2]];
    signal tmp_199[3] <== GLCMul()([20, 0, 0], evals[25]);
    signal tmp_200[3] <== [tmp_198[0] + tmp_199[0], tmp_198[1] + tmp_199[1], tmp_198[2] + tmp_199[2]];
    signal tmp_201[3] <== GLCMul()([17, 0, 0], evals[26]);
    signal tmp_202[3] <== [tmp_200[0] + tmp_201[0], tmp_200[1] + tmp_201[1], tmp_200[2] + tmp_201[2]];
    signal tmp_203[3] <== GLCMul()([15, 0, 0], evals[27]);
    signal tmp_204[3] <== [tmp_202[0] + tmp_203[0], tmp_202[1] + tmp_203[1], tmp_202[2] + tmp_203[2]];
    signal tmp_205[3] <== GLCMul()([41, 0, 0], evals[28]);
    signal tmp_206[3] <== [tmp_204[0] + tmp_205[0], tmp_204[1] + tmp_205[1], tmp_204[2] + tmp_205[2]];
    signal tmp_207[3] <== GLCMul()([16, 0, 0], evals[29]);
    signal tmp_208[3] <== [tmp_206[0] + tmp_207[0], tmp_206[1] + tmp_207[1], tmp_206[2] + tmp_207[2]];
    signal tmp_209[3] <== GLCMul()([2, 0, 0], evals[30]);
    signal tmp_210[3] <== [tmp_208[0] + tmp_209[0], tmp_208[1] + tmp_209[1], tmp_208[2] + tmp_209[2]];
    signal tmp_211[3] <== GLCMul()([28, 0, 0], evals[31]);
    signal tmp_212[3] <== [tmp_210[0] + tmp_211[0], tmp_210[1] + tmp_211[1], tmp_210[2] + tmp_211[2]];
    signal tmp_213[3] <== [evals[39][0] - tmp_212[0] + p, evals[39][1] - tmp_212[1] + p, evals[39][2] - tmp_212[2] + p];
    signal tmp_214[3] <== GLCMul()(evals[33], tmp_213);
    signal tmp_947[3] <== [tmp_214[0] - 0 + p, tmp_214[1], tmp_214[2]];
    signal tmp_215[3] <== GLCMul()([28, 0, 0], tmp_940);
    signal tmp_216[3] <== GLCMul()([13, 0, 0], evals[21]);
    signal tmp_217[3] <== [tmp_215[0] + tmp_216[0], tmp_215[1] + tmp_216[1], tmp_215[2] + tmp_216[2]];
    signal tmp_218[3] <== GLCMul()([13, 0, 0], evals[22]);
    signal tmp_219[3] <== [tmp_217[0] + tmp_218[0], tmp_217[1] + tmp_218[1], tmp_217[2] + tmp_218[2]];
    signal tmp_220[3] <== GLCMul()([39, 0, 0], evals[23]);
    signal tmp_221[3] <== [tmp_219[0] + tmp_220[0], tmp_219[1] + tmp_220[1], tmp_219[2] + tmp_220[2]];
    signal tmp_222[3] <== GLCMul()([18, 0, 0], evals[24]);
    signal tmp_223[3] <== [tmp_221[0] + tmp_222[0], tmp_221[1] + tmp_222[1], tmp_221[2] + tmp_222[2]];
    signal tmp_224[3] <== GLCMul()([34, 0, 0], evals[25]);
    signal tmp_225[3] <== [tmp_223[0] + tmp_224[0], tmp_223[1] + tmp_224[1], tmp_223[2] + tmp_224[2]];
    signal tmp_226[3] <== GLCMul()([20, 0, 0], evals[26]);
    signal tmp_227[3] <== [tmp_225[0] + tmp_226[0], tmp_225[1] + tmp_226[1], tmp_225[2] + tmp_226[2]];
    signal tmp_228[3] <== GLCMul()([17, 0, 0], evals[27]);
    signal tmp_229[3] <== [tmp_227[0] + tmp_228[0], tmp_227[1] + tmp_228[1], tmp_227[2] + tmp_228[2]];
    signal tmp_230[3] <== GLCMul()([15, 0, 0], evals[28]);
    signal tmp_231[3] <== [tmp_229[0] + tmp_230[0], tmp_229[1] + tmp_230[1], tmp_229[2] + tmp_230[2]];
    signal tmp_232[3] <== GLCMul()([41, 0, 0], evals[29]);
    signal tmp_233[3] <== [tmp_231[0] + tmp_232[0], tmp_231[1] + tmp_232[1], tmp_231[2] + tmp_232[2]];
    signal tmp_234[3] <== GLCMul()([16, 0, 0], evals[30]);
    signal tmp_235[3] <== [tmp_233[0] + tmp_234[0], tmp_233[1] + tmp_234[1], tmp_233[2] + tmp_234[2]];
    signal tmp_236[3] <== GLCMul()([2, 0, 0], evals[31]);
    signal tmp_237[3] <== [tmp_235[0] + tmp_236[0], tmp_235[1] + tmp_236[1], tmp_235[2] + tmp_236[2]];
    signal tmp_238[3] <== [evals[40][0] - tmp_237[0] + p, evals[40][1] - tmp_237[1] + p, evals[40][2] - tmp_237[2] + p];
    signal tmp_239[3] <== GLCMul()(evals[33], tmp_238);
    signal tmp_948[3] <== [tmp_239[0] - 0 + p, tmp_239[1], tmp_239[2]];
    signal tmp_240[3] <== GLCMul()([2, 0, 0], tmp_940);
    signal tmp_241[3] <== GLCMul()([28, 0, 0], evals[21]);
    signal tmp_242[3] <== [tmp_240[0] + tmp_241[0], tmp_240[1] + tmp_241[1], tmp_240[2] + tmp_241[2]];
    signal tmp_243[3] <== GLCMul()([13, 0, 0], evals[22]);
    signal tmp_244[3] <== [tmp_242[0] + tmp_243[0], tmp_242[1] + tmp_243[1], tmp_242[2] + tmp_243[2]];
    signal tmp_245[3] <== GLCMul()([13, 0, 0], evals[23]);
    signal tmp_246[3] <== [tmp_244[0] + tmp_245[0], tmp_244[1] + tmp_245[1], tmp_244[2] + tmp_245[2]];
    signal tmp_247[3] <== GLCMul()([39, 0, 0], evals[24]);
    signal tmp_248[3] <== [tmp_246[0] + tmp_247[0], tmp_246[1] + tmp_247[1], tmp_246[2] + tmp_247[2]];
    signal tmp_249[3] <== GLCMul()([18, 0, 0], evals[25]);
    signal tmp_250[3] <== [tmp_248[0] + tmp_249[0], tmp_248[1] + tmp_249[1], tmp_248[2] + tmp_249[2]];
    signal tmp_251[3] <== GLCMul()([34, 0, 0], evals[26]);
    signal tmp_252[3] <== [tmp_250[0] + tmp_251[0], tmp_250[1] + tmp_251[1], tmp_250[2] + tmp_251[2]];
    signal tmp_253[3] <== GLCMul()([20, 0, 0], evals[27]);
    signal tmp_254[3] <== [tmp_252[0] + tmp_253[0], tmp_252[1] + tmp_253[1], tmp_252[2] + tmp_253[2]];
    signal tmp_255[3] <== GLCMul()([17, 0, 0], evals[28]);
    signal tmp_256[3] <== [tmp_254[0] + tmp_255[0], tmp_254[1] + tmp_255[1], tmp_254[2] + tmp_255[2]];
    signal tmp_257[3] <== GLCMul()([15, 0, 0], evals[29]);
    signal tmp_258[3] <== [tmp_256[0] + tmp_257[0], tmp_256[1] + tmp_257[1], tmp_256[2] + tmp_257[2]];
    signal tmp_259[3] <== GLCMul()([41, 0, 0], evals[30]);
    signal tmp_260[3] <== [tmp_258[0] + tmp_259[0], tmp_258[1] + tmp_259[1], tmp_258[2] + tmp_259[2]];
    signal tmp_261[3] <== GLCMul()([16, 0, 0], evals[31]);
    signal tmp_262[3] <== [tmp_260[0] + tmp_261[0], tmp_260[1] + tmp_261[1], tmp_260[2] + tmp_261[2]];
    signal tmp_263[3] <== [evals[41][0] - tmp_262[0] + p, evals[41][1] - tmp_262[1] + p, evals[41][2] - tmp_262[2] + p];
    signal tmp_264[3] <== GLCMul()(evals[33], tmp_263);
    signal tmp_949[3] <== [tmp_264[0] - 0 + p, tmp_264[1], tmp_264[2]];
    signal tmp_265[3] <== GLCMul()([16, 0, 0], tmp_940);
    signal tmp_266[3] <== GLCMul()([2, 0, 0], evals[21]);
    signal tmp_267[3] <== [tmp_265[0] + tmp_266[0], tmp_265[1] + tmp_266[1], tmp_265[2] + tmp_266[2]];
    signal tmp_268[3] <== GLCMul()([28, 0, 0], evals[22]);
    signal tmp_269[3] <== [tmp_267[0] + tmp_268[0], tmp_267[1] + tmp_268[1], tmp_267[2] + tmp_268[2]];
    signal tmp_270[3] <== GLCMul()([13, 0, 0], evals[23]);
    signal tmp_271[3] <== [tmp_269[0] + tmp_270[0], tmp_269[1] + tmp_270[1], tmp_269[2] + tmp_270[2]];
    signal tmp_272[3] <== GLCMul()([13, 0, 0], evals[24]);
    signal tmp_273[3] <== [tmp_271[0] + tmp_272[0], tmp_271[1] + tmp_272[1], tmp_271[2] + tmp_272[2]];
    signal tmp_274[3] <== GLCMul()([39, 0, 0], evals[25]);
    signal tmp_275[3] <== [tmp_273[0] + tmp_274[0], tmp_273[1] + tmp_274[1], tmp_273[2] + tmp_274[2]];
    signal tmp_276[3] <== GLCMul()([18, 0, 0], evals[26]);
    signal tmp_277[3] <== [tmp_275[0] + tmp_276[0], tmp_275[1] + tmp_276[1], tmp_275[2] + tmp_276[2]];
    signal tmp_278[3] <== GLCMul()([34, 0, 0], evals[27]);
    signal tmp_279[3] <== [tmp_277[0] + tmp_278[0], tmp_277[1] + tmp_278[1], tmp_277[2] + tmp_278[2]];
    signal tmp_280[3] <== GLCMul()([20, 0, 0], evals[28]);
    signal tmp_281[3] <== [tmp_279[0] + tmp_280[0], tmp_279[1] + tmp_280[1], tmp_279[2] + tmp_280[2]];
    signal tmp_282[3] <== GLCMul()([17, 0, 0], evals[29]);
    signal tmp_283[3] <== [tmp_281[0] + tmp_282[0], tmp_281[1] + tmp_282[1], tmp_281[2] + tmp_282[2]];
    signal tmp_284[3] <== GLCMul()([15, 0, 0], evals[30]);
    signal tmp_285[3] <== [tmp_283[0] + tmp_284[0], tmp_283[1] + tmp_284[1], tmp_283[2] + tmp_284[2]];
    signal tmp_286[3] <== GLCMul()([41, 0, 0], evals[31]);
    signal tmp_287[3] <== [tmp_285[0] + tmp_286[0], tmp_285[1] + tmp_286[1], tmp_285[2] + tmp_286[2]];
    signal tmp_288[3] <== [evals[42][0] - tmp_287[0] + p, evals[42][1] - tmp_287[1] + p, evals[42][2] - tmp_287[2] + p];
    signal tmp_289[3] <== GLCMul()(evals[33], tmp_288);
    signal tmp_950[3] <== [tmp_289[0] - 0 + p, tmp_289[1], tmp_289[2]];
    signal tmp_290[3] <== GLCMul()([41, 0, 0], tmp_940);
    signal tmp_291[3] <== GLCMul()([16, 0, 0], evals[21]);
    signal tmp_292[3] <== [tmp_290[0] + tmp_291[0], tmp_290[1] + tmp_291[1], tmp_290[2] + tmp_291[2]];
    signal tmp_293[3] <== GLCMul()([2, 0, 0], evals[22]);
    signal tmp_294[3] <== [tmp_292[0] + tmp_293[0], tmp_292[1] + tmp_293[1], tmp_292[2] + tmp_293[2]];
    signal tmp_295[3] <== GLCMul()([28, 0, 0], evals[23]);
    signal tmp_296[3] <== [tmp_294[0] + tmp_295[0], tmp_294[1] + tmp_295[1], tmp_294[2] + tmp_295[2]];
    signal tmp_297[3] <== GLCMul()([13, 0, 0], evals[24]);
    signal tmp_298[3] <== [tmp_296[0] + tmp_297[0], tmp_296[1] + tmp_297[1], tmp_296[2] + tmp_297[2]];
    signal tmp_299[3] <== GLCMul()([13, 0, 0], evals[25]);
    signal tmp_300[3] <== [tmp_298[0] + tmp_299[0], tmp_298[1] + tmp_299[1], tmp_298[2] + tmp_299[2]];
    signal tmp_301[3] <== GLCMul()([39, 0, 0], evals[26]);
    signal tmp_302[3] <== [tmp_300[0] + tmp_301[0], tmp_300[1] + tmp_301[1], tmp_300[2] + tmp_301[2]];
    signal tmp_303[3] <== GLCMul()([18, 0, 0], evals[27]);
    signal tmp_304[3] <== [tmp_302[0] + tmp_303[0], tmp_302[1] + tmp_303[1], tmp_302[2] + tmp_303[2]];
    signal tmp_305[3] <== GLCMul()([34, 0, 0], evals[28]);
    signal tmp_306[3] <== [tmp_304[0] + tmp_305[0], tmp_304[1] + tmp_305[1], tmp_304[2] + tmp_305[2]];
    signal tmp_307[3] <== GLCMul()([20, 0, 0], evals[29]);
    signal tmp_308[3] <== [tmp_306[0] + tmp_307[0], tmp_306[1] + tmp_307[1], tmp_306[2] + tmp_307[2]];
    signal tmp_309[3] <== GLCMul()([17, 0, 0], evals[30]);
    signal tmp_310[3] <== [tmp_308[0] + tmp_309[0], tmp_308[1] + tmp_309[1], tmp_308[2] + tmp_309[2]];
    signal tmp_311[3] <== GLCMul()([15, 0, 0], evals[31]);
    signal tmp_312[3] <== [tmp_310[0] + tmp_311[0], tmp_310[1] + tmp_311[1], tmp_310[2] + tmp_311[2]];
    signal tmp_313[3] <== [evals[43][0] - tmp_312[0] + p, evals[43][1] - tmp_312[1] + p, evals[43][2] - tmp_312[2] + p];
    signal tmp_314[3] <== GLCMul()(evals[33], tmp_313);
    signal tmp_951[3] <== [tmp_314[0] - 0 + p, tmp_314[1], tmp_314[2]];
    signal tmp_315[3] <== GLCMul()([15, 0, 0], tmp_940);
    signal tmp_316[3] <== GLCMul()([41, 0, 0], evals[21]);
    signal tmp_317[3] <== [tmp_315[0] + tmp_316[0], tmp_315[1] + tmp_316[1], tmp_315[2] + tmp_316[2]];
    signal tmp_318[3] <== GLCMul()([16, 0, 0], evals[22]);
    signal tmp_319[3] <== [tmp_317[0] + tmp_318[0], tmp_317[1] + tmp_318[1], tmp_317[2] + tmp_318[2]];
    signal tmp_320[3] <== GLCMul()([2, 0, 0], evals[23]);
    signal tmp_321[3] <== [tmp_319[0] + tmp_320[0], tmp_319[1] + tmp_320[1], tmp_319[2] + tmp_320[2]];
    signal tmp_322[3] <== GLCMul()([28, 0, 0], evals[24]);
    signal tmp_323[3] <== [tmp_321[0] + tmp_322[0], tmp_321[1] + tmp_322[1], tmp_321[2] + tmp_322[2]];
    signal tmp_324[3] <== GLCMul()([13, 0, 0], evals[25]);
    signal tmp_325[3] <== [tmp_323[0] + tmp_324[0], tmp_323[1] + tmp_324[1], tmp_323[2] + tmp_324[2]];
    signal tmp_326[3] <== GLCMul()([13, 0, 0], evals[26]);
    signal tmp_327[3] <== [tmp_325[0] + tmp_326[0], tmp_325[1] + tmp_326[1], tmp_325[2] + tmp_326[2]];
    signal tmp_328[3] <== GLCMul()([39, 0, 0], evals[27]);
    signal tmp_329[3] <== [tmp_327[0] + tmp_328[0], tmp_327[1] + tmp_328[1], tmp_327[2] + tmp_328[2]];
    signal tmp_330[3] <== GLCMul()([18, 0, 0], evals[28]);
    signal tmp_331[3] <== [tmp_329[0] + tmp_330[0], tmp_329[1] + tmp_330[1], tmp_329[2] + tmp_330[2]];
    signal tmp_332[3] <== GLCMul()([34, 0, 0], evals[29]);
    signal tmp_333[3] <== [tmp_331[0] + tmp_332[0], tmp_331[1] + tmp_332[1], tmp_331[2] + tmp_332[2]];
    signal tmp_334[3] <== GLCMul()([20, 0, 0], evals[30]);
    signal tmp_335[3] <== [tmp_333[0] + tmp_334[0], tmp_333[1] + tmp_334[1], tmp_333[2] + tmp_334[2]];
    signal tmp_336[3] <== GLCMul()([17, 0, 0], evals[31]);
    signal tmp_337[3] <== [tmp_335[0] + tmp_336[0], tmp_335[1] + tmp_336[1], tmp_335[2] + tmp_336[2]];
    signal tmp_338[3] <== [evals[44][0] - tmp_337[0] + p, evals[44][1] - tmp_337[1] + p, evals[44][2] - tmp_337[2] + p];
    signal tmp_339[3] <== GLCMul()(evals[33], tmp_338);
    signal tmp_952[3] <== [tmp_339[0] - 0 + p, tmp_339[1], tmp_339[2]];
    signal tmp_953[3] <== evals[10];
    signal tmp_340[3] <== [evals[7][0] + evals[45][0], evals[7][1] + evals[45][1], evals[7][2] + evals[45][2]];
    signal tmp_954[3] <== GLCMul()(tmp_340, evals[46]);
    signal tmp_341[3] <== [evals[47][0] + evals[48][0], evals[47][1] + evals[48][1], evals[47][2] + evals[48][2]];
    signal tmp_342[3] <== [tmp_341[0] - evals[49][0] + p, tmp_341[1] - evals[49][1] + p, tmp_341[2] - evals[49][2] + p];
    signal tmp_343[3] <== [tmp_342[0] - evals[50][0] + p, tmp_342[1] - evals[50][1] + p, tmp_342[2] - evals[50][2] + p];
    signal tmp_344[3] <== [tmp_953[0] - tmp_343[0] + p, tmp_953[1] - tmp_343[1] + p, tmp_953[2] - tmp_343[2] + p];
    signal tmp_345[3] <== [tmp_344[0] - tmp_954[0] + p, tmp_344[1] - tmp_954[1] + p, tmp_344[2] - tmp_954[2] + p];
    signal tmp_346[3] <== GLCMul()(evals[51], tmp_345);
    signal tmp_955[3] <== [tmp_346[0] - 0 + p, tmp_346[1], tmp_346[2]];
    signal tmp_956[3] <== evals[11];
    signal tmp_347[3] <== [evals[8][0] + evals[52][0], evals[8][1] + evals[52][1], evals[8][2] + evals[52][2]];
    signal tmp_957[3] <== GLCMul()(tmp_347, evals[46]);
    signal tmp_348[3] <== [evals[53][0] + evals[47][0], evals[53][1] + evals[47][1], evals[53][2] + evals[47][2]];
    signal tmp_349[3] <== GLCMul()([2, 0, 0], evals[49]);
    signal tmp_350[3] <== [tmp_348[0] - tmp_349[0] + p, tmp_348[1] - tmp_349[1] + p, tmp_348[2] - tmp_349[2] + p];
    signal tmp_351[3] <== [tmp_350[0] - evals[48][0] + p, tmp_350[1] - evals[48][1] + p, tmp_350[2] - evals[48][2] + p];
    signal tmp_352[3] <== [tmp_956[0] - tmp_351[0] + p, tmp_956[1] - tmp_351[1] + p, tmp_956[2] - tmp_351[2] + p];
    signal tmp_353[3] <== [tmp_352[0] - tmp_957[0] + p, tmp_352[1] - tmp_957[1] + p, tmp_352[2] - tmp_957[2] + p];
    signal tmp_354[3] <== GLCMul()(evals[51], tmp_353);
    signal tmp_958[3] <== [tmp_354[0] - 0 + p, tmp_354[1], tmp_354[2]];
    signal tmp_959[3] <== evals[12];
    signal tmp_355[3] <== [evals[9][0] + evals[54][0], evals[9][1] + evals[54][1], evals[9][2] + evals[54][2]];
    signal tmp_960[3] <== GLCMul()(tmp_355, evals[46]);
    signal tmp_356[3] <== [evals[55][0] - evals[48][0] + p, evals[55][1] - evals[48][1] + p, evals[55][2] - evals[48][2] + p];
    signal tmp_357[3] <== [tmp_356[0] + evals[49][0], tmp_356[1] + evals[49][1], tmp_356[2] + evals[49][2]];
    signal tmp_358[3] <== [tmp_959[0] - tmp_357[0] + p, tmp_959[1] - tmp_357[1] + p, tmp_959[2] - tmp_357[2] + p];
    signal tmp_359[3] <== [tmp_358[0] - tmp_960[0] + p, tmp_358[1] - tmp_960[1] + p, tmp_358[2] - tmp_960[2] + p];
    signal tmp_360[3] <== GLCMul()(evals[51], tmp_359);
    signal tmp_961[3] <== [tmp_360[0] - 0 + p, tmp_360[1], tmp_360[2]];
    signal tmp_361[3] <== GLCMul()(evals[19], evals[0]);
    signal tmp_362[3] <== GLCMul()(evals[56], evals[4]);
    signal tmp_363[3] <== [tmp_361[0] + tmp_362[0], tmp_361[1] + tmp_362[1], tmp_361[2] + tmp_362[2]];
    signal tmp_364[3] <== GLCMul()(evals[57], evals[7]);
    signal tmp_365[3] <== [tmp_363[0] + tmp_364[0], tmp_363[1] + tmp_364[1], tmp_363[2] + tmp_364[2]];
    signal tmp_366[3] <== GLCMul()(evals[58], evals[10]);
    signal tmp_367[3] <== [tmp_365[0] + tmp_366[0], tmp_365[1] + tmp_366[1], tmp_365[2] + tmp_366[2]];
    signal tmp_368[3] <== GLCMul()(evals[45], evals[0]);
    signal tmp_369[3] <== [tmp_367[0] + tmp_368[0], tmp_367[1] + tmp_368[1], tmp_367[2] + tmp_368[2]];
    signal tmp_370[3] <== GLCMul()(evals[52], evals[4]);
    signal tmp_962[3] <== [tmp_369[0] + tmp_370[0], tmp_369[1] + tmp_370[1], tmp_369[2] + tmp_370[2]];
    signal tmp_371[3] <== [evals[32][0] - tmp_962[0] + p, evals[32][1] - tmp_962[1] + p, evals[32][2] - tmp_962[2] + p];
    signal tmp_372[3] <== GLCMul()(evals[59], tmp_371);
    signal tmp_963[3] <== [tmp_372[0] - 0 + p, tmp_372[1], tmp_372[2]];
    signal tmp_373[3] <== GLCMul()(evals[19], evals[2]);
    signal tmp_374[3] <== GLCMul()(evals[56], evals[5]);
    signal tmp_375[3] <== [tmp_373[0] + tmp_374[0], tmp_373[1] + tmp_374[1], tmp_373[2] + tmp_374[2]];
    signal tmp_376[3] <== GLCMul()(evals[57], evals[8]);
    signal tmp_377[3] <== [tmp_375[0] + tmp_376[0], tmp_375[1] + tmp_376[1], tmp_375[2] + tmp_376[2]];
    signal tmp_378[3] <== GLCMul()(evals[58], evals[11]);
    signal tmp_379[3] <== [tmp_377[0] + tmp_378[0], tmp_377[1] + tmp_378[1], tmp_377[2] + tmp_378[2]];
    signal tmp_380[3] <== GLCMul()(evals[45], evals[2]);
    signal tmp_381[3] <== [tmp_379[0] + tmp_380[0], tmp_379[1] + tmp_380[1], tmp_379[2] + tmp_380[2]];
    signal tmp_382[3] <== GLCMul()(evals[52], evals[5]);
    signal tmp_964[3] <== [tmp_381[0] + tmp_382[0], tmp_381[1] + tmp_382[1], tmp_381[2] + tmp_382[2]];
    signal tmp_383[3] <== [evals[34][0] - tmp_964[0] + p, evals[34][1] - tmp_964[1] + p, evals[34][2] - tmp_964[2] + p];
    signal tmp_384[3] <== GLCMul()(evals[59], tmp_383);
    signal tmp_965[3] <== [tmp_384[0] - 0 + p, tmp_384[1], tmp_384[2]];
    signal tmp_385[3] <== GLCMul()(evals[19], evals[3]);
    signal tmp_386[3] <== GLCMul()(evals[56], evals[6]);
    signal tmp_387[3] <== [tmp_385[0] + tmp_386[0], tmp_385[1] + tmp_386[1], tmp_385[2] + tmp_386[2]];
    signal tmp_388[3] <== GLCMul()(evals[57], evals[9]);
    signal tmp_389[3] <== [tmp_387[0] + tmp_388[0], tmp_387[1] + tmp_388[1], tmp_387[2] + tmp_388[2]];
    signal tmp_390[3] <== GLCMul()(evals[58], evals[12]);
    signal tmp_391[3] <== [tmp_389[0] + tmp_390[0], tmp_389[1] + tmp_390[1], tmp_389[2] + tmp_390[2]];
    signal tmp_392[3] <== GLCMul()(evals[45], evals[3]);
    signal tmp_393[3] <== [tmp_391[0] + tmp_392[0], tmp_391[1] + tmp_392[1], tmp_391[2] + tmp_392[2]];
    signal tmp_394[3] <== GLCMul()(evals[52], evals[6]);
    signal tmp_966[3] <== [tmp_393[0] + tmp_394[0], tmp_393[1] + tmp_394[1], tmp_393[2] + tmp_394[2]];
    signal tmp_395[3] <== [evals[35][0] - tmp_966[0] + p, evals[35][1] - tmp_966[1] + p, evals[35][2] - tmp_966[2] + p];
    signal tmp_396[3] <== GLCMul()(evals[59], tmp_395);
    signal tmp_967[3] <== [tmp_396[0] - 0 + p, tmp_396[1], tmp_396[2]];
    signal tmp_397[3] <== GLCMul()(evals[19], evals[0]);
    signal tmp_398[3] <== GLCMul()(evals[56], evals[4]);
    signal tmp_399[3] <== [tmp_397[0] - tmp_398[0] + p, tmp_397[1] - tmp_398[1] + p, tmp_397[2] - tmp_398[2] + p];
    signal tmp_400[3] <== GLCMul()(evals[60], evals[7]);
    signal tmp_401[3] <== [tmp_399[0] + tmp_400[0], tmp_399[1] + tmp_400[1], tmp_399[2] + tmp_400[2]];
    signal tmp_402[3] <== GLCMul()(evals[61], evals[10]);
    signal tmp_403[3] <== [tmp_401[0] - tmp_402[0] + p, tmp_401[1] - tmp_402[1] + p, tmp_401[2] - tmp_402[2] + p];
    signal tmp_404[3] <== GLCMul()(evals[45], evals[0]);
    signal tmp_405[3] <== [tmp_403[0] + tmp_404[0], tmp_403[1] + tmp_404[1], tmp_403[2] + tmp_404[2]];
    signal tmp_406[3] <== GLCMul()(evals[52], evals[4]);
    signal tmp_968[3] <== [tmp_405[0] - tmp_406[0] + p, tmp_405[1] - tmp_406[1] + p, tmp_405[2] - tmp_406[2] + p];
    signal tmp_407[3] <== [evals[36][0] - tmp_968[0] + p, evals[36][1] - tmp_968[1] + p, evals[36][2] - tmp_968[2] + p];
    signal tmp_408[3] <== GLCMul()(evals[59], tmp_407);
    signal tmp_969[3] <== [tmp_408[0] - 0 + p, tmp_408[1], tmp_408[2]];
    signal tmp_409[3] <== GLCMul()(evals[19], evals[2]);
    signal tmp_410[3] <== GLCMul()(evals[56], evals[5]);
    signal tmp_411[3] <== [tmp_409[0] - tmp_410[0] + p, tmp_409[1] - tmp_410[1] + p, tmp_409[2] - tmp_410[2] + p];
    signal tmp_412[3] <== GLCMul()(evals[60], evals[8]);
    signal tmp_413[3] <== [tmp_411[0] + tmp_412[0], tmp_411[1] + tmp_412[1], tmp_411[2] + tmp_412[2]];
    signal tmp_414[3] <== GLCMul()(evals[61], evals[11]);
    signal tmp_415[3] <== [tmp_413[0] - tmp_414[0] + p, tmp_413[1] - tmp_414[1] + p, tmp_413[2] - tmp_414[2] + p];
    signal tmp_416[3] <== GLCMul()(evals[45], evals[2]);
    signal tmp_417[3] <== [tmp_415[0] + tmp_416[0], tmp_415[1] + tmp_416[1], tmp_415[2] + tmp_416[2]];
    signal tmp_418[3] <== GLCMul()(evals[52], evals[5]);
    signal tmp_970[3] <== [tmp_417[0] - tmp_418[0] + p, tmp_417[1] - tmp_418[1] + p, tmp_417[2] - tmp_418[2] + p];
    signal tmp_419[3] <== [evals[37][0] - tmp_970[0] + p, evals[37][1] - tmp_970[1] + p, evals[37][2] - tmp_970[2] + p];
    signal tmp_420[3] <== GLCMul()(evals[59], tmp_419);
    signal tmp_971[3] <== [tmp_420[0] - 0 + p, tmp_420[1], tmp_420[2]];
    signal tmp_421[3] <== GLCMul()(evals[19], evals[3]);
    signal tmp_422[3] <== GLCMul()(evals[56], evals[6]);
    signal tmp_423[3] <== [tmp_421[0] - tmp_422[0] + p, tmp_421[1] - tmp_422[1] + p, tmp_421[2] - tmp_422[2] + p];
    signal tmp_424[3] <== GLCMul()(evals[60], evals[9]);
    signal tmp_425[3] <== [tmp_423[0] + tmp_424[0], tmp_423[1] + tmp_424[1], tmp_423[2] + tmp_424[2]];
    signal tmp_426[3] <== GLCMul()(evals[61], evals[12]);
    signal tmp_427[3] <== [tmp_425[0] - tmp_426[0] + p, tmp_425[1] - tmp_426[1] + p, tmp_425[2] - tmp_426[2] + p];
    signal tmp_428[3] <== GLCMul()(evals[45], evals[3]);
    signal tmp_429[3] <== [tmp_427[0] + tmp_428[0], tmp_427[1] + tmp_428[1], tmp_427[2] + tmp_428[2]];
    signal tmp_430[3] <== GLCMul()(evals[52], evals[6]);
    signal tmp_972[3] <== [tmp_429[0] - tmp_430[0] + p, tmp_429[1] - tmp_430[1] + p, tmp_429[2] - tmp_430[2] + p];
    signal tmp_431[3] <== [evals[38][0] - tmp_972[0] + p, evals[38][1] - tmp_972[1] + p, evals[38][2] - tmp_972[2] + p];
    signal tmp_432[3] <== GLCMul()(evals[59], tmp_431);
    signal tmp_973[3] <== [tmp_432[0] - 0 + p, tmp_432[1], tmp_432[2]];
    signal tmp_433[3] <== GLCMul()(evals[19], evals[0]);
    signal tmp_434[3] <== GLCMul()(evals[56], evals[4]);
    signal tmp_435[3] <== [tmp_433[0] + tmp_434[0], tmp_433[1] + tmp_434[1], tmp_433[2] + tmp_434[2]];
    signal tmp_436[3] <== GLCMul()(evals[57], evals[7]);
    signal tmp_437[3] <== [tmp_435[0] - tmp_436[0] + p, tmp_435[1] - tmp_436[1] + p, tmp_435[2] - tmp_436[2] + p];
    signal tmp_438[3] <== GLCMul()(evals[58], evals[10]);
    signal tmp_439[3] <== [tmp_437[0] - tmp_438[0] + p, tmp_437[1] - tmp_438[1] + p, tmp_437[2] - tmp_438[2] + p];
    signal tmp_440[3] <== GLCMul()(evals[45], evals[7]);
    signal tmp_441[3] <== [tmp_439[0] + tmp_440[0], tmp_439[1] + tmp_440[1], tmp_439[2] + tmp_440[2]];
    signal tmp_442[3] <== GLCMul()(evals[54], evals[10]);
    signal tmp_974[3] <== [tmp_441[0] + tmp_442[0], tmp_441[1] + tmp_442[1], tmp_441[2] + tmp_442[2]];
    signal tmp_443[3] <== [evals[39][0] - tmp_974[0] + p, evals[39][1] - tmp_974[1] + p, evals[39][2] - tmp_974[2] + p];
    signal tmp_444[3] <== GLCMul()(evals[59], tmp_443);
    signal tmp_975[3] <== [tmp_444[0] - 0 + p, tmp_444[1], tmp_444[2]];
    signal tmp_445[3] <== GLCMul()(evals[19], evals[2]);
    signal tmp_446[3] <== GLCMul()(evals[56], evals[5]);
    signal tmp_447[3] <== [tmp_445[0] + tmp_446[0], tmp_445[1] + tmp_446[1], tmp_445[2] + tmp_446[2]];
    signal tmp_448[3] <== GLCMul()(evals[57], evals[8]);
    signal tmp_449[3] <== [tmp_447[0] - tmp_448[0] + p, tmp_447[1] - tmp_448[1] + p, tmp_447[2] - tmp_448[2] + p];
    signal tmp_450[3] <== GLCMul()(evals[58], evals[11]);
    signal tmp_451[3] <== [tmp_449[0] - tmp_450[0] + p, tmp_449[1] - tmp_450[1] + p, tmp_449[2] - tmp_450[2] + p];
    signal tmp_452[3] <== GLCMul()(evals[45], evals[8]);
    signal tmp_453[3] <== [tmp_451[0] + tmp_452[0], tmp_451[1] + tmp_452[1], tmp_451[2] + tmp_452[2]];
    signal tmp_454[3] <== GLCMul()(evals[54], evals[11]);
    signal tmp_976[3] <== [tmp_453[0] + tmp_454[0], tmp_453[1] + tmp_454[1], tmp_453[2] + tmp_454[2]];
    signal tmp_455[3] <== [evals[40][0] - tmp_976[0] + p, evals[40][1] - tmp_976[1] + p, evals[40][2] - tmp_976[2] + p];
    signal tmp_456[3] <== GLCMul()(evals[59], tmp_455);
    signal tmp_977[3] <== [tmp_456[0] - 0 + p, tmp_456[1], tmp_456[2]];
    signal tmp_457[3] <== GLCMul()(evals[19], evals[3]);
    signal tmp_458[3] <== GLCMul()(evals[56], evals[6]);
    signal tmp_459[3] <== [tmp_457[0] + tmp_458[0], tmp_457[1] + tmp_458[1], tmp_457[2] + tmp_458[2]];
    signal tmp_460[3] <== GLCMul()(evals[57], evals[9]);
    signal tmp_461[3] <== [tmp_459[0] - tmp_460[0] + p, tmp_459[1] - tmp_460[1] + p, tmp_459[2] - tmp_460[2] + p];
    signal tmp_462[3] <== GLCMul()(evals[58], evals[12]);
    signal tmp_463[3] <== [tmp_461[0] - tmp_462[0] + p, tmp_461[1] - tmp_462[1] + p, tmp_461[2] - tmp_462[2] + p];
    signal tmp_464[3] <== GLCMul()(evals[45], evals[9]);
    signal tmp_465[3] <== [tmp_463[0] + tmp_464[0], tmp_463[1] + tmp_464[1], tmp_463[2] + tmp_464[2]];
    signal tmp_466[3] <== GLCMul()(evals[54], evals[12]);
    signal tmp_978[3] <== [tmp_465[0] + tmp_466[0], tmp_465[1] + tmp_466[1], tmp_465[2] + tmp_466[2]];
    signal tmp_467[3] <== [evals[41][0] - tmp_978[0] + p, evals[41][1] - tmp_978[1] + p, evals[41][2] - tmp_978[2] + p];
    signal tmp_468[3] <== GLCMul()(evals[59], tmp_467);
    signal tmp_979[3] <== [tmp_468[0] - 0 + p, tmp_468[1], tmp_468[2]];
    signal tmp_469[3] <== GLCMul()(evals[19], evals[0]);
    signal tmp_470[3] <== GLCMul()(evals[56], evals[4]);
    signal tmp_471[3] <== [tmp_469[0] - tmp_470[0] + p, tmp_469[1] - tmp_470[1] + p, tmp_469[2] - tmp_470[2] + p];
    signal tmp_472[3] <== GLCMul()(evals[60], evals[7]);
    signal tmp_473[3] <== [tmp_471[0] - tmp_472[0] + p, tmp_471[1] - tmp_472[1] + p, tmp_471[2] - tmp_472[2] + p];
    signal tmp_474[3] <== GLCMul()(evals[61], evals[10]);
    signal tmp_475[3] <== [tmp_473[0] + tmp_474[0], tmp_473[1] + tmp_474[1], tmp_473[2] + tmp_474[2]];
    signal tmp_476[3] <== GLCMul()(evals[45], evals[7]);
    signal tmp_477[3] <== [tmp_475[0] + tmp_476[0], tmp_475[1] + tmp_476[1], tmp_475[2] + tmp_476[2]];
    signal tmp_478[3] <== GLCMul()(evals[54], evals[10]);
    signal tmp_980[3] <== [tmp_477[0] - tmp_478[0] + p, tmp_477[1] - tmp_478[1] + p, tmp_477[2] - tmp_478[2] + p];
    signal tmp_479[3] <== [evals[42][0] - tmp_980[0] + p, evals[42][1] - tmp_980[1] + p, evals[42][2] - tmp_980[2] + p];
    signal tmp_480[3] <== GLCMul()(evals[59], tmp_479);
    signal tmp_981[3] <== [tmp_480[0] - 0 + p, tmp_480[1], tmp_480[2]];
    signal tmp_481[3] <== GLCMul()(evals[19], evals[2]);
    signal tmp_482[3] <== GLCMul()(evals[56], evals[5]);
    signal tmp_483[3] <== [tmp_481[0] - tmp_482[0] + p, tmp_481[1] - tmp_482[1] + p, tmp_481[2] - tmp_482[2] + p];
    signal tmp_484[3] <== GLCMul()(evals[60], evals[8]);
    signal tmp_485[3] <== [tmp_483[0] - tmp_484[0] + p, tmp_483[1] - tmp_484[1] + p, tmp_483[2] - tmp_484[2] + p];
    signal tmp_486[3] <== GLCMul()(evals[61], evals[11]);
    signal tmp_487[3] <== [tmp_485[0] + tmp_486[0], tmp_485[1] + tmp_486[1], tmp_485[2] + tmp_486[2]];
    signal tmp_488[3] <== GLCMul()(evals[45], evals[8]);
    signal tmp_489[3] <== [tmp_487[0] + tmp_488[0], tmp_487[1] + tmp_488[1], tmp_487[2] + tmp_488[2]];
    signal tmp_490[3] <== GLCMul()(evals[54], evals[11]);
    signal tmp_982[3] <== [tmp_489[0] - tmp_490[0] + p, tmp_489[1] - tmp_490[1] + p, tmp_489[2] - tmp_490[2] + p];
    signal tmp_491[3] <== [evals[43][0] - tmp_982[0] + p, evals[43][1] - tmp_982[1] + p, evals[43][2] - tmp_982[2] + p];
    signal tmp_492[3] <== GLCMul()(evals[59], tmp_491);
    signal tmp_983[3] <== [tmp_492[0] - 0 + p, tmp_492[1], tmp_492[2]];
    signal tmp_493[3] <== GLCMul()(evals[19], evals[3]);
    signal tmp_494[3] <== GLCMul()(evals[56], evals[6]);
    signal tmp_495[3] <== [tmp_493[0] - tmp_494[0] + p, tmp_493[1] - tmp_494[1] + p, tmp_493[2] - tmp_494[2] + p];
    signal tmp_496[3] <== GLCMul()(evals[60], evals[9]);
    signal tmp_497[3] <== [tmp_495[0] - tmp_496[0] + p, tmp_495[1] - tmp_496[1] + p, tmp_495[2] - tmp_496[2] + p];
    signal tmp_498[3] <== GLCMul()(evals[61], evals[12]);
    signal tmp_499[3] <== [tmp_497[0] + tmp_498[0], tmp_497[1] + tmp_498[1], tmp_497[2] + tmp_498[2]];
    signal tmp_500[3] <== GLCMul()(evals[45], evals[9]);
    signal tmp_501[3] <== [tmp_499[0] + tmp_500[0], tmp_499[1] + tmp_500[1], tmp_499[2] + tmp_500[2]];
    signal tmp_502[3] <== GLCMul()(evals[54], evals[12]);
    signal tmp_984[3] <== [tmp_501[0] - tmp_502[0] + p, tmp_501[1] - tmp_502[1] + p, tmp_501[2] - tmp_502[2] + p];
    signal tmp_503[3] <== [evals[44][0] - tmp_984[0] + p, evals[44][1] - tmp_984[1] + p, evals[44][2] - tmp_984[2] + p];
    signal tmp_504[3] <== GLCMul()(evals[59], tmp_503);
    signal tmp_985[3] <== [tmp_504[0] - 0 + p, tmp_504[1], tmp_504[2]];
    signal tmp_505[3] <== [evals[39][0] - evals[62][0] + p, evals[39][1] - evals[62][1] + p, evals[39][2] - evals[62][2] + p];
    signal tmp_506[3] <== GLCMul()(evals[63], tmp_505);
    signal tmp_986[3] <== [tmp_506[0] - 0 + p, tmp_506[1], tmp_506[2]];
    signal tmp_507[3] <== [evals[40][0] - evals[64][0] + p, evals[40][1] - evals[64][1] + p, evals[40][2] - evals[64][2] + p];
    signal tmp_508[3] <== GLCMul()(evals[63], tmp_507);
    signal tmp_987[3] <== [tmp_508[0] - 0 + p, tmp_508[1], tmp_508[2]];
    signal tmp_509[3] <== [evals[41][0] - evals[65][0] + p, evals[41][1] - evals[65][1] + p, evals[41][2] - evals[65][2] + p];
    signal tmp_510[3] <== GLCMul()(evals[63], tmp_509);
    signal tmp_988[3] <== [tmp_510[0] - 0 + p, tmp_510[1], tmp_510[2]];
    signal tmp_511[3] <== [evals[66][0] - 1 + p, evals[66][1], evals[66][2]];
    signal tmp_989[3] <== GLCMul()(evals[1], tmp_511);
    signal tmp_990[3] <== evals[12];
    signal tmp_991[3] <== evals[67];
    signal tmp_512[3] <== GLCMul()(challenges[3], tmp_991);
    signal tmp_513[3] <== [tmp_990[0] + tmp_512[0], tmp_990[1] + tmp_512[1], tmp_990[2] + tmp_512[2]];
    signal tmp_514[3] <== [tmp_513[0] + challenges[2][0], tmp_513[1] + challenges[2][1], tmp_513[2] + challenges[2][2]];
    signal tmp_992[3] <== GLCMul()(evals[68], tmp_514);
    signal tmp_515[3] <== GLCMul()(challenges[3], [12756200801261202346, 0, 0]);
    signal tmp_516[3] <== GLCMul()(tmp_515, challenges[7]);
    signal tmp_517[3] <== [tmp_990[0] + tmp_516[0], tmp_990[1] + tmp_516[1], tmp_990[2] + tmp_516[2]];
    signal tmp_518[3] <== [tmp_517[0] + challenges[2][0], tmp_517[1] + challenges[2][1], tmp_517[2] + challenges[2][2]];
    signal tmp_993[3] <== GLCMul()(evals[69], tmp_518);
    signal tmp_519[3] <== GLCMul()(evals[70], tmp_992);
    signal tmp_520[3] <== GLCMul()(evals[66], tmp_993);
    signal tmp_994[3] <== [tmp_519[0] - tmp_520[0] + p, tmp_519[1] - tmp_520[1] + p, tmp_519[2] - tmp_520[2] + p];
    signal tmp_995[3] <== GLCMul()(evals[0], evals[2]);
    signal tmp_996[3] <== GLCMul()(evals[4], evals[5]);
    signal tmp_997[3] <== GLCMul()(evals[7], evals[8]);
    signal tmp_998[3] <== GLCMul()(evals[10], evals[11]);
    signal tmp_999[3] <== GLCMul()(evals[71], evals[71]);
    signal tmp_1000[3] <== [evals[2][0] + evals[56][0], evals[2][1] + evals[56][1], evals[2][2] + evals[56][2]];
    signal tmp_1001[3] <== GLCMul()(evals[72], evals[72]);
    signal tmp_1002[3] <== GLCMul()(evals[73], tmp_1000);
    signal tmp_1003[3] <== [evals[3][0] + evals[57][0], evals[3][1] + evals[57][1], evals[3][2] + evals[57][2]];
    signal tmp_1004[3] <== GLCMul()(evals[74], evals[74]);
    signal tmp_1005[3] <== GLCMul()(evals[75], tmp_1003);
    signal tmp_1006[3] <== [evals[4][0] + evals[58][0], evals[4][1] + evals[58][1], evals[4][2] + evals[58][2]];
    signal tmp_1007[3] <== GLCMul()(evals[76], evals[76]);
    signal tmp_1008[3] <== GLCMul()(evals[77], tmp_1006);
    signal tmp_1009[3] <== [evals[5][0] + evals[60][0], evals[5][1] + evals[60][1], evals[5][2] + evals[60][2]];
    signal tmp_1010[3] <== GLCMul()(evals[78], evals[78]);
    signal tmp_1011[3] <== GLCMul()(evals[79], tmp_1009);
    signal tmp_1012[3] <== [evals[6][0] + evals[61][0], evals[6][1] + evals[61][1], evals[6][2] + evals[61][2]];
    signal tmp_1013[3] <== GLCMul()(evals[80], evals[80]);
    signal tmp_1014[3] <== GLCMul()(evals[81], tmp_1012);
    signal tmp_1015[3] <== [evals[7][0] + evals[45][0], evals[7][1] + evals[45][1], evals[7][2] + evals[45][2]];
    signal tmp_1016[3] <== GLCMul()(evals[82], evals[82]);
    signal tmp_1017[3] <== GLCMul()(evals[83], tmp_1015);
    signal tmp_1018[3] <== [evals[8][0] + evals[52][0], evals[8][1] + evals[52][1], evals[8][2] + evals[52][2]];
    signal tmp_1019[3] <== GLCMul()(evals[84], evals[84]);
    signal tmp_1020[3] <== GLCMul()(evals[85], tmp_1018);
    signal tmp_1021[3] <== [evals[9][0] + evals[54][0], evals[9][1] + evals[54][1], evals[9][2] + evals[54][2]];
    signal tmp_1022[3] <== GLCMul()(evals[86], evals[86]);
    signal tmp_1023[3] <== GLCMul()(evals[87], tmp_1021);
    signal tmp_1024[3] <== [evals[10][0] + evals[88][0], evals[10][1] + evals[88][1], evals[10][2] + evals[88][2]];
    signal tmp_1025[3] <== GLCMul()(evals[89], evals[89]);
    signal tmp_1026[3] <== GLCMul()(evals[90], tmp_1024);
    signal tmp_1027[3] <== [evals[11][0] + evals[46][0], evals[11][1] + evals[46][1], evals[11][2] + evals[46][2]];
    signal tmp_1028[3] <== GLCMul()(evals[91], evals[91]);
    signal tmp_1029[3] <== GLCMul()(evals[92], tmp_1027);
    signal tmp_1030[3] <== [evals[12][0] + evals[93][0], evals[12][1] + evals[93][1], evals[12][2] + evals[93][2]];
    signal tmp_1031[3] <== GLCMul()(evals[94], evals[94]);
    signal tmp_1032[3] <== GLCMul()(evals[95], tmp_1030);
    signal tmp_521[3] <== [evals[0][0] + evals[19][0], evals[0][1] + evals[19][1], evals[0][2] + evals[19][2]];
    signal tmp_1033[3] <== GLCMul()(tmp_521, evals[88]);
    signal tmp_522[3] <== [evals[2][0] + evals[56][0], evals[2][1] + evals[56][1], evals[2][2] + evals[56][2]];
    signal tmp_1034[3] <== GLCMul()(tmp_522, evals[88]);
    signal tmp_1035[3] <== [evals[4][0] + evals[58][0], evals[4][1] + evals[58][1], evals[4][2] + evals[58][2]];
    signal tmp_1036[3] <== [evals[5][0] + evals[60][0], evals[5][1] + evals[60][1], evals[5][2] + evals[60][2]];
    signal tmp_523[3] <== [evals[3][0] + evals[57][0], evals[3][1] + evals[57][1], evals[3][2] + evals[57][2]];
    signal tmp_1037[3] <== GLCMul()(tmp_523, evals[88]);
    signal tmp_1038[3] <== [evals[6][0] + evals[61][0], evals[6][1] + evals[61][1], evals[6][2] + evals[61][2]];
    signal tmp_524[3] <== [evals[32][0] + evals[34][0], evals[32][1] + evals[34][1], evals[32][2] + evals[34][2]];
    signal tmp_525[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1039[3] <== GLCMul()(tmp_524, tmp_525);
    signal tmp_526[3] <== [evals[34][0] + evals[35][0], evals[34][1] + evals[35][1], evals[34][2] + evals[35][2]];
    signal tmp_527[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1040[3] <== GLCMul()(tmp_526, tmp_527);
    signal tmp_1041[3] <== GLCMul()(evals[34], evals[37]);
    signal tmp_1042[3] <== GLCMul()(evals[32], evals[36]);
    signal tmp_528[3] <== [tmp_1039[0] + tmp_1040[0], tmp_1039[1] + tmp_1040[1], tmp_1039[2] + tmp_1040[2]];
    signal tmp_529[3] <== GLCMul()([2, 0, 0], tmp_1041);
    signal tmp_530[3] <== [tmp_528[0] - tmp_529[0] + p, tmp_528[1] - tmp_529[1] + p, tmp_528[2] - tmp_529[2] + p];
    signal tmp_531[3] <== [tmp_530[0] - tmp_1042[0] + p, tmp_530[1] - tmp_1042[1] + p, tmp_530[2] - tmp_1042[2] + p];
    signal tmp_1043[3] <== [tmp_531[0] + evals[11][0], tmp_531[1] + evals[11][1], tmp_531[2] + evals[11][2]];
    signal tmp_532[3] <== [evals[32][0] + evals[35][0], evals[32][1] + evals[35][1], evals[32][2] + evals[35][2]];
    signal tmp_533[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1044[3] <== GLCMul()(tmp_532, tmp_533);
    signal tmp_534[3] <== [tmp_1044[0] - tmp_1042[0] + p, tmp_1044[1] - tmp_1042[1] + p, tmp_1044[2] - tmp_1042[2] + p];
    signal tmp_535[3] <== [tmp_534[0] + tmp_1041[0], tmp_534[1] + tmp_1041[1], tmp_534[2] + tmp_1041[2]];
    signal tmp_1045[3] <== [tmp_535[0] + evals[12][0], tmp_535[1] + evals[12][1], tmp_535[2] + evals[12][2]];
    signal tmp_536[3] <== [tmp_1043[0] + tmp_1045[0], tmp_1043[1] + tmp_1045[1], tmp_1043[2] + tmp_1045[2]];
    signal tmp_537[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1046[3] <== GLCMul()(tmp_536, tmp_537);
    signal tmp_1047[3] <== GLCMul()(evals[35], evals[38]);
    signal tmp_538[3] <== [tmp_1040[0] + tmp_1042[0], tmp_1040[1] + tmp_1042[1], tmp_1040[2] + tmp_1042[2]];
    signal tmp_539[3] <== [tmp_538[0] - tmp_1041[0] + p, tmp_538[1] - tmp_1041[1] + p, tmp_538[2] - tmp_1041[2] + p];
    signal tmp_540[3] <== [tmp_539[0] - tmp_1047[0] + p, tmp_539[1] - tmp_1047[1] + p, tmp_539[2] - tmp_1047[2] + p];
    signal tmp_1048[3] <== [tmp_540[0] + evals[10][0], tmp_540[1] + evals[10][1], tmp_540[2] + evals[10][2]];
    signal tmp_1049[3] <== GLCMul()(tmp_1048, evals[36]);
    signal tmp_1050[3] <== GLCMul()(tmp_1043, evals[37]);
    signal tmp_1051[3] <== GLCMul()(tmp_1045, evals[38]);
    signal tmp_541[3] <== [tmp_1048[0] + tmp_1043[0], tmp_1048[1] + tmp_1043[1], tmp_1048[2] + tmp_1043[2]];
    signal tmp_542[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1052[3] <== GLCMul()(tmp_541, tmp_542);
    signal tmp_543[3] <== [tmp_1048[0] + tmp_1045[0], tmp_1048[1] + tmp_1045[1], tmp_1048[2] + tmp_1045[2]];
    signal tmp_544[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1053[3] <== GLCMul()(tmp_543, tmp_544);
    signal tmp_545[3] <== [evals[96][0] + evals[97][0], evals[96][1] + evals[97][1], evals[96][2] + evals[97][2]];
    signal tmp_546[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1054[3] <== GLCMul()(tmp_545, tmp_546);
    signal tmp_547[3] <== [evals[97][0] + evals[98][0], evals[97][1] + evals[98][1], evals[97][2] + evals[98][2]];
    signal tmp_548[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1055[3] <== GLCMul()(tmp_547, tmp_548);
    signal tmp_1056[3] <== GLCMul()(evals[97], evals[37]);
    signal tmp_1057[3] <== GLCMul()(evals[96], evals[36]);
    signal tmp_549[3] <== [tmp_1054[0] + tmp_1055[0], tmp_1054[1] + tmp_1055[1], tmp_1054[2] + tmp_1055[2]];
    signal tmp_550[3] <== GLCMul()([2, 0, 0], tmp_1056);
    signal tmp_551[3] <== [tmp_549[0] - tmp_550[0] + p, tmp_549[1] - tmp_550[1] + p, tmp_549[2] - tmp_550[2] + p];
    signal tmp_552[3] <== [tmp_551[0] - tmp_1057[0] + p, tmp_551[1] - tmp_1057[1] + p, tmp_551[2] - tmp_1057[2] + p];
    signal tmp_1058[3] <== [tmp_552[0] + evals[5][0], tmp_552[1] + evals[5][1], tmp_552[2] + evals[5][2]];
    signal tmp_553[3] <== [evals[96][0] + evals[98][0], evals[96][1] + evals[98][1], evals[96][2] + evals[98][2]];
    signal tmp_554[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1059[3] <== GLCMul()(tmp_553, tmp_554);
    signal tmp_555[3] <== [tmp_1059[0] - tmp_1057[0] + p, tmp_1059[1] - tmp_1057[1] + p, tmp_1059[2] - tmp_1057[2] + p];
    signal tmp_556[3] <== [tmp_555[0] + tmp_1056[0], tmp_555[1] + tmp_1056[1], tmp_555[2] + tmp_1056[2]];
    signal tmp_1060[3] <== [tmp_556[0] + evals[6][0], tmp_556[1] + evals[6][1], tmp_556[2] + evals[6][2]];
    signal tmp_557[3] <== [tmp_1058[0] + tmp_1060[0], tmp_1058[1] + tmp_1060[1], tmp_1058[2] + tmp_1060[2]];
    signal tmp_558[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1061[3] <== GLCMul()(tmp_557, tmp_558);
    signal tmp_1062[3] <== GLCMul()(evals[98], evals[38]);
    signal tmp_559[3] <== [tmp_1055[0] + tmp_1057[0], tmp_1055[1] + tmp_1057[1], tmp_1055[2] + tmp_1057[2]];
    signal tmp_560[3] <== [tmp_559[0] - tmp_1056[0] + p, tmp_559[1] - tmp_1056[1] + p, tmp_559[2] - tmp_1056[2] + p];
    signal tmp_561[3] <== [tmp_560[0] - tmp_1062[0] + p, tmp_560[1] - tmp_1062[1] + p, tmp_560[2] - tmp_1062[2] + p];
    signal tmp_1063[3] <== [tmp_561[0] + evals[4][0], tmp_561[1] + evals[4][1], tmp_561[2] + evals[4][2]];
    signal tmp_1064[3] <== GLCMul()(tmp_1063, evals[36]);
    signal tmp_1065[3] <== GLCMul()(tmp_1058, evals[37]);
    signal tmp_1066[3] <== GLCMul()(tmp_1060, evals[38]);
    signal tmp_562[3] <== [tmp_1063[0] + tmp_1058[0], tmp_1063[1] + tmp_1058[1], tmp_1063[2] + tmp_1058[2]];
    signal tmp_563[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1067[3] <== GLCMul()(tmp_562, tmp_563);
    signal tmp_564[3] <== [tmp_1063[0] + tmp_1060[0], tmp_1063[1] + tmp_1060[1], tmp_1063[2] + tmp_1060[2]];
    signal tmp_565[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1068[3] <== GLCMul()(tmp_564, tmp_565);
    signal tmp_1069[3] <== evals[0];
    signal tmp_566[3] <== GLCMul()(challenges[3], challenges[7]);
    signal tmp_567[3] <== [tmp_1069[0] + tmp_566[0], tmp_1069[1] + tmp_566[1], tmp_1069[2] + tmp_566[2]];
    signal tmp_1070[3] <== [tmp_567[0] + challenges[2][0], tmp_567[1] + challenges[2][1], tmp_567[2] + challenges[2][2]];
    signal tmp_1071[3] <== evals[2];
    signal tmp_568[3] <== GLCMul()(challenges[3], [12275445934081160404, 0, 0]);
    signal tmp_569[3] <== GLCMul()(tmp_568, challenges[7]);
    signal tmp_570[3] <== [tmp_1071[0] + tmp_569[0], tmp_1071[1] + tmp_569[1], tmp_1071[2] + tmp_569[2]];
    signal tmp_571[3] <== [tmp_570[0] + challenges[2][0], tmp_570[1] + challenges[2][1], tmp_570[2] + challenges[2][2]];
    signal tmp_1072[3] <== GLCMul()(tmp_1070, tmp_571);
    signal tmp_1073[3] <== evals[3];
    signal tmp_1074[3] <== evals[99];
    signal tmp_572[3] <== GLCMul()(challenges[3], tmp_1074);
    signal tmp_573[3] <== [tmp_1069[0] + tmp_572[0], tmp_1069[1] + tmp_572[1], tmp_1069[2] + tmp_572[2]];
    signal tmp_1075[3] <== [tmp_573[0] + challenges[2][0], tmp_573[1] + challenges[2][1], tmp_573[2] + challenges[2][2]];
    signal tmp_1076[3] <== evals[100];
    signal tmp_574[3] <== GLCMul()(challenges[3], tmp_1076);
    signal tmp_575[3] <== [tmp_1071[0] + tmp_574[0], tmp_1071[1] + tmp_574[1], tmp_1071[2] + tmp_574[2]];
    signal tmp_576[3] <== [tmp_575[0] + challenges[2][0], tmp_575[1] + challenges[2][1], tmp_575[2] + challenges[2][2]];
    signal tmp_1077[3] <== GLCMul()(tmp_1075, tmp_576);
    signal tmp_1078[3] <== evals[101];
    signal tmp_1079[3] <== evals[4];
    signal tmp_577[3] <== GLCMul()(challenges[3], [1279992132519201448, 0, 0]);
    signal tmp_578[3] <== GLCMul()(tmp_577, challenges[7]);
    signal tmp_579[3] <== [tmp_1079[0] + tmp_578[0], tmp_1079[1] + tmp_578[1], tmp_1079[2] + tmp_578[2]];
    signal tmp_580[3] <== [tmp_579[0] + challenges[2][0], tmp_579[1] + challenges[2][1], tmp_579[2] + challenges[2][2]];
    signal tmp_1080[3] <== GLCMul()(evals[102], tmp_580);
    signal tmp_1081[3] <== evals[5];
    signal tmp_1082[3] <== evals[103];
    signal tmp_581[3] <== GLCMul()(challenges[3], tmp_1082);
    signal tmp_582[3] <== [tmp_1079[0] + tmp_581[0], tmp_1079[1] + tmp_581[1], tmp_1079[2] + tmp_581[2]];
    signal tmp_583[3] <== [tmp_582[0] + challenges[2][0], tmp_582[1] + challenges[2][1], tmp_582[2] + challenges[2][2]];
    signal tmp_1083[3] <== GLCMul()(evals[104], tmp_583);
    signal tmp_1084[3] <== evals[105];
    signal tmp_1085[3] <== evals[6];
    signal tmp_584[3] <== GLCMul()(challenges[3], [7781028390488215464, 0, 0]);
    signal tmp_585[3] <== GLCMul()(tmp_584, challenges[7]);
    signal tmp_586[3] <== [tmp_1085[0] + tmp_585[0], tmp_1085[1] + tmp_585[1], tmp_1085[2] + tmp_585[2]];
    signal tmp_587[3] <== [tmp_586[0] + challenges[2][0], tmp_586[1] + challenges[2][1], tmp_586[2] + challenges[2][2]];
    signal tmp_1086[3] <== GLCMul()(evals[106], tmp_587);
    signal tmp_1087[3] <== evals[7];
    signal tmp_1088[3] <== evals[107];
    signal tmp_588[3] <== GLCMul()(challenges[3], tmp_1088);
    signal tmp_589[3] <== [tmp_1085[0] + tmp_588[0], tmp_1085[1] + tmp_588[1], tmp_1085[2] + tmp_588[2]];
    signal tmp_590[3] <== [tmp_589[0] + challenges[2][0], tmp_589[1] + challenges[2][1], tmp_589[2] + challenges[2][2]];
    signal tmp_1089[3] <== GLCMul()(evals[108], tmp_590);
    signal tmp_1090[3] <== evals[109];
    signal tmp_1091[3] <== evals[8];
    signal tmp_591[3] <== GLCMul()(challenges[3], [4549350404001778198, 0, 0]);
    signal tmp_592[3] <== GLCMul()(tmp_591, challenges[7]);
    signal tmp_593[3] <== [tmp_1091[0] + tmp_592[0], tmp_1091[1] + tmp_592[1], tmp_1091[2] + tmp_592[2]];
    signal tmp_594[3] <== [tmp_593[0] + challenges[2][0], tmp_593[1] + challenges[2][1], tmp_593[2] + challenges[2][2]];
    signal tmp_1092[3] <== GLCMul()(evals[110], tmp_594);
    signal tmp_1093[3] <== evals[9];
    signal tmp_1094[3] <== evals[111];
    signal tmp_595[3] <== GLCMul()(challenges[3], tmp_1094);
    signal tmp_596[3] <== [tmp_1091[0] + tmp_595[0], tmp_1091[1] + tmp_595[1], tmp_1091[2] + tmp_595[2]];
    signal tmp_597[3] <== [tmp_596[0] + challenges[2][0], tmp_596[1] + challenges[2][1], tmp_596[2] + challenges[2][2]];
    signal tmp_1095[3] <== GLCMul()(evals[112], tmp_597);
    signal tmp_1096[3] <== evals[113];
    signal tmp_1097[3] <== evals[10];
    signal tmp_598[3] <== GLCMul()(challenges[3], [16725109960945739746, 0, 0]);
    signal tmp_599[3] <== GLCMul()(tmp_598, challenges[7]);
    signal tmp_600[3] <== [tmp_1097[0] + tmp_599[0], tmp_1097[1] + tmp_599[1], tmp_1097[2] + tmp_599[2]];
    signal tmp_601[3] <== [tmp_600[0] + challenges[2][0], tmp_600[1] + challenges[2][1], tmp_600[2] + challenges[2][2]];
    signal tmp_1098[3] <== GLCMul()(evals[114], tmp_601);
    signal tmp_1099[3] <== evals[11];
    signal tmp_1100[3] <== evals[115];
    signal tmp_602[3] <== GLCMul()(challenges[3], tmp_1100);
    signal tmp_603[3] <== [tmp_1097[0] + tmp_602[0], tmp_1097[1] + tmp_602[1], tmp_1097[2] + tmp_602[2]];
    signal tmp_604[3] <== [tmp_603[0] + challenges[2][0], tmp_603[1] + challenges[2][1], tmp_603[2] + challenges[2][2]];
    signal tmp_1101[3] <== GLCMul()(evals[116], tmp_604);
    signal tmp_1102[3] <== evals[117];
    signal tmp_605[3] <== GLCMulAdd()(challenges[4], tmp_916, tmp_917);
    signal tmp_606[3] <== GLCMulAdd()(challenges[4], tmp_605, tmp_918);
    signal tmp_607[3] <== GLCMulAdd()(challenges[4], tmp_606, tmp_919);
    signal tmp_608[3] <== GLCMulAdd()(challenges[4], tmp_607, tmp_920);
    signal tmp_609[3] <== GLCMulAdd()(challenges[4], tmp_608, tmp_921);
    signal tmp_610[3] <== GLCMulAdd()(challenges[4], tmp_609, tmp_922);
    signal tmp_611[3] <== GLCMulAdd()(challenges[4], tmp_610, tmp_923);
    signal tmp_612[3] <== GLCMulAdd()(challenges[4], tmp_611, tmp_924);
    signal tmp_613[3] <== GLCMulAdd()(challenges[4], tmp_612, tmp_925);
    signal tmp_614[3] <== GLCMulAdd()(challenges[4], tmp_613, tmp_926);
    signal tmp_615[3] <== GLCMulAdd()(challenges[4], tmp_614, tmp_927);
    signal tmp_616[3] <== GLCMulAdd()(challenges[4], tmp_615, tmp_928);
    signal tmp_617[3] <== GLCMulAdd()(challenges[4], tmp_616, tmp_929);
    signal tmp_618[3] <== GLCMulAdd()(challenges[4], tmp_617, tmp_930);
    signal tmp_619[3] <== GLCMulAdd()(challenges[4], tmp_618, tmp_931);
    signal tmp_620[3] <== GLCMulAdd()(challenges[4], tmp_619, tmp_932);
    signal tmp_621[3] <== GLCMulAdd()(challenges[4], tmp_620, tmp_933);
    signal tmp_622[3] <== GLCMulAdd()(challenges[4], tmp_621, tmp_934);
    signal tmp_623[3] <== GLCMulAdd()(challenges[4], tmp_622, tmp_935);
    signal tmp_624[3] <== GLCMulAdd()(challenges[4], tmp_623, tmp_936);
    signal tmp_625[3] <== GLCMulAdd()(challenges[4], tmp_624, tmp_937);
    signal tmp_626[3] <== GLCMulAdd()(challenges[4], tmp_625, tmp_941);
    signal tmp_627[3] <== GLCMulAdd()(challenges[4], tmp_626, tmp_942);
    signal tmp_628[3] <== GLCMulAdd()(challenges[4], tmp_627, tmp_943);
    signal tmp_629[3] <== GLCMulAdd()(challenges[4], tmp_628, tmp_944);
    signal tmp_630[3] <== GLCMulAdd()(challenges[4], tmp_629, tmp_945);
    signal tmp_631[3] <== GLCMulAdd()(challenges[4], tmp_630, tmp_946);
    signal tmp_632[3] <== GLCMulAdd()(challenges[4], tmp_631, tmp_947);
    signal tmp_633[3] <== GLCMulAdd()(challenges[4], tmp_632, tmp_948);
    signal tmp_634[3] <== GLCMulAdd()(challenges[4], tmp_633, tmp_949);
    signal tmp_635[3] <== GLCMulAdd()(challenges[4], tmp_634, tmp_950);
    signal tmp_636[3] <== GLCMulAdd()(challenges[4], tmp_635, tmp_951);
    signal tmp_637[3] <== GLCMulAdd()(challenges[4], tmp_636, tmp_952);
    signal tmp_638[3] <== GLCMulAdd()(challenges[4], tmp_637, tmp_955);
    signal tmp_639[3] <== GLCMulAdd()(challenges[4], tmp_638, tmp_958);
    signal tmp_640[3] <== GLCMulAdd()(challenges[4], tmp_639, tmp_961);
    signal tmp_641[3] <== GLCMulAdd()(challenges[4], tmp_640, tmp_963);
    signal tmp_642[3] <== GLCMulAdd()(challenges[4], tmp_641, tmp_965);
    signal tmp_643[3] <== GLCMulAdd()(challenges[4], tmp_642, tmp_967);
    signal tmp_644[3] <== GLCMulAdd()(challenges[4], tmp_643, tmp_969);
    signal tmp_645[3] <== GLCMulAdd()(challenges[4], tmp_644, tmp_971);
    signal tmp_646[3] <== GLCMulAdd()(challenges[4], tmp_645, tmp_973);
    signal tmp_647[3] <== GLCMulAdd()(challenges[4], tmp_646, tmp_975);
    signal tmp_648[3] <== GLCMulAdd()(challenges[4], tmp_647, tmp_977);
    signal tmp_649[3] <== GLCMulAdd()(challenges[4], tmp_648, tmp_979);
    signal tmp_650[3] <== GLCMulAdd()(challenges[4], tmp_649, tmp_981);
    signal tmp_651[3] <== GLCMulAdd()(challenges[4], tmp_650, tmp_983);
    signal tmp_652[3] <== GLCMulAdd()(challenges[4], tmp_651, tmp_985);
    signal tmp_653[3] <== GLCMulAdd()(challenges[4], tmp_652, tmp_986);
    signal tmp_654[3] <== GLCMulAdd()(challenges[4], tmp_653, tmp_987);
    signal tmp_655[3] <== GLCMulAdd()(challenges[4], tmp_654, tmp_988);
    signal tmp_656[3] <== GLCMulAdd()(challenges[4], tmp_655, tmp_989);
    signal tmp_657[3] <== GLCMulAdd()(challenges[4], tmp_656, tmp_994);
    signal tmp_658[3] <== GLCMul()(evals[19], evals[0]);
    signal tmp_659[3] <== GLCMulAdd()(evals[58], tmp_995, tmp_658);
    signal tmp_660[3] <== GLCMulAdd()(evals[56], evals[2], tmp_659);
    signal tmp_661[3] <== GLCMulAdd()(evals[57], evals[3], tmp_660);
    signal tmp_662[3] <== [tmp_661[0] + evals[60][0], tmp_661[1] + evals[60][1], tmp_661[2] + evals[60][2]];
    signal tmp_663[3] <== [tmp_662[0] - evals[14][0] + p, tmp_662[1] - evals[14][1] + p, tmp_662[2] - evals[14][2] + p];
    signal tmp_664[3] <== GLCMulAdd()(challenges[4], tmp_657, tmp_663);
    signal tmp_665[3] <== GLCMul()(evals[19], evals[4]);
    signal tmp_666[3] <== GLCMulAdd()(evals[58], tmp_996, tmp_665);
    signal tmp_667[3] <== GLCMulAdd()(evals[56], evals[5], tmp_666);
    signal tmp_668[3] <== GLCMulAdd()(evals[57], evals[6], tmp_667);
    signal tmp_669[3] <== [tmp_668[0] + evals[60][0], tmp_668[1] + evals[60][1], tmp_668[2] + evals[60][2]];
    signal tmp_670[3] <== [tmp_669[0] - evals[16][0] + p, tmp_669[1] - evals[16][1] + p, tmp_669[2] - evals[16][2] + p];
    signal tmp_671[3] <== GLCMulAdd()(challenges[4], tmp_664, tmp_670);
    signal tmp_672[3] <== GLCMul()(evals[45], evals[7]);
    signal tmp_673[3] <== GLCMulAdd()(evals[88], tmp_997, tmp_672);
    signal tmp_674[3] <== GLCMulAdd()(evals[52], evals[8], tmp_673);
    signal tmp_675[3] <== GLCMulAdd()(evals[54], evals[9], tmp_674);
    signal tmp_676[3] <== [tmp_675[0] + evals[46][0], tmp_675[1] + evals[46][1], tmp_675[2] + evals[46][2]];
    signal tmp_677[3] <== [tmp_676[0] - evals[17][0] + p, tmp_676[1] - evals[17][1] + p, tmp_676[2] - evals[17][2] + p];
    signal tmp_678[3] <== GLCMulAdd()(challenges[4], tmp_671, tmp_677);
    signal tmp_679[3] <== GLCMul()(evals[45], evals[10]);
    signal tmp_680[3] <== GLCMulAdd()(evals[88], tmp_998, tmp_679);
    signal tmp_681[3] <== GLCMulAdd()(evals[52], evals[11], tmp_680);
    signal tmp_682[3] <== GLCMulAdd()(evals[54], evals[12], tmp_681);
    signal tmp_683[3] <== [tmp_682[0] + evals[46][0], tmp_682[1] + evals[46][1], tmp_682[2] + evals[46][2]];
    signal tmp_684[3] <== [tmp_683[0] - evals[18][0] + p, tmp_683[1] - evals[18][1] + p, tmp_683[2] - evals[18][2] + p];
    signal tmp_685[3] <== GLCMulAdd()(challenges[4], tmp_678, tmp_684);
    signal tmp_686[3] <== GLCMul()(tmp_938, tmp_938);
    signal tmp_687[3] <== [tmp_686[0] - evals[71][0] + p, tmp_686[1] - evals[71][1] + p, tmp_686[2] - evals[71][2] + p];
    signal tmp_688[3] <== GLCMulAdd()(challenges[4], tmp_685, tmp_687);
    signal tmp_689[3] <== GLCMul()(tmp_999, evals[71]);
    signal tmp_690[3] <== [tmp_689[0] - evals[20][0] + p, tmp_689[1] - evals[20][1] + p, tmp_689[2] - evals[20][2] + p];
    signal tmp_691[3] <== GLCMulAdd()(challenges[4], tmp_688, tmp_690);
    signal tmp_692[3] <== GLCMul()(tmp_1000, tmp_1000);
    signal tmp_693[3] <== [tmp_692[0] - evals[72][0] + p, tmp_692[1] - evals[72][1] + p, tmp_692[2] - evals[72][2] + p];
    signal tmp_694[3] <== GLCMulAdd()(challenges[4], tmp_691, tmp_693);
    signal tmp_695[3] <== GLCMul()(tmp_1001, evals[72]);
    signal tmp_696[3] <== [tmp_695[0] - evals[73][0] + p, tmp_695[1] - evals[73][1] + p, tmp_695[2] - evals[73][2] + p];
    signal tmp_697[3] <== GLCMulAdd()(challenges[4], tmp_694, tmp_696);
    signal tmp_698[3] <== [tmp_1000[0] - tmp_1002[0] + p, tmp_1000[1] - tmp_1002[1] + p, tmp_1000[2] - tmp_1002[2] + p];
    signal tmp_699[3] <== GLCMulAdd()(evals[118], tmp_698, tmp_1002);
    signal tmp_700[3] <== [tmp_699[0] - evals[21][0] + p, tmp_699[1] - evals[21][1] + p, tmp_699[2] - evals[21][2] + p];
    signal tmp_701[3] <== GLCMulAdd()(challenges[4], tmp_697, tmp_700);
    signal tmp_702[3] <== GLCMul()(tmp_1003, tmp_1003);
    signal tmp_703[3] <== [tmp_702[0] - evals[74][0] + p, tmp_702[1] - evals[74][1] + p, tmp_702[2] - evals[74][2] + p];
    signal tmp_704[3] <== GLCMulAdd()(challenges[4], tmp_701, tmp_703);
    signal tmp_705[3] <== GLCMul()(tmp_1004, evals[74]);
    signal tmp_706[3] <== [tmp_705[0] - evals[75][0] + p, tmp_705[1] - evals[75][1] + p, tmp_705[2] - evals[75][2] + p];
    signal tmp_707[3] <== GLCMulAdd()(challenges[4], tmp_704, tmp_706);
    signal tmp_708[3] <== [tmp_1003[0] - tmp_1005[0] + p, tmp_1003[1] - tmp_1005[1] + p, tmp_1003[2] - tmp_1005[2] + p];
    signal tmp_709[3] <== GLCMulAdd()(evals[118], tmp_708, tmp_1005);
    signal tmp_710[3] <== [tmp_709[0] - evals[22][0] + p, tmp_709[1] - evals[22][1] + p, tmp_709[2] - evals[22][2] + p];
    signal tmp_711[3] <== GLCMulAdd()(challenges[4], tmp_707, tmp_710);
    signal tmp_712[3] <== GLCMul()(tmp_1006, tmp_1006);
    signal tmp_713[3] <== [tmp_712[0] - evals[76][0] + p, tmp_712[1] - evals[76][1] + p, tmp_712[2] - evals[76][2] + p];
    signal tmp_714[3] <== GLCMulAdd()(challenges[4], tmp_711, tmp_713);
    signal tmp_715[3] <== GLCMul()(tmp_1007, evals[76]);
    signal tmp_716[3] <== [tmp_715[0] - evals[77][0] + p, tmp_715[1] - evals[77][1] + p, tmp_715[2] - evals[77][2] + p];
    signal tmp_717[3] <== GLCMulAdd()(challenges[4], tmp_714, tmp_716);
    signal tmp_718[3] <== [tmp_1006[0] - tmp_1008[0] + p, tmp_1006[1] - tmp_1008[1] + p, tmp_1006[2] - tmp_1008[2] + p];
    signal tmp_719[3] <== GLCMulAdd()(evals[118], tmp_718, tmp_1008);
    signal tmp_720[3] <== [tmp_719[0] - evals[23][0] + p, tmp_719[1] - evals[23][1] + p, tmp_719[2] - evals[23][2] + p];
    signal tmp_721[3] <== GLCMulAdd()(challenges[4], tmp_717, tmp_720);
    signal tmp_722[3] <== GLCMul()(tmp_1009, tmp_1009);
    signal tmp_723[3] <== [tmp_722[0] - evals[78][0] + p, tmp_722[1] - evals[78][1] + p, tmp_722[2] - evals[78][2] + p];
    signal tmp_724[3] <== GLCMulAdd()(challenges[4], tmp_721, tmp_723);
    signal tmp_725[3] <== GLCMul()(tmp_1010, evals[78]);
    signal tmp_726[3] <== [tmp_725[0] - evals[79][0] + p, tmp_725[1] - evals[79][1] + p, tmp_725[2] - evals[79][2] + p];
    signal tmp_727[3] <== GLCMulAdd()(challenges[4], tmp_724, tmp_726);
    signal tmp_728[3] <== [tmp_1009[0] - tmp_1011[0] + p, tmp_1009[1] - tmp_1011[1] + p, tmp_1009[2] - tmp_1011[2] + p];
    signal tmp_729[3] <== GLCMulAdd()(evals[118], tmp_728, tmp_1011);
    signal tmp_730[3] <== [tmp_729[0] - evals[24][0] + p, tmp_729[1] - evals[24][1] + p, tmp_729[2] - evals[24][2] + p];
    signal tmp_731[3] <== GLCMulAdd()(challenges[4], tmp_727, tmp_730);
    signal tmp_732[3] <== GLCMul()(tmp_1012, tmp_1012);
    signal tmp_733[3] <== [tmp_732[0] - evals[80][0] + p, tmp_732[1] - evals[80][1] + p, tmp_732[2] - evals[80][2] + p];
    signal tmp_734[3] <== GLCMulAdd()(challenges[4], tmp_731, tmp_733);
    signal tmp_735[3] <== GLCMul()(tmp_1013, evals[80]);
    signal tmp_736[3] <== [tmp_735[0] - evals[81][0] + p, tmp_735[1] - evals[81][1] + p, tmp_735[2] - evals[81][2] + p];
    signal tmp_737[3] <== GLCMulAdd()(challenges[4], tmp_734, tmp_736);
    signal tmp_738[3] <== [tmp_1012[0] - tmp_1014[0] + p, tmp_1012[1] - tmp_1014[1] + p, tmp_1012[2] - tmp_1014[2] + p];
    signal tmp_739[3] <== GLCMulAdd()(evals[118], tmp_738, tmp_1014);
    signal tmp_740[3] <== [tmp_739[0] - evals[25][0] + p, tmp_739[1] - evals[25][1] + p, tmp_739[2] - evals[25][2] + p];
    signal tmp_741[3] <== GLCMulAdd()(challenges[4], tmp_737, tmp_740);
    signal tmp_742[3] <== GLCMul()(tmp_1015, tmp_1015);
    signal tmp_743[3] <== [tmp_742[0] - evals[82][0] + p, tmp_742[1] - evals[82][1] + p, tmp_742[2] - evals[82][2] + p];
    signal tmp_744[3] <== GLCMulAdd()(challenges[4], tmp_741, tmp_743);
    signal tmp_745[3] <== GLCMul()(tmp_1016, evals[82]);
    signal tmp_746[3] <== [tmp_745[0] - evals[83][0] + p, tmp_745[1] - evals[83][1] + p, tmp_745[2] - evals[83][2] + p];
    signal tmp_747[3] <== GLCMulAdd()(challenges[4], tmp_744, tmp_746);
    signal tmp_748[3] <== [tmp_1015[0] - tmp_1017[0] + p, tmp_1015[1] - tmp_1017[1] + p, tmp_1015[2] - tmp_1017[2] + p];
    signal tmp_749[3] <== GLCMulAdd()(evals[118], tmp_748, tmp_1017);
    signal tmp_750[3] <== [tmp_749[0] - evals[26][0] + p, tmp_749[1] - evals[26][1] + p, tmp_749[2] - evals[26][2] + p];
    signal tmp_751[3] <== GLCMulAdd()(challenges[4], tmp_747, tmp_750);
    signal tmp_752[3] <== GLCMul()(tmp_1018, tmp_1018);
    signal tmp_753[3] <== [tmp_752[0] - evals[84][0] + p, tmp_752[1] - evals[84][1] + p, tmp_752[2] - evals[84][2] + p];
    signal tmp_754[3] <== GLCMulAdd()(challenges[4], tmp_751, tmp_753);
    signal tmp_755[3] <== GLCMul()(tmp_1019, evals[84]);
    signal tmp_756[3] <== [tmp_755[0] - evals[85][0] + p, tmp_755[1] - evals[85][1] + p, tmp_755[2] - evals[85][2] + p];
    signal tmp_757[3] <== GLCMulAdd()(challenges[4], tmp_754, tmp_756);
    signal tmp_758[3] <== [tmp_1018[0] - tmp_1020[0] + p, tmp_1018[1] - tmp_1020[1] + p, tmp_1018[2] - tmp_1020[2] + p];
    signal tmp_759[3] <== GLCMulAdd()(evals[118], tmp_758, tmp_1020);
    signal tmp_760[3] <== [tmp_759[0] - evals[27][0] + p, tmp_759[1] - evals[27][1] + p, tmp_759[2] - evals[27][2] + p];
    signal tmp_761[3] <== GLCMulAdd()(challenges[4], tmp_757, tmp_760);
    signal tmp_762[3] <== GLCMul()(tmp_1021, tmp_1021);
    signal tmp_763[3] <== [tmp_762[0] - evals[86][0] + p, tmp_762[1] - evals[86][1] + p, tmp_762[2] - evals[86][2] + p];
    signal tmp_764[3] <== GLCMulAdd()(challenges[4], tmp_761, tmp_763);
    signal tmp_765[3] <== GLCMul()(tmp_1022, evals[86]);
    signal tmp_766[3] <== [tmp_765[0] - evals[87][0] + p, tmp_765[1] - evals[87][1] + p, tmp_765[2] - evals[87][2] + p];
    signal tmp_767[3] <== GLCMulAdd()(challenges[4], tmp_764, tmp_766);
    signal tmp_768[3] <== [tmp_1021[0] - tmp_1023[0] + p, tmp_1021[1] - tmp_1023[1] + p, tmp_1021[2] - tmp_1023[2] + p];
    signal tmp_769[3] <== GLCMulAdd()(evals[118], tmp_768, tmp_1023);
    signal tmp_770[3] <== [tmp_769[0] - evals[28][0] + p, tmp_769[1] - evals[28][1] + p, tmp_769[2] - evals[28][2] + p];
    signal tmp_771[3] <== GLCMulAdd()(challenges[4], tmp_767, tmp_770);
    signal tmp_772[3] <== GLCMul()(tmp_1024, tmp_1024);
    signal tmp_773[3] <== [tmp_772[0] - evals[89][0] + p, tmp_772[1] - evals[89][1] + p, tmp_772[2] - evals[89][2] + p];
    signal tmp_774[3] <== GLCMulAdd()(challenges[4], tmp_771, tmp_773);
    signal tmp_775[3] <== GLCMul()(tmp_1025, evals[89]);
    signal tmp_776[3] <== [tmp_775[0] - evals[90][0] + p, tmp_775[1] - evals[90][1] + p, tmp_775[2] - evals[90][2] + p];
    signal tmp_777[3] <== GLCMulAdd()(challenges[4], tmp_774, tmp_776);
    signal tmp_778[3] <== [tmp_1024[0] - tmp_1026[0] + p, tmp_1024[1] - tmp_1026[1] + p, tmp_1024[2] - tmp_1026[2] + p];
    signal tmp_779[3] <== GLCMulAdd()(evals[118], tmp_778, tmp_1026);
    signal tmp_780[3] <== [tmp_779[0] - evals[29][0] + p, tmp_779[1] - evals[29][1] + p, tmp_779[2] - evals[29][2] + p];
    signal tmp_781[3] <== GLCMulAdd()(challenges[4], tmp_777, tmp_780);
    signal tmp_782[3] <== GLCMul()(tmp_1027, tmp_1027);
    signal tmp_783[3] <== [tmp_782[0] - evals[91][0] + p, tmp_782[1] - evals[91][1] + p, tmp_782[2] - evals[91][2] + p];
    signal tmp_784[3] <== GLCMulAdd()(challenges[4], tmp_781, tmp_783);
    signal tmp_785[3] <== GLCMul()(tmp_1028, evals[91]);
    signal tmp_786[3] <== [tmp_785[0] - evals[92][0] + p, tmp_785[1] - evals[92][1] + p, tmp_785[2] - evals[92][2] + p];
    signal tmp_787[3] <== GLCMulAdd()(challenges[4], tmp_784, tmp_786);
    signal tmp_788[3] <== [tmp_1027[0] - tmp_1029[0] + p, tmp_1027[1] - tmp_1029[1] + p, tmp_1027[2] - tmp_1029[2] + p];
    signal tmp_789[3] <== GLCMulAdd()(evals[118], tmp_788, tmp_1029);
    signal tmp_790[3] <== [tmp_789[0] - evals[30][0] + p, tmp_789[1] - evals[30][1] + p, tmp_789[2] - evals[30][2] + p];
    signal tmp_791[3] <== GLCMulAdd()(challenges[4], tmp_787, tmp_790);
    signal tmp_792[3] <== GLCMul()(tmp_1030, tmp_1030);
    signal tmp_793[3] <== [tmp_792[0] - evals[94][0] + p, tmp_792[1] - evals[94][1] + p, tmp_792[2] - evals[94][2] + p];
    signal tmp_794[3] <== GLCMulAdd()(challenges[4], tmp_791, tmp_793);
    signal tmp_795[3] <== GLCMul()(tmp_1031, evals[94]);
    signal tmp_796[3] <== [tmp_795[0] - evals[95][0] + p, tmp_795[1] - evals[95][1] + p, tmp_795[2] - evals[95][2] + p];
    signal tmp_797[3] <== GLCMulAdd()(challenges[4], tmp_794, tmp_796);
    signal tmp_798[3] <== [tmp_1030[0] - tmp_1032[0] + p, tmp_1030[1] - tmp_1032[1] + p, tmp_1030[2] - tmp_1032[2] + p];
    signal tmp_799[3] <== GLCMulAdd()(evals[118], tmp_798, tmp_1032);
    signal tmp_800[3] <== [tmp_799[0] - evals[31][0] + p, tmp_799[1] - evals[31][1] + p, tmp_799[2] - evals[31][2] + p];
    signal tmp_801[3] <== GLCMulAdd()(challenges[4], tmp_797, tmp_800);
    signal tmp_802[3] <== [tmp_1033[0] + tmp_1034[0], tmp_1033[1] + tmp_1034[1], tmp_1033[2] + tmp_1034[2]];
    signal tmp_803[3] <== [tmp_1035[0] + tmp_1036[0], tmp_1035[1] + tmp_1036[1], tmp_1035[2] + tmp_1036[2]];
    signal tmp_804[3] <== GLCMul()(tmp_802, tmp_803);
    signal tmp_805[3] <== [tmp_804[0] - evals[53][0] + p, tmp_804[1] - evals[53][1] + p, tmp_804[2] - evals[53][2] + p];
    signal tmp_806[3] <== GLCMulAdd()(challenges[4], tmp_801, tmp_805);
    signal tmp_807[3] <== [tmp_1033[0] + tmp_1037[0], tmp_1033[1] + tmp_1037[1], tmp_1033[2] + tmp_1037[2]];
    signal tmp_808[3] <== [tmp_1035[0] + tmp_1038[0], tmp_1035[1] + tmp_1038[1], tmp_1035[2] + tmp_1038[2]];
    signal tmp_809[3] <== GLCMul()(tmp_807, tmp_808);
    signal tmp_810[3] <== [tmp_809[0] - evals[55][0] + p, tmp_809[1] - evals[55][1] + p, tmp_809[2] - evals[55][2] + p];
    signal tmp_811[3] <== GLCMulAdd()(challenges[4], tmp_806, tmp_810);
    signal tmp_812[3] <== [tmp_1034[0] + tmp_1037[0], tmp_1034[1] + tmp_1037[1], tmp_1034[2] + tmp_1037[2]];
    signal tmp_813[3] <== [tmp_1036[0] + tmp_1038[0], tmp_1036[1] + tmp_1038[1], tmp_1036[2] + tmp_1038[2]];
    signal tmp_814[3] <== GLCMul()(tmp_812, tmp_813);
    signal tmp_815[3] <== [tmp_814[0] - evals[47][0] + p, tmp_814[1] - evals[47][1] + p, tmp_814[2] - evals[47][2] + p];
    signal tmp_816[3] <== GLCMulAdd()(challenges[4], tmp_811, tmp_815);
    signal tmp_817[3] <== GLCMul()(tmp_1033, tmp_1035);
    signal tmp_818[3] <== [tmp_817[0] - evals[48][0] + p, tmp_817[1] - evals[48][1] + p, tmp_817[2] - evals[48][2] + p];
    signal tmp_819[3] <== GLCMulAdd()(challenges[4], tmp_816, tmp_818);
    signal tmp_820[3] <== GLCMul()(tmp_1034, tmp_1036);
    signal tmp_821[3] <== [tmp_820[0] - evals[49][0] + p, tmp_820[1] - evals[49][1] + p, tmp_820[2] - evals[49][2] + p];
    signal tmp_822[3] <== GLCMulAdd()(challenges[4], tmp_819, tmp_821);
    signal tmp_823[3] <== GLCMul()(tmp_1037, tmp_1038);
    signal tmp_824[3] <== [tmp_823[0] - evals[50][0] + p, tmp_823[1] - evals[50][1] + p, tmp_823[2] - evals[50][2] + p];
    signal tmp_825[3] <== GLCMulAdd()(challenges[4], tmp_822, tmp_824);
    signal tmp_826[3] <== [tmp_1046[0] + tmp_1049[0], tmp_1046[1] + tmp_1049[1], tmp_1046[2] + tmp_1049[2]];
    signal tmp_827[3] <== [tmp_826[0] - tmp_1050[0] + p, tmp_826[1] - tmp_1050[1] + p, tmp_826[2] - tmp_1050[2] + p];
    signal tmp_828[3] <== [tmp_827[0] - tmp_1051[0] + p, tmp_827[1] - tmp_1051[1] + p, tmp_827[2] - tmp_1051[2] + p];
    signal tmp_829[3] <== [tmp_828[0] + evals[7][0], tmp_828[1] + evals[7][1], tmp_828[2] + evals[7][2]];
    signal tmp_830[3] <== [tmp_829[0] - evals[96][0] + p, tmp_829[1] - evals[96][1] + p, tmp_829[2] - evals[96][2] + p];
    signal tmp_831[3] <== GLCMulAdd()(challenges[4], tmp_825, tmp_830);
    signal tmp_832[3] <== [tmp_1052[0] + tmp_1046[0], tmp_1052[1] + tmp_1046[1], tmp_1052[2] + tmp_1046[2]];
    signal tmp_833[3] <== GLCMul()([2, 0, 0], tmp_1050);
    signal tmp_834[3] <== [tmp_832[0] - tmp_833[0] + p, tmp_832[1] - tmp_833[1] + p, tmp_832[2] - tmp_833[2] + p];
    signal tmp_835[3] <== [tmp_834[0] - tmp_1049[0] + p, tmp_834[1] - tmp_1049[1] + p, tmp_834[2] - tmp_1049[2] + p];
    signal tmp_836[3] <== [tmp_835[0] + evals[8][0], tmp_835[1] + evals[8][1], tmp_835[2] + evals[8][2]];
    signal tmp_837[3] <== [tmp_836[0] - evals[97][0] + p, tmp_836[1] - evals[97][1] + p, tmp_836[2] - evals[97][2] + p];
    signal tmp_838[3] <== GLCMulAdd()(challenges[4], tmp_831, tmp_837);
    signal tmp_839[3] <== [tmp_1053[0] - tmp_1049[0] + p, tmp_1053[1] - tmp_1049[1] + p, tmp_1053[2] - tmp_1049[2] + p];
    signal tmp_840[3] <== [tmp_839[0] + tmp_1050[0], tmp_839[1] + tmp_1050[1], tmp_839[2] + tmp_1050[2]];
    signal tmp_841[3] <== [tmp_840[0] + evals[9][0], tmp_840[1] + evals[9][1], tmp_840[2] + evals[9][2]];
    signal tmp_842[3] <== [tmp_841[0] - evals[98][0] + p, tmp_841[1] - evals[98][1] + p, tmp_841[2] - evals[98][2] + p];
    signal tmp_843[3] <== GLCMulAdd()(challenges[4], tmp_838, tmp_842);
    signal tmp_844[3] <== [tmp_1061[0] + tmp_1064[0], tmp_1061[1] + tmp_1064[1], tmp_1061[2] + tmp_1064[2]];
    signal tmp_845[3] <== [tmp_844[0] - tmp_1065[0] + p, tmp_844[1] - tmp_1065[1] + p, tmp_844[2] - tmp_1065[2] + p];
    signal tmp_846[3] <== [tmp_845[0] - tmp_1066[0] + p, tmp_845[1] - tmp_1066[1] + p, tmp_845[2] - tmp_1066[2] + p];
    signal tmp_847[3] <== [tmp_846[0] + evals[0][0], tmp_846[1] + evals[0][1], tmp_846[2] + evals[0][2]];
    signal tmp_848[3] <== [tmp_847[0] - evals[62][0] + p, tmp_847[1] - evals[62][1] + p, tmp_847[2] - evals[62][2] + p];
    signal tmp_849[3] <== GLCMulAdd()(challenges[4], tmp_843, tmp_848);
    signal tmp_850[3] <== [tmp_1067[0] + tmp_1061[0], tmp_1067[1] + tmp_1061[1], tmp_1067[2] + tmp_1061[2]];
    signal tmp_851[3] <== GLCMul()([2, 0, 0], tmp_1065);
    signal tmp_852[3] <== [tmp_850[0] - tmp_851[0] + p, tmp_850[1] - tmp_851[1] + p, tmp_850[2] - tmp_851[2] + p];
    signal tmp_853[3] <== [tmp_852[0] - tmp_1064[0] + p, tmp_852[1] - tmp_1064[1] + p, tmp_852[2] - tmp_1064[2] + p];
    signal tmp_854[3] <== [tmp_853[0] + evals[2][0], tmp_853[1] + evals[2][1], tmp_853[2] + evals[2][2]];
    signal tmp_855[3] <== [tmp_854[0] - evals[64][0] + p, tmp_854[1] - evals[64][1] + p, tmp_854[2] - evals[64][2] + p];
    signal tmp_856[3] <== GLCMulAdd()(challenges[4], tmp_849, tmp_855);
    signal tmp_857[3] <== [tmp_1068[0] - tmp_1064[0] + p, tmp_1068[1] - tmp_1064[1] + p, tmp_1068[2] - tmp_1064[2] + p];
    signal tmp_858[3] <== [tmp_857[0] + tmp_1065[0], tmp_857[1] + tmp_1065[1], tmp_857[2] + tmp_1065[2]];
    signal tmp_859[3] <== [tmp_858[0] + evals[3][0], tmp_858[1] + evals[3][1], tmp_858[2] + evals[3][2]];
    signal tmp_860[3] <== [tmp_859[0] - evals[65][0] + p, tmp_859[1] - evals[65][1] + p, tmp_859[2] - evals[65][2] + p];
    signal tmp_861[3] <== GLCMulAdd()(challenges[4], tmp_856, tmp_860);
    signal tmp_862[3] <== GLCMul()(challenges[3], [4756475762779100925, 0, 0]);
    signal tmp_863[3] <== GLCMulAdd()(tmp_862, challenges[7], tmp_1073);
    signal tmp_864[3] <== [tmp_863[0] + challenges[2][0], tmp_863[1] + challenges[2][1], tmp_863[2] + challenges[2][2]];
    signal tmp_865[3] <== GLCMul()(tmp_1072, tmp_864);
    signal tmp_866[3] <== [tmp_865[0] - evals[102][0] + p, tmp_865[1] - evals[102][1] + p, tmp_865[2] - evals[102][2] + p];
    signal tmp_867[3] <== GLCMulAdd()(challenges[4], tmp_861, tmp_866);
    signal tmp_868[3] <== GLCMulAdd()(challenges[3], tmp_1078, tmp_1073);
    signal tmp_869[3] <== [tmp_868[0] + challenges[2][0], tmp_868[1] + challenges[2][1], tmp_868[2] + challenges[2][2]];
    signal tmp_870[3] <== GLCMul()(tmp_1077, tmp_869);
    signal tmp_871[3] <== [tmp_870[0] - evals[104][0] + p, tmp_870[1] - evals[104][1] + p, tmp_870[2] - evals[104][2] + p];
    signal tmp_872[3] <== GLCMulAdd()(challenges[4], tmp_867, tmp_871);
    signal tmp_873[3] <== GLCMul()(challenges[3], [8312008622371998338, 0, 0]);
    signal tmp_874[3] <== GLCMulAdd()(tmp_873, challenges[7], tmp_1081);
    signal tmp_875[3] <== [tmp_874[0] + challenges[2][0], tmp_874[1] + challenges[2][1], tmp_874[2] + challenges[2][2]];
    signal tmp_876[3] <== GLCMul()(tmp_1080, tmp_875);
    signal tmp_877[3] <== [tmp_876[0] - evals[106][0] + p, tmp_876[1] - evals[106][1] + p, tmp_876[2] - evals[106][2] + p];
    signal tmp_878[3] <== GLCMulAdd()(challenges[4], tmp_872, tmp_877);
    signal tmp_879[3] <== GLCMulAdd()(challenges[3], tmp_1084, tmp_1081);
    signal tmp_880[3] <== [tmp_879[0] + challenges[2][0], tmp_879[1] + challenges[2][1], tmp_879[2] + challenges[2][2]];
    signal tmp_881[3] <== GLCMul()(tmp_1083, tmp_880);
    signal tmp_882[3] <== [tmp_881[0] - evals[108][0] + p, tmp_881[1] - evals[108][1] + p, tmp_881[2] - evals[108][2] + p];
    signal tmp_883[3] <== GLCMulAdd()(challenges[4], tmp_878, tmp_882);
    signal tmp_884[3] <== GLCMul()(challenges[3], [11302600489504509467, 0, 0]);
    signal tmp_885[3] <== GLCMulAdd()(tmp_884, challenges[7], tmp_1087);
    signal tmp_886[3] <== [tmp_885[0] + challenges[2][0], tmp_885[1] + challenges[2][1], tmp_885[2] + challenges[2][2]];
    signal tmp_887[3] <== GLCMul()(tmp_1086, tmp_886);
    signal tmp_888[3] <== [tmp_887[0] - evals[110][0] + p, tmp_887[1] - evals[110][1] + p, tmp_887[2] - evals[110][2] + p];
    signal tmp_889[3] <== GLCMulAdd()(challenges[4], tmp_883, tmp_888);
    signal tmp_890[3] <== GLCMulAdd()(challenges[3], tmp_1090, tmp_1087);
    signal tmp_891[3] <== [tmp_890[0] + challenges[2][0], tmp_890[1] + challenges[2][1], tmp_890[2] + challenges[2][2]];
    signal tmp_892[3] <== GLCMul()(tmp_1089, tmp_891);
    signal tmp_893[3] <== [tmp_892[0] - evals[112][0] + p, tmp_892[1] - evals[112][1] + p, tmp_892[2] - evals[112][2] + p];
    signal tmp_894[3] <== GLCMulAdd()(challenges[4], tmp_889, tmp_893);
    signal tmp_895[3] <== GLCMul()(challenges[3], [3688660304411827445, 0, 0]);
    signal tmp_896[3] <== GLCMulAdd()(tmp_895, challenges[7], tmp_1093);
    signal tmp_897[3] <== [tmp_896[0] + challenges[2][0], tmp_896[1] + challenges[2][1], tmp_896[2] + challenges[2][2]];
    signal tmp_898[3] <== GLCMul()(tmp_1092, tmp_897);
    signal tmp_899[3] <== [tmp_898[0] - evals[114][0] + p, tmp_898[1] - evals[114][1] + p, tmp_898[2] - evals[114][2] + p];
    signal tmp_900[3] <== GLCMulAdd()(challenges[4], tmp_894, tmp_899);
    signal tmp_901[3] <== GLCMulAdd()(challenges[3], tmp_1096, tmp_1093);
    signal tmp_902[3] <== [tmp_901[0] + challenges[2][0], tmp_901[1] + challenges[2][1], tmp_901[2] + challenges[2][2]];
    signal tmp_903[3] <== GLCMul()(tmp_1095, tmp_902);
    signal tmp_904[3] <== [tmp_903[0] - evals[116][0] + p, tmp_903[1] - evals[116][1] + p, tmp_903[2] - evals[116][2] + p];
    signal tmp_905[3] <== GLCMulAdd()(challenges[4], tmp_900, tmp_904);
    signal tmp_906[3] <== GLCMul()(challenges[3], [16538725463549498621, 0, 0]);
    signal tmp_907[3] <== GLCMulAdd()(tmp_906, challenges[7], tmp_1099);
    signal tmp_908[3] <== [tmp_907[0] + challenges[2][0], tmp_907[1] + challenges[2][1], tmp_907[2] + challenges[2][2]];
    signal tmp_909[3] <== GLCMul()(tmp_1098, tmp_908);
    signal tmp_910[3] <== [tmp_909[0] - evals[69][0] + p, tmp_909[1] - evals[69][1] + p, tmp_909[2] - evals[69][2] + p];
    signal tmp_911[3] <== GLCMulAdd()(challenges[4], tmp_905, tmp_910);
    signal tmp_912[3] <== GLCMulAdd()(challenges[3], tmp_1102, tmp_1099);
    signal tmp_913[3] <== [tmp_912[0] + challenges[2][0], tmp_912[1] + challenges[2][1], tmp_912[2] + challenges[2][2]];
    signal tmp_914[3] <== GLCMul()(tmp_1101, tmp_913);
    signal tmp_915[3] <== [tmp_914[0] - evals[68][0] + p, tmp_914[1] - evals[68][1] + p, tmp_914[2] - evals[68][2] + p];
    signal tmp_1103[3] <== GLCMulAdd()(challenges[4], tmp_911, tmp_915);
    signal xN[3] <== zMul[15].out;

    signal xAcc[2][3];
    signal qStep[1][3];
    signal qAcc[2][3];
    for (var i=0; i< 2; i++) {
        if (i==0) {
            xAcc[0] <== [1, 0, 0];
            qAcc[0] <== evals[119+i];
        } else {
            xAcc[i] <== GLCMul()(xAcc[i-1], xN);
            qStep[i-1] <== GLCMul()(xAcc[i], evals[119+i]);

            qAcc[i][0] <== qAcc[i-1][0] + qStep[i-1][0];
            qAcc[i][1] <== qAcc[i-1][1] + qStep[i-1][1];
            qAcc[i][2] <== qAcc[i-1][2] + qStep[i-1][2];
        }
    }
    signal qZ[3] <== GLCMul()(qAcc[1], Z);

// Final Verification
    component normC = GLCNorm();
    normC.in[0] <== tmp_1103[0] - qZ[0];
    normC.in[1] <== tmp_1103[1] - qZ[1];
    normC.in[2] <== tmp_1103[2] - qZ[2];

    enable * normC.out[0] === 0;
    enable * normC.out[1] === 0;
    enable * normC.out[2] === 0;
}
        
template parallel VerifyQuery() {
    signal input ys[17];
    signal input challenges[8][3];
    signal input evals[121][3];
    signal input tree1[12];
    
    signal input tree3[84];
            
    signal input tree4[6];
    signal input consts[32];
    signal output out[3];
        
    component mapValues = MapValues();

    for (var i=0; i< 12; i++ ) {
        mapValues.vals1[i] <== tree1[i];
    }
    for (var i=0; i< 84; i++ ) {
        mapValues.vals3[i] <== tree3[i];
    }
    for (var i=0; i< 6; i++ ) {
        mapValues.vals4[i] <== tree4[i];
    }
    var p = 0xFFFFFFFF00000001;

    component xacc[17-1];
    for (var i=1; i<17; i++ ) {
        xacc[i-1] = GLMul();
        if (i==1) {
            xacc[i-1].ina <== ys[0]*(49 * roots(17)-49) + 49;
        } else {
            xacc[i-1].ina <== xacc[i-2].out;
        }
        xacc[i-1].inb <== ys[i]*(roots(17 - i) - 1) +1;
    }
    signal X <== xacc[15].out;
        
    component den1inv = GLCInv();
    den1inv.in[0] <== X - challenges[7][0] + p;
    den1inv.in[1] <== -challenges[7][1] + p;
    den1inv.in[2] <== -challenges[7][2] + p;

    component xDivXSubXi = GLCMul();
    xDivXSubXi.ina[0] <== X;
    xDivXSubXi.ina[1] <== 0;
    xDivXSubXi.ina[2] <== 0;
    xDivXSubXi.inb[0] <== den1inv.out[0];
    xDivXSubXi.inb[1] <== den1inv.out[1];
    xDivXSubXi.inb[2] <== den1inv.out[2];

    component wXi = GLCMul();
    wXi.ina[0] <== roots(16);
    wXi.ina[1] <== 0;
    wXi.ina[2] <== 0;
    wXi.inb[0] <== challenges[7][0];
    wXi.inb[1] <== challenges[7][1];
    wXi.inb[2] <== challenges[7][2];

    component den2inv = GLCInv();
    den2inv.in[0] <== X - wXi.out[0] + p;
    den2inv.in[1] <== -wXi.out[1] + p;
    den2inv.in[2] <== -wXi.out[2] + p;

    component xDivXSubWXi = GLCMul();
    xDivXSubWXi.ina[0] <== X;
    xDivXSubWXi.ina[1] <== 0;
    xDivXSubWXi.ina[2] <== 0;
    xDivXSubWXi.inb[0] <== den2inv.out[0];
    xDivXSubWXi.inb[1] <== den2inv.out[1];
    xDivXSubWXi.inb[2] <== den2inv.out[2];
    
    signal tmp_0[3] <== GLCMulAdd()(challenges[5], [mapValues.tree1_0, 0, 0], [mapValues.tree1_1, 0, 0]);
    signal tmp_1[3] <== GLCMulAdd()(challenges[5], tmp_0, [mapValues.tree1_2, 0, 0]);
    signal tmp_2[3] <== GLCMulAdd()(challenges[5], tmp_1, [mapValues.tree1_3, 0, 0]);
    signal tmp_3[3] <== GLCMulAdd()(challenges[5], tmp_2, [mapValues.tree1_4, 0, 0]);
    signal tmp_4[3] <== GLCMulAdd()(challenges[5], tmp_3, [mapValues.tree1_5, 0, 0]);
    signal tmp_5[3] <== GLCMulAdd()(challenges[5], tmp_4, [mapValues.tree1_6, 0, 0]);
    signal tmp_6[3] <== GLCMulAdd()(challenges[5], tmp_5, [mapValues.tree1_7, 0, 0]);
    signal tmp_7[3] <== GLCMulAdd()(challenges[5], tmp_6, [mapValues.tree1_8, 0, 0]);
    signal tmp_8[3] <== GLCMulAdd()(challenges[5], tmp_7, [mapValues.tree1_9, 0, 0]);
    signal tmp_9[3] <== GLCMulAdd()(challenges[5], tmp_8, [mapValues.tree1_10, 0, 0]);
    signal tmp_10[3] <== GLCMulAdd()(challenges[5], tmp_9, [mapValues.tree1_11, 0, 0]);
    signal tmp_11[3] <== GLCMulAdd()(challenges[5], tmp_10, mapValues.tree3_0);
    signal tmp_12[3] <== GLCMulAdd()(challenges[5], tmp_11, [mapValues.tree3_1, 0, 0]);
    signal tmp_13[3] <== GLCMulAdd()(challenges[5], tmp_12, [mapValues.tree3_2, 0, 0]);
    signal tmp_14[3] <== GLCMulAdd()(challenges[5], tmp_13, [mapValues.tree3_3, 0, 0]);
    signal tmp_15[3] <== GLCMulAdd()(challenges[5], tmp_14, [mapValues.tree3_4, 0, 0]);
    signal tmp_16[3] <== GLCMulAdd()(challenges[5], tmp_15, [mapValues.tree3_5, 0, 0]);
    signal tmp_17[3] <== GLCMulAdd()(challenges[5], tmp_16, [mapValues.tree3_6, 0, 0]);
    signal tmp_18[3] <== GLCMulAdd()(challenges[5], tmp_17, [mapValues.tree3_7, 0, 0]);
    signal tmp_19[3] <== GLCMulAdd()(challenges[5], tmp_18, [mapValues.tree3_8, 0, 0]);
    signal tmp_20[3] <== GLCMulAdd()(challenges[5], tmp_19, [mapValues.tree3_9, 0, 0]);
    signal tmp_21[3] <== GLCMulAdd()(challenges[5], tmp_20, [mapValues.tree3_10, 0, 0]);
    signal tmp_22[3] <== GLCMulAdd()(challenges[5], tmp_21, [mapValues.tree3_11, 0, 0]);
    signal tmp_23[3] <== GLCMulAdd()(challenges[5], tmp_22, [mapValues.tree3_12, 0, 0]);
    signal tmp_24[3] <== GLCMulAdd()(challenges[5], tmp_23, [mapValues.tree3_13, 0, 0]);
    signal tmp_25[3] <== GLCMulAdd()(challenges[5], tmp_24, [mapValues.tree3_14, 0, 0]);
    signal tmp_26[3] <== GLCMulAdd()(challenges[5], tmp_25, [mapValues.tree3_15, 0, 0]);
    signal tmp_27[3] <== GLCMulAdd()(challenges[5], tmp_26, [mapValues.tree3_16, 0, 0]);
    signal tmp_28[3] <== GLCMulAdd()(challenges[5], tmp_27, [mapValues.tree3_17, 0, 0]);
    signal tmp_29[3] <== GLCMulAdd()(challenges[5], tmp_28, [mapValues.tree3_18, 0, 0]);
    signal tmp_30[3] <== GLCMulAdd()(challenges[5], tmp_29, [mapValues.tree3_19, 0, 0]);
    signal tmp_31[3] <== GLCMulAdd()(challenges[5], tmp_30, [mapValues.tree3_20, 0, 0]);
    signal tmp_32[3] <== GLCMulAdd()(challenges[5], tmp_31, [mapValues.tree3_21, 0, 0]);
    signal tmp_33[3] <== GLCMulAdd()(challenges[5], tmp_32, [mapValues.tree3_22, 0, 0]);
    signal tmp_34[3] <== GLCMulAdd()(challenges[5], tmp_33, [mapValues.tree3_23, 0, 0]);
    signal tmp_35[3] <== GLCMulAdd()(challenges[5], tmp_34, [mapValues.tree3_24, 0, 0]);
    signal tmp_36[3] <== GLCMulAdd()(challenges[5], tmp_35, [mapValues.tree3_25, 0, 0]);
    signal tmp_37[3] <== GLCMulAdd()(challenges[5], tmp_36, [mapValues.tree3_26, 0, 0]);
    signal tmp_38[3] <== GLCMulAdd()(challenges[5], tmp_37, [mapValues.tree3_27, 0, 0]);
    signal tmp_39[3] <== GLCMulAdd()(challenges[5], tmp_38, [mapValues.tree3_28, 0, 0]);
    signal tmp_40[3] <== GLCMulAdd()(challenges[5], tmp_39, [mapValues.tree3_29, 0, 0]);
    signal tmp_41[3] <== GLCMulAdd()(challenges[5], tmp_40, [mapValues.tree3_30, 0, 0]);
    signal tmp_42[3] <== GLCMulAdd()(challenges[5], tmp_41, [mapValues.tree3_31, 0, 0]);
    signal tmp_43[3] <== GLCMulAdd()(challenges[5], tmp_42, [mapValues.tree3_32, 0, 0]);
    signal tmp_44[3] <== GLCMulAdd()(challenges[5], tmp_43, [mapValues.tree3_33, 0, 0]);
    signal tmp_45[3] <== GLCMulAdd()(challenges[5], tmp_44, [mapValues.tree3_34, 0, 0]);
    signal tmp_46[3] <== GLCMulAdd()(challenges[5], tmp_45, [mapValues.tree3_35, 0, 0]);
    signal tmp_47[3] <== GLCMulAdd()(challenges[5], tmp_46, [mapValues.tree3_36, 0, 0]);
    signal tmp_48[3] <== GLCMulAdd()(challenges[5], tmp_47, [mapValues.tree3_37, 0, 0]);
    signal tmp_49[3] <== GLCMulAdd()(challenges[5], tmp_48, [mapValues.tree3_38, 0, 0]);
    signal tmp_50[3] <== GLCMulAdd()(challenges[5], tmp_49, [mapValues.tree3_39, 0, 0]);
    signal tmp_51[3] <== GLCMulAdd()(challenges[5], tmp_50, [mapValues.tree3_40, 0, 0]);
    signal tmp_52[3] <== GLCMulAdd()(challenges[5], tmp_51, [mapValues.tree3_41, 0, 0]);
    signal tmp_53[3] <== GLCMulAdd()(challenges[5], tmp_52, [mapValues.tree3_42, 0, 0]);
    signal tmp_54[3] <== GLCMulAdd()(challenges[5], tmp_53, [mapValues.tree3_43, 0, 0]);
    signal tmp_55[3] <== GLCMulAdd()(challenges[5], tmp_54, [mapValues.tree3_44, 0, 0]);
    signal tmp_56[3] <== GLCMulAdd()(challenges[5], tmp_55, [mapValues.tree3_45, 0, 0]);
    signal tmp_57[3] <== GLCMulAdd()(challenges[5], tmp_56, [mapValues.tree3_46, 0, 0]);
    signal tmp_58[3] <== GLCMulAdd()(challenges[5], tmp_57, [mapValues.tree3_47, 0, 0]);
    signal tmp_59[3] <== GLCMulAdd()(challenges[5], tmp_58, [mapValues.tree3_48, 0, 0]);
    signal tmp_60[3] <== GLCMulAdd()(challenges[5], tmp_59, [mapValues.tree3_49, 0, 0]);
    signal tmp_61[3] <== GLCMulAdd()(challenges[5], tmp_60, [mapValues.tree3_50, 0, 0]);
    signal tmp_62[3] <== GLCMulAdd()(challenges[5], tmp_61, [mapValues.tree3_51, 0, 0]);
    signal tmp_63[3] <== GLCMulAdd()(challenges[5], tmp_62, mapValues.tree3_52);
    signal tmp_64[3] <== GLCMulAdd()(challenges[5], tmp_63, mapValues.tree3_53);
    signal tmp_65[3] <== GLCMulAdd()(challenges[5], tmp_64, mapValues.tree3_54);
    signal tmp_66[3] <== GLCMulAdd()(challenges[5], tmp_65, mapValues.tree3_55);
    signal tmp_67[3] <== GLCMulAdd()(challenges[5], tmp_66, mapValues.tree3_56);
    signal tmp_68[3] <== GLCMulAdd()(challenges[5], tmp_67, mapValues.tree3_57);
    signal tmp_69[3] <== GLCMulAdd()(challenges[5], tmp_68, mapValues.tree3_58);
    signal tmp_70[3] <== GLCMulAdd()(challenges[5], tmp_69, mapValues.tree3_59);
    signal tmp_71[3] <== GLCMulAdd()(challenges[5], tmp_70, mapValues.tree3_60);
    signal tmp_72[3] <== GLCMulAdd()(challenges[5], tmp_71, mapValues.tree3_61);
    signal tmp_73[3] <== GLCMulAdd()(challenges[5], tmp_72, mapValues.tree4_0);
    signal tmp_74[3] <== GLCMulAdd()(challenges[5], tmp_73, mapValues.tree4_1);
    signal tmp_75[3] <== [mapValues.tree1_0 - evals[0][0] + p, -evals[0][1] + p, -evals[0][2] + p];
    signal tmp_76[3] <== [consts[0] - evals[1][0] + p, -evals[1][1] + p, -evals[1][2] + p];
    signal tmp_77[3] <== GLCMulAdd()(tmp_75, challenges[6], tmp_76);
    signal tmp_78[3] <== [mapValues.tree1_1 - evals[2][0] + p, -evals[2][1] + p, -evals[2][2] + p];
    signal tmp_79[3] <== GLCMulAdd()(tmp_77, challenges[6], tmp_78);
    signal tmp_80[3] <== [mapValues.tree1_2 - evals[3][0] + p, -evals[3][1] + p, -evals[3][2] + p];
    signal tmp_81[3] <== GLCMulAdd()(tmp_79, challenges[6], tmp_80);
    signal tmp_82[3] <== [mapValues.tree1_3 - evals[4][0] + p, -evals[4][1] + p, -evals[4][2] + p];
    signal tmp_83[3] <== GLCMulAdd()(tmp_81, challenges[6], tmp_82);
    signal tmp_84[3] <== [mapValues.tree1_4 - evals[5][0] + p, -evals[5][1] + p, -evals[5][2] + p];
    signal tmp_85[3] <== GLCMulAdd()(tmp_83, challenges[6], tmp_84);
    signal tmp_86[3] <== [mapValues.tree1_5 - evals[6][0] + p, -evals[6][1] + p, -evals[6][2] + p];
    signal tmp_87[3] <== GLCMulAdd()(tmp_85, challenges[6], tmp_86);
    signal tmp_88[3] <== [mapValues.tree1_6 - evals[7][0] + p, -evals[7][1] + p, -evals[7][2] + p];
    signal tmp_89[3] <== GLCMulAdd()(tmp_87, challenges[6], tmp_88);
    signal tmp_90[3] <== [mapValues.tree1_7 - evals[8][0] + p, -evals[8][1] + p, -evals[8][2] + p];
    signal tmp_91[3] <== GLCMulAdd()(tmp_89, challenges[6], tmp_90);
    signal tmp_92[3] <== [mapValues.tree1_8 - evals[9][0] + p, -evals[9][1] + p, -evals[9][2] + p];
    signal tmp_93[3] <== GLCMulAdd()(tmp_91, challenges[6], tmp_92);
    signal tmp_94[3] <== [mapValues.tree1_9 - evals[10][0] + p, -evals[10][1] + p, -evals[10][2] + p];
    signal tmp_95[3] <== GLCMulAdd()(tmp_93, challenges[6], tmp_94);
    signal tmp_96[3] <== [mapValues.tree1_10 - evals[11][0] + p, -evals[11][1] + p, -evals[11][2] + p];
    signal tmp_97[3] <== GLCMulAdd()(tmp_95, challenges[6], tmp_96);
    signal tmp_98[3] <== [mapValues.tree1_11 - evals[12][0] + p, -evals[12][1] + p, -evals[12][2] + p];
    signal tmp_99[3] <== GLCMulAdd()(tmp_97, challenges[6], tmp_98);
    signal tmp_100[3] <== [consts[1] - evals[13][0] + p, -evals[13][1] + p, -evals[13][2] + p];
    signal tmp_101[3] <== GLCMulAdd()(tmp_99, challenges[6], tmp_100);
    signal tmp_102[3] <== [mapValues.tree3_1 - evals[14][0] + p, -evals[14][1] + p, -evals[14][2] + p];
    signal tmp_103[3] <== GLCMulAdd()(tmp_101, challenges[6], tmp_102);
    signal tmp_104[3] <== [consts[28] - evals[15][0] + p, -evals[15][1] + p, -evals[15][2] + p];
    signal tmp_105[3] <== GLCMulAdd()(tmp_103, challenges[6], tmp_104);
    signal tmp_106[3] <== [mapValues.tree3_2 - evals[16][0] + p, -evals[16][1] + p, -evals[16][2] + p];
    signal tmp_107[3] <== GLCMulAdd()(tmp_105, challenges[6], tmp_106);
    signal tmp_108[3] <== [mapValues.tree3_3 - evals[17][0] + p, -evals[17][1] + p, -evals[17][2] + p];
    signal tmp_109[3] <== GLCMulAdd()(tmp_107, challenges[6], tmp_108);
    signal tmp_110[3] <== [mapValues.tree3_4 - evals[18][0] + p, -evals[18][1] + p, -evals[18][2] + p];
    signal tmp_111[3] <== GLCMulAdd()(tmp_109, challenges[6], tmp_110);
    signal tmp_112[3] <== [consts[14] - evals[19][0] + p, -evals[19][1] + p, -evals[19][2] + p];
    signal tmp_113[3] <== GLCMulAdd()(tmp_111, challenges[6], tmp_112);
    signal tmp_114[3] <== [mapValues.tree3_6 - evals[20][0] + p, -evals[20][1] + p, -evals[20][2] + p];
    signal tmp_115[3] <== GLCMulAdd()(tmp_113, challenges[6], tmp_114);
    signal tmp_116[3] <== [mapValues.tree3_9 - evals[21][0] + p, -evals[21][1] + p, -evals[21][2] + p];
    signal tmp_117[3] <== GLCMulAdd()(tmp_115, challenges[6], tmp_116);
    signal tmp_118[3] <== [mapValues.tree3_12 - evals[22][0] + p, -evals[22][1] + p, -evals[22][2] + p];
    signal tmp_119[3] <== GLCMulAdd()(tmp_117, challenges[6], tmp_118);
    signal tmp_120[3] <== [mapValues.tree3_15 - evals[23][0] + p, -evals[23][1] + p, -evals[23][2] + p];
    signal tmp_121[3] <== GLCMulAdd()(tmp_119, challenges[6], tmp_120);
    signal tmp_122[3] <== [mapValues.tree3_18 - evals[24][0] + p, -evals[24][1] + p, -evals[24][2] + p];
    signal tmp_123[3] <== GLCMulAdd()(tmp_121, challenges[6], tmp_122);
    signal tmp_124[3] <== [mapValues.tree3_21 - evals[25][0] + p, -evals[25][1] + p, -evals[25][2] + p];
    signal tmp_125[3] <== GLCMulAdd()(tmp_123, challenges[6], tmp_124);
    signal tmp_126[3] <== [mapValues.tree3_24 - evals[26][0] + p, -evals[26][1] + p, -evals[26][2] + p];
    signal tmp_127[3] <== GLCMulAdd()(tmp_125, challenges[6], tmp_126);
    signal tmp_128[3] <== [mapValues.tree3_27 - evals[27][0] + p, -evals[27][1] + p, -evals[27][2] + p];
    signal tmp_129[3] <== GLCMulAdd()(tmp_127, challenges[6], tmp_128);
    signal tmp_130[3] <== [mapValues.tree3_30 - evals[28][0] + p, -evals[28][1] + p, -evals[28][2] + p];
    signal tmp_131[3] <== GLCMulAdd()(tmp_129, challenges[6], tmp_130);
    signal tmp_132[3] <== [mapValues.tree3_33 - evals[29][0] + p, -evals[29][1] + p, -evals[29][2] + p];
    signal tmp_133[3] <== GLCMulAdd()(tmp_131, challenges[6], tmp_132);
    signal tmp_134[3] <== [mapValues.tree3_36 - evals[30][0] + p, -evals[30][1] + p, -evals[30][2] + p];
    signal tmp_135[3] <== GLCMulAdd()(tmp_133, challenges[6], tmp_134);
    signal tmp_136[3] <== [mapValues.tree3_39 - evals[31][0] + p, -evals[31][1] + p, -evals[31][2] + p];
    signal tmp_137[3] <== GLCMulAdd()(tmp_135, challenges[6], tmp_136);
    signal tmp_138[3] <== [consts[27] - evals[33][0] + p, -evals[33][1] + p, -evals[33][2] + p];
    signal tmp_139[3] <== GLCMulAdd()(tmp_137, challenges[6], tmp_138);
    signal tmp_140[3] <== [consts[20] - evals[45][0] + p, -evals[45][1] + p, -evals[45][2] + p];
    signal tmp_141[3] <== GLCMulAdd()(tmp_139, challenges[6], tmp_140);
    signal tmp_142[3] <== [consts[24] - evals[46][0] + p, -evals[46][1] + p, -evals[46][2] + p];
    signal tmp_143[3] <== GLCMulAdd()(tmp_141, challenges[6], tmp_142);
    signal tmp_144[3] <== [mapValues.tree3_42 - evals[47][0] + p, -evals[47][1] + p, -evals[47][2] + p];
    signal tmp_145[3] <== GLCMulAdd()(tmp_143, challenges[6], tmp_144);
    signal tmp_146[3] <== [mapValues.tree3_43 - evals[48][0] + p, -evals[48][1] + p, -evals[48][2] + p];
    signal tmp_147[3] <== GLCMulAdd()(tmp_145, challenges[6], tmp_146);
    signal tmp_148[3] <== [mapValues.tree3_44 - evals[49][0] + p, -evals[49][1] + p, -evals[49][2] + p];
    signal tmp_149[3] <== GLCMulAdd()(tmp_147, challenges[6], tmp_148);
    signal tmp_150[3] <== [mapValues.tree3_45 - evals[50][0] + p, -evals[50][1] + p, -evals[50][2] + p];
    signal tmp_151[3] <== GLCMulAdd()(tmp_149, challenges[6], tmp_150);
    signal tmp_152[3] <== [consts[29] - evals[51][0] + p, -evals[51][1] + p, -evals[51][2] + p];
    signal tmp_153[3] <== GLCMulAdd()(tmp_151, challenges[6], tmp_152);
    signal tmp_154[3] <== [consts[21] - evals[52][0] + p, -evals[52][1] + p, -evals[52][2] + p];
    signal tmp_155[3] <== GLCMulAdd()(tmp_153, challenges[6], tmp_154);
    signal tmp_156[3] <== [mapValues.tree3_40 - evals[53][0] + p, -evals[53][1] + p, -evals[53][2] + p];
    signal tmp_157[3] <== GLCMulAdd()(tmp_155, challenges[6], tmp_156);
    signal tmp_158[3] <== [consts[22] - evals[54][0] + p, -evals[54][1] + p, -evals[54][2] + p];
    signal tmp_159[3] <== GLCMulAdd()(tmp_157, challenges[6], tmp_158);
    signal tmp_160[3] <== [mapValues.tree3_41 - evals[55][0] + p, -evals[55][1] + p, -evals[55][2] + p];
    signal tmp_161[3] <== GLCMulAdd()(tmp_159, challenges[6], tmp_160);
    signal tmp_162[3] <== [consts[15] - evals[56][0] + p, -evals[56][1] + p, -evals[56][2] + p];
    signal tmp_163[3] <== GLCMulAdd()(tmp_161, challenges[6], tmp_162);
    signal tmp_164[3] <== [consts[16] - evals[57][0] + p, -evals[57][1] + p, -evals[57][2] + p];
    signal tmp_165[3] <== GLCMulAdd()(tmp_163, challenges[6], tmp_164);
    signal tmp_166[3] <== [consts[17] - evals[58][0] + p, -evals[58][1] + p, -evals[58][2] + p];
    signal tmp_167[3] <== GLCMulAdd()(tmp_165, challenges[6], tmp_166);
    signal tmp_168[3] <== [consts[31] - evals[59][0] + p, -evals[59][1] + p, -evals[59][2] + p];
    signal tmp_169[3] <== GLCMulAdd()(tmp_167, challenges[6], tmp_168);
    signal tmp_170[3] <== [consts[18] - evals[60][0] + p, -evals[60][1] + p, -evals[60][2] + p];
    signal tmp_171[3] <== GLCMulAdd()(tmp_169, challenges[6], tmp_170);
    signal tmp_172[3] <== [consts[19] - evals[61][0] + p, -evals[61][1] + p, -evals[61][2] + p];
    signal tmp_173[3] <== GLCMulAdd()(tmp_171, challenges[6], tmp_172);
    signal tmp_174[3] <== [mapValues.tree3_49 - evals[62][0] + p, -evals[62][1] + p, -evals[62][2] + p];
    signal tmp_175[3] <== GLCMulAdd()(tmp_173, challenges[6], tmp_174);
    signal tmp_176[3] <== [consts[30] - evals[63][0] + p, -evals[63][1] + p, -evals[63][2] + p];
    signal tmp_177[3] <== GLCMulAdd()(tmp_175, challenges[6], tmp_176);
    signal tmp_178[3] <== [mapValues.tree3_50 - evals[64][0] + p, -evals[64][1] + p, -evals[64][2] + p];
    signal tmp_179[3] <== GLCMulAdd()(tmp_177, challenges[6], tmp_178);
    signal tmp_180[3] <== [mapValues.tree3_51 - evals[65][0] + p, -evals[65][1] + p, -evals[65][2] + p];
    signal tmp_181[3] <== GLCMulAdd()(tmp_179, challenges[6], tmp_180);
    signal tmp_182[3] <== [mapValues.tree3_0[0] - evals[66][0] + p, mapValues.tree3_0[1] - evals[66][1] + p, mapValues.tree3_0[2] - evals[66][2] + p];
    signal tmp_183[3] <== GLCMulAdd()(tmp_181, challenges[6], tmp_182);
    signal tmp_184[3] <== [consts[13] - evals[67][0] + p, -evals[67][1] + p, -evals[67][2] + p];
    signal tmp_185[3] <== GLCMulAdd()(tmp_183, challenges[6], tmp_184);
    signal tmp_186[3] <== [mapValues.tree3_61[0] - evals[68][0] + p, mapValues.tree3_61[1] - evals[68][1] + p, mapValues.tree3_61[2] - evals[68][2] + p];
    signal tmp_187[3] <== GLCMulAdd()(tmp_185, challenges[6], tmp_186);
    signal tmp_188[3] <== [mapValues.tree3_60[0] - evals[69][0] + p, mapValues.tree3_60[1] - evals[69][1] + p, mapValues.tree3_60[2] - evals[69][2] + p];
    signal tmp_189[3] <== GLCMulAdd()(tmp_187, challenges[6], tmp_188);
    signal tmp_190[3] <== [mapValues.tree3_5 - evals[71][0] + p, -evals[71][1] + p, -evals[71][2] + p];
    signal tmp_191[3] <== GLCMulAdd()(tmp_189, challenges[6], tmp_190);
    signal tmp_192[3] <== [mapValues.tree3_7 - evals[72][0] + p, -evals[72][1] + p, -evals[72][2] + p];
    signal tmp_193[3] <== GLCMulAdd()(tmp_191, challenges[6], tmp_192);
    signal tmp_194[3] <== [mapValues.tree3_8 - evals[73][0] + p, -evals[73][1] + p, -evals[73][2] + p];
    signal tmp_195[3] <== GLCMulAdd()(tmp_193, challenges[6], tmp_194);
    signal tmp_196[3] <== [mapValues.tree3_10 - evals[74][0] + p, -evals[74][1] + p, -evals[74][2] + p];
    signal tmp_197[3] <== GLCMulAdd()(tmp_195, challenges[6], tmp_196);
    signal tmp_198[3] <== [mapValues.tree3_11 - evals[75][0] + p, -evals[75][1] + p, -evals[75][2] + p];
    signal tmp_199[3] <== GLCMulAdd()(tmp_197, challenges[6], tmp_198);
    signal tmp_200[3] <== [mapValues.tree3_13 - evals[76][0] + p, -evals[76][1] + p, -evals[76][2] + p];
    signal tmp_201[3] <== GLCMulAdd()(tmp_199, challenges[6], tmp_200);
    signal tmp_202[3] <== [mapValues.tree3_14 - evals[77][0] + p, -evals[77][1] + p, -evals[77][2] + p];
    signal tmp_203[3] <== GLCMulAdd()(tmp_201, challenges[6], tmp_202);
    signal tmp_204[3] <== [mapValues.tree3_16 - evals[78][0] + p, -evals[78][1] + p, -evals[78][2] + p];
    signal tmp_205[3] <== GLCMulAdd()(tmp_203, challenges[6], tmp_204);
    signal tmp_206[3] <== [mapValues.tree3_17 - evals[79][0] + p, -evals[79][1] + p, -evals[79][2] + p];
    signal tmp_207[3] <== GLCMulAdd()(tmp_205, challenges[6], tmp_206);
    signal tmp_208[3] <== [mapValues.tree3_19 - evals[80][0] + p, -evals[80][1] + p, -evals[80][2] + p];
    signal tmp_209[3] <== GLCMulAdd()(tmp_207, challenges[6], tmp_208);
    signal tmp_210[3] <== [mapValues.tree3_20 - evals[81][0] + p, -evals[81][1] + p, -evals[81][2] + p];
    signal tmp_211[3] <== GLCMulAdd()(tmp_209, challenges[6], tmp_210);
    signal tmp_212[3] <== [mapValues.tree3_22 - evals[82][0] + p, -evals[82][1] + p, -evals[82][2] + p];
    signal tmp_213[3] <== GLCMulAdd()(tmp_211, challenges[6], tmp_212);
    signal tmp_214[3] <== [mapValues.tree3_23 - evals[83][0] + p, -evals[83][1] + p, -evals[83][2] + p];
    signal tmp_215[3] <== GLCMulAdd()(tmp_213, challenges[6], tmp_214);
    signal tmp_216[3] <== [mapValues.tree3_25 - evals[84][0] + p, -evals[84][1] + p, -evals[84][2] + p];
    signal tmp_217[3] <== GLCMulAdd()(tmp_215, challenges[6], tmp_216);
    signal tmp_218[3] <== [mapValues.tree3_26 - evals[85][0] + p, -evals[85][1] + p, -evals[85][2] + p];
    signal tmp_219[3] <== GLCMulAdd()(tmp_217, challenges[6], tmp_218);
    signal tmp_220[3] <== [mapValues.tree3_28 - evals[86][0] + p, -evals[86][1] + p, -evals[86][2] + p];
    signal tmp_221[3] <== GLCMulAdd()(tmp_219, challenges[6], tmp_220);
    signal tmp_222[3] <== [mapValues.tree3_29 - evals[87][0] + p, -evals[87][1] + p, -evals[87][2] + p];
    signal tmp_223[3] <== GLCMulAdd()(tmp_221, challenges[6], tmp_222);
    signal tmp_224[3] <== [consts[23] - evals[88][0] + p, -evals[88][1] + p, -evals[88][2] + p];
    signal tmp_225[3] <== GLCMulAdd()(tmp_223, challenges[6], tmp_224);
    signal tmp_226[3] <== [mapValues.tree3_31 - evals[89][0] + p, -evals[89][1] + p, -evals[89][2] + p];
    signal tmp_227[3] <== GLCMulAdd()(tmp_225, challenges[6], tmp_226);
    signal tmp_228[3] <== [mapValues.tree3_32 - evals[90][0] + p, -evals[90][1] + p, -evals[90][2] + p];
    signal tmp_229[3] <== GLCMulAdd()(tmp_227, challenges[6], tmp_228);
    signal tmp_230[3] <== [mapValues.tree3_34 - evals[91][0] + p, -evals[91][1] + p, -evals[91][2] + p];
    signal tmp_231[3] <== GLCMulAdd()(tmp_229, challenges[6], tmp_230);
    signal tmp_232[3] <== [mapValues.tree3_35 - evals[92][0] + p, -evals[92][1] + p, -evals[92][2] + p];
    signal tmp_233[3] <== GLCMulAdd()(tmp_231, challenges[6], tmp_232);
    signal tmp_234[3] <== [consts[25] - evals[93][0] + p, -evals[93][1] + p, -evals[93][2] + p];
    signal tmp_235[3] <== GLCMulAdd()(tmp_233, challenges[6], tmp_234);
    signal tmp_236[3] <== [mapValues.tree3_37 - evals[94][0] + p, -evals[94][1] + p, -evals[94][2] + p];
    signal tmp_237[3] <== GLCMulAdd()(tmp_235, challenges[6], tmp_236);
    signal tmp_238[3] <== [mapValues.tree3_38 - evals[95][0] + p, -evals[95][1] + p, -evals[95][2] + p];
    signal tmp_239[3] <== GLCMulAdd()(tmp_237, challenges[6], tmp_238);
    signal tmp_240[3] <== [mapValues.tree3_46 - evals[96][0] + p, -evals[96][1] + p, -evals[96][2] + p];
    signal tmp_241[3] <== GLCMulAdd()(tmp_239, challenges[6], tmp_240);
    signal tmp_242[3] <== [mapValues.tree3_47 - evals[97][0] + p, -evals[97][1] + p, -evals[97][2] + p];
    signal tmp_243[3] <== GLCMulAdd()(tmp_241, challenges[6], tmp_242);
    signal tmp_244[3] <== [mapValues.tree3_48 - evals[98][0] + p, -evals[98][1] + p, -evals[98][2] + p];
    signal tmp_245[3] <== GLCMulAdd()(tmp_243, challenges[6], tmp_244);
    signal tmp_246[3] <== [consts[2] - evals[99][0] + p, -evals[99][1] + p, -evals[99][2] + p];
    signal tmp_247[3] <== GLCMulAdd()(tmp_245, challenges[6], tmp_246);
    signal tmp_248[3] <== [consts[3] - evals[100][0] + p, -evals[100][1] + p, -evals[100][2] + p];
    signal tmp_249[3] <== GLCMulAdd()(tmp_247, challenges[6], tmp_248);
    signal tmp_250[3] <== [consts[4] - evals[101][0] + p, -evals[101][1] + p, -evals[101][2] + p];
    signal tmp_251[3] <== GLCMulAdd()(tmp_249, challenges[6], tmp_250);
    signal tmp_252[3] <== [mapValues.tree3_52[0] - evals[102][0] + p, mapValues.tree3_52[1] - evals[102][1] + p, mapValues.tree3_52[2] - evals[102][2] + p];
    signal tmp_253[3] <== GLCMulAdd()(tmp_251, challenges[6], tmp_252);
    signal tmp_254[3] <== [consts[5] - evals[103][0] + p, -evals[103][1] + p, -evals[103][2] + p];
    signal tmp_255[3] <== GLCMulAdd()(tmp_253, challenges[6], tmp_254);
    signal tmp_256[3] <== [mapValues.tree3_53[0] - evals[104][0] + p, mapValues.tree3_53[1] - evals[104][1] + p, mapValues.tree3_53[2] - evals[104][2] + p];
    signal tmp_257[3] <== GLCMulAdd()(tmp_255, challenges[6], tmp_256);
    signal tmp_258[3] <== [consts[6] - evals[105][0] + p, -evals[105][1] + p, -evals[105][2] + p];
    signal tmp_259[3] <== GLCMulAdd()(tmp_257, challenges[6], tmp_258);
    signal tmp_260[3] <== [mapValues.tree3_54[0] - evals[106][0] + p, mapValues.tree3_54[1] - evals[106][1] + p, mapValues.tree3_54[2] - evals[106][2] + p];
    signal tmp_261[3] <== GLCMulAdd()(tmp_259, challenges[6], tmp_260);
    signal tmp_262[3] <== [consts[7] - evals[107][0] + p, -evals[107][1] + p, -evals[107][2] + p];
    signal tmp_263[3] <== GLCMulAdd()(tmp_261, challenges[6], tmp_262);
    signal tmp_264[3] <== [mapValues.tree3_55[0] - evals[108][0] + p, mapValues.tree3_55[1] - evals[108][1] + p, mapValues.tree3_55[2] - evals[108][2] + p];
    signal tmp_265[3] <== GLCMulAdd()(tmp_263, challenges[6], tmp_264);
    signal tmp_266[3] <== [consts[8] - evals[109][0] + p, -evals[109][1] + p, -evals[109][2] + p];
    signal tmp_267[3] <== GLCMulAdd()(tmp_265, challenges[6], tmp_266);
    signal tmp_268[3] <== [mapValues.tree3_56[0] - evals[110][0] + p, mapValues.tree3_56[1] - evals[110][1] + p, mapValues.tree3_56[2] - evals[110][2] + p];
    signal tmp_269[3] <== GLCMulAdd()(tmp_267, challenges[6], tmp_268);
    signal tmp_270[3] <== [consts[9] - evals[111][0] + p, -evals[111][1] + p, -evals[111][2] + p];
    signal tmp_271[3] <== GLCMulAdd()(tmp_269, challenges[6], tmp_270);
    signal tmp_272[3] <== [mapValues.tree3_57[0] - evals[112][0] + p, mapValues.tree3_57[1] - evals[112][1] + p, mapValues.tree3_57[2] - evals[112][2] + p];
    signal tmp_273[3] <== GLCMulAdd()(tmp_271, challenges[6], tmp_272);
    signal tmp_274[3] <== [consts[10] - evals[113][0] + p, -evals[113][1] + p, -evals[113][2] + p];
    signal tmp_275[3] <== GLCMulAdd()(tmp_273, challenges[6], tmp_274);
    signal tmp_276[3] <== [mapValues.tree3_58[0] - evals[114][0] + p, mapValues.tree3_58[1] - evals[114][1] + p, mapValues.tree3_58[2] - evals[114][2] + p];
    signal tmp_277[3] <== GLCMulAdd()(tmp_275, challenges[6], tmp_276);
    signal tmp_278[3] <== [consts[11] - evals[115][0] + p, -evals[115][1] + p, -evals[115][2] + p];
    signal tmp_279[3] <== GLCMulAdd()(tmp_277, challenges[6], tmp_278);
    signal tmp_280[3] <== [mapValues.tree3_59[0] - evals[116][0] + p, mapValues.tree3_59[1] - evals[116][1] + p, mapValues.tree3_59[2] - evals[116][2] + p];
    signal tmp_281[3] <== GLCMulAdd()(tmp_279, challenges[6], tmp_280);
    signal tmp_282[3] <== [consts[12] - evals[117][0] + p, -evals[117][1] + p, -evals[117][2] + p];
    signal tmp_283[3] <== GLCMulAdd()(tmp_281, challenges[6], tmp_282);
    signal tmp_284[3] <== [consts[26] - evals[118][0] + p, -evals[118][1] + p, -evals[118][2] + p];
    signal tmp_285[3] <== GLCMulAdd()(tmp_283, challenges[6], tmp_284);
    signal tmp_286[3] <== [mapValues.tree4_0[0] - evals[119][0] + p, mapValues.tree4_0[1] - evals[119][1] + p, mapValues.tree4_0[2] - evals[119][2] + p];
    signal tmp_287[3] <== GLCMulAdd()(tmp_285, challenges[6], tmp_286);
    signal tmp_288[3] <== [mapValues.tree4_1[0] - evals[120][0] + p, mapValues.tree4_1[1] - evals[120][1] + p, mapValues.tree4_1[2] - evals[120][2] + p];
    signal tmp_289[3] <== GLCMulAdd()(tmp_287, challenges[6], tmp_288);
    signal tmp_290[3] <== GLCMul()(tmp_289, xDivXSubXi.out);
    signal tmp_291[3] <== GLCMulAdd()(challenges[5], tmp_74, tmp_290);
    signal tmp_292[3] <== [mapValues.tree1_0 - evals[32][0] + p, -evals[32][1] + p, -evals[32][2] + p];
    signal tmp_293[3] <== [mapValues.tree1_1 - evals[34][0] + p, -evals[34][1] + p, -evals[34][2] + p];
    signal tmp_294[3] <== GLCMulAdd()(tmp_292, challenges[6], tmp_293);
    signal tmp_295[3] <== [mapValues.tree1_2 - evals[35][0] + p, -evals[35][1] + p, -evals[35][2] + p];
    signal tmp_296[3] <== GLCMulAdd()(tmp_294, challenges[6], tmp_295);
    signal tmp_297[3] <== [mapValues.tree1_3 - evals[36][0] + p, -evals[36][1] + p, -evals[36][2] + p];
    signal tmp_298[3] <== GLCMulAdd()(tmp_296, challenges[6], tmp_297);
    signal tmp_299[3] <== [mapValues.tree1_4 - evals[37][0] + p, -evals[37][1] + p, -evals[37][2] + p];
    signal tmp_300[3] <== GLCMulAdd()(tmp_298, challenges[6], tmp_299);
    signal tmp_301[3] <== [mapValues.tree1_5 - evals[38][0] + p, -evals[38][1] + p, -evals[38][2] + p];
    signal tmp_302[3] <== GLCMulAdd()(tmp_300, challenges[6], tmp_301);
    signal tmp_303[3] <== [mapValues.tree1_6 - evals[39][0] + p, -evals[39][1] + p, -evals[39][2] + p];
    signal tmp_304[3] <== GLCMulAdd()(tmp_302, challenges[6], tmp_303);
    signal tmp_305[3] <== [mapValues.tree1_7 - evals[40][0] + p, -evals[40][1] + p, -evals[40][2] + p];
    signal tmp_306[3] <== GLCMulAdd()(tmp_304, challenges[6], tmp_305);
    signal tmp_307[3] <== [mapValues.tree1_8 - evals[41][0] + p, -evals[41][1] + p, -evals[41][2] + p];
    signal tmp_308[3] <== GLCMulAdd()(tmp_306, challenges[6], tmp_307);
    signal tmp_309[3] <== [mapValues.tree1_9 - evals[42][0] + p, -evals[42][1] + p, -evals[42][2] + p];
    signal tmp_310[3] <== GLCMulAdd()(tmp_308, challenges[6], tmp_309);
    signal tmp_311[3] <== [mapValues.tree1_10 - evals[43][0] + p, -evals[43][1] + p, -evals[43][2] + p];
    signal tmp_312[3] <== GLCMulAdd()(tmp_310, challenges[6], tmp_311);
    signal tmp_313[3] <== [mapValues.tree1_11 - evals[44][0] + p, -evals[44][1] + p, -evals[44][2] + p];
    signal tmp_314[3] <== GLCMulAdd()(tmp_312, challenges[6], tmp_313);
    signal tmp_315[3] <== [mapValues.tree3_0[0] - evals[70][0] + p, mapValues.tree3_0[1] - evals[70][1] + p, mapValues.tree3_0[2] - evals[70][2] + p];
    signal tmp_316[3] <== GLCMulAdd()(tmp_314, challenges[6], tmp_315);
    signal tmp_317[3] <== GLCMul()(tmp_316, xDivXSubWXi.out);
    signal tmp_318[3] <== GLCMulAdd()(challenges[5], tmp_291, tmp_317);
    component normC = GLCNorm();
    normC.in[0] <== tmp_318[0];
    normC.in[1] <== tmp_318[1];
    normC.in[2] <== tmp_318[2];

    out[0] <== normC.out[0];
    out[1] <== normC.out[1];
    out[2] <== normC.out[2];
}
    
template MapValues() {
    signal input vals1[12];
    signal input vals3[84];
    signal input vals4[6];
    signal output tree1_0;
    signal output tree1_1;
    signal output tree1_2;
    signal output tree1_3;
    signal output tree1_4;
    signal output tree1_5;
    signal output tree1_6;
    signal output tree1_7;
    signal output tree1_8;
    signal output tree1_9;
    signal output tree1_10;
    signal output tree1_11;
    signal output tree3_0[3];
    signal output tree3_1;
    signal output tree3_2;
    signal output tree3_3;
    signal output tree3_4;
    signal output tree3_5;
    signal output tree3_6;
    signal output tree3_7;
    signal output tree3_8;
    signal output tree3_9;
    signal output tree3_10;
    signal output tree3_11;
    signal output tree3_12;
    signal output tree3_13;
    signal output tree3_14;
    signal output tree3_15;
    signal output tree3_16;
    signal output tree3_17;
    signal output tree3_18;
    signal output tree3_19;
    signal output tree3_20;
    signal output tree3_21;
    signal output tree3_22;
    signal output tree3_23;
    signal output tree3_24;
    signal output tree3_25;
    signal output tree3_26;
    signal output tree3_27;
    signal output tree3_28;
    signal output tree3_29;
    signal output tree3_30;
    signal output tree3_31;
    signal output tree3_32;
    signal output tree3_33;
    signal output tree3_34;
    signal output tree3_35;
    signal output tree3_36;
    signal output tree3_37;
    signal output tree3_38;
    signal output tree3_39;
    signal output tree3_40;
    signal output tree3_41;
    signal output tree3_42;
    signal output tree3_43;
    signal output tree3_44;
    signal output tree3_45;
    signal output tree3_46;
    signal output tree3_47;
    signal output tree3_48;
    signal output tree3_49;
    signal output tree3_50;
    signal output tree3_51;
    signal output tree3_52[3];
    signal output tree3_53[3];
    signal output tree3_54[3];
    signal output tree3_55[3];
    signal output tree3_56[3];
    signal output tree3_57[3];
    signal output tree3_58[3];
    signal output tree3_59[3];
    signal output tree3_60[3];
    signal output tree3_61[3];
    signal output tree4_0[3];
    signal output tree4_1[3];
    tree1_0 <== vals1[0];
    tree1_1 <== vals1[1];
    tree1_2 <== vals1[2];
    tree1_3 <== vals1[3];
    tree1_4 <== vals1[4];
    tree1_5 <== vals1[5];
    tree1_6 <== vals1[6];
    tree1_7 <== vals1[7];
    tree1_8 <== vals1[8];
    tree1_9 <== vals1[9];
    tree1_10 <== vals1[10];
    tree1_11 <== vals1[11];
    tree3_0[0] <== vals3[51];
    tree3_0[1] <== vals3[52];
    tree3_0[2] <== vals3[53];
    tree3_1 <== vals3[0];
    tree3_2 <== vals3[1];
    tree3_3 <== vals3[2];
    tree3_4 <== vals3[3];
    tree3_5 <== vals3[4];
    tree3_6 <== vals3[5];
    tree3_7 <== vals3[6];
    tree3_8 <== vals3[7];
    tree3_9 <== vals3[8];
    tree3_10 <== vals3[9];
    tree3_11 <== vals3[10];
    tree3_12 <== vals3[11];
    tree3_13 <== vals3[12];
    tree3_14 <== vals3[13];
    tree3_15 <== vals3[14];
    tree3_16 <== vals3[15];
    tree3_17 <== vals3[16];
    tree3_18 <== vals3[17];
    tree3_19 <== vals3[18];
    tree3_20 <== vals3[19];
    tree3_21 <== vals3[20];
    tree3_22 <== vals3[21];
    tree3_23 <== vals3[22];
    tree3_24 <== vals3[23];
    tree3_25 <== vals3[24];
    tree3_26 <== vals3[25];
    tree3_27 <== vals3[26];
    tree3_28 <== vals3[27];
    tree3_29 <== vals3[28];
    tree3_30 <== vals3[29];
    tree3_31 <== vals3[30];
    tree3_32 <== vals3[31];
    tree3_33 <== vals3[32];
    tree3_34 <== vals3[33];
    tree3_35 <== vals3[34];
    tree3_36 <== vals3[35];
    tree3_37 <== vals3[36];
    tree3_38 <== vals3[37];
    tree3_39 <== vals3[38];
    tree3_40 <== vals3[39];
    tree3_41 <== vals3[40];
    tree3_42 <== vals3[41];
    tree3_43 <== vals3[42];
    tree3_44 <== vals3[43];
    tree3_45 <== vals3[44];
    tree3_46 <== vals3[45];
    tree3_47 <== vals3[46];
    tree3_48 <== vals3[47];
    tree3_49 <== vals3[48];
    tree3_50 <== vals3[49];
    tree3_51 <== vals3[50];
    tree3_52[0] <== vals3[54];
    tree3_52[1] <== vals3[55];
    tree3_52[2] <== vals3[56];
    tree3_53[0] <== vals3[57];
    tree3_53[1] <== vals3[58];
    tree3_53[2] <== vals3[59];
    tree3_54[0] <== vals3[60];
    tree3_54[1] <== vals3[61];
    tree3_54[2] <== vals3[62];
    tree3_55[0] <== vals3[63];
    tree3_55[1] <== vals3[64];
    tree3_55[2] <== vals3[65];
    tree3_56[0] <== vals3[66];
    tree3_56[1] <== vals3[67];
    tree3_56[2] <== vals3[68];
    tree3_57[0] <== vals3[69];
    tree3_57[1] <== vals3[70];
    tree3_57[2] <== vals3[71];
    tree3_58[0] <== vals3[72];
    tree3_58[1] <== vals3[73];
    tree3_58[2] <== vals3[74];
    tree3_59[0] <== vals3[75];
    tree3_59[1] <== vals3[76];
    tree3_59[2] <== vals3[77];
    tree3_60[0] <== vals3[78];
    tree3_60[1] <== vals3[79];
    tree3_60[2] <== vals3[80];
    tree3_61[0] <== vals3[81];
    tree3_61[1] <== vals3[82];
    tree3_61[2] <== vals3[83];
    tree4_0[0] <== vals4[0];
    tree4_0[1] <== vals4[1];
    tree4_0[2] <== vals4[2];
    tree4_1[0] <== vals4[3];
    tree4_1[1] <== vals4[4];
    tree4_1[2] <== vals4[5];
}
template StarkVerifier() {
    signal input publics[18];
    signal input root1;
    signal input root2;
    signal input root3;
    signal input root4;

    signal input rootC;

    signal input evals[121][3];
    signal input s0_vals1[8][12];
    
    signal input s0_vals3[8][84];
        
    signal input s0_vals4[8][6];
    signal input s0_valsC[8][32];
    signal input s0_siblings1[8][5][16];

    signal input s0_siblings3[8][5][16];
        
    signal input s0_siblings4[8][5][16];
    signal input s0_siblingsC[8][5][16];
        
    signal input s1_root;
        
    signal input s2_root;
        
    signal input s3_root;
        
    signal input s1_vals[8][192];
    signal input s1_siblings[8][3][16];
        
    signal input s2_vals[8][48];
    signal input s2_siblings[8][2][16];
        
    signal input s3_vals[8][24];
    signal input s3_siblings[8][1][16];
        
    signal input finalPol[16][3];
    
    signal enable;
    enable <== 1;
    
    signal challenges[8][3];
    
    signal s0_specialX[3];
    
    signal s1_specialX[3];
    
    signal s2_specialX[3];
    
    signal s3_specialX[3];
    
    signal ys[8][17];

    var p = 0xFFFFFFFF00000001;
        component tcHahs_0 = PoseidonEx(16,17);
    tcHahs_0.inputs[0] <== publics[0];
    tcHahs_0.inputs[1] <== publics[1];
    tcHahs_0.inputs[2] <== publics[2];
    tcHahs_0.inputs[3] <== publics[3];
    tcHahs_0.inputs[4] <== publics[4];
    tcHahs_0.inputs[5] <== publics[5];
    tcHahs_0.inputs[6] <== publics[6];
    tcHahs_0.inputs[7] <== publics[7];
    tcHahs_0.inputs[8] <== publics[8];
    tcHahs_0.inputs[9] <== publics[9];
    tcHahs_0.inputs[10] <== publics[10];
    tcHahs_0.inputs[11] <== publics[11];
    tcHahs_0.inputs[12] <== publics[12];
    tcHahs_0.inputs[13] <== publics[13];
    tcHahs_0.inputs[14] <== publics[14];
    tcHahs_0.inputs[15] <== publics[15];
    tcHahs_0.initialState <== 0;
    component tcHahs_1 = PoseidonEx(16,17);
    tcHahs_1.inputs[0] <== publics[16];
    tcHahs_1.inputs[1] <== publics[17];
    tcHahs_1.inputs[2] <== root1;
    tcHahs_1.inputs[3] <== 0;
    tcHahs_1.inputs[4] <== 0;
    tcHahs_1.inputs[5] <== 0;
    tcHahs_1.inputs[6] <== 0;
    tcHahs_1.inputs[7] <== 0;
    tcHahs_1.inputs[8] <== 0;
    tcHahs_1.inputs[9] <== 0;
    tcHahs_1.inputs[10] <== 0;
    tcHahs_1.inputs[11] <== 0;
    tcHahs_1.inputs[12] <== 0;
    tcHahs_1.inputs[13] <== 0;
    tcHahs_1.inputs[14] <== 0;
    tcHahs_1.inputs[15] <== 0;
    tcHahs_1.initialState <== tcHahs_0.out[0];
    component bn1togl3_0 = BN1toGL3();
    bn1togl3_0.in <== tcHahs_1.out[0];
    challenges[0][0] <== bn1togl3_0.out[0];
    challenges[0][1] <== bn1togl3_0.out[1];
    challenges[0][2] <== bn1togl3_0.out[2];
    component bn1togl3_1 = BN1toGL3();
    bn1togl3_1.in <== tcHahs_1.out[1];
    challenges[1][0] <== bn1togl3_1.out[0];
    challenges[1][1] <== bn1togl3_1.out[1];
    challenges[1][2] <== bn1togl3_1.out[2];
    component tcHahs_2 = PoseidonEx(16,17);
    tcHahs_2.inputs[0] <== root2;
    tcHahs_2.inputs[1] <== 0;
    tcHahs_2.inputs[2] <== 0;
    tcHahs_2.inputs[3] <== 0;
    tcHahs_2.inputs[4] <== 0;
    tcHahs_2.inputs[5] <== 0;
    tcHahs_2.inputs[6] <== 0;
    tcHahs_2.inputs[7] <== 0;
    tcHahs_2.inputs[8] <== 0;
    tcHahs_2.inputs[9] <== 0;
    tcHahs_2.inputs[10] <== 0;
    tcHahs_2.inputs[11] <== 0;
    tcHahs_2.inputs[12] <== 0;
    tcHahs_2.inputs[13] <== 0;
    tcHahs_2.inputs[14] <== 0;
    tcHahs_2.inputs[15] <== 0;
    tcHahs_2.initialState <== tcHahs_1.out[0];
    component bn1togl3_2 = BN1toGL3();
    bn1togl3_2.in <== tcHahs_2.out[0];
    challenges[2][0] <== bn1togl3_2.out[0];
    challenges[2][1] <== bn1togl3_2.out[1];
    challenges[2][2] <== bn1togl3_2.out[2];
    component bn1togl3_3 = BN1toGL3();
    bn1togl3_3.in <== tcHahs_2.out[1];
    challenges[3][0] <== bn1togl3_3.out[0];
    challenges[3][1] <== bn1togl3_3.out[1];
    challenges[3][2] <== bn1togl3_3.out[2];
    component tcHahs_3 = PoseidonEx(16,17);
    tcHahs_3.inputs[0] <== root3;
    tcHahs_3.inputs[1] <== 0;
    tcHahs_3.inputs[2] <== 0;
    tcHahs_3.inputs[3] <== 0;
    tcHahs_3.inputs[4] <== 0;
    tcHahs_3.inputs[5] <== 0;
    tcHahs_3.inputs[6] <== 0;
    tcHahs_3.inputs[7] <== 0;
    tcHahs_3.inputs[8] <== 0;
    tcHahs_3.inputs[9] <== 0;
    tcHahs_3.inputs[10] <== 0;
    tcHahs_3.inputs[11] <== 0;
    tcHahs_3.inputs[12] <== 0;
    tcHahs_3.inputs[13] <== 0;
    tcHahs_3.inputs[14] <== 0;
    tcHahs_3.inputs[15] <== 0;
    tcHahs_3.initialState <== tcHahs_2.out[0];
    component bn1togl3_4 = BN1toGL3();
    bn1togl3_4.in <== tcHahs_3.out[0];
    challenges[4][0] <== bn1togl3_4.out[0];
    challenges[4][1] <== bn1togl3_4.out[1];
    challenges[4][2] <== bn1togl3_4.out[2];
    component tcHahs_4 = PoseidonEx(16,17);
    tcHahs_4.inputs[0] <== root4;
    tcHahs_4.inputs[1] <== 0;
    tcHahs_4.inputs[2] <== 0;
    tcHahs_4.inputs[3] <== 0;
    tcHahs_4.inputs[4] <== 0;
    tcHahs_4.inputs[5] <== 0;
    tcHahs_4.inputs[6] <== 0;
    tcHahs_4.inputs[7] <== 0;
    tcHahs_4.inputs[8] <== 0;
    tcHahs_4.inputs[9] <== 0;
    tcHahs_4.inputs[10] <== 0;
    tcHahs_4.inputs[11] <== 0;
    tcHahs_4.inputs[12] <== 0;
    tcHahs_4.inputs[13] <== 0;
    tcHahs_4.inputs[14] <== 0;
    tcHahs_4.inputs[15] <== 0;
    tcHahs_4.initialState <== tcHahs_3.out[0];
    component bn1togl3_5 = BN1toGL3();
    bn1togl3_5.in <== tcHahs_4.out[0];
    challenges[7][0] <== bn1togl3_5.out[0];
    challenges[7][1] <== bn1togl3_5.out[1];
    challenges[7][2] <== bn1togl3_5.out[2];
    component tcHahs_5 = PoseidonEx(16,17);
    tcHahs_5.inputs[0] <== evals[0][0];
    tcHahs_5.inputs[1] <== evals[0][1];
    tcHahs_5.inputs[2] <== evals[0][2];
    tcHahs_5.inputs[3] <== evals[1][0];
    tcHahs_5.inputs[4] <== evals[1][1];
    tcHahs_5.inputs[5] <== evals[1][2];
    tcHahs_5.inputs[6] <== evals[2][0];
    tcHahs_5.inputs[7] <== evals[2][1];
    tcHahs_5.inputs[8] <== evals[2][2];
    tcHahs_5.inputs[9] <== evals[3][0];
    tcHahs_5.inputs[10] <== evals[3][1];
    tcHahs_5.inputs[11] <== evals[3][2];
    tcHahs_5.inputs[12] <== evals[4][0];
    tcHahs_5.inputs[13] <== evals[4][1];
    tcHahs_5.inputs[14] <== evals[4][2];
    tcHahs_5.inputs[15] <== evals[5][0];
    tcHahs_5.initialState <== tcHahs_4.out[0];
    component tcHahs_6 = PoseidonEx(16,17);
    tcHahs_6.inputs[0] <== evals[5][1];
    tcHahs_6.inputs[1] <== evals[5][2];
    tcHahs_6.inputs[2] <== evals[6][0];
    tcHahs_6.inputs[3] <== evals[6][1];
    tcHahs_6.inputs[4] <== evals[6][2];
    tcHahs_6.inputs[5] <== evals[7][0];
    tcHahs_6.inputs[6] <== evals[7][1];
    tcHahs_6.inputs[7] <== evals[7][2];
    tcHahs_6.inputs[8] <== evals[8][0];
    tcHahs_6.inputs[9] <== evals[8][1];
    tcHahs_6.inputs[10] <== evals[8][2];
    tcHahs_6.inputs[11] <== evals[9][0];
    tcHahs_6.inputs[12] <== evals[9][1];
    tcHahs_6.inputs[13] <== evals[9][2];
    tcHahs_6.inputs[14] <== evals[10][0];
    tcHahs_6.inputs[15] <== evals[10][1];
    tcHahs_6.initialState <== tcHahs_5.out[0];
    component tcHahs_7 = PoseidonEx(16,17);
    tcHahs_7.inputs[0] <== evals[10][2];
    tcHahs_7.inputs[1] <== evals[11][0];
    tcHahs_7.inputs[2] <== evals[11][1];
    tcHahs_7.inputs[3] <== evals[11][2];
    tcHahs_7.inputs[4] <== evals[12][0];
    tcHahs_7.inputs[5] <== evals[12][1];
    tcHahs_7.inputs[6] <== evals[12][2];
    tcHahs_7.inputs[7] <== evals[13][0];
    tcHahs_7.inputs[8] <== evals[13][1];
    tcHahs_7.inputs[9] <== evals[13][2];
    tcHahs_7.inputs[10] <== evals[14][0];
    tcHahs_7.inputs[11] <== evals[14][1];
    tcHahs_7.inputs[12] <== evals[14][2];
    tcHahs_7.inputs[13] <== evals[15][0];
    tcHahs_7.inputs[14] <== evals[15][1];
    tcHahs_7.inputs[15] <== evals[15][2];
    tcHahs_7.initialState <== tcHahs_6.out[0];
    component tcHahs_8 = PoseidonEx(16,17);
    tcHahs_8.inputs[0] <== evals[16][0];
    tcHahs_8.inputs[1] <== evals[16][1];
    tcHahs_8.inputs[2] <== evals[16][2];
    tcHahs_8.inputs[3] <== evals[17][0];
    tcHahs_8.inputs[4] <== evals[17][1];
    tcHahs_8.inputs[5] <== evals[17][2];
    tcHahs_8.inputs[6] <== evals[18][0];
    tcHahs_8.inputs[7] <== evals[18][1];
    tcHahs_8.inputs[8] <== evals[18][2];
    tcHahs_8.inputs[9] <== evals[19][0];
    tcHahs_8.inputs[10] <== evals[19][1];
    tcHahs_8.inputs[11] <== evals[19][2];
    tcHahs_8.inputs[12] <== evals[20][0];
    tcHahs_8.inputs[13] <== evals[20][1];
    tcHahs_8.inputs[14] <== evals[20][2];
    tcHahs_8.inputs[15] <== evals[21][0];
    tcHahs_8.initialState <== tcHahs_7.out[0];
    component tcHahs_9 = PoseidonEx(16,17);
    tcHahs_9.inputs[0] <== evals[21][1];
    tcHahs_9.inputs[1] <== evals[21][2];
    tcHahs_9.inputs[2] <== evals[22][0];
    tcHahs_9.inputs[3] <== evals[22][1];
    tcHahs_9.inputs[4] <== evals[22][2];
    tcHahs_9.inputs[5] <== evals[23][0];
    tcHahs_9.inputs[6] <== evals[23][1];
    tcHahs_9.inputs[7] <== evals[23][2];
    tcHahs_9.inputs[8] <== evals[24][0];
    tcHahs_9.inputs[9] <== evals[24][1];
    tcHahs_9.inputs[10] <== evals[24][2];
    tcHahs_9.inputs[11] <== evals[25][0];
    tcHahs_9.inputs[12] <== evals[25][1];
    tcHahs_9.inputs[13] <== evals[25][2];
    tcHahs_9.inputs[14] <== evals[26][0];
    tcHahs_9.inputs[15] <== evals[26][1];
    tcHahs_9.initialState <== tcHahs_8.out[0];
    component tcHahs_10 = PoseidonEx(16,17);
    tcHahs_10.inputs[0] <== evals[26][2];
    tcHahs_10.inputs[1] <== evals[27][0];
    tcHahs_10.inputs[2] <== evals[27][1];
    tcHahs_10.inputs[3] <== evals[27][2];
    tcHahs_10.inputs[4] <== evals[28][0];
    tcHahs_10.inputs[5] <== evals[28][1];
    tcHahs_10.inputs[6] <== evals[28][2];
    tcHahs_10.inputs[7] <== evals[29][0];
    tcHahs_10.inputs[8] <== evals[29][1];
    tcHahs_10.inputs[9] <== evals[29][2];
    tcHahs_10.inputs[10] <== evals[30][0];
    tcHahs_10.inputs[11] <== evals[30][1];
    tcHahs_10.inputs[12] <== evals[30][2];
    tcHahs_10.inputs[13] <== evals[31][0];
    tcHahs_10.inputs[14] <== evals[31][1];
    tcHahs_10.inputs[15] <== evals[31][2];
    tcHahs_10.initialState <== tcHahs_9.out[0];
    component tcHahs_11 = PoseidonEx(16,17);
    tcHahs_11.inputs[0] <== evals[32][0];
    tcHahs_11.inputs[1] <== evals[32][1];
    tcHahs_11.inputs[2] <== evals[32][2];
    tcHahs_11.inputs[3] <== evals[33][0];
    tcHahs_11.inputs[4] <== evals[33][1];
    tcHahs_11.inputs[5] <== evals[33][2];
    tcHahs_11.inputs[6] <== evals[34][0];
    tcHahs_11.inputs[7] <== evals[34][1];
    tcHahs_11.inputs[8] <== evals[34][2];
    tcHahs_11.inputs[9] <== evals[35][0];
    tcHahs_11.inputs[10] <== evals[35][1];
    tcHahs_11.inputs[11] <== evals[35][2];
    tcHahs_11.inputs[12] <== evals[36][0];
    tcHahs_11.inputs[13] <== evals[36][1];
    tcHahs_11.inputs[14] <== evals[36][2];
    tcHahs_11.inputs[15] <== evals[37][0];
    tcHahs_11.initialState <== tcHahs_10.out[0];
    component tcHahs_12 = PoseidonEx(16,17);
    tcHahs_12.inputs[0] <== evals[37][1];
    tcHahs_12.inputs[1] <== evals[37][2];
    tcHahs_12.inputs[2] <== evals[38][0];
    tcHahs_12.inputs[3] <== evals[38][1];
    tcHahs_12.inputs[4] <== evals[38][2];
    tcHahs_12.inputs[5] <== evals[39][0];
    tcHahs_12.inputs[6] <== evals[39][1];
    tcHahs_12.inputs[7] <== evals[39][2];
    tcHahs_12.inputs[8] <== evals[40][0];
    tcHahs_12.inputs[9] <== evals[40][1];
    tcHahs_12.inputs[10] <== evals[40][2];
    tcHahs_12.inputs[11] <== evals[41][0];
    tcHahs_12.inputs[12] <== evals[41][1];
    tcHahs_12.inputs[13] <== evals[41][2];
    tcHahs_12.inputs[14] <== evals[42][0];
    tcHahs_12.inputs[15] <== evals[42][1];
    tcHahs_12.initialState <== tcHahs_11.out[0];
    component tcHahs_13 = PoseidonEx(16,17);
    tcHahs_13.inputs[0] <== evals[42][2];
    tcHahs_13.inputs[1] <== evals[43][0];
    tcHahs_13.inputs[2] <== evals[43][1];
    tcHahs_13.inputs[3] <== evals[43][2];
    tcHahs_13.inputs[4] <== evals[44][0];
    tcHahs_13.inputs[5] <== evals[44][1];
    tcHahs_13.inputs[6] <== evals[44][2];
    tcHahs_13.inputs[7] <== evals[45][0];
    tcHahs_13.inputs[8] <== evals[45][1];
    tcHahs_13.inputs[9] <== evals[45][2];
    tcHahs_13.inputs[10] <== evals[46][0];
    tcHahs_13.inputs[11] <== evals[46][1];
    tcHahs_13.inputs[12] <== evals[46][2];
    tcHahs_13.inputs[13] <== evals[47][0];
    tcHahs_13.inputs[14] <== evals[47][1];
    tcHahs_13.inputs[15] <== evals[47][2];
    tcHahs_13.initialState <== tcHahs_12.out[0];
    component tcHahs_14 = PoseidonEx(16,17);
    tcHahs_14.inputs[0] <== evals[48][0];
    tcHahs_14.inputs[1] <== evals[48][1];
    tcHahs_14.inputs[2] <== evals[48][2];
    tcHahs_14.inputs[3] <== evals[49][0];
    tcHahs_14.inputs[4] <== evals[49][1];
    tcHahs_14.inputs[5] <== evals[49][2];
    tcHahs_14.inputs[6] <== evals[50][0];
    tcHahs_14.inputs[7] <== evals[50][1];
    tcHahs_14.inputs[8] <== evals[50][2];
    tcHahs_14.inputs[9] <== evals[51][0];
    tcHahs_14.inputs[10] <== evals[51][1];
    tcHahs_14.inputs[11] <== evals[51][2];
    tcHahs_14.inputs[12] <== evals[52][0];
    tcHahs_14.inputs[13] <== evals[52][1];
    tcHahs_14.inputs[14] <== evals[52][2];
    tcHahs_14.inputs[15] <== evals[53][0];
    tcHahs_14.initialState <== tcHahs_13.out[0];
    component tcHahs_15 = PoseidonEx(16,17);
    tcHahs_15.inputs[0] <== evals[53][1];
    tcHahs_15.inputs[1] <== evals[53][2];
    tcHahs_15.inputs[2] <== evals[54][0];
    tcHahs_15.inputs[3] <== evals[54][1];
    tcHahs_15.inputs[4] <== evals[54][2];
    tcHahs_15.inputs[5] <== evals[55][0];
    tcHahs_15.inputs[6] <== evals[55][1];
    tcHahs_15.inputs[7] <== evals[55][2];
    tcHahs_15.inputs[8] <== evals[56][0];
    tcHahs_15.inputs[9] <== evals[56][1];
    tcHahs_15.inputs[10] <== evals[56][2];
    tcHahs_15.inputs[11] <== evals[57][0];
    tcHahs_15.inputs[12] <== evals[57][1];
    tcHahs_15.inputs[13] <== evals[57][2];
    tcHahs_15.inputs[14] <== evals[58][0];
    tcHahs_15.inputs[15] <== evals[58][1];
    tcHahs_15.initialState <== tcHahs_14.out[0];
    component tcHahs_16 = PoseidonEx(16,17);
    tcHahs_16.inputs[0] <== evals[58][2];
    tcHahs_16.inputs[1] <== evals[59][0];
    tcHahs_16.inputs[2] <== evals[59][1];
    tcHahs_16.inputs[3] <== evals[59][2];
    tcHahs_16.inputs[4] <== evals[60][0];
    tcHahs_16.inputs[5] <== evals[60][1];
    tcHahs_16.inputs[6] <== evals[60][2];
    tcHahs_16.inputs[7] <== evals[61][0];
    tcHahs_16.inputs[8] <== evals[61][1];
    tcHahs_16.inputs[9] <== evals[61][2];
    tcHahs_16.inputs[10] <== evals[62][0];
    tcHahs_16.inputs[11] <== evals[62][1];
    tcHahs_16.inputs[12] <== evals[62][2];
    tcHahs_16.inputs[13] <== evals[63][0];
    tcHahs_16.inputs[14] <== evals[63][1];
    tcHahs_16.inputs[15] <== evals[63][2];
    tcHahs_16.initialState <== tcHahs_15.out[0];
    component tcHahs_17 = PoseidonEx(16,17);
    tcHahs_17.inputs[0] <== evals[64][0];
    tcHahs_17.inputs[1] <== evals[64][1];
    tcHahs_17.inputs[2] <== evals[64][2];
    tcHahs_17.inputs[3] <== evals[65][0];
    tcHahs_17.inputs[4] <== evals[65][1];
    tcHahs_17.inputs[5] <== evals[65][2];
    tcHahs_17.inputs[6] <== evals[66][0];
    tcHahs_17.inputs[7] <== evals[66][1];
    tcHahs_17.inputs[8] <== evals[66][2];
    tcHahs_17.inputs[9] <== evals[67][0];
    tcHahs_17.inputs[10] <== evals[67][1];
    tcHahs_17.inputs[11] <== evals[67][2];
    tcHahs_17.inputs[12] <== evals[68][0];
    tcHahs_17.inputs[13] <== evals[68][1];
    tcHahs_17.inputs[14] <== evals[68][2];
    tcHahs_17.inputs[15] <== evals[69][0];
    tcHahs_17.initialState <== tcHahs_16.out[0];
    component tcHahs_18 = PoseidonEx(16,17);
    tcHahs_18.inputs[0] <== evals[69][1];
    tcHahs_18.inputs[1] <== evals[69][2];
    tcHahs_18.inputs[2] <== evals[70][0];
    tcHahs_18.inputs[3] <== evals[70][1];
    tcHahs_18.inputs[4] <== evals[70][2];
    tcHahs_18.inputs[5] <== evals[71][0];
    tcHahs_18.inputs[6] <== evals[71][1];
    tcHahs_18.inputs[7] <== evals[71][2];
    tcHahs_18.inputs[8] <== evals[72][0];
    tcHahs_18.inputs[9] <== evals[72][1];
    tcHahs_18.inputs[10] <== evals[72][2];
    tcHahs_18.inputs[11] <== evals[73][0];
    tcHahs_18.inputs[12] <== evals[73][1];
    tcHahs_18.inputs[13] <== evals[73][2];
    tcHahs_18.inputs[14] <== evals[74][0];
    tcHahs_18.inputs[15] <== evals[74][1];
    tcHahs_18.initialState <== tcHahs_17.out[0];
    component tcHahs_19 = PoseidonEx(16,17);
    tcHahs_19.inputs[0] <== evals[74][2];
    tcHahs_19.inputs[1] <== evals[75][0];
    tcHahs_19.inputs[2] <== evals[75][1];
    tcHahs_19.inputs[3] <== evals[75][2];
    tcHahs_19.inputs[4] <== evals[76][0];
    tcHahs_19.inputs[5] <== evals[76][1];
    tcHahs_19.inputs[6] <== evals[76][2];
    tcHahs_19.inputs[7] <== evals[77][0];
    tcHahs_19.inputs[8] <== evals[77][1];
    tcHahs_19.inputs[9] <== evals[77][2];
    tcHahs_19.inputs[10] <== evals[78][0];
    tcHahs_19.inputs[11] <== evals[78][1];
    tcHahs_19.inputs[12] <== evals[78][2];
    tcHahs_19.inputs[13] <== evals[79][0];
    tcHahs_19.inputs[14] <== evals[79][1];
    tcHahs_19.inputs[15] <== evals[79][2];
    tcHahs_19.initialState <== tcHahs_18.out[0];
    component tcHahs_20 = PoseidonEx(16,17);
    tcHahs_20.inputs[0] <== evals[80][0];
    tcHahs_20.inputs[1] <== evals[80][1];
    tcHahs_20.inputs[2] <== evals[80][2];
    tcHahs_20.inputs[3] <== evals[81][0];
    tcHahs_20.inputs[4] <== evals[81][1];
    tcHahs_20.inputs[5] <== evals[81][2];
    tcHahs_20.inputs[6] <== evals[82][0];
    tcHahs_20.inputs[7] <== evals[82][1];
    tcHahs_20.inputs[8] <== evals[82][2];
    tcHahs_20.inputs[9] <== evals[83][0];
    tcHahs_20.inputs[10] <== evals[83][1];
    tcHahs_20.inputs[11] <== evals[83][2];
    tcHahs_20.inputs[12] <== evals[84][0];
    tcHahs_20.inputs[13] <== evals[84][1];
    tcHahs_20.inputs[14] <== evals[84][2];
    tcHahs_20.inputs[15] <== evals[85][0];
    tcHahs_20.initialState <== tcHahs_19.out[0];
    component tcHahs_21 = PoseidonEx(16,17);
    tcHahs_21.inputs[0] <== evals[85][1];
    tcHahs_21.inputs[1] <== evals[85][2];
    tcHahs_21.inputs[2] <== evals[86][0];
    tcHahs_21.inputs[3] <== evals[86][1];
    tcHahs_21.inputs[4] <== evals[86][2];
    tcHahs_21.inputs[5] <== evals[87][0];
    tcHahs_21.inputs[6] <== evals[87][1];
    tcHahs_21.inputs[7] <== evals[87][2];
    tcHahs_21.inputs[8] <== evals[88][0];
    tcHahs_21.inputs[9] <== evals[88][1];
    tcHahs_21.inputs[10] <== evals[88][2];
    tcHahs_21.inputs[11] <== evals[89][0];
    tcHahs_21.inputs[12] <== evals[89][1];
    tcHahs_21.inputs[13] <== evals[89][2];
    tcHahs_21.inputs[14] <== evals[90][0];
    tcHahs_21.inputs[15] <== evals[90][1];
    tcHahs_21.initialState <== tcHahs_20.out[0];
    component tcHahs_22 = PoseidonEx(16,17);
    tcHahs_22.inputs[0] <== evals[90][2];
    tcHahs_22.inputs[1] <== evals[91][0];
    tcHahs_22.inputs[2] <== evals[91][1];
    tcHahs_22.inputs[3] <== evals[91][2];
    tcHahs_22.inputs[4] <== evals[92][0];
    tcHahs_22.inputs[5] <== evals[92][1];
    tcHahs_22.inputs[6] <== evals[92][2];
    tcHahs_22.inputs[7] <== evals[93][0];
    tcHahs_22.inputs[8] <== evals[93][1];
    tcHahs_22.inputs[9] <== evals[93][2];
    tcHahs_22.inputs[10] <== evals[94][0];
    tcHahs_22.inputs[11] <== evals[94][1];
    tcHahs_22.inputs[12] <== evals[94][2];
    tcHahs_22.inputs[13] <== evals[95][0];
    tcHahs_22.inputs[14] <== evals[95][1];
    tcHahs_22.inputs[15] <== evals[95][2];
    tcHahs_22.initialState <== tcHahs_21.out[0];
    component tcHahs_23 = PoseidonEx(16,17);
    tcHahs_23.inputs[0] <== evals[96][0];
    tcHahs_23.inputs[1] <== evals[96][1];
    tcHahs_23.inputs[2] <== evals[96][2];
    tcHahs_23.inputs[3] <== evals[97][0];
    tcHahs_23.inputs[4] <== evals[97][1];
    tcHahs_23.inputs[5] <== evals[97][2];
    tcHahs_23.inputs[6] <== evals[98][0];
    tcHahs_23.inputs[7] <== evals[98][1];
    tcHahs_23.inputs[8] <== evals[98][2];
    tcHahs_23.inputs[9] <== evals[99][0];
    tcHahs_23.inputs[10] <== evals[99][1];
    tcHahs_23.inputs[11] <== evals[99][2];
    tcHahs_23.inputs[12] <== evals[100][0];
    tcHahs_23.inputs[13] <== evals[100][1];
    tcHahs_23.inputs[14] <== evals[100][2];
    tcHahs_23.inputs[15] <== evals[101][0];
    tcHahs_23.initialState <== tcHahs_22.out[0];
    component tcHahs_24 = PoseidonEx(16,17);
    tcHahs_24.inputs[0] <== evals[101][1];
    tcHahs_24.inputs[1] <== evals[101][2];
    tcHahs_24.inputs[2] <== evals[102][0];
    tcHahs_24.inputs[3] <== evals[102][1];
    tcHahs_24.inputs[4] <== evals[102][2];
    tcHahs_24.inputs[5] <== evals[103][0];
    tcHahs_24.inputs[6] <== evals[103][1];
    tcHahs_24.inputs[7] <== evals[103][2];
    tcHahs_24.inputs[8] <== evals[104][0];
    tcHahs_24.inputs[9] <== evals[104][1];
    tcHahs_24.inputs[10] <== evals[104][2];
    tcHahs_24.inputs[11] <== evals[105][0];
    tcHahs_24.inputs[12] <== evals[105][1];
    tcHahs_24.inputs[13] <== evals[105][2];
    tcHahs_24.inputs[14] <== evals[106][0];
    tcHahs_24.inputs[15] <== evals[106][1];
    tcHahs_24.initialState <== tcHahs_23.out[0];
    component tcHahs_25 = PoseidonEx(16,17);
    tcHahs_25.inputs[0] <== evals[106][2];
    tcHahs_25.inputs[1] <== evals[107][0];
    tcHahs_25.inputs[2] <== evals[107][1];
    tcHahs_25.inputs[3] <== evals[107][2];
    tcHahs_25.inputs[4] <== evals[108][0];
    tcHahs_25.inputs[5] <== evals[108][1];
    tcHahs_25.inputs[6] <== evals[108][2];
    tcHahs_25.inputs[7] <== evals[109][0];
    tcHahs_25.inputs[8] <== evals[109][1];
    tcHahs_25.inputs[9] <== evals[109][2];
    tcHahs_25.inputs[10] <== evals[110][0];
    tcHahs_25.inputs[11] <== evals[110][1];
    tcHahs_25.inputs[12] <== evals[110][2];
    tcHahs_25.inputs[13] <== evals[111][0];
    tcHahs_25.inputs[14] <== evals[111][1];
    tcHahs_25.inputs[15] <== evals[111][2];
    tcHahs_25.initialState <== tcHahs_24.out[0];
    component tcHahs_26 = PoseidonEx(16,17);
    tcHahs_26.inputs[0] <== evals[112][0];
    tcHahs_26.inputs[1] <== evals[112][1];
    tcHahs_26.inputs[2] <== evals[112][2];
    tcHahs_26.inputs[3] <== evals[113][0];
    tcHahs_26.inputs[4] <== evals[113][1];
    tcHahs_26.inputs[5] <== evals[113][2];
    tcHahs_26.inputs[6] <== evals[114][0];
    tcHahs_26.inputs[7] <== evals[114][1];
    tcHahs_26.inputs[8] <== evals[114][2];
    tcHahs_26.inputs[9] <== evals[115][0];
    tcHahs_26.inputs[10] <== evals[115][1];
    tcHahs_26.inputs[11] <== evals[115][2];
    tcHahs_26.inputs[12] <== evals[116][0];
    tcHahs_26.inputs[13] <== evals[116][1];
    tcHahs_26.inputs[14] <== evals[116][2];
    tcHahs_26.inputs[15] <== evals[117][0];
    tcHahs_26.initialState <== tcHahs_25.out[0];
    component tcHahs_27 = PoseidonEx(16,17);
    tcHahs_27.inputs[0] <== evals[117][1];
    tcHahs_27.inputs[1] <== evals[117][2];
    tcHahs_27.inputs[2] <== evals[118][0];
    tcHahs_27.inputs[3] <== evals[118][1];
    tcHahs_27.inputs[4] <== evals[118][2];
    tcHahs_27.inputs[5] <== evals[119][0];
    tcHahs_27.inputs[6] <== evals[119][1];
    tcHahs_27.inputs[7] <== evals[119][2];
    tcHahs_27.inputs[8] <== evals[120][0];
    tcHahs_27.inputs[9] <== evals[120][1];
    tcHahs_27.inputs[10] <== evals[120][2];
    tcHahs_27.inputs[11] <== 0;
    tcHahs_27.inputs[12] <== 0;
    tcHahs_27.inputs[13] <== 0;
    tcHahs_27.inputs[14] <== 0;
    tcHahs_27.inputs[15] <== 0;
    tcHahs_27.initialState <== tcHahs_26.out[0];
    component bn1togl3_6 = BN1toGL3();
    bn1togl3_6.in <== tcHahs_27.out[0];
    challenges[5][0] <== bn1togl3_6.out[0];
    challenges[5][1] <== bn1togl3_6.out[1];
    challenges[5][2] <== bn1togl3_6.out[2];
    component bn1togl3_7 = BN1toGL3();
    bn1togl3_7.in <== tcHahs_27.out[1];
    challenges[6][0] <== bn1togl3_7.out[0];
    challenges[6][1] <== bn1togl3_7.out[1];
    challenges[6][2] <== bn1togl3_7.out[2];
    component bn1togl3_8 = BN1toGL3();
    bn1togl3_8.in <== tcHahs_27.out[2];
    s0_specialX[0] <== bn1togl3_8.out[0];
    s0_specialX[1] <== bn1togl3_8.out[1];
    s0_specialX[2] <== bn1togl3_8.out[2];
    component tcHahs_28 = PoseidonEx(16,17);
    tcHahs_28.inputs[0] <== s1_root;
    tcHahs_28.inputs[1] <== 0;
    tcHahs_28.inputs[2] <== 0;
    tcHahs_28.inputs[3] <== 0;
    tcHahs_28.inputs[4] <== 0;
    tcHahs_28.inputs[5] <== 0;
    tcHahs_28.inputs[6] <== 0;
    tcHahs_28.inputs[7] <== 0;
    tcHahs_28.inputs[8] <== 0;
    tcHahs_28.inputs[9] <== 0;
    tcHahs_28.inputs[10] <== 0;
    tcHahs_28.inputs[11] <== 0;
    tcHahs_28.inputs[12] <== 0;
    tcHahs_28.inputs[13] <== 0;
    tcHahs_28.inputs[14] <== 0;
    tcHahs_28.inputs[15] <== 0;
    tcHahs_28.initialState <== tcHahs_27.out[0];
    component bn1togl3_9 = BN1toGL3();
    bn1togl3_9.in <== tcHahs_28.out[0];
    s1_specialX[0] <== bn1togl3_9.out[0];
    s1_specialX[1] <== bn1togl3_9.out[1];
    s1_specialX[2] <== bn1togl3_9.out[2];
    component tcHahs_29 = PoseidonEx(16,17);
    tcHahs_29.inputs[0] <== s2_root;
    tcHahs_29.inputs[1] <== 0;
    tcHahs_29.inputs[2] <== 0;
    tcHahs_29.inputs[3] <== 0;
    tcHahs_29.inputs[4] <== 0;
    tcHahs_29.inputs[5] <== 0;
    tcHahs_29.inputs[6] <== 0;
    tcHahs_29.inputs[7] <== 0;
    tcHahs_29.inputs[8] <== 0;
    tcHahs_29.inputs[9] <== 0;
    tcHahs_29.inputs[10] <== 0;
    tcHahs_29.inputs[11] <== 0;
    tcHahs_29.inputs[12] <== 0;
    tcHahs_29.inputs[13] <== 0;
    tcHahs_29.inputs[14] <== 0;
    tcHahs_29.inputs[15] <== 0;
    tcHahs_29.initialState <== tcHahs_28.out[0];
    component bn1togl3_10 = BN1toGL3();
    bn1togl3_10.in <== tcHahs_29.out[0];
    s2_specialX[0] <== bn1togl3_10.out[0];
    s2_specialX[1] <== bn1togl3_10.out[1];
    s2_specialX[2] <== bn1togl3_10.out[2];
    component tcHahs_30 = PoseidonEx(16,17);
    tcHahs_30.inputs[0] <== s3_root;
    tcHahs_30.inputs[1] <== 0;
    tcHahs_30.inputs[2] <== 0;
    tcHahs_30.inputs[3] <== 0;
    tcHahs_30.inputs[4] <== 0;
    tcHahs_30.inputs[5] <== 0;
    tcHahs_30.inputs[6] <== 0;
    tcHahs_30.inputs[7] <== 0;
    tcHahs_30.inputs[8] <== 0;
    tcHahs_30.inputs[9] <== 0;
    tcHahs_30.inputs[10] <== 0;
    tcHahs_30.inputs[11] <== 0;
    tcHahs_30.inputs[12] <== 0;
    tcHahs_30.inputs[13] <== 0;
    tcHahs_30.inputs[14] <== 0;
    tcHahs_30.inputs[15] <== 0;
    tcHahs_30.initialState <== tcHahs_29.out[0];
    component bn1togl3_11 = BN1toGL3();
    bn1togl3_11.in <== tcHahs_30.out[0];
    s3_specialX[0] <== bn1togl3_11.out[0];
    s3_specialX[1] <== bn1togl3_11.out[1];
    s3_specialX[2] <== bn1togl3_11.out[2];
    component tcHahs_31 = PoseidonEx(16,17);
    tcHahs_31.inputs[0] <== finalPol[0][0];
    tcHahs_31.inputs[1] <== finalPol[0][1];
    tcHahs_31.inputs[2] <== finalPol[0][2];
    tcHahs_31.inputs[3] <== finalPol[1][0];
    tcHahs_31.inputs[4] <== finalPol[1][1];
    tcHahs_31.inputs[5] <== finalPol[1][2];
    tcHahs_31.inputs[6] <== finalPol[2][0];
    tcHahs_31.inputs[7] <== finalPol[2][1];
    tcHahs_31.inputs[8] <== finalPol[2][2];
    tcHahs_31.inputs[9] <== finalPol[3][0];
    tcHahs_31.inputs[10] <== finalPol[3][1];
    tcHahs_31.inputs[11] <== finalPol[3][2];
    tcHahs_31.inputs[12] <== finalPol[4][0];
    tcHahs_31.inputs[13] <== finalPol[4][1];
    tcHahs_31.inputs[14] <== finalPol[4][2];
    tcHahs_31.inputs[15] <== finalPol[5][0];
    tcHahs_31.initialState <== tcHahs_30.out[0];
    component tcHahs_32 = PoseidonEx(16,17);
    tcHahs_32.inputs[0] <== finalPol[5][1];
    tcHahs_32.inputs[1] <== finalPol[5][2];
    tcHahs_32.inputs[2] <== finalPol[6][0];
    tcHahs_32.inputs[3] <== finalPol[6][1];
    tcHahs_32.inputs[4] <== finalPol[6][2];
    tcHahs_32.inputs[5] <== finalPol[7][0];
    tcHahs_32.inputs[6] <== finalPol[7][1];
    tcHahs_32.inputs[7] <== finalPol[7][2];
    tcHahs_32.inputs[8] <== finalPol[8][0];
    tcHahs_32.inputs[9] <== finalPol[8][1];
    tcHahs_32.inputs[10] <== finalPol[8][2];
    tcHahs_32.inputs[11] <== finalPol[9][0];
    tcHahs_32.inputs[12] <== finalPol[9][1];
    tcHahs_32.inputs[13] <== finalPol[9][2];
    tcHahs_32.inputs[14] <== finalPol[10][0];
    tcHahs_32.inputs[15] <== finalPol[10][1];
    tcHahs_32.initialState <== tcHahs_31.out[0];
    component tcHahs_33 = PoseidonEx(16,17);
    tcHahs_33.inputs[0] <== finalPol[10][2];
    tcHahs_33.inputs[1] <== finalPol[11][0];
    tcHahs_33.inputs[2] <== finalPol[11][1];
    tcHahs_33.inputs[3] <== finalPol[11][2];
    tcHahs_33.inputs[4] <== finalPol[12][0];
    tcHahs_33.inputs[5] <== finalPol[12][1];
    tcHahs_33.inputs[6] <== finalPol[12][2];
    tcHahs_33.inputs[7] <== finalPol[13][0];
    tcHahs_33.inputs[8] <== finalPol[13][1];
    tcHahs_33.inputs[9] <== finalPol[13][2];
    tcHahs_33.inputs[10] <== finalPol[14][0];
    tcHahs_33.inputs[11] <== finalPol[14][1];
    tcHahs_33.inputs[12] <== finalPol[14][2];
    tcHahs_33.inputs[13] <== finalPol[15][0];
    tcHahs_33.inputs[14] <== finalPol[15][1];
    tcHahs_33.inputs[15] <== finalPol[15][2];
    tcHahs_33.initialState <== tcHahs_32.out[0];
    component tcN2b_0 = Num2Bits_strict();
    tcN2b_0.in <== tcHahs_33.out[0];
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
    ys[0][11] <== tcN2b_0.out[11];
    ys[0][12] <== tcN2b_0.out[12];
    ys[0][13] <== tcN2b_0.out[13];
    ys[0][14] <== tcN2b_0.out[14];
    ys[0][15] <== tcN2b_0.out[15];
    ys[0][16] <== tcN2b_0.out[16];
    ys[1][0] <== tcN2b_0.out[17];
    ys[1][1] <== tcN2b_0.out[18];
    ys[1][2] <== tcN2b_0.out[19];
    ys[1][3] <== tcN2b_0.out[20];
    ys[1][4] <== tcN2b_0.out[21];
    ys[1][5] <== tcN2b_0.out[22];
    ys[1][6] <== tcN2b_0.out[23];
    ys[1][7] <== tcN2b_0.out[24];
    ys[1][8] <== tcN2b_0.out[25];
    ys[1][9] <== tcN2b_0.out[26];
    ys[1][10] <== tcN2b_0.out[27];
    ys[1][11] <== tcN2b_0.out[28];
    ys[1][12] <== tcN2b_0.out[29];
    ys[1][13] <== tcN2b_0.out[30];
    ys[1][14] <== tcN2b_0.out[31];
    ys[1][15] <== tcN2b_0.out[32];
    ys[1][16] <== tcN2b_0.out[33];
    ys[2][0] <== tcN2b_0.out[34];
    ys[2][1] <== tcN2b_0.out[35];
    ys[2][2] <== tcN2b_0.out[36];
    ys[2][3] <== tcN2b_0.out[37];
    ys[2][4] <== tcN2b_0.out[38];
    ys[2][5] <== tcN2b_0.out[39];
    ys[2][6] <== tcN2b_0.out[40];
    ys[2][7] <== tcN2b_0.out[41];
    ys[2][8] <== tcN2b_0.out[42];
    ys[2][9] <== tcN2b_0.out[43];
    ys[2][10] <== tcN2b_0.out[44];
    ys[2][11] <== tcN2b_0.out[45];
    ys[2][12] <== tcN2b_0.out[46];
    ys[2][13] <== tcN2b_0.out[47];
    ys[2][14] <== tcN2b_0.out[48];
    ys[2][15] <== tcN2b_0.out[49];
    ys[2][16] <== tcN2b_0.out[50];
    ys[3][0] <== tcN2b_0.out[51];
    ys[3][1] <== tcN2b_0.out[52];
    ys[3][2] <== tcN2b_0.out[53];
    ys[3][3] <== tcN2b_0.out[54];
    ys[3][4] <== tcN2b_0.out[55];
    ys[3][5] <== tcN2b_0.out[56];
    ys[3][6] <== tcN2b_0.out[57];
    ys[3][7] <== tcN2b_0.out[58];
    ys[3][8] <== tcN2b_0.out[59];
    ys[3][9] <== tcN2b_0.out[60];
    ys[3][10] <== tcN2b_0.out[61];
    ys[3][11] <== tcN2b_0.out[62];
    ys[3][12] <== tcN2b_0.out[63];
    ys[3][13] <== tcN2b_0.out[64];
    ys[3][14] <== tcN2b_0.out[65];
    ys[3][15] <== tcN2b_0.out[66];
    ys[3][16] <== tcN2b_0.out[67];
    ys[4][0] <== tcN2b_0.out[68];
    ys[4][1] <== tcN2b_0.out[69];
    ys[4][2] <== tcN2b_0.out[70];
    ys[4][3] <== tcN2b_0.out[71];
    ys[4][4] <== tcN2b_0.out[72];
    ys[4][5] <== tcN2b_0.out[73];
    ys[4][6] <== tcN2b_0.out[74];
    ys[4][7] <== tcN2b_0.out[75];
    ys[4][8] <== tcN2b_0.out[76];
    ys[4][9] <== tcN2b_0.out[77];
    ys[4][10] <== tcN2b_0.out[78];
    ys[4][11] <== tcN2b_0.out[79];
    ys[4][12] <== tcN2b_0.out[80];
    ys[4][13] <== tcN2b_0.out[81];
    ys[4][14] <== tcN2b_0.out[82];
    ys[4][15] <== tcN2b_0.out[83];
    ys[4][16] <== tcN2b_0.out[84];
    ys[5][0] <== tcN2b_0.out[85];
    ys[5][1] <== tcN2b_0.out[86];
    ys[5][2] <== tcN2b_0.out[87];
    ys[5][3] <== tcN2b_0.out[88];
    ys[5][4] <== tcN2b_0.out[89];
    ys[5][5] <== tcN2b_0.out[90];
    ys[5][6] <== tcN2b_0.out[91];
    ys[5][7] <== tcN2b_0.out[92];
    ys[5][8] <== tcN2b_0.out[93];
    ys[5][9] <== tcN2b_0.out[94];
    ys[5][10] <== tcN2b_0.out[95];
    ys[5][11] <== tcN2b_0.out[96];
    ys[5][12] <== tcN2b_0.out[97];
    ys[5][13] <== tcN2b_0.out[98];
    ys[5][14] <== tcN2b_0.out[99];
    ys[5][15] <== tcN2b_0.out[100];
    ys[5][16] <== tcN2b_0.out[101];
    ys[6][0] <== tcN2b_0.out[102];
    ys[6][1] <== tcN2b_0.out[103];
    ys[6][2] <== tcN2b_0.out[104];
    ys[6][3] <== tcN2b_0.out[105];
    ys[6][4] <== tcN2b_0.out[106];
    ys[6][5] <== tcN2b_0.out[107];
    ys[6][6] <== tcN2b_0.out[108];
    ys[6][7] <== tcN2b_0.out[109];
    ys[6][8] <== tcN2b_0.out[110];
    ys[6][9] <== tcN2b_0.out[111];
    ys[6][10] <== tcN2b_0.out[112];
    ys[6][11] <== tcN2b_0.out[113];
    ys[6][12] <== tcN2b_0.out[114];
    ys[6][13] <== tcN2b_0.out[115];
    ys[6][14] <== tcN2b_0.out[116];
    ys[6][15] <== tcN2b_0.out[117];
    ys[6][16] <== tcN2b_0.out[118];
    ys[7][0] <== tcN2b_0.out[119];
    ys[7][1] <== tcN2b_0.out[120];
    ys[7][2] <== tcN2b_0.out[121];
    ys[7][3] <== tcN2b_0.out[122];
    ys[7][4] <== tcN2b_0.out[123];
    ys[7][5] <== tcN2b_0.out[124];
    ys[7][6] <== tcN2b_0.out[125];
    ys[7][7] <== tcN2b_0.out[126];
    ys[7][8] <== tcN2b_0.out[127];
    ys[7][9] <== tcN2b_0.out[128];
    ys[7][10] <== tcN2b_0.out[129];
    ys[7][11] <== tcN2b_0.out[130];
    ys[7][12] <== tcN2b_0.out[131];
    ys[7][13] <== tcN2b_0.out[132];
    ys[7][14] <== tcN2b_0.out[133];
    ys[7][15] <== tcN2b_0.out[134];
    ys[7][16] <== tcN2b_0.out[135];
    component verifyEvaluations = VerifyEvaluations();
    verifyEvaluations.enable <== enable;
    for (var i=0; i<8; i++) {
        for (var k=0; k<3; k++) {
            verifyEvaluations.challenges[i][k] <== challenges[i][k];
        }
    }
    for (var i=0; i<18; i++) {
        verifyEvaluations.publics[i] <== publics[i];
    }
    for (var i=0; i<121; i++) {
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
        s0_merkle1[q] = MerkleHash(1, 12, 131072);
    
        s0_merkle3[q] = MerkleHash(1, 84, 131072);
    
        s0_merkle4[q] = MerkleHash(1, 6, 131072);
        s0_merkleC[q] = MerkleHash(1, 32, 131072);
        s0_lowValues[q] = TreeSelector(6, 3) ;
    
        for (var i=0; i<17; i++ ) {
            verifyQueries[q].ys[i] <== ys[q][i];
            s0_merkle1[q].key[i] <== ys[q][i];
    
            s0_merkle3[q].key[i] <== ys[q][i];
    
            s0_merkle4[q].key[i] <== ys[q][i];
            s0_merkleC[q].key[i] <== ys[q][i];
        }
        for (var i=0; i<12; i++ ) {
            verifyQueries[q].tree1[i] <== s0_vals1[q][i];
            s0_merkle1[q].values[i][0] <== s0_vals1[q][i];
        }
    
        for (var i=0; i<84; i++ ) {
            verifyQueries[q].tree3[i] <== s0_vals3[q][i];
            s0_merkle3[q].values[i][0] <== s0_vals3[q][i];
        }
    
        for (var i=0; i<6; i++ ) {
            verifyQueries[q].tree4[i] <== s0_vals4[q][i];
            s0_merkle4[q].values[i][0] <== s0_vals4[q][i];
        }
        for (var i=0; i<32; i++ ) {
            verifyQueries[q].consts[i] <== s0_valsC[q][i];
            s0_merkleC[q].values[i][0] <== s0_valsC[q][i];
        }
        for (var i=0; i<8; i++) {
            for (var e=0; e<3; e++) {
                verifyQueries[q].challenges[i][e] <== challenges[i][e];
            }
        }
        for (var i=0; i<121; i++) {
            for (var e=0; e<3; e++) {
                verifyQueries[q].evals[i][e] <== evals[i][e];
            }
        }
        for (var i=0; i<5;i++) {
            for (var j=0; j<16; j++) {
                s0_merkle1[q].siblings[i][j] <== s0_siblings1[q][i][j];
    
                s0_merkle3[q].siblings[i][j] <== s0_siblings3[q][i][j];
        
                s0_merkle4[q].siblings[i][j] <== s0_siblings4[q][i][j];
                s0_merkleC[q].siblings[i][j] <== s0_siblingsC[q][i][j];
            }
        }
        
        for (var i=0; i<64; i++) {
            for (var e=0; e<3; e++) {
                s0_lowValues[q].values[i][e] <== s1_vals[q][i*3+e];
            }
        }
        for (var i=0; i<6; i++) {
            s0_lowValues[q].key[i] <== ys[q][i + 11];
        }
        
    }
    component s1_merkle[8];
    component s1_fft[8];
    component s1_evalPol[8];
    component s1_lowValues[8];
    component s1_cNorm[8];
    component s1_sx[8][10];
    component s1_evalXprime[8];
    signal s1_X[8];
        
    for (var q=0; q<8; q++) {
        s1_merkle[q] = MerkleHash(3, 64, 2048);
        s1_fft[q] = FFT(6, 1);
        s1_evalPol[q] = EvalPol(64);
        s1_lowValues[q] = TreeSelector(4, 3) ;
        for (var i=0; i< 64; i++) {
            for (var e=0; e<3; e++) {
                s1_merkle[q].values[i][e] <== s1_vals[q][i*3+e];
                s1_fft[q].in[i][e] <== s1_vals[q][i*3+e];
            }
        }
        for (var i=0; i<3; i++) {
            for (var j=0; j<16; j++) {
                s1_merkle[q].siblings[i][j] <== s1_siblings[q][i][j];
            }
        }
        for (var i=0; i<11; i++) {
            s1_merkle[q].key[i] <== ys[q][i];
        }
        
        for (var i=1; i<11; i++ ) {
            s1_sx[q][i-1] = GLMul();
            if (i==1) {
                s1_sx[q][i-1].ina <== ys[q][0] * (6700795321283767376 - 5646962470228954384) + 5646962470228954384;
            } else {
                s1_sx[q][i-1].ina <== s1_sx[q][i-2].out;
            }
            s1_sx[q][i-1].inb <== ys[q][i] * (_inv1(roots(17 -i)) -1) +1;
        }
        s1_X[q] <== s1_sx[q][9].out;
        
        for (var i=0; i< 64; i++) {
            for (var e=0; e<3; e++) {
                s1_evalPol[q].pol[i][e] <== s1_fft[q].out[i][e];
            }
        }
        s1_evalXprime[q] = GLCMul();
        s1_evalXprime[q].ina[0] <== s1_specialX[0];
        s1_evalXprime[q].ina[1] <== s1_specialX[1];
        s1_evalXprime[q].ina[2] <== s1_specialX[2];
        s1_evalXprime[q].inb[0] <== s1_X[q];
        s1_evalXprime[q].inb[1] <== 0;
        s1_evalXprime[q].inb[2] <== 0;
        for (var e=0; e<3; e++) {
            s1_evalPol[q].x[e] <== s1_evalXprime[q].out[e];
        }
        
        for (var i=0; i<16; i++) {
            for (var e=0; e<3; e++) {
                s1_lowValues[q].values[i][e] <== s2_vals[q][i*3+e];
            }
        }
        for (var i=0; i<4; i++) {
            s1_lowValues[q].key[i] <== ys[q][i + 7];
        }
        
        s1_cNorm[q] = GLCNorm();
        for (var e=0; e<3; e++) {
            s1_cNorm[q].in[e] <== s1_evalPol[q].out[e] - s1_lowValues[q].out[e] + p;
        }
    }
    component s2_merkle[8];
    component s2_fft[8];
    component s2_evalPol[8];
    component s2_lowValues[8];
    component s2_cNorm[8];
    component s2_sx[8][6];
    component s2_evalXprime[8];
    signal s2_X[8];
        
    for (var q=0; q<8; q++) {
        s2_merkle[q] = MerkleHash(3, 16, 128);
        s2_fft[q] = FFT(4, 1);
        s2_evalPol[q] = EvalPol(16);
        s2_lowValues[q] = TreeSelector(3, 3) ;
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s2_merkle[q].values[i][e] <== s2_vals[q][i*3+e];
                s2_fft[q].in[i][e] <== s2_vals[q][i*3+e];
            }
        }
        for (var i=0; i<2; i++) {
            for (var j=0; j<16; j++) {
                s2_merkle[q].siblings[i][j] <== s2_siblings[q][i][j];
            }
        }
        for (var i=0; i<7; i++) {
            s2_merkle[q].key[i] <== ys[q][i];
        }
        
        for (var i=1; i<7; i++ ) {
            s2_sx[q][i-1] = GLMul();
            if (i==1) {
                s2_sx[q][i-1].ina <== ys[q][0] * (4483865097005565255 - 14859683105753436876) + 14859683105753436876;
            } else {
                s2_sx[q][i-1].ina <== s2_sx[q][i-2].out;
            }
            s2_sx[q][i-1].inb <== ys[q][i] * (_inv1(roots(11 -i)) -1) +1;
        }
        s2_X[q] <== s2_sx[q][5].out;
        
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s2_evalPol[q].pol[i][e] <== s2_fft[q].out[i][e];
            }
        }
        s2_evalXprime[q] = GLCMul();
        s2_evalXprime[q].ina[0] <== s2_specialX[0];
        s2_evalXprime[q].ina[1] <== s2_specialX[1];
        s2_evalXprime[q].ina[2] <== s2_specialX[2];
        s2_evalXprime[q].inb[0] <== s2_X[q];
        s2_evalXprime[q].inb[1] <== 0;
        s2_evalXprime[q].inb[2] <== 0;
        for (var e=0; e<3; e++) {
            s2_evalPol[q].x[e] <== s2_evalXprime[q].out[e];
        }
        
        for (var i=0; i<8; i++) {
            for (var e=0; e<3; e++) {
                s2_lowValues[q].values[i][e] <== s3_vals[q][i*3+e];
            }
        }
        for (var i=0; i<3; i++) {
            s2_lowValues[q].key[i] <== ys[q][i + 4];
        }
        
        s2_cNorm[q] = GLCNorm();
        for (var e=0; e<3; e++) {
            s2_cNorm[q].in[e] <== s2_evalPol[q].out[e] - s2_lowValues[q].out[e] + p;
        }
    }
    component s3_merkle[8];
    component s3_fft[8];
    component s3_evalPol[8];
    component s3_lowValues[8];
    component s3_cNorm[8];
    component s3_sx[8][3];
    component s3_evalXprime[8];
    signal s3_X[8];
        
    for (var q=0; q<8; q++) {
        s3_merkle[q] = MerkleHash(3, 8, 16);
        s3_fft[q] = FFT(3, 1);
        s3_evalPol[q] = EvalPol(8);
        s3_lowValues[q] = TreeSelector(4, 3) ;
        for (var i=0; i< 8; i++) {
            for (var e=0; e<3; e++) {
                s3_merkle[q].values[i][e] <== s3_vals[q][i*3+e];
                s3_fft[q].in[i][e] <== s3_vals[q][i*3+e];
            }
        }
        for (var i=0; i<1; i++) {
            for (var j=0; j<16; j++) {
                s3_merkle[q].siblings[i][j] <== s3_siblings[q][i][j];
            }
        }
        for (var i=0; i<4; i++) {
            s3_merkle[q].key[i] <== ys[q][i];
        }
        
        for (var i=1; i<4; i++ ) {
            s3_sx[q][i-1] = GLMul();
            if (i==1) {
                s3_sx[q][i-1].ina <== ys[q][0] * (1455388002775739939 - 18352195122931766578) + 18352195122931766578;
            } else {
                s3_sx[q][i-1].ina <== s3_sx[q][i-2].out;
            }
            s3_sx[q][i-1].inb <== ys[q][i] * (_inv1(roots(7 -i)) -1) +1;
        }
        s3_X[q] <== s3_sx[q][2].out;
        
        for (var i=0; i< 8; i++) {
            for (var e=0; e<3; e++) {
                s3_evalPol[q].pol[i][e] <== s3_fft[q].out[i][e];
            }
        }
        s3_evalXprime[q] = GLCMul();
        s3_evalXprime[q].ina[0] <== s3_specialX[0];
        s3_evalXprime[q].ina[1] <== s3_specialX[1];
        s3_evalXprime[q].ina[2] <== s3_specialX[2];
        s3_evalXprime[q].inb[0] <== s3_X[q];
        s3_evalXprime[q].inb[1] <== 0;
        s3_evalXprime[q].inb[2] <== 0;
        for (var e=0; e<3; e++) {
            s3_evalPol[q].x[e] <== s3_evalXprime[q].out[e];
        }
        
        for (var i=0; i<16; i++) {
            for (var e=0; e<3; e++) {
                s3_lowValues[q].values[i][e] <== finalPol[i][e];
            }
        }
        for (var i=0; i<4; i++) {
            s3_lowValues[q].key[i] <== ys[q][i];
        }
        s3_cNorm[q] = GLCNorm();
        for (var e=0; e<3; e++) {
            s3_cNorm[q].in[e] <== s3_evalPol[q].out[e] - s3_lowValues[q].out[e] + p;
        }
    }
    for (var q=0; q < 8; q ++) {
        enable * (s0_merkle1[q].root - root1) === 0;
        enable * (s0_merkle3[q].root - root3) === 0;
        enable * (s0_merkle4[q].root - root4) === 0;
        enable * (s0_merkleC[q].root - rootC) === 0;
        for (var e=0; e<3; e++) {
            enable * (s0_lowValues[q].out[e] - verifyQueries[q].out[e]) === 0;
        }
    }
    for (var q = 0; q < 8; q ++) {
        for (var e=0; e<3; e++) {
            enable * s1_cNorm[q].out[e] === 0;
        }
        enable * (s1_merkle[q].root - s1_root) === 0;
    }
    for (var q = 0; q < 8; q ++) {
        for (var e=0; e<3; e++) {
            enable * s2_cNorm[q].out[e] === 0;
        }
        enable * (s2_merkle[q].root - s2_root) === 0;
    }
    for (var q = 0; q < 8; q ++) {
        for (var e=0; e<3; e++) {
            enable * s3_cNorm[q].out[e] === 0;
        }
        enable * (s3_merkle[q].root - s3_root) === 0;
    }
    component lastIFFT = FFT(4, 1);

    for (var k=0; k< 16; k++ ){
        for (var e=0; e<3; e++) {
            lastIFFT.in[k][e] <== finalPol[k][e];
        }
    }

    for (var k= 8; k< 16; k++ ) {
        for (var e=0; e<3; e++) {
            enable * lastIFFT.out[k][e] === 0;
        }
    }
}

template Main() {
    signal input proverAddr;
    signal output publicsHash;

    signal input publics[18];
    signal input rootC; 
    signal input root1;
    signal input root2;
    signal input root3;
    signal input root4;
    signal input evals[121][3];

    signal input s0_vals1[8][12];

    signal input s0_vals3[8][84];

    signal input s0_vals4[8][6];
    signal input s0_valsC[8][32];
    signal input s0_siblings1[8][5][16];

    signal input s0_siblings3[8][5][16];

    signal input s0_siblings4[8][5][16];
    signal input s0_siblingsC[8][5][16];

    signal input s1_root;
    
    signal input s2_root;
    
    signal input s3_root;
    
    signal input s1_vals[8][192];
    signal input s1_siblings[8][3][16];

    signal input s2_vals[8][48];
    signal input s2_siblings[8][2][16];

    signal input s3_vals[8][24];
    signal input s3_siblings[8][1][16];

    signal input finalPol[16][3];

    component sv = StarkVerifier();

    sv.publics <== publics;
    sv.rootC <== rootC; 
    sv.root1 <== root1;
    sv.root2 <== root2;
    sv.root3 <== root3;
    sv.root4 <== root4;
    sv.evals <== evals;

    sv.s0_vals1 <== s0_vals1;

    sv.s0_vals3 <== s0_vals3;
    
    sv.s0_vals4 <== s0_vals4;
    sv.s0_valsC <== s0_valsC;
    sv.s0_siblings1 <== s0_siblings1;
    
    sv.s0_siblings3 <== s0_siblings3;
    
    sv.s0_siblings4 <== s0_siblings4;
    sv.s0_siblingsC <== s0_siblingsC;
    
    sv.s1_root <== s1_root;
    
    sv.s2_root <== s2_root;
    
    sv.s3_root <== s3_root;
    
    sv.s1_vals <== s1_vals;
    sv.s1_siblings <== s1_siblings;
    
    sv.s2_vals <== s2_vals;
    sv.s2_siblings <== s2_siblings;
    
    sv.s3_vals <== s3_vals;
    sv.s3_siblings <== s3_siblings;
    
    sv.finalPol <== finalPol;
    
    component publicsHasher = Sha256(1312);
    component n2bProverAddr = Num2Bits(160);
    component n2bPublics[18];
    component cmpPublics[18];

    n2bProverAddr.in <== proverAddr;
    for (var i=0; i<160; i++) {
        publicsHasher.in[160 - 1 -i] <== n2bProverAddr.out[i];
    }

    var offset = 160;
    for (var i=0; i<18; i++) {
        n2bPublics[i] = Num2Bits(64);
        cmpPublics[i] = CompConstant64(0xFFFFFFFF00000000);
        n2bPublics[i].in <== publics[i];
        for (var j=0; j<64; j++) {
            publicsHasher.in[offset + 64 - 1 -j] <== n2bPublics[i].out[j];
            cmpPublics[i].in[j] <== n2bPublics[i].out[j];
        }
        cmpPublics[i].out === 0;
        offset += 64;
    }

    component n2bPublicsHash = Bits2Num(256);
    for (var i = 0; i < 256; i++) {
        n2bPublicsHash.in[i] <== publicsHasher.out[255-i];
    }

    publicsHash <== n2bPublicsHash.out;
}

component main {public [rootC]} = Main();
