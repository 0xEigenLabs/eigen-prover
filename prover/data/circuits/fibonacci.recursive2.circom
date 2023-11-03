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
    signal input evals[121][3];
    signal input publics[14];
    signal input enable;

    signal zMul[17][3];
    
    for (var i=0; i< 17; i++) {
        if (i==0) {
            zMul[i] <== CMul()(challenges[7], challenges[7]);
        } else {
            zMul[i] <== CMul()(zMul[i-1], zMul[i-1]);
        }
    }
        
    signal Z[3];

    Z[0] <== zMul[16][0] -1;
    Z[1] <== zMul[16][1];
    Z[2] <== zMul[16][2];
    signal tmp_0[3] <== [evals[0][0] - publics[0], evals[0][1], evals[0][2]];
    signal tmp_1[3] <== CMul()(evals[1], tmp_0);
    signal tmp_904[3] <== [tmp_1[0] - 0, tmp_1[1], tmp_1[2]];
    signal tmp_2[3] <== [evals[2][0] - publics[1], evals[2][1], evals[2][2]];
    signal tmp_3[3] <== CMul()(evals[1], tmp_2);
    signal tmp_905[3] <== [tmp_3[0] - 0, tmp_3[1], tmp_3[2]];
    signal tmp_4[3] <== [evals[3][0] - publics[2], evals[3][1], evals[3][2]];
    signal tmp_5[3] <== CMul()(evals[1], tmp_4);
    signal tmp_906[3] <== [tmp_5[0] - 0, tmp_5[1], tmp_5[2]];
    signal tmp_6[3] <== [evals[4][0] - publics[3], evals[4][1], evals[4][2]];
    signal tmp_7[3] <== CMul()(evals[1], tmp_6);
    signal tmp_907[3] <== [tmp_7[0] - 0, tmp_7[1], tmp_7[2]];
    signal tmp_8[3] <== [evals[5][0] - publics[4], evals[5][1], evals[5][2]];
    signal tmp_9[3] <== CMul()(evals[1], tmp_8);
    signal tmp_908[3] <== [tmp_9[0] - 0, tmp_9[1], tmp_9[2]];
    signal tmp_10[3] <== [evals[6][0] - publics[5], evals[6][1], evals[6][2]];
    signal tmp_11[3] <== CMul()(evals[1], tmp_10);
    signal tmp_909[3] <== [tmp_11[0] - 0, tmp_11[1], tmp_11[2]];
    signal tmp_12[3] <== [evals[7][0] - publics[6], evals[7][1], evals[7][2]];
    signal tmp_13[3] <== CMul()(evals[1], tmp_12);
    signal tmp_910[3] <== [tmp_13[0] - 0, tmp_13[1], tmp_13[2]];
    signal tmp_14[3] <== [evals[8][0] - publics[7], evals[8][1], evals[8][2]];
    signal tmp_15[3] <== CMul()(evals[1], tmp_14);
    signal tmp_911[3] <== [tmp_15[0] - 0, tmp_15[1], tmp_15[2]];
    signal tmp_16[3] <== [evals[9][0] - publics[8], evals[9][1], evals[9][2]];
    signal tmp_17[3] <== CMul()(evals[1], tmp_16);
    signal tmp_912[3] <== [tmp_17[0] - 0, tmp_17[1], tmp_17[2]];
    signal tmp_18[3] <== [evals[10][0] - publics[9], evals[10][1], evals[10][2]];
    signal tmp_19[3] <== CMul()(evals[1], tmp_18);
    signal tmp_913[3] <== [tmp_19[0] - 0, tmp_19[1], tmp_19[2]];
    signal tmp_20[3] <== [evals[11][0] - publics[10], evals[11][1], evals[11][2]];
    signal tmp_21[3] <== CMul()(evals[1], tmp_20);
    signal tmp_914[3] <== [tmp_21[0] - 0, tmp_21[1], tmp_21[2]];
    signal tmp_22[3] <== [evals[12][0] - publics[11], evals[12][1], evals[12][2]];
    signal tmp_23[3] <== CMul()(evals[1], tmp_22);
    signal tmp_915[3] <== [tmp_23[0] - 0, tmp_23[1], tmp_23[2]];
    signal tmp_24[3] <== [evals[0][0] - publics[12], evals[0][1], evals[0][2]];
    signal tmp_25[3] <== CMul()(evals[13], tmp_24);
    signal tmp_916[3] <== [tmp_25[0] - 0, tmp_25[1], tmp_25[2]];
    signal tmp_26[3] <== [evals[2][0] - publics[13], evals[2][1], evals[2][2]];
    signal tmp_27[3] <== CMul()(evals[13], tmp_26);
    signal tmp_917[3] <== [tmp_27[0] - 0, tmp_27[1], tmp_27[2]];
    signal tmp_28[3] <== CMul()(evals[14], evals[15]);
    signal tmp_918[3] <== [tmp_28[0] - 0, tmp_28[1], tmp_28[2]];
    signal tmp_29[3] <== CMul()(evals[16], evals[15]);
    signal tmp_919[3] <== [tmp_29[0] - 0, tmp_29[1], tmp_29[2]];
    signal tmp_30[3] <== CMul()(evals[17], evals[15]);
    signal tmp_920[3] <== [tmp_30[0] - 0, tmp_30[1], tmp_30[2]];
    signal tmp_31[3] <== CMul()(evals[18], evals[15]);
    signal tmp_921[3] <== [tmp_31[0] - 0, tmp_31[1], tmp_31[2]];
    signal tmp_922[3] <== [evals[0][0] + evals[19][0], evals[0][1] + evals[19][1], evals[0][2] + evals[19][2]];
    signal tmp_923[3] <== CMul()(evals[20], tmp_922);
    signal tmp_924[3] <== tmp_923;
    signal tmp_32[3] <== [25 * tmp_924[0], 25 * tmp_924[1], 25 * tmp_924[2]];
    signal tmp_33[3] <== [15 * evals[21][0], 15 * evals[21][1], 15 * evals[21][2]];
    signal tmp_34[3] <== [tmp_32[0] + tmp_33[0], tmp_32[1] + tmp_33[1], tmp_32[2] + tmp_33[2]];
    signal tmp_35[3] <== [41 * evals[22][0], 41 * evals[22][1], 41 * evals[22][2]];
    signal tmp_36[3] <== [tmp_34[0] + tmp_35[0], tmp_34[1] + tmp_35[1], tmp_34[2] + tmp_35[2]];
    signal tmp_37[3] <== [16 * evals[23][0], 16 * evals[23][1], 16 * evals[23][2]];
    signal tmp_38[3] <== [tmp_36[0] + tmp_37[0], tmp_36[1] + tmp_37[1], tmp_36[2] + tmp_37[2]];
    signal tmp_39[3] <== [2 * evals[24][0], 2 * evals[24][1], 2 * evals[24][2]];
    signal tmp_40[3] <== [tmp_38[0] + tmp_39[0], tmp_38[1] + tmp_39[1], tmp_38[2] + tmp_39[2]];
    signal tmp_41[3] <== [28 * evals[25][0], 28 * evals[25][1], 28 * evals[25][2]];
    signal tmp_42[3] <== [tmp_40[0] + tmp_41[0], tmp_40[1] + tmp_41[1], tmp_40[2] + tmp_41[2]];
    signal tmp_43[3] <== [13 * evals[26][0], 13 * evals[26][1], 13 * evals[26][2]];
    signal tmp_44[3] <== [tmp_42[0] + tmp_43[0], tmp_42[1] + tmp_43[1], tmp_42[2] + tmp_43[2]];
    signal tmp_45[3] <== [13 * evals[27][0], 13 * evals[27][1], 13 * evals[27][2]];
    signal tmp_46[3] <== [tmp_44[0] + tmp_45[0], tmp_44[1] + tmp_45[1], tmp_44[2] + tmp_45[2]];
    signal tmp_47[3] <== [39 * evals[28][0], 39 * evals[28][1], 39 * evals[28][2]];
    signal tmp_48[3] <== [tmp_46[0] + tmp_47[0], tmp_46[1] + tmp_47[1], tmp_46[2] + tmp_47[2]];
    signal tmp_49[3] <== [18 * evals[29][0], 18 * evals[29][1], 18 * evals[29][2]];
    signal tmp_50[3] <== [tmp_48[0] + tmp_49[0], tmp_48[1] + tmp_49[1], tmp_48[2] + tmp_49[2]];
    signal tmp_51[3] <== [34 * evals[30][0], 34 * evals[30][1], 34 * evals[30][2]];
    signal tmp_52[3] <== [tmp_50[0] + tmp_51[0], tmp_50[1] + tmp_51[1], tmp_50[2] + tmp_51[2]];
    signal tmp_53[3] <== [20 * evals[31][0], 20 * evals[31][1], 20 * evals[31][2]];
    signal tmp_54[3] <== [tmp_52[0] + tmp_53[0], tmp_52[1] + tmp_53[1], tmp_52[2] + tmp_53[2]];
    signal tmp_55[3] <== [evals[32][0] - tmp_54[0], evals[32][1] - tmp_54[1], evals[32][2] - tmp_54[2]];
    signal tmp_56[3] <== CMul()(evals[33], tmp_55);
    signal tmp_925[3] <== [tmp_56[0] - 0, tmp_56[1], tmp_56[2]];
    signal tmp_57[3] <== [20 * tmp_924[0], 20 * tmp_924[1], 20 * tmp_924[2]];
    signal tmp_58[3] <== [17 * evals[21][0], 17 * evals[21][1], 17 * evals[21][2]];
    signal tmp_59[3] <== [tmp_57[0] + tmp_58[0], tmp_57[1] + tmp_58[1], tmp_57[2] + tmp_58[2]];
    signal tmp_60[3] <== [15 * evals[22][0], 15 * evals[22][1], 15 * evals[22][2]];
    signal tmp_61[3] <== [tmp_59[0] + tmp_60[0], tmp_59[1] + tmp_60[1], tmp_59[2] + tmp_60[2]];
    signal tmp_62[3] <== [41 * evals[23][0], 41 * evals[23][1], 41 * evals[23][2]];
    signal tmp_63[3] <== [tmp_61[0] + tmp_62[0], tmp_61[1] + tmp_62[1], tmp_61[2] + tmp_62[2]];
    signal tmp_64[3] <== [16 * evals[24][0], 16 * evals[24][1], 16 * evals[24][2]];
    signal tmp_65[3] <== [tmp_63[0] + tmp_64[0], tmp_63[1] + tmp_64[1], tmp_63[2] + tmp_64[2]];
    signal tmp_66[3] <== [2 * evals[25][0], 2 * evals[25][1], 2 * evals[25][2]];
    signal tmp_67[3] <== [tmp_65[0] + tmp_66[0], tmp_65[1] + tmp_66[1], tmp_65[2] + tmp_66[2]];
    signal tmp_68[3] <== [28 * evals[26][0], 28 * evals[26][1], 28 * evals[26][2]];
    signal tmp_69[3] <== [tmp_67[0] + tmp_68[0], tmp_67[1] + tmp_68[1], tmp_67[2] + tmp_68[2]];
    signal tmp_70[3] <== [13 * evals[27][0], 13 * evals[27][1], 13 * evals[27][2]];
    signal tmp_71[3] <== [tmp_69[0] + tmp_70[0], tmp_69[1] + tmp_70[1], tmp_69[2] + tmp_70[2]];
    signal tmp_72[3] <== [13 * evals[28][0], 13 * evals[28][1], 13 * evals[28][2]];
    signal tmp_73[3] <== [tmp_71[0] + tmp_72[0], tmp_71[1] + tmp_72[1], tmp_71[2] + tmp_72[2]];
    signal tmp_74[3] <== [39 * evals[29][0], 39 * evals[29][1], 39 * evals[29][2]];
    signal tmp_75[3] <== [tmp_73[0] + tmp_74[0], tmp_73[1] + tmp_74[1], tmp_73[2] + tmp_74[2]];
    signal tmp_76[3] <== [18 * evals[30][0], 18 * evals[30][1], 18 * evals[30][2]];
    signal tmp_77[3] <== [tmp_75[0] + tmp_76[0], tmp_75[1] + tmp_76[1], tmp_75[2] + tmp_76[2]];
    signal tmp_78[3] <== [34 * evals[31][0], 34 * evals[31][1], 34 * evals[31][2]];
    signal tmp_79[3] <== [tmp_77[0] + tmp_78[0], tmp_77[1] + tmp_78[1], tmp_77[2] + tmp_78[2]];
    signal tmp_80[3] <== [evals[34][0] - tmp_79[0], evals[34][1] - tmp_79[1], evals[34][2] - tmp_79[2]];
    signal tmp_81[3] <== CMul()(evals[33], tmp_80);
    signal tmp_926[3] <== [tmp_81[0] - 0, tmp_81[1], tmp_81[2]];
    signal tmp_82[3] <== [34 * tmp_924[0], 34 * tmp_924[1], 34 * tmp_924[2]];
    signal tmp_83[3] <== [20 * evals[21][0], 20 * evals[21][1], 20 * evals[21][2]];
    signal tmp_84[3] <== [tmp_82[0] + tmp_83[0], tmp_82[1] + tmp_83[1], tmp_82[2] + tmp_83[2]];
    signal tmp_85[3] <== [17 * evals[22][0], 17 * evals[22][1], 17 * evals[22][2]];
    signal tmp_86[3] <== [tmp_84[0] + tmp_85[0], tmp_84[1] + tmp_85[1], tmp_84[2] + tmp_85[2]];
    signal tmp_87[3] <== [15 * evals[23][0], 15 * evals[23][1], 15 * evals[23][2]];
    signal tmp_88[3] <== [tmp_86[0] + tmp_87[0], tmp_86[1] + tmp_87[1], tmp_86[2] + tmp_87[2]];
    signal tmp_89[3] <== [41 * evals[24][0], 41 * evals[24][1], 41 * evals[24][2]];
    signal tmp_90[3] <== [tmp_88[0] + tmp_89[0], tmp_88[1] + tmp_89[1], tmp_88[2] + tmp_89[2]];
    signal tmp_91[3] <== [16 * evals[25][0], 16 * evals[25][1], 16 * evals[25][2]];
    signal tmp_92[3] <== [tmp_90[0] + tmp_91[0], tmp_90[1] + tmp_91[1], tmp_90[2] + tmp_91[2]];
    signal tmp_93[3] <== [2 * evals[26][0], 2 * evals[26][1], 2 * evals[26][2]];
    signal tmp_94[3] <== [tmp_92[0] + tmp_93[0], tmp_92[1] + tmp_93[1], tmp_92[2] + tmp_93[2]];
    signal tmp_95[3] <== [28 * evals[27][0], 28 * evals[27][1], 28 * evals[27][2]];
    signal tmp_96[3] <== [tmp_94[0] + tmp_95[0], tmp_94[1] + tmp_95[1], tmp_94[2] + tmp_95[2]];
    signal tmp_97[3] <== [13 * evals[28][0], 13 * evals[28][1], 13 * evals[28][2]];
    signal tmp_98[3] <== [tmp_96[0] + tmp_97[0], tmp_96[1] + tmp_97[1], tmp_96[2] + tmp_97[2]];
    signal tmp_99[3] <== [13 * evals[29][0], 13 * evals[29][1], 13 * evals[29][2]];
    signal tmp_100[3] <== [tmp_98[0] + tmp_99[0], tmp_98[1] + tmp_99[1], tmp_98[2] + tmp_99[2]];
    signal tmp_101[3] <== [39 * evals[30][0], 39 * evals[30][1], 39 * evals[30][2]];
    signal tmp_102[3] <== [tmp_100[0] + tmp_101[0], tmp_100[1] + tmp_101[1], tmp_100[2] + tmp_101[2]];
    signal tmp_103[3] <== [18 * evals[31][0], 18 * evals[31][1], 18 * evals[31][2]];
    signal tmp_104[3] <== [tmp_102[0] + tmp_103[0], tmp_102[1] + tmp_103[1], tmp_102[2] + tmp_103[2]];
    signal tmp_105[3] <== [evals[35][0] - tmp_104[0], evals[35][1] - tmp_104[1], evals[35][2] - tmp_104[2]];
    signal tmp_106[3] <== CMul()(evals[33], tmp_105);
    signal tmp_927[3] <== [tmp_106[0] - 0, tmp_106[1], tmp_106[2]];
    signal tmp_107[3] <== [18 * tmp_924[0], 18 * tmp_924[1], 18 * tmp_924[2]];
    signal tmp_108[3] <== [34 * evals[21][0], 34 * evals[21][1], 34 * evals[21][2]];
    signal tmp_109[3] <== [tmp_107[0] + tmp_108[0], tmp_107[1] + tmp_108[1], tmp_107[2] + tmp_108[2]];
    signal tmp_110[3] <== [20 * evals[22][0], 20 * evals[22][1], 20 * evals[22][2]];
    signal tmp_111[3] <== [tmp_109[0] + tmp_110[0], tmp_109[1] + tmp_110[1], tmp_109[2] + tmp_110[2]];
    signal tmp_112[3] <== [17 * evals[23][0], 17 * evals[23][1], 17 * evals[23][2]];
    signal tmp_113[3] <== [tmp_111[0] + tmp_112[0], tmp_111[1] + tmp_112[1], tmp_111[2] + tmp_112[2]];
    signal tmp_114[3] <== [15 * evals[24][0], 15 * evals[24][1], 15 * evals[24][2]];
    signal tmp_115[3] <== [tmp_113[0] + tmp_114[0], tmp_113[1] + tmp_114[1], tmp_113[2] + tmp_114[2]];
    signal tmp_116[3] <== [41 * evals[25][0], 41 * evals[25][1], 41 * evals[25][2]];
    signal tmp_117[3] <== [tmp_115[0] + tmp_116[0], tmp_115[1] + tmp_116[1], tmp_115[2] + tmp_116[2]];
    signal tmp_118[3] <== [16 * evals[26][0], 16 * evals[26][1], 16 * evals[26][2]];
    signal tmp_119[3] <== [tmp_117[0] + tmp_118[0], tmp_117[1] + tmp_118[1], tmp_117[2] + tmp_118[2]];
    signal tmp_120[3] <== [2 * evals[27][0], 2 * evals[27][1], 2 * evals[27][2]];
    signal tmp_121[3] <== [tmp_119[0] + tmp_120[0], tmp_119[1] + tmp_120[1], tmp_119[2] + tmp_120[2]];
    signal tmp_122[3] <== [28 * evals[28][0], 28 * evals[28][1], 28 * evals[28][2]];
    signal tmp_123[3] <== [tmp_121[0] + tmp_122[0], tmp_121[1] + tmp_122[1], tmp_121[2] + tmp_122[2]];
    signal tmp_124[3] <== [13 * evals[29][0], 13 * evals[29][1], 13 * evals[29][2]];
    signal tmp_125[3] <== [tmp_123[0] + tmp_124[0], tmp_123[1] + tmp_124[1], tmp_123[2] + tmp_124[2]];
    signal tmp_126[3] <== [13 * evals[30][0], 13 * evals[30][1], 13 * evals[30][2]];
    signal tmp_127[3] <== [tmp_125[0] + tmp_126[0], tmp_125[1] + tmp_126[1], tmp_125[2] + tmp_126[2]];
    signal tmp_128[3] <== [39 * evals[31][0], 39 * evals[31][1], 39 * evals[31][2]];
    signal tmp_129[3] <== [tmp_127[0] + tmp_128[0], tmp_127[1] + tmp_128[1], tmp_127[2] + tmp_128[2]];
    signal tmp_130[3] <== [evals[36][0] - tmp_129[0], evals[36][1] - tmp_129[1], evals[36][2] - tmp_129[2]];
    signal tmp_131[3] <== CMul()(evals[33], tmp_130);
    signal tmp_928[3] <== [tmp_131[0] - 0, tmp_131[1], tmp_131[2]];
    signal tmp_132[3] <== [39 * tmp_924[0], 39 * tmp_924[1], 39 * tmp_924[2]];
    signal tmp_133[3] <== [18 * evals[21][0], 18 * evals[21][1], 18 * evals[21][2]];
    signal tmp_134[3] <== [tmp_132[0] + tmp_133[0], tmp_132[1] + tmp_133[1], tmp_132[2] + tmp_133[2]];
    signal tmp_135[3] <== [34 * evals[22][0], 34 * evals[22][1], 34 * evals[22][2]];
    signal tmp_136[3] <== [tmp_134[0] + tmp_135[0], tmp_134[1] + tmp_135[1], tmp_134[2] + tmp_135[2]];
    signal tmp_137[3] <== [20 * evals[23][0], 20 * evals[23][1], 20 * evals[23][2]];
    signal tmp_138[3] <== [tmp_136[0] + tmp_137[0], tmp_136[1] + tmp_137[1], tmp_136[2] + tmp_137[2]];
    signal tmp_139[3] <== [17 * evals[24][0], 17 * evals[24][1], 17 * evals[24][2]];
    signal tmp_140[3] <== [tmp_138[0] + tmp_139[0], tmp_138[1] + tmp_139[1], tmp_138[2] + tmp_139[2]];
    signal tmp_141[3] <== [15 * evals[25][0], 15 * evals[25][1], 15 * evals[25][2]];
    signal tmp_142[3] <== [tmp_140[0] + tmp_141[0], tmp_140[1] + tmp_141[1], tmp_140[2] + tmp_141[2]];
    signal tmp_143[3] <== [41 * evals[26][0], 41 * evals[26][1], 41 * evals[26][2]];
    signal tmp_144[3] <== [tmp_142[0] + tmp_143[0], tmp_142[1] + tmp_143[1], tmp_142[2] + tmp_143[2]];
    signal tmp_145[3] <== [16 * evals[27][0], 16 * evals[27][1], 16 * evals[27][2]];
    signal tmp_146[3] <== [tmp_144[0] + tmp_145[0], tmp_144[1] + tmp_145[1], tmp_144[2] + tmp_145[2]];
    signal tmp_147[3] <== [2 * evals[28][0], 2 * evals[28][1], 2 * evals[28][2]];
    signal tmp_148[3] <== [tmp_146[0] + tmp_147[0], tmp_146[1] + tmp_147[1], tmp_146[2] + tmp_147[2]];
    signal tmp_149[3] <== [28 * evals[29][0], 28 * evals[29][1], 28 * evals[29][2]];
    signal tmp_150[3] <== [tmp_148[0] + tmp_149[0], tmp_148[1] + tmp_149[1], tmp_148[2] + tmp_149[2]];
    signal tmp_151[3] <== [13 * evals[30][0], 13 * evals[30][1], 13 * evals[30][2]];
    signal tmp_152[3] <== [tmp_150[0] + tmp_151[0], tmp_150[1] + tmp_151[1], tmp_150[2] + tmp_151[2]];
    signal tmp_153[3] <== [13 * evals[31][0], 13 * evals[31][1], 13 * evals[31][2]];
    signal tmp_154[3] <== [tmp_152[0] + tmp_153[0], tmp_152[1] + tmp_153[1], tmp_152[2] + tmp_153[2]];
    signal tmp_155[3] <== [evals[37][0] - tmp_154[0], evals[37][1] - tmp_154[1], evals[37][2] - tmp_154[2]];
    signal tmp_156[3] <== CMul()(evals[33], tmp_155);
    signal tmp_929[3] <== [tmp_156[0] - 0, tmp_156[1], tmp_156[2]];
    signal tmp_157[3] <== [13 * tmp_924[0], 13 * tmp_924[1], 13 * tmp_924[2]];
    signal tmp_158[3] <== [39 * evals[21][0], 39 * evals[21][1], 39 * evals[21][2]];
    signal tmp_159[3] <== [tmp_157[0] + tmp_158[0], tmp_157[1] + tmp_158[1], tmp_157[2] + tmp_158[2]];
    signal tmp_160[3] <== [18 * evals[22][0], 18 * evals[22][1], 18 * evals[22][2]];
    signal tmp_161[3] <== [tmp_159[0] + tmp_160[0], tmp_159[1] + tmp_160[1], tmp_159[2] + tmp_160[2]];
    signal tmp_162[3] <== [34 * evals[23][0], 34 * evals[23][1], 34 * evals[23][2]];
    signal tmp_163[3] <== [tmp_161[0] + tmp_162[0], tmp_161[1] + tmp_162[1], tmp_161[2] + tmp_162[2]];
    signal tmp_164[3] <== [20 * evals[24][0], 20 * evals[24][1], 20 * evals[24][2]];
    signal tmp_165[3] <== [tmp_163[0] + tmp_164[0], tmp_163[1] + tmp_164[1], tmp_163[2] + tmp_164[2]];
    signal tmp_166[3] <== [17 * evals[25][0], 17 * evals[25][1], 17 * evals[25][2]];
    signal tmp_167[3] <== [tmp_165[0] + tmp_166[0], tmp_165[1] + tmp_166[1], tmp_165[2] + tmp_166[2]];
    signal tmp_168[3] <== [15 * evals[26][0], 15 * evals[26][1], 15 * evals[26][2]];
    signal tmp_169[3] <== [tmp_167[0] + tmp_168[0], tmp_167[1] + tmp_168[1], tmp_167[2] + tmp_168[2]];
    signal tmp_170[3] <== [41 * evals[27][0], 41 * evals[27][1], 41 * evals[27][2]];
    signal tmp_171[3] <== [tmp_169[0] + tmp_170[0], tmp_169[1] + tmp_170[1], tmp_169[2] + tmp_170[2]];
    signal tmp_172[3] <== [16 * evals[28][0], 16 * evals[28][1], 16 * evals[28][2]];
    signal tmp_173[3] <== [tmp_171[0] + tmp_172[0], tmp_171[1] + tmp_172[1], tmp_171[2] + tmp_172[2]];
    signal tmp_174[3] <== [2 * evals[29][0], 2 * evals[29][1], 2 * evals[29][2]];
    signal tmp_175[3] <== [tmp_173[0] + tmp_174[0], tmp_173[1] + tmp_174[1], tmp_173[2] + tmp_174[2]];
    signal tmp_176[3] <== [28 * evals[30][0], 28 * evals[30][1], 28 * evals[30][2]];
    signal tmp_177[3] <== [tmp_175[0] + tmp_176[0], tmp_175[1] + tmp_176[1], tmp_175[2] + tmp_176[2]];
    signal tmp_178[3] <== [13 * evals[31][0], 13 * evals[31][1], 13 * evals[31][2]];
    signal tmp_179[3] <== [tmp_177[0] + tmp_178[0], tmp_177[1] + tmp_178[1], tmp_177[2] + tmp_178[2]];
    signal tmp_180[3] <== [evals[38][0] - tmp_179[0], evals[38][1] - tmp_179[1], evals[38][2] - tmp_179[2]];
    signal tmp_181[3] <== CMul()(evals[33], tmp_180);
    signal tmp_930[3] <== [tmp_181[0] - 0, tmp_181[1], tmp_181[2]];
    signal tmp_182[3] <== [13 * tmp_924[0], 13 * tmp_924[1], 13 * tmp_924[2]];
    signal tmp_183[3] <== [13 * evals[21][0], 13 * evals[21][1], 13 * evals[21][2]];
    signal tmp_184[3] <== [tmp_182[0] + tmp_183[0], tmp_182[1] + tmp_183[1], tmp_182[2] + tmp_183[2]];
    signal tmp_185[3] <== [39 * evals[22][0], 39 * evals[22][1], 39 * evals[22][2]];
    signal tmp_186[3] <== [tmp_184[0] + tmp_185[0], tmp_184[1] + tmp_185[1], tmp_184[2] + tmp_185[2]];
    signal tmp_187[3] <== [18 * evals[23][0], 18 * evals[23][1], 18 * evals[23][2]];
    signal tmp_188[3] <== [tmp_186[0] + tmp_187[0], tmp_186[1] + tmp_187[1], tmp_186[2] + tmp_187[2]];
    signal tmp_189[3] <== [34 * evals[24][0], 34 * evals[24][1], 34 * evals[24][2]];
    signal tmp_190[3] <== [tmp_188[0] + tmp_189[0], tmp_188[1] + tmp_189[1], tmp_188[2] + tmp_189[2]];
    signal tmp_191[3] <== [20 * evals[25][0], 20 * evals[25][1], 20 * evals[25][2]];
    signal tmp_192[3] <== [tmp_190[0] + tmp_191[0], tmp_190[1] + tmp_191[1], tmp_190[2] + tmp_191[2]];
    signal tmp_193[3] <== [17 * evals[26][0], 17 * evals[26][1], 17 * evals[26][2]];
    signal tmp_194[3] <== [tmp_192[0] + tmp_193[0], tmp_192[1] + tmp_193[1], tmp_192[2] + tmp_193[2]];
    signal tmp_195[3] <== [15 * evals[27][0], 15 * evals[27][1], 15 * evals[27][2]];
    signal tmp_196[3] <== [tmp_194[0] + tmp_195[0], tmp_194[1] + tmp_195[1], tmp_194[2] + tmp_195[2]];
    signal tmp_197[3] <== [41 * evals[28][0], 41 * evals[28][1], 41 * evals[28][2]];
    signal tmp_198[3] <== [tmp_196[0] + tmp_197[0], tmp_196[1] + tmp_197[1], tmp_196[2] + tmp_197[2]];
    signal tmp_199[3] <== [16 * evals[29][0], 16 * evals[29][1], 16 * evals[29][2]];
    signal tmp_200[3] <== [tmp_198[0] + tmp_199[0], tmp_198[1] + tmp_199[1], tmp_198[2] + tmp_199[2]];
    signal tmp_201[3] <== [2 * evals[30][0], 2 * evals[30][1], 2 * evals[30][2]];
    signal tmp_202[3] <== [tmp_200[0] + tmp_201[0], tmp_200[1] + tmp_201[1], tmp_200[2] + tmp_201[2]];
    signal tmp_203[3] <== [28 * evals[31][0], 28 * evals[31][1], 28 * evals[31][2]];
    signal tmp_204[3] <== [tmp_202[0] + tmp_203[0], tmp_202[1] + tmp_203[1], tmp_202[2] + tmp_203[2]];
    signal tmp_205[3] <== [evals[39][0] - tmp_204[0], evals[39][1] - tmp_204[1], evals[39][2] - tmp_204[2]];
    signal tmp_206[3] <== CMul()(evals[33], tmp_205);
    signal tmp_931[3] <== [tmp_206[0] - 0, tmp_206[1], tmp_206[2]];
    signal tmp_207[3] <== [28 * tmp_924[0], 28 * tmp_924[1], 28 * tmp_924[2]];
    signal tmp_208[3] <== [13 * evals[21][0], 13 * evals[21][1], 13 * evals[21][2]];
    signal tmp_209[3] <== [tmp_207[0] + tmp_208[0], tmp_207[1] + tmp_208[1], tmp_207[2] + tmp_208[2]];
    signal tmp_210[3] <== [13 * evals[22][0], 13 * evals[22][1], 13 * evals[22][2]];
    signal tmp_211[3] <== [tmp_209[0] + tmp_210[0], tmp_209[1] + tmp_210[1], tmp_209[2] + tmp_210[2]];
    signal tmp_212[3] <== [39 * evals[23][0], 39 * evals[23][1], 39 * evals[23][2]];
    signal tmp_213[3] <== [tmp_211[0] + tmp_212[0], tmp_211[1] + tmp_212[1], tmp_211[2] + tmp_212[2]];
    signal tmp_214[3] <== [18 * evals[24][0], 18 * evals[24][1], 18 * evals[24][2]];
    signal tmp_215[3] <== [tmp_213[0] + tmp_214[0], tmp_213[1] + tmp_214[1], tmp_213[2] + tmp_214[2]];
    signal tmp_216[3] <== [34 * evals[25][0], 34 * evals[25][1], 34 * evals[25][2]];
    signal tmp_217[3] <== [tmp_215[0] + tmp_216[0], tmp_215[1] + tmp_216[1], tmp_215[2] + tmp_216[2]];
    signal tmp_218[3] <== [20 * evals[26][0], 20 * evals[26][1], 20 * evals[26][2]];
    signal tmp_219[3] <== [tmp_217[0] + tmp_218[0], tmp_217[1] + tmp_218[1], tmp_217[2] + tmp_218[2]];
    signal tmp_220[3] <== [17 * evals[27][0], 17 * evals[27][1], 17 * evals[27][2]];
    signal tmp_221[3] <== [tmp_219[0] + tmp_220[0], tmp_219[1] + tmp_220[1], tmp_219[2] + tmp_220[2]];
    signal tmp_222[3] <== [15 * evals[28][0], 15 * evals[28][1], 15 * evals[28][2]];
    signal tmp_223[3] <== [tmp_221[0] + tmp_222[0], tmp_221[1] + tmp_222[1], tmp_221[2] + tmp_222[2]];
    signal tmp_224[3] <== [41 * evals[29][0], 41 * evals[29][1], 41 * evals[29][2]];
    signal tmp_225[3] <== [tmp_223[0] + tmp_224[0], tmp_223[1] + tmp_224[1], tmp_223[2] + tmp_224[2]];
    signal tmp_226[3] <== [16 * evals[30][0], 16 * evals[30][1], 16 * evals[30][2]];
    signal tmp_227[3] <== [tmp_225[0] + tmp_226[0], tmp_225[1] + tmp_226[1], tmp_225[2] + tmp_226[2]];
    signal tmp_228[3] <== [2 * evals[31][0], 2 * evals[31][1], 2 * evals[31][2]];
    signal tmp_229[3] <== [tmp_227[0] + tmp_228[0], tmp_227[1] + tmp_228[1], tmp_227[2] + tmp_228[2]];
    signal tmp_230[3] <== [evals[40][0] - tmp_229[0], evals[40][1] - tmp_229[1], evals[40][2] - tmp_229[2]];
    signal tmp_231[3] <== CMul()(evals[33], tmp_230);
    signal tmp_932[3] <== [tmp_231[0] - 0, tmp_231[1], tmp_231[2]];
    signal tmp_232[3] <== [2 * tmp_924[0], 2 * tmp_924[1], 2 * tmp_924[2]];
    signal tmp_233[3] <== [28 * evals[21][0], 28 * evals[21][1], 28 * evals[21][2]];
    signal tmp_234[3] <== [tmp_232[0] + tmp_233[0], tmp_232[1] + tmp_233[1], tmp_232[2] + tmp_233[2]];
    signal tmp_235[3] <== [13 * evals[22][0], 13 * evals[22][1], 13 * evals[22][2]];
    signal tmp_236[3] <== [tmp_234[0] + tmp_235[0], tmp_234[1] + tmp_235[1], tmp_234[2] + tmp_235[2]];
    signal tmp_237[3] <== [13 * evals[23][0], 13 * evals[23][1], 13 * evals[23][2]];
    signal tmp_238[3] <== [tmp_236[0] + tmp_237[0], tmp_236[1] + tmp_237[1], tmp_236[2] + tmp_237[2]];
    signal tmp_239[3] <== [39 * evals[24][0], 39 * evals[24][1], 39 * evals[24][2]];
    signal tmp_240[3] <== [tmp_238[0] + tmp_239[0], tmp_238[1] + tmp_239[1], tmp_238[2] + tmp_239[2]];
    signal tmp_241[3] <== [18 * evals[25][0], 18 * evals[25][1], 18 * evals[25][2]];
    signal tmp_242[3] <== [tmp_240[0] + tmp_241[0], tmp_240[1] + tmp_241[1], tmp_240[2] + tmp_241[2]];
    signal tmp_243[3] <== [34 * evals[26][0], 34 * evals[26][1], 34 * evals[26][2]];
    signal tmp_244[3] <== [tmp_242[0] + tmp_243[0], tmp_242[1] + tmp_243[1], tmp_242[2] + tmp_243[2]];
    signal tmp_245[3] <== [20 * evals[27][0], 20 * evals[27][1], 20 * evals[27][2]];
    signal tmp_246[3] <== [tmp_244[0] + tmp_245[0], tmp_244[1] + tmp_245[1], tmp_244[2] + tmp_245[2]];
    signal tmp_247[3] <== [17 * evals[28][0], 17 * evals[28][1], 17 * evals[28][2]];
    signal tmp_248[3] <== [tmp_246[0] + tmp_247[0], tmp_246[1] + tmp_247[1], tmp_246[2] + tmp_247[2]];
    signal tmp_249[3] <== [15 * evals[29][0], 15 * evals[29][1], 15 * evals[29][2]];
    signal tmp_250[3] <== [tmp_248[0] + tmp_249[0], tmp_248[1] + tmp_249[1], tmp_248[2] + tmp_249[2]];
    signal tmp_251[3] <== [41 * evals[30][0], 41 * evals[30][1], 41 * evals[30][2]];
    signal tmp_252[3] <== [tmp_250[0] + tmp_251[0], tmp_250[1] + tmp_251[1], tmp_250[2] + tmp_251[2]];
    signal tmp_253[3] <== [16 * evals[31][0], 16 * evals[31][1], 16 * evals[31][2]];
    signal tmp_254[3] <== [tmp_252[0] + tmp_253[0], tmp_252[1] + tmp_253[1], tmp_252[2] + tmp_253[2]];
    signal tmp_255[3] <== [evals[41][0] - tmp_254[0], evals[41][1] - tmp_254[1], evals[41][2] - tmp_254[2]];
    signal tmp_256[3] <== CMul()(evals[33], tmp_255);
    signal tmp_933[3] <== [tmp_256[0] - 0, tmp_256[1], tmp_256[2]];
    signal tmp_257[3] <== [16 * tmp_924[0], 16 * tmp_924[1], 16 * tmp_924[2]];
    signal tmp_258[3] <== [2 * evals[21][0], 2 * evals[21][1], 2 * evals[21][2]];
    signal tmp_259[3] <== [tmp_257[0] + tmp_258[0], tmp_257[1] + tmp_258[1], tmp_257[2] + tmp_258[2]];
    signal tmp_260[3] <== [28 * evals[22][0], 28 * evals[22][1], 28 * evals[22][2]];
    signal tmp_261[3] <== [tmp_259[0] + tmp_260[0], tmp_259[1] + tmp_260[1], tmp_259[2] + tmp_260[2]];
    signal tmp_262[3] <== [13 * evals[23][0], 13 * evals[23][1], 13 * evals[23][2]];
    signal tmp_263[3] <== [tmp_261[0] + tmp_262[0], tmp_261[1] + tmp_262[1], tmp_261[2] + tmp_262[2]];
    signal tmp_264[3] <== [13 * evals[24][0], 13 * evals[24][1], 13 * evals[24][2]];
    signal tmp_265[3] <== [tmp_263[0] + tmp_264[0], tmp_263[1] + tmp_264[1], tmp_263[2] + tmp_264[2]];
    signal tmp_266[3] <== [39 * evals[25][0], 39 * evals[25][1], 39 * evals[25][2]];
    signal tmp_267[3] <== [tmp_265[0] + tmp_266[0], tmp_265[1] + tmp_266[1], tmp_265[2] + tmp_266[2]];
    signal tmp_268[3] <== [18 * evals[26][0], 18 * evals[26][1], 18 * evals[26][2]];
    signal tmp_269[3] <== [tmp_267[0] + tmp_268[0], tmp_267[1] + tmp_268[1], tmp_267[2] + tmp_268[2]];
    signal tmp_270[3] <== [34 * evals[27][0], 34 * evals[27][1], 34 * evals[27][2]];
    signal tmp_271[3] <== [tmp_269[0] + tmp_270[0], tmp_269[1] + tmp_270[1], tmp_269[2] + tmp_270[2]];
    signal tmp_272[3] <== [20 * evals[28][0], 20 * evals[28][1], 20 * evals[28][2]];
    signal tmp_273[3] <== [tmp_271[0] + tmp_272[0], tmp_271[1] + tmp_272[1], tmp_271[2] + tmp_272[2]];
    signal tmp_274[3] <== [17 * evals[29][0], 17 * evals[29][1], 17 * evals[29][2]];
    signal tmp_275[3] <== [tmp_273[0] + tmp_274[0], tmp_273[1] + tmp_274[1], tmp_273[2] + tmp_274[2]];
    signal tmp_276[3] <== [15 * evals[30][0], 15 * evals[30][1], 15 * evals[30][2]];
    signal tmp_277[3] <== [tmp_275[0] + tmp_276[0], tmp_275[1] + tmp_276[1], tmp_275[2] + tmp_276[2]];
    signal tmp_278[3] <== [41 * evals[31][0], 41 * evals[31][1], 41 * evals[31][2]];
    signal tmp_279[3] <== [tmp_277[0] + tmp_278[0], tmp_277[1] + tmp_278[1], tmp_277[2] + tmp_278[2]];
    signal tmp_280[3] <== [evals[42][0] - tmp_279[0], evals[42][1] - tmp_279[1], evals[42][2] - tmp_279[2]];
    signal tmp_281[3] <== CMul()(evals[33], tmp_280);
    signal tmp_934[3] <== [tmp_281[0] - 0, tmp_281[1], tmp_281[2]];
    signal tmp_282[3] <== [41 * tmp_924[0], 41 * tmp_924[1], 41 * tmp_924[2]];
    signal tmp_283[3] <== [16 * evals[21][0], 16 * evals[21][1], 16 * evals[21][2]];
    signal tmp_284[3] <== [tmp_282[0] + tmp_283[0], tmp_282[1] + tmp_283[1], tmp_282[2] + tmp_283[2]];
    signal tmp_285[3] <== [2 * evals[22][0], 2 * evals[22][1], 2 * evals[22][2]];
    signal tmp_286[3] <== [tmp_284[0] + tmp_285[0], tmp_284[1] + tmp_285[1], tmp_284[2] + tmp_285[2]];
    signal tmp_287[3] <== [28 * evals[23][0], 28 * evals[23][1], 28 * evals[23][2]];
    signal tmp_288[3] <== [tmp_286[0] + tmp_287[0], tmp_286[1] + tmp_287[1], tmp_286[2] + tmp_287[2]];
    signal tmp_289[3] <== [13 * evals[24][0], 13 * evals[24][1], 13 * evals[24][2]];
    signal tmp_290[3] <== [tmp_288[0] + tmp_289[0], tmp_288[1] + tmp_289[1], tmp_288[2] + tmp_289[2]];
    signal tmp_291[3] <== [13 * evals[25][0], 13 * evals[25][1], 13 * evals[25][2]];
    signal tmp_292[3] <== [tmp_290[0] + tmp_291[0], tmp_290[1] + tmp_291[1], tmp_290[2] + tmp_291[2]];
    signal tmp_293[3] <== [39 * evals[26][0], 39 * evals[26][1], 39 * evals[26][2]];
    signal tmp_294[3] <== [tmp_292[0] + tmp_293[0], tmp_292[1] + tmp_293[1], tmp_292[2] + tmp_293[2]];
    signal tmp_295[3] <== [18 * evals[27][0], 18 * evals[27][1], 18 * evals[27][2]];
    signal tmp_296[3] <== [tmp_294[0] + tmp_295[0], tmp_294[1] + tmp_295[1], tmp_294[2] + tmp_295[2]];
    signal tmp_297[3] <== [34 * evals[28][0], 34 * evals[28][1], 34 * evals[28][2]];
    signal tmp_298[3] <== [tmp_296[0] + tmp_297[0], tmp_296[1] + tmp_297[1], tmp_296[2] + tmp_297[2]];
    signal tmp_299[3] <== [20 * evals[29][0], 20 * evals[29][1], 20 * evals[29][2]];
    signal tmp_300[3] <== [tmp_298[0] + tmp_299[0], tmp_298[1] + tmp_299[1], tmp_298[2] + tmp_299[2]];
    signal tmp_301[3] <== [17 * evals[30][0], 17 * evals[30][1], 17 * evals[30][2]];
    signal tmp_302[3] <== [tmp_300[0] + tmp_301[0], tmp_300[1] + tmp_301[1], tmp_300[2] + tmp_301[2]];
    signal tmp_303[3] <== [15 * evals[31][0], 15 * evals[31][1], 15 * evals[31][2]];
    signal tmp_304[3] <== [tmp_302[0] + tmp_303[0], tmp_302[1] + tmp_303[1], tmp_302[2] + tmp_303[2]];
    signal tmp_305[3] <== [evals[43][0] - tmp_304[0], evals[43][1] - tmp_304[1], evals[43][2] - tmp_304[2]];
    signal tmp_306[3] <== CMul()(evals[33], tmp_305);
    signal tmp_935[3] <== [tmp_306[0] - 0, tmp_306[1], tmp_306[2]];
    signal tmp_307[3] <== [15 * tmp_924[0], 15 * tmp_924[1], 15 * tmp_924[2]];
    signal tmp_308[3] <== [41 * evals[21][0], 41 * evals[21][1], 41 * evals[21][2]];
    signal tmp_309[3] <== [tmp_307[0] + tmp_308[0], tmp_307[1] + tmp_308[1], tmp_307[2] + tmp_308[2]];
    signal tmp_310[3] <== [16 * evals[22][0], 16 * evals[22][1], 16 * evals[22][2]];
    signal tmp_311[3] <== [tmp_309[0] + tmp_310[0], tmp_309[1] + tmp_310[1], tmp_309[2] + tmp_310[2]];
    signal tmp_312[3] <== [2 * evals[23][0], 2 * evals[23][1], 2 * evals[23][2]];
    signal tmp_313[3] <== [tmp_311[0] + tmp_312[0], tmp_311[1] + tmp_312[1], tmp_311[2] + tmp_312[2]];
    signal tmp_314[3] <== [28 * evals[24][0], 28 * evals[24][1], 28 * evals[24][2]];
    signal tmp_315[3] <== [tmp_313[0] + tmp_314[0], tmp_313[1] + tmp_314[1], tmp_313[2] + tmp_314[2]];
    signal tmp_316[3] <== [13 * evals[25][0], 13 * evals[25][1], 13 * evals[25][2]];
    signal tmp_317[3] <== [tmp_315[0] + tmp_316[0], tmp_315[1] + tmp_316[1], tmp_315[2] + tmp_316[2]];
    signal tmp_318[3] <== [13 * evals[26][0], 13 * evals[26][1], 13 * evals[26][2]];
    signal tmp_319[3] <== [tmp_317[0] + tmp_318[0], tmp_317[1] + tmp_318[1], tmp_317[2] + tmp_318[2]];
    signal tmp_320[3] <== [39 * evals[27][0], 39 * evals[27][1], 39 * evals[27][2]];
    signal tmp_321[3] <== [tmp_319[0] + tmp_320[0], tmp_319[1] + tmp_320[1], tmp_319[2] + tmp_320[2]];
    signal tmp_322[3] <== [18 * evals[28][0], 18 * evals[28][1], 18 * evals[28][2]];
    signal tmp_323[3] <== [tmp_321[0] + tmp_322[0], tmp_321[1] + tmp_322[1], tmp_321[2] + tmp_322[2]];
    signal tmp_324[3] <== [34 * evals[29][0], 34 * evals[29][1], 34 * evals[29][2]];
    signal tmp_325[3] <== [tmp_323[0] + tmp_324[0], tmp_323[1] + tmp_324[1], tmp_323[2] + tmp_324[2]];
    signal tmp_326[3] <== [20 * evals[30][0], 20 * evals[30][1], 20 * evals[30][2]];
    signal tmp_327[3] <== [tmp_325[0] + tmp_326[0], tmp_325[1] + tmp_326[1], tmp_325[2] + tmp_326[2]];
    signal tmp_328[3] <== [17 * evals[31][0], 17 * evals[31][1], 17 * evals[31][2]];
    signal tmp_329[3] <== [tmp_327[0] + tmp_328[0], tmp_327[1] + tmp_328[1], tmp_327[2] + tmp_328[2]];
    signal tmp_330[3] <== [evals[44][0] - tmp_329[0], evals[44][1] - tmp_329[1], evals[44][2] - tmp_329[2]];
    signal tmp_331[3] <== CMul()(evals[33], tmp_330);
    signal tmp_936[3] <== [tmp_331[0] - 0, tmp_331[1], tmp_331[2]];
    signal tmp_937[3] <== evals[10];
    signal tmp_332[3] <== [evals[7][0] + evals[45][0], evals[7][1] + evals[45][1], evals[7][2] + evals[45][2]];
    signal tmp_938[3] <== CMul()(tmp_332, evals[46]);
    signal tmp_333[3] <== [evals[47][0] + evals[48][0], evals[47][1] + evals[48][1], evals[47][2] + evals[48][2]];
    signal tmp_334[3] <== [tmp_333[0] - evals[49][0], tmp_333[1] - evals[49][1], tmp_333[2] - evals[49][2]];
    signal tmp_335[3] <== [tmp_334[0] - evals[50][0], tmp_334[1] - evals[50][1], tmp_334[2] - evals[50][2]];
    signal tmp_336[3] <== [tmp_937[0] - tmp_335[0], tmp_937[1] - tmp_335[1], tmp_937[2] - tmp_335[2]];
    signal tmp_337[3] <== [tmp_336[0] - tmp_938[0], tmp_336[1] - tmp_938[1], tmp_336[2] - tmp_938[2]];
    signal tmp_338[3] <== CMul()(evals[51], tmp_337);
    signal tmp_939[3] <== [tmp_338[0] - 0, tmp_338[1], tmp_338[2]];
    signal tmp_940[3] <== evals[11];
    signal tmp_339[3] <== [evals[8][0] + evals[52][0], evals[8][1] + evals[52][1], evals[8][2] + evals[52][2]];
    signal tmp_941[3] <== CMul()(tmp_339, evals[46]);
    signal tmp_340[3] <== [evals[53][0] + evals[47][0], evals[53][1] + evals[47][1], evals[53][2] + evals[47][2]];
    signal tmp_341[3] <== [2 * evals[49][0], 2 * evals[49][1], 2 * evals[49][2]];
    signal tmp_342[3] <== [tmp_340[0] - tmp_341[0], tmp_340[1] - tmp_341[1], tmp_340[2] - tmp_341[2]];
    signal tmp_343[3] <== [tmp_342[0] - evals[48][0], tmp_342[1] - evals[48][1], tmp_342[2] - evals[48][2]];
    signal tmp_344[3] <== [tmp_940[0] - tmp_343[0], tmp_940[1] - tmp_343[1], tmp_940[2] - tmp_343[2]];
    signal tmp_345[3] <== [tmp_344[0] - tmp_941[0], tmp_344[1] - tmp_941[1], tmp_344[2] - tmp_941[2]];
    signal tmp_346[3] <== CMul()(evals[51], tmp_345);
    signal tmp_942[3] <== [tmp_346[0] - 0, tmp_346[1], tmp_346[2]];
    signal tmp_943[3] <== evals[12];
    signal tmp_347[3] <== [evals[9][0] + evals[54][0], evals[9][1] + evals[54][1], evals[9][2] + evals[54][2]];
    signal tmp_944[3] <== CMul()(tmp_347, evals[46]);
    signal tmp_348[3] <== [evals[55][0] - evals[48][0], evals[55][1] - evals[48][1], evals[55][2] - evals[48][2]];
    signal tmp_349[3] <== [tmp_348[0] + evals[49][0], tmp_348[1] + evals[49][1], tmp_348[2] + evals[49][2]];
    signal tmp_350[3] <== [tmp_943[0] - tmp_349[0], tmp_943[1] - tmp_349[1], tmp_943[2] - tmp_349[2]];
    signal tmp_351[3] <== [tmp_350[0] - tmp_944[0], tmp_350[1] - tmp_944[1], tmp_350[2] - tmp_944[2]];
    signal tmp_352[3] <== CMul()(evals[51], tmp_351);
    signal tmp_945[3] <== [tmp_352[0] - 0, tmp_352[1], tmp_352[2]];
    signal tmp_353[3] <== CMul()(evals[19], evals[0]);
    signal tmp_354[3] <== CMul()(evals[56], evals[4]);
    signal tmp_355[3] <== [tmp_353[0] + tmp_354[0], tmp_353[1] + tmp_354[1], tmp_353[2] + tmp_354[2]];
    signal tmp_356[3] <== CMul()(evals[57], evals[7]);
    signal tmp_357[3] <== [tmp_355[0] + tmp_356[0], tmp_355[1] + tmp_356[1], tmp_355[2] + tmp_356[2]];
    signal tmp_358[3] <== CMul()(evals[58], evals[10]);
    signal tmp_359[3] <== [tmp_357[0] + tmp_358[0], tmp_357[1] + tmp_358[1], tmp_357[2] + tmp_358[2]];
    signal tmp_360[3] <== CMul()(evals[45], evals[0]);
    signal tmp_361[3] <== [tmp_359[0] + tmp_360[0], tmp_359[1] + tmp_360[1], tmp_359[2] + tmp_360[2]];
    signal tmp_362[3] <== CMul()(evals[52], evals[4]);
    signal tmp_946[3] <== [tmp_361[0] + tmp_362[0], tmp_361[1] + tmp_362[1], tmp_361[2] + tmp_362[2]];
    signal tmp_363[3] <== [evals[32][0] - tmp_946[0], evals[32][1] - tmp_946[1], evals[32][2] - tmp_946[2]];
    signal tmp_364[3] <== CMul()(evals[59], tmp_363);
    signal tmp_947[3] <== [tmp_364[0] - 0, tmp_364[1], tmp_364[2]];
    signal tmp_365[3] <== CMul()(evals[19], evals[2]);
    signal tmp_366[3] <== CMul()(evals[56], evals[5]);
    signal tmp_367[3] <== [tmp_365[0] + tmp_366[0], tmp_365[1] + tmp_366[1], tmp_365[2] + tmp_366[2]];
    signal tmp_368[3] <== CMul()(evals[57], evals[8]);
    signal tmp_369[3] <== [tmp_367[0] + tmp_368[0], tmp_367[1] + tmp_368[1], tmp_367[2] + tmp_368[2]];
    signal tmp_370[3] <== CMul()(evals[58], evals[11]);
    signal tmp_371[3] <== [tmp_369[0] + tmp_370[0], tmp_369[1] + tmp_370[1], tmp_369[2] + tmp_370[2]];
    signal tmp_372[3] <== CMul()(evals[45], evals[2]);
    signal tmp_373[3] <== [tmp_371[0] + tmp_372[0], tmp_371[1] + tmp_372[1], tmp_371[2] + tmp_372[2]];
    signal tmp_374[3] <== CMul()(evals[52], evals[5]);
    signal tmp_948[3] <== [tmp_373[0] + tmp_374[0], tmp_373[1] + tmp_374[1], tmp_373[2] + tmp_374[2]];
    signal tmp_375[3] <== [evals[34][0] - tmp_948[0], evals[34][1] - tmp_948[1], evals[34][2] - tmp_948[2]];
    signal tmp_376[3] <== CMul()(evals[59], tmp_375);
    signal tmp_949[3] <== [tmp_376[0] - 0, tmp_376[1], tmp_376[2]];
    signal tmp_377[3] <== CMul()(evals[19], evals[3]);
    signal tmp_378[3] <== CMul()(evals[56], evals[6]);
    signal tmp_379[3] <== [tmp_377[0] + tmp_378[0], tmp_377[1] + tmp_378[1], tmp_377[2] + tmp_378[2]];
    signal tmp_380[3] <== CMul()(evals[57], evals[9]);
    signal tmp_381[3] <== [tmp_379[0] + tmp_380[0], tmp_379[1] + tmp_380[1], tmp_379[2] + tmp_380[2]];
    signal tmp_382[3] <== CMul()(evals[58], evals[12]);
    signal tmp_383[3] <== [tmp_381[0] + tmp_382[0], tmp_381[1] + tmp_382[1], tmp_381[2] + tmp_382[2]];
    signal tmp_384[3] <== CMul()(evals[45], evals[3]);
    signal tmp_385[3] <== [tmp_383[0] + tmp_384[0], tmp_383[1] + tmp_384[1], tmp_383[2] + tmp_384[2]];
    signal tmp_386[3] <== CMul()(evals[52], evals[6]);
    signal tmp_950[3] <== [tmp_385[0] + tmp_386[0], tmp_385[1] + tmp_386[1], tmp_385[2] + tmp_386[2]];
    signal tmp_387[3] <== [evals[35][0] - tmp_950[0], evals[35][1] - tmp_950[1], evals[35][2] - tmp_950[2]];
    signal tmp_388[3] <== CMul()(evals[59], tmp_387);
    signal tmp_951[3] <== [tmp_388[0] - 0, tmp_388[1], tmp_388[2]];
    signal tmp_389[3] <== CMul()(evals[19], evals[0]);
    signal tmp_390[3] <== CMul()(evals[56], evals[4]);
    signal tmp_391[3] <== [tmp_389[0] - tmp_390[0], tmp_389[1] - tmp_390[1], tmp_389[2] - tmp_390[2]];
    signal tmp_392[3] <== CMul()(evals[60], evals[7]);
    signal tmp_393[3] <== [tmp_391[0] + tmp_392[0], tmp_391[1] + tmp_392[1], tmp_391[2] + tmp_392[2]];
    signal tmp_394[3] <== CMul()(evals[61], evals[10]);
    signal tmp_395[3] <== [tmp_393[0] - tmp_394[0], tmp_393[1] - tmp_394[1], tmp_393[2] - tmp_394[2]];
    signal tmp_396[3] <== CMul()(evals[45], evals[0]);
    signal tmp_397[3] <== [tmp_395[0] + tmp_396[0], tmp_395[1] + tmp_396[1], tmp_395[2] + tmp_396[2]];
    signal tmp_398[3] <== CMul()(evals[52], evals[4]);
    signal tmp_952[3] <== [tmp_397[0] - tmp_398[0], tmp_397[1] - tmp_398[1], tmp_397[2] - tmp_398[2]];
    signal tmp_399[3] <== [evals[36][0] - tmp_952[0], evals[36][1] - tmp_952[1], evals[36][2] - tmp_952[2]];
    signal tmp_400[3] <== CMul()(evals[59], tmp_399);
    signal tmp_953[3] <== [tmp_400[0] - 0, tmp_400[1], tmp_400[2]];
    signal tmp_401[3] <== CMul()(evals[19], evals[2]);
    signal tmp_402[3] <== CMul()(evals[56], evals[5]);
    signal tmp_403[3] <== [tmp_401[0] - tmp_402[0], tmp_401[1] - tmp_402[1], tmp_401[2] - tmp_402[2]];
    signal tmp_404[3] <== CMul()(evals[60], evals[8]);
    signal tmp_405[3] <== [tmp_403[0] + tmp_404[0], tmp_403[1] + tmp_404[1], tmp_403[2] + tmp_404[2]];
    signal tmp_406[3] <== CMul()(evals[61], evals[11]);
    signal tmp_407[3] <== [tmp_405[0] - tmp_406[0], tmp_405[1] - tmp_406[1], tmp_405[2] - tmp_406[2]];
    signal tmp_408[3] <== CMul()(evals[45], evals[2]);
    signal tmp_409[3] <== [tmp_407[0] + tmp_408[0], tmp_407[1] + tmp_408[1], tmp_407[2] + tmp_408[2]];
    signal tmp_410[3] <== CMul()(evals[52], evals[5]);
    signal tmp_954[3] <== [tmp_409[0] - tmp_410[0], tmp_409[1] - tmp_410[1], tmp_409[2] - tmp_410[2]];
    signal tmp_411[3] <== [evals[37][0] - tmp_954[0], evals[37][1] - tmp_954[1], evals[37][2] - tmp_954[2]];
    signal tmp_412[3] <== CMul()(evals[59], tmp_411);
    signal tmp_955[3] <== [tmp_412[0] - 0, tmp_412[1], tmp_412[2]];
    signal tmp_413[3] <== CMul()(evals[19], evals[3]);
    signal tmp_414[3] <== CMul()(evals[56], evals[6]);
    signal tmp_415[3] <== [tmp_413[0] - tmp_414[0], tmp_413[1] - tmp_414[1], tmp_413[2] - tmp_414[2]];
    signal tmp_416[3] <== CMul()(evals[60], evals[9]);
    signal tmp_417[3] <== [tmp_415[0] + tmp_416[0], tmp_415[1] + tmp_416[1], tmp_415[2] + tmp_416[2]];
    signal tmp_418[3] <== CMul()(evals[61], evals[12]);
    signal tmp_419[3] <== [tmp_417[0] - tmp_418[0], tmp_417[1] - tmp_418[1], tmp_417[2] - tmp_418[2]];
    signal tmp_420[3] <== CMul()(evals[45], evals[3]);
    signal tmp_421[3] <== [tmp_419[0] + tmp_420[0], tmp_419[1] + tmp_420[1], tmp_419[2] + tmp_420[2]];
    signal tmp_422[3] <== CMul()(evals[52], evals[6]);
    signal tmp_956[3] <== [tmp_421[0] - tmp_422[0], tmp_421[1] - tmp_422[1], tmp_421[2] - tmp_422[2]];
    signal tmp_423[3] <== [evals[38][0] - tmp_956[0], evals[38][1] - tmp_956[1], evals[38][2] - tmp_956[2]];
    signal tmp_424[3] <== CMul()(evals[59], tmp_423);
    signal tmp_957[3] <== [tmp_424[0] - 0, tmp_424[1], tmp_424[2]];
    signal tmp_425[3] <== CMul()(evals[19], evals[0]);
    signal tmp_426[3] <== CMul()(evals[56], evals[4]);
    signal tmp_427[3] <== [tmp_425[0] + tmp_426[0], tmp_425[1] + tmp_426[1], tmp_425[2] + tmp_426[2]];
    signal tmp_428[3] <== CMul()(evals[57], evals[7]);
    signal tmp_429[3] <== [tmp_427[0] - tmp_428[0], tmp_427[1] - tmp_428[1], tmp_427[2] - tmp_428[2]];
    signal tmp_430[3] <== CMul()(evals[58], evals[10]);
    signal tmp_431[3] <== [tmp_429[0] - tmp_430[0], tmp_429[1] - tmp_430[1], tmp_429[2] - tmp_430[2]];
    signal tmp_432[3] <== CMul()(evals[45], evals[7]);
    signal tmp_433[3] <== [tmp_431[0] + tmp_432[0], tmp_431[1] + tmp_432[1], tmp_431[2] + tmp_432[2]];
    signal tmp_434[3] <== CMul()(evals[54], evals[10]);
    signal tmp_958[3] <== [tmp_433[0] + tmp_434[0], tmp_433[1] + tmp_434[1], tmp_433[2] + tmp_434[2]];
    signal tmp_435[3] <== [evals[39][0] - tmp_958[0], evals[39][1] - tmp_958[1], evals[39][2] - tmp_958[2]];
    signal tmp_436[3] <== CMul()(evals[59], tmp_435);
    signal tmp_959[3] <== [tmp_436[0] - 0, tmp_436[1], tmp_436[2]];
    signal tmp_437[3] <== CMul()(evals[19], evals[2]);
    signal tmp_438[3] <== CMul()(evals[56], evals[5]);
    signal tmp_439[3] <== [tmp_437[0] + tmp_438[0], tmp_437[1] + tmp_438[1], tmp_437[2] + tmp_438[2]];
    signal tmp_440[3] <== CMul()(evals[57], evals[8]);
    signal tmp_441[3] <== [tmp_439[0] - tmp_440[0], tmp_439[1] - tmp_440[1], tmp_439[2] - tmp_440[2]];
    signal tmp_442[3] <== CMul()(evals[58], evals[11]);
    signal tmp_443[3] <== [tmp_441[0] - tmp_442[0], tmp_441[1] - tmp_442[1], tmp_441[2] - tmp_442[2]];
    signal tmp_444[3] <== CMul()(evals[45], evals[8]);
    signal tmp_445[3] <== [tmp_443[0] + tmp_444[0], tmp_443[1] + tmp_444[1], tmp_443[2] + tmp_444[2]];
    signal tmp_446[3] <== CMul()(evals[54], evals[11]);
    signal tmp_960[3] <== [tmp_445[0] + tmp_446[0], tmp_445[1] + tmp_446[1], tmp_445[2] + tmp_446[2]];
    signal tmp_447[3] <== [evals[40][0] - tmp_960[0], evals[40][1] - tmp_960[1], evals[40][2] - tmp_960[2]];
    signal tmp_448[3] <== CMul()(evals[59], tmp_447);
    signal tmp_961[3] <== [tmp_448[0] - 0, tmp_448[1], tmp_448[2]];
    signal tmp_449[3] <== CMul()(evals[19], evals[3]);
    signal tmp_450[3] <== CMul()(evals[56], evals[6]);
    signal tmp_451[3] <== [tmp_449[0] + tmp_450[0], tmp_449[1] + tmp_450[1], tmp_449[2] + tmp_450[2]];
    signal tmp_452[3] <== CMul()(evals[57], evals[9]);
    signal tmp_453[3] <== [tmp_451[0] - tmp_452[0], tmp_451[1] - tmp_452[1], tmp_451[2] - tmp_452[2]];
    signal tmp_454[3] <== CMul()(evals[58], evals[12]);
    signal tmp_455[3] <== [tmp_453[0] - tmp_454[0], tmp_453[1] - tmp_454[1], tmp_453[2] - tmp_454[2]];
    signal tmp_456[3] <== CMul()(evals[45], evals[9]);
    signal tmp_457[3] <== [tmp_455[0] + tmp_456[0], tmp_455[1] + tmp_456[1], tmp_455[2] + tmp_456[2]];
    signal tmp_458[3] <== CMul()(evals[54], evals[12]);
    signal tmp_962[3] <== [tmp_457[0] + tmp_458[0], tmp_457[1] + tmp_458[1], tmp_457[2] + tmp_458[2]];
    signal tmp_459[3] <== [evals[41][0] - tmp_962[0], evals[41][1] - tmp_962[1], evals[41][2] - tmp_962[2]];
    signal tmp_460[3] <== CMul()(evals[59], tmp_459);
    signal tmp_963[3] <== [tmp_460[0] - 0, tmp_460[1], tmp_460[2]];
    signal tmp_461[3] <== CMul()(evals[19], evals[0]);
    signal tmp_462[3] <== CMul()(evals[56], evals[4]);
    signal tmp_463[3] <== [tmp_461[0] - tmp_462[0], tmp_461[1] - tmp_462[1], tmp_461[2] - tmp_462[2]];
    signal tmp_464[3] <== CMul()(evals[60], evals[7]);
    signal tmp_465[3] <== [tmp_463[0] - tmp_464[0], tmp_463[1] - tmp_464[1], tmp_463[2] - tmp_464[2]];
    signal tmp_466[3] <== CMul()(evals[61], evals[10]);
    signal tmp_467[3] <== [tmp_465[0] + tmp_466[0], tmp_465[1] + tmp_466[1], tmp_465[2] + tmp_466[2]];
    signal tmp_468[3] <== CMul()(evals[45], evals[7]);
    signal tmp_469[3] <== [tmp_467[0] + tmp_468[0], tmp_467[1] + tmp_468[1], tmp_467[2] + tmp_468[2]];
    signal tmp_470[3] <== CMul()(evals[54], evals[10]);
    signal tmp_964[3] <== [tmp_469[0] - tmp_470[0], tmp_469[1] - tmp_470[1], tmp_469[2] - tmp_470[2]];
    signal tmp_471[3] <== [evals[42][0] - tmp_964[0], evals[42][1] - tmp_964[1], evals[42][2] - tmp_964[2]];
    signal tmp_472[3] <== CMul()(evals[59], tmp_471);
    signal tmp_965[3] <== [tmp_472[0] - 0, tmp_472[1], tmp_472[2]];
    signal tmp_473[3] <== CMul()(evals[19], evals[2]);
    signal tmp_474[3] <== CMul()(evals[56], evals[5]);
    signal tmp_475[3] <== [tmp_473[0] - tmp_474[0], tmp_473[1] - tmp_474[1], tmp_473[2] - tmp_474[2]];
    signal tmp_476[3] <== CMul()(evals[60], evals[8]);
    signal tmp_477[3] <== [tmp_475[0] - tmp_476[0], tmp_475[1] - tmp_476[1], tmp_475[2] - tmp_476[2]];
    signal tmp_478[3] <== CMul()(evals[61], evals[11]);
    signal tmp_479[3] <== [tmp_477[0] + tmp_478[0], tmp_477[1] + tmp_478[1], tmp_477[2] + tmp_478[2]];
    signal tmp_480[3] <== CMul()(evals[45], evals[8]);
    signal tmp_481[3] <== [tmp_479[0] + tmp_480[0], tmp_479[1] + tmp_480[1], tmp_479[2] + tmp_480[2]];
    signal tmp_482[3] <== CMul()(evals[54], evals[11]);
    signal tmp_966[3] <== [tmp_481[0] - tmp_482[0], tmp_481[1] - tmp_482[1], tmp_481[2] - tmp_482[2]];
    signal tmp_483[3] <== [evals[43][0] - tmp_966[0], evals[43][1] - tmp_966[1], evals[43][2] - tmp_966[2]];
    signal tmp_484[3] <== CMul()(evals[59], tmp_483);
    signal tmp_967[3] <== [tmp_484[0] - 0, tmp_484[1], tmp_484[2]];
    signal tmp_485[3] <== CMul()(evals[19], evals[3]);
    signal tmp_486[3] <== CMul()(evals[56], evals[6]);
    signal tmp_487[3] <== [tmp_485[0] - tmp_486[0], tmp_485[1] - tmp_486[1], tmp_485[2] - tmp_486[2]];
    signal tmp_488[3] <== CMul()(evals[60], evals[9]);
    signal tmp_489[3] <== [tmp_487[0] - tmp_488[0], tmp_487[1] - tmp_488[1], tmp_487[2] - tmp_488[2]];
    signal tmp_490[3] <== CMul()(evals[61], evals[12]);
    signal tmp_491[3] <== [tmp_489[0] + tmp_490[0], tmp_489[1] + tmp_490[1], tmp_489[2] + tmp_490[2]];
    signal tmp_492[3] <== CMul()(evals[45], evals[9]);
    signal tmp_493[3] <== [tmp_491[0] + tmp_492[0], tmp_491[1] + tmp_492[1], tmp_491[2] + tmp_492[2]];
    signal tmp_494[3] <== CMul()(evals[54], evals[12]);
    signal tmp_968[3] <== [tmp_493[0] - tmp_494[0], tmp_493[1] - tmp_494[1], tmp_493[2] - tmp_494[2]];
    signal tmp_495[3] <== [evals[44][0] - tmp_968[0], evals[44][1] - tmp_968[1], evals[44][2] - tmp_968[2]];
    signal tmp_496[3] <== CMul()(evals[59], tmp_495);
    signal tmp_969[3] <== [tmp_496[0] - 0, tmp_496[1], tmp_496[2]];
    signal tmp_497[3] <== [evals[39][0] - evals[62][0], evals[39][1] - evals[62][1], evals[39][2] - evals[62][2]];
    signal tmp_498[3] <== CMul()(evals[63], tmp_497);
    signal tmp_970[3] <== [tmp_498[0] - 0, tmp_498[1], tmp_498[2]];
    signal tmp_499[3] <== [evals[40][0] - evals[64][0], evals[40][1] - evals[64][1], evals[40][2] - evals[64][2]];
    signal tmp_500[3] <== CMul()(evals[63], tmp_499);
    signal tmp_971[3] <== [tmp_500[0] - 0, tmp_500[1], tmp_500[2]];
    signal tmp_501[3] <== [evals[41][0] - evals[65][0], evals[41][1] - evals[65][1], evals[41][2] - evals[65][2]];
    signal tmp_502[3] <== CMul()(evals[63], tmp_501);
    signal tmp_972[3] <== [tmp_502[0] - 0, tmp_502[1], tmp_502[2]];
    signal tmp_503[3] <== [evals[66][0] - 1, evals[66][1], evals[66][2]];
    signal tmp_973[3] <== CMul()(evals[1], tmp_503);
    signal tmp_974[3] <== evals[12];
    signal tmp_975[3] <== evals[67];
    signal tmp_504[3] <== CMul()(challenges[3], tmp_975);
    signal tmp_505[3] <== [tmp_974[0] + tmp_504[0], tmp_974[1] + tmp_504[1], tmp_974[2] + tmp_504[2]];
    signal tmp_506[3] <== [tmp_505[0] + challenges[2][0], tmp_505[1] + challenges[2][1], tmp_505[2] + challenges[2][2]];
    signal tmp_976[3] <== CMul()(evals[68], tmp_506);
    signal tmp_507[3] <== [challenges[3][0] * 12756200801261202346, challenges[3][1] * 12756200801261202346, challenges[3][2] * 12756200801261202346];
    signal tmp_508[3] <== CMul()(tmp_507, challenges[7]);
    signal tmp_509[3] <== [tmp_974[0] + tmp_508[0], tmp_974[1] + tmp_508[1], tmp_974[2] + tmp_508[2]];
    signal tmp_510[3] <== [tmp_509[0] + challenges[2][0], tmp_509[1] + challenges[2][1], tmp_509[2] + challenges[2][2]];
    signal tmp_977[3] <== CMul()(evals[69], tmp_510);
    signal tmp_511[3] <== CMul()(evals[70], tmp_976);
    signal tmp_512[3] <== CMul()(evals[66], tmp_977);
    signal tmp_978[3] <== [tmp_511[0] - tmp_512[0], tmp_511[1] - tmp_512[1], tmp_511[2] - tmp_512[2]];
    signal tmp_979[3] <== CMul()(evals[0], evals[2]);
    signal tmp_980[3] <== CMul()(evals[4], evals[5]);
    signal tmp_981[3] <== CMul()(evals[7], evals[8]);
    signal tmp_982[3] <== CMul()(evals[10], evals[11]);
    signal tmp_983[3] <== CMul()(evals[71], evals[71]);
    signal tmp_984[3] <== [evals[2][0] + evals[56][0], evals[2][1] + evals[56][1], evals[2][2] + evals[56][2]];
    signal tmp_985[3] <== CMul()(evals[72], evals[72]);
    signal tmp_986[3] <== CMul()(evals[73], tmp_984);
    signal tmp_987[3] <== [evals[3][0] + evals[57][0], evals[3][1] + evals[57][1], evals[3][2] + evals[57][2]];
    signal tmp_988[3] <== CMul()(evals[74], evals[74]);
    signal tmp_989[3] <== CMul()(evals[75], tmp_987);
    signal tmp_990[3] <== [evals[4][0] + evals[58][0], evals[4][1] + evals[58][1], evals[4][2] + evals[58][2]];
    signal tmp_991[3] <== CMul()(evals[76], evals[76]);
    signal tmp_992[3] <== CMul()(evals[77], tmp_990);
    signal tmp_993[3] <== [evals[5][0] + evals[60][0], evals[5][1] + evals[60][1], evals[5][2] + evals[60][2]];
    signal tmp_994[3] <== CMul()(evals[78], evals[78]);
    signal tmp_995[3] <== CMul()(evals[79], tmp_993);
    signal tmp_996[3] <== [evals[6][0] + evals[61][0], evals[6][1] + evals[61][1], evals[6][2] + evals[61][2]];
    signal tmp_997[3] <== CMul()(evals[80], evals[80]);
    signal tmp_998[3] <== CMul()(evals[81], tmp_996);
    signal tmp_999[3] <== [evals[7][0] + evals[45][0], evals[7][1] + evals[45][1], evals[7][2] + evals[45][2]];
    signal tmp_1000[3] <== CMul()(evals[82], evals[82]);
    signal tmp_1001[3] <== CMul()(evals[83], tmp_999);
    signal tmp_1002[3] <== [evals[8][0] + evals[52][0], evals[8][1] + evals[52][1], evals[8][2] + evals[52][2]];
    signal tmp_1003[3] <== CMul()(evals[84], evals[84]);
    signal tmp_1004[3] <== CMul()(evals[85], tmp_1002);
    signal tmp_1005[3] <== [evals[9][0] + evals[54][0], evals[9][1] + evals[54][1], evals[9][2] + evals[54][2]];
    signal tmp_1006[3] <== CMul()(evals[86], evals[86]);
    signal tmp_1007[3] <== CMul()(evals[87], tmp_1005);
    signal tmp_1008[3] <== [evals[10][0] + evals[88][0], evals[10][1] + evals[88][1], evals[10][2] + evals[88][2]];
    signal tmp_1009[3] <== CMul()(evals[89], evals[89]);
    signal tmp_1010[3] <== CMul()(evals[90], tmp_1008);
    signal tmp_1011[3] <== [evals[11][0] + evals[46][0], evals[11][1] + evals[46][1], evals[11][2] + evals[46][2]];
    signal tmp_1012[3] <== CMul()(evals[91], evals[91]);
    signal tmp_1013[3] <== CMul()(evals[92], tmp_1011);
    signal tmp_1014[3] <== [evals[12][0] + evals[93][0], evals[12][1] + evals[93][1], evals[12][2] + evals[93][2]];
    signal tmp_1015[3] <== CMul()(evals[94], evals[94]);
    signal tmp_1016[3] <== CMul()(evals[95], tmp_1014);
    signal tmp_513[3] <== [evals[0][0] + evals[19][0], evals[0][1] + evals[19][1], evals[0][2] + evals[19][2]];
    signal tmp_1017[3] <== CMul()(tmp_513, evals[88]);
    signal tmp_514[3] <== [evals[2][0] + evals[56][0], evals[2][1] + evals[56][1], evals[2][2] + evals[56][2]];
    signal tmp_1018[3] <== CMul()(tmp_514, evals[88]);
    signal tmp_1019[3] <== [evals[4][0] + evals[58][0], evals[4][1] + evals[58][1], evals[4][2] + evals[58][2]];
    signal tmp_1020[3] <== [evals[5][0] + evals[60][0], evals[5][1] + evals[60][1], evals[5][2] + evals[60][2]];
    signal tmp_515[3] <== [evals[3][0] + evals[57][0], evals[3][1] + evals[57][1], evals[3][2] + evals[57][2]];
    signal tmp_1021[3] <== CMul()(tmp_515, evals[88]);
    signal tmp_1022[3] <== [evals[6][0] + evals[61][0], evals[6][1] + evals[61][1], evals[6][2] + evals[61][2]];
    signal tmp_516[3] <== [evals[32][0] + evals[34][0], evals[32][1] + evals[34][1], evals[32][2] + evals[34][2]];
    signal tmp_517[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1023[3] <== CMul()(tmp_516, tmp_517);
    signal tmp_518[3] <== [evals[34][0] + evals[35][0], evals[34][1] + evals[35][1], evals[34][2] + evals[35][2]];
    signal tmp_519[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1024[3] <== CMul()(tmp_518, tmp_519);
    signal tmp_1025[3] <== CMul()(evals[34], evals[37]);
    signal tmp_1026[3] <== CMul()(evals[32], evals[36]);
    signal tmp_520[3] <== [tmp_1023[0] + tmp_1024[0], tmp_1023[1] + tmp_1024[1], tmp_1023[2] + tmp_1024[2]];
    signal tmp_521[3] <== [2 * tmp_1025[0], 2 * tmp_1025[1], 2 * tmp_1025[2]];
    signal tmp_522[3] <== [tmp_520[0] - tmp_521[0], tmp_520[1] - tmp_521[1], tmp_520[2] - tmp_521[2]];
    signal tmp_523[3] <== [tmp_522[0] - tmp_1026[0], tmp_522[1] - tmp_1026[1], tmp_522[2] - tmp_1026[2]];
    signal tmp_1027[3] <== [tmp_523[0] + evals[11][0], tmp_523[1] + evals[11][1], tmp_523[2] + evals[11][2]];
    signal tmp_524[3] <== [evals[32][0] + evals[35][0], evals[32][1] + evals[35][1], evals[32][2] + evals[35][2]];
    signal tmp_525[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1028[3] <== CMul()(tmp_524, tmp_525);
    signal tmp_526[3] <== [tmp_1028[0] - tmp_1026[0], tmp_1028[1] - tmp_1026[1], tmp_1028[2] - tmp_1026[2]];
    signal tmp_527[3] <== [tmp_526[0] + tmp_1025[0], tmp_526[1] + tmp_1025[1], tmp_526[2] + tmp_1025[2]];
    signal tmp_1029[3] <== [tmp_527[0] + evals[12][0], tmp_527[1] + evals[12][1], tmp_527[2] + evals[12][2]];
    signal tmp_528[3] <== [tmp_1027[0] + tmp_1029[0], tmp_1027[1] + tmp_1029[1], tmp_1027[2] + tmp_1029[2]];
    signal tmp_529[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1030[3] <== CMul()(tmp_528, tmp_529);
    signal tmp_1031[3] <== CMul()(evals[35], evals[38]);
    signal tmp_530[3] <== [tmp_1024[0] + tmp_1026[0], tmp_1024[1] + tmp_1026[1], tmp_1024[2] + tmp_1026[2]];
    signal tmp_531[3] <== [tmp_530[0] - tmp_1025[0], tmp_530[1] - tmp_1025[1], tmp_530[2] - tmp_1025[2]];
    signal tmp_532[3] <== [tmp_531[0] - tmp_1031[0], tmp_531[1] - tmp_1031[1], tmp_531[2] - tmp_1031[2]];
    signal tmp_1032[3] <== [tmp_532[0] + evals[10][0], tmp_532[1] + evals[10][1], tmp_532[2] + evals[10][2]];
    signal tmp_1033[3] <== CMul()(tmp_1032, evals[36]);
    signal tmp_1034[3] <== CMul()(tmp_1027, evals[37]);
    signal tmp_1035[3] <== CMul()(tmp_1029, evals[38]);
    signal tmp_533[3] <== [tmp_1032[0] + tmp_1027[0], tmp_1032[1] + tmp_1027[1], tmp_1032[2] + tmp_1027[2]];
    signal tmp_534[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1036[3] <== CMul()(tmp_533, tmp_534);
    signal tmp_535[3] <== [tmp_1032[0] + tmp_1029[0], tmp_1032[1] + tmp_1029[1], tmp_1032[2] + tmp_1029[2]];
    signal tmp_536[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1037[3] <== CMul()(tmp_535, tmp_536);
    signal tmp_537[3] <== [evals[96][0] + evals[97][0], evals[96][1] + evals[97][1], evals[96][2] + evals[97][2]];
    signal tmp_538[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1038[3] <== CMul()(tmp_537, tmp_538);
    signal tmp_539[3] <== [evals[97][0] + evals[98][0], evals[97][1] + evals[98][1], evals[97][2] + evals[98][2]];
    signal tmp_540[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1039[3] <== CMul()(tmp_539, tmp_540);
    signal tmp_1040[3] <== CMul()(evals[97], evals[37]);
    signal tmp_1041[3] <== CMul()(evals[96], evals[36]);
    signal tmp_541[3] <== [tmp_1038[0] + tmp_1039[0], tmp_1038[1] + tmp_1039[1], tmp_1038[2] + tmp_1039[2]];
    signal tmp_542[3] <== [2 * tmp_1040[0], 2 * tmp_1040[1], 2 * tmp_1040[2]];
    signal tmp_543[3] <== [tmp_541[0] - tmp_542[0], tmp_541[1] - tmp_542[1], tmp_541[2] - tmp_542[2]];
    signal tmp_544[3] <== [tmp_543[0] - tmp_1041[0], tmp_543[1] - tmp_1041[1], tmp_543[2] - tmp_1041[2]];
    signal tmp_1042[3] <== [tmp_544[0] + evals[5][0], tmp_544[1] + evals[5][1], tmp_544[2] + evals[5][2]];
    signal tmp_545[3] <== [evals[96][0] + evals[98][0], evals[96][1] + evals[98][1], evals[96][2] + evals[98][2]];
    signal tmp_546[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1043[3] <== CMul()(tmp_545, tmp_546);
    signal tmp_547[3] <== [tmp_1043[0] - tmp_1041[0], tmp_1043[1] - tmp_1041[1], tmp_1043[2] - tmp_1041[2]];
    signal tmp_548[3] <== [tmp_547[0] + tmp_1040[0], tmp_547[1] + tmp_1040[1], tmp_547[2] + tmp_1040[2]];
    signal tmp_1044[3] <== [tmp_548[0] + evals[6][0], tmp_548[1] + evals[6][1], tmp_548[2] + evals[6][2]];
    signal tmp_549[3] <== [tmp_1042[0] + tmp_1044[0], tmp_1042[1] + tmp_1044[1], tmp_1042[2] + tmp_1044[2]];
    signal tmp_550[3] <== [evals[37][0] + evals[38][0], evals[37][1] + evals[38][1], evals[37][2] + evals[38][2]];
    signal tmp_1045[3] <== CMul()(tmp_549, tmp_550);
    signal tmp_1046[3] <== CMul()(evals[98], evals[38]);
    signal tmp_551[3] <== [tmp_1039[0] + tmp_1041[0], tmp_1039[1] + tmp_1041[1], tmp_1039[2] + tmp_1041[2]];
    signal tmp_552[3] <== [tmp_551[0] - tmp_1040[0], tmp_551[1] - tmp_1040[1], tmp_551[2] - tmp_1040[2]];
    signal tmp_553[3] <== [tmp_552[0] - tmp_1046[0], tmp_552[1] - tmp_1046[1], tmp_552[2] - tmp_1046[2]];
    signal tmp_1047[3] <== [tmp_553[0] + evals[4][0], tmp_553[1] + evals[4][1], tmp_553[2] + evals[4][2]];
    signal tmp_1048[3] <== CMul()(tmp_1047, evals[36]);
    signal tmp_1049[3] <== CMul()(tmp_1042, evals[37]);
    signal tmp_1050[3] <== CMul()(tmp_1044, evals[38]);
    signal tmp_554[3] <== [tmp_1047[0] + tmp_1042[0], tmp_1047[1] + tmp_1042[1], tmp_1047[2] + tmp_1042[2]];
    signal tmp_555[3] <== [evals[36][0] + evals[37][0], evals[36][1] + evals[37][1], evals[36][2] + evals[37][2]];
    signal tmp_1051[3] <== CMul()(tmp_554, tmp_555);
    signal tmp_556[3] <== [tmp_1047[0] + tmp_1044[0], tmp_1047[1] + tmp_1044[1], tmp_1047[2] + tmp_1044[2]];
    signal tmp_557[3] <== [evals[36][0] + evals[38][0], evals[36][1] + evals[38][1], evals[36][2] + evals[38][2]];
    signal tmp_1052[3] <== CMul()(tmp_556, tmp_557);
    signal tmp_1053[3] <== evals[0];
    signal tmp_558[3] <== CMul()(challenges[3], challenges[7]);
    signal tmp_559[3] <== [tmp_1053[0] + tmp_558[0], tmp_1053[1] + tmp_558[1], tmp_1053[2] + tmp_558[2]];
    signal tmp_1054[3] <== [tmp_559[0] + challenges[2][0], tmp_559[1] + challenges[2][1], tmp_559[2] + challenges[2][2]];
    signal tmp_1055[3] <== evals[2];
    signal tmp_560[3] <== [challenges[3][0] * 12275445934081160404, challenges[3][1] * 12275445934081160404, challenges[3][2] * 12275445934081160404];
    signal tmp_561[3] <== CMul()(tmp_560, challenges[7]);
    signal tmp_562[3] <== [tmp_1055[0] + tmp_561[0], tmp_1055[1] + tmp_561[1], tmp_1055[2] + tmp_561[2]];
    signal tmp_563[3] <== [tmp_562[0] + challenges[2][0], tmp_562[1] + challenges[2][1], tmp_562[2] + challenges[2][2]];
    signal tmp_1056[3] <== CMul()(tmp_1054, tmp_563);
    signal tmp_1057[3] <== evals[3];
    signal tmp_1058[3] <== evals[99];
    signal tmp_564[3] <== CMul()(challenges[3], tmp_1058);
    signal tmp_565[3] <== [tmp_1053[0] + tmp_564[0], tmp_1053[1] + tmp_564[1], tmp_1053[2] + tmp_564[2]];
    signal tmp_1059[3] <== [tmp_565[0] + challenges[2][0], tmp_565[1] + challenges[2][1], tmp_565[2] + challenges[2][2]];
    signal tmp_1060[3] <== evals[100];
    signal tmp_566[3] <== CMul()(challenges[3], tmp_1060);
    signal tmp_567[3] <== [tmp_1055[0] + tmp_566[0], tmp_1055[1] + tmp_566[1], tmp_1055[2] + tmp_566[2]];
    signal tmp_568[3] <== [tmp_567[0] + challenges[2][0], tmp_567[1] + challenges[2][1], tmp_567[2] + challenges[2][2]];
    signal tmp_1061[3] <== CMul()(tmp_1059, tmp_568);
    signal tmp_1062[3] <== evals[101];
    signal tmp_1063[3] <== evals[4];
    signal tmp_569[3] <== [challenges[3][0] * 1279992132519201448, challenges[3][1] * 1279992132519201448, challenges[3][2] * 1279992132519201448];
    signal tmp_570[3] <== CMul()(tmp_569, challenges[7]);
    signal tmp_571[3] <== [tmp_1063[0] + tmp_570[0], tmp_1063[1] + tmp_570[1], tmp_1063[2] + tmp_570[2]];
    signal tmp_572[3] <== [tmp_571[0] + challenges[2][0], tmp_571[1] + challenges[2][1], tmp_571[2] + challenges[2][2]];
    signal tmp_1064[3] <== CMul()(evals[102], tmp_572);
    signal tmp_1065[3] <== evals[5];
    signal tmp_1066[3] <== evals[103];
    signal tmp_573[3] <== CMul()(challenges[3], tmp_1066);
    signal tmp_574[3] <== [tmp_1063[0] + tmp_573[0], tmp_1063[1] + tmp_573[1], tmp_1063[2] + tmp_573[2]];
    signal tmp_575[3] <== [tmp_574[0] + challenges[2][0], tmp_574[1] + challenges[2][1], tmp_574[2] + challenges[2][2]];
    signal tmp_1067[3] <== CMul()(evals[104], tmp_575);
    signal tmp_1068[3] <== evals[105];
    signal tmp_1069[3] <== evals[6];
    signal tmp_576[3] <== [challenges[3][0] * 7781028390488215464, challenges[3][1] * 7781028390488215464, challenges[3][2] * 7781028390488215464];
    signal tmp_577[3] <== CMul()(tmp_576, challenges[7]);
    signal tmp_578[3] <== [tmp_1069[0] + tmp_577[0], tmp_1069[1] + tmp_577[1], tmp_1069[2] + tmp_577[2]];
    signal tmp_579[3] <== [tmp_578[0] + challenges[2][0], tmp_578[1] + challenges[2][1], tmp_578[2] + challenges[2][2]];
    signal tmp_1070[3] <== CMul()(evals[106], tmp_579);
    signal tmp_1071[3] <== evals[7];
    signal tmp_1072[3] <== evals[107];
    signal tmp_580[3] <== CMul()(challenges[3], tmp_1072);
    signal tmp_581[3] <== [tmp_1069[0] + tmp_580[0], tmp_1069[1] + tmp_580[1], tmp_1069[2] + tmp_580[2]];
    signal tmp_582[3] <== [tmp_581[0] + challenges[2][0], tmp_581[1] + challenges[2][1], tmp_581[2] + challenges[2][2]];
    signal tmp_1073[3] <== CMul()(evals[108], tmp_582);
    signal tmp_1074[3] <== evals[109];
    signal tmp_1075[3] <== evals[8];
    signal tmp_583[3] <== [challenges[3][0] * 4549350404001778198, challenges[3][1] * 4549350404001778198, challenges[3][2] * 4549350404001778198];
    signal tmp_584[3] <== CMul()(tmp_583, challenges[7]);
    signal tmp_585[3] <== [tmp_1075[0] + tmp_584[0], tmp_1075[1] + tmp_584[1], tmp_1075[2] + tmp_584[2]];
    signal tmp_586[3] <== [tmp_585[0] + challenges[2][0], tmp_585[1] + challenges[2][1], tmp_585[2] + challenges[2][2]];
    signal tmp_1076[3] <== CMul()(evals[110], tmp_586);
    signal tmp_1077[3] <== evals[9];
    signal tmp_1078[3] <== evals[111];
    signal tmp_587[3] <== CMul()(challenges[3], tmp_1078);
    signal tmp_588[3] <== [tmp_1075[0] + tmp_587[0], tmp_1075[1] + tmp_587[1], tmp_1075[2] + tmp_587[2]];
    signal tmp_589[3] <== [tmp_588[0] + challenges[2][0], tmp_588[1] + challenges[2][1], tmp_588[2] + challenges[2][2]];
    signal tmp_1079[3] <== CMul()(evals[112], tmp_589);
    signal tmp_1080[3] <== evals[113];
    signal tmp_1081[3] <== evals[10];
    signal tmp_590[3] <== [challenges[3][0] * 16725109960945739746, challenges[3][1] * 16725109960945739746, challenges[3][2] * 16725109960945739746];
    signal tmp_591[3] <== CMul()(tmp_590, challenges[7]);
    signal tmp_592[3] <== [tmp_1081[0] + tmp_591[0], tmp_1081[1] + tmp_591[1], tmp_1081[2] + tmp_591[2]];
    signal tmp_593[3] <== [tmp_592[0] + challenges[2][0], tmp_592[1] + challenges[2][1], tmp_592[2] + challenges[2][2]];
    signal tmp_1082[3] <== CMul()(evals[114], tmp_593);
    signal tmp_1083[3] <== evals[11];
    signal tmp_1084[3] <== evals[115];
    signal tmp_594[3] <== CMul()(challenges[3], tmp_1084);
    signal tmp_595[3] <== [tmp_1081[0] + tmp_594[0], tmp_1081[1] + tmp_594[1], tmp_1081[2] + tmp_594[2]];
    signal tmp_596[3] <== [tmp_595[0] + challenges[2][0], tmp_595[1] + challenges[2][1], tmp_595[2] + challenges[2][2]];
    signal tmp_1085[3] <== CMul()(evals[116], tmp_596);
    signal tmp_1086[3] <== evals[117];
    signal tmp_597[3] <== CMulAdd()(challenges[4], tmp_904, tmp_905);
    signal tmp_598[3] <== CMulAdd()(challenges[4], tmp_597, tmp_906);
    signal tmp_599[3] <== CMulAdd()(challenges[4], tmp_598, tmp_907);
    signal tmp_600[3] <== CMulAdd()(challenges[4], tmp_599, tmp_908);
    signal tmp_601[3] <== CMulAdd()(challenges[4], tmp_600, tmp_909);
    signal tmp_602[3] <== CMulAdd()(challenges[4], tmp_601, tmp_910);
    signal tmp_603[3] <== CMulAdd()(challenges[4], tmp_602, tmp_911);
    signal tmp_604[3] <== CMulAdd()(challenges[4], tmp_603, tmp_912);
    signal tmp_605[3] <== CMulAdd()(challenges[4], tmp_604, tmp_913);
    signal tmp_606[3] <== CMulAdd()(challenges[4], tmp_605, tmp_914);
    signal tmp_607[3] <== CMulAdd()(challenges[4], tmp_606, tmp_915);
    signal tmp_608[3] <== CMulAdd()(challenges[4], tmp_607, tmp_916);
    signal tmp_609[3] <== CMulAdd()(challenges[4], tmp_608, tmp_917);
    signal tmp_610[3] <== CMulAdd()(challenges[4], tmp_609, tmp_918);
    signal tmp_611[3] <== CMulAdd()(challenges[4], tmp_610, tmp_919);
    signal tmp_612[3] <== CMulAdd()(challenges[4], tmp_611, tmp_920);
    signal tmp_613[3] <== CMulAdd()(challenges[4], tmp_612, tmp_921);
    signal tmp_614[3] <== CMulAdd()(challenges[4], tmp_613, tmp_925);
    signal tmp_615[3] <== CMulAdd()(challenges[4], tmp_614, tmp_926);
    signal tmp_616[3] <== CMulAdd()(challenges[4], tmp_615, tmp_927);
    signal tmp_617[3] <== CMulAdd()(challenges[4], tmp_616, tmp_928);
    signal tmp_618[3] <== CMulAdd()(challenges[4], tmp_617, tmp_929);
    signal tmp_619[3] <== CMulAdd()(challenges[4], tmp_618, tmp_930);
    signal tmp_620[3] <== CMulAdd()(challenges[4], tmp_619, tmp_931);
    signal tmp_621[3] <== CMulAdd()(challenges[4], tmp_620, tmp_932);
    signal tmp_622[3] <== CMulAdd()(challenges[4], tmp_621, tmp_933);
    signal tmp_623[3] <== CMulAdd()(challenges[4], tmp_622, tmp_934);
    signal tmp_624[3] <== CMulAdd()(challenges[4], tmp_623, tmp_935);
    signal tmp_625[3] <== CMulAdd()(challenges[4], tmp_624, tmp_936);
    signal tmp_626[3] <== CMulAdd()(challenges[4], tmp_625, tmp_939);
    signal tmp_627[3] <== CMulAdd()(challenges[4], tmp_626, tmp_942);
    signal tmp_628[3] <== CMulAdd()(challenges[4], tmp_627, tmp_945);
    signal tmp_629[3] <== CMulAdd()(challenges[4], tmp_628, tmp_947);
    signal tmp_630[3] <== CMulAdd()(challenges[4], tmp_629, tmp_949);
    signal tmp_631[3] <== CMulAdd()(challenges[4], tmp_630, tmp_951);
    signal tmp_632[3] <== CMulAdd()(challenges[4], tmp_631, tmp_953);
    signal tmp_633[3] <== CMulAdd()(challenges[4], tmp_632, tmp_955);
    signal tmp_634[3] <== CMulAdd()(challenges[4], tmp_633, tmp_957);
    signal tmp_635[3] <== CMulAdd()(challenges[4], tmp_634, tmp_959);
    signal tmp_636[3] <== CMulAdd()(challenges[4], tmp_635, tmp_961);
    signal tmp_637[3] <== CMulAdd()(challenges[4], tmp_636, tmp_963);
    signal tmp_638[3] <== CMulAdd()(challenges[4], tmp_637, tmp_965);
    signal tmp_639[3] <== CMulAdd()(challenges[4], tmp_638, tmp_967);
    signal tmp_640[3] <== CMulAdd()(challenges[4], tmp_639, tmp_969);
    signal tmp_641[3] <== CMulAdd()(challenges[4], tmp_640, tmp_970);
    signal tmp_642[3] <== CMulAdd()(challenges[4], tmp_641, tmp_971);
    signal tmp_643[3] <== CMulAdd()(challenges[4], tmp_642, tmp_972);
    signal tmp_644[3] <== CMulAdd()(challenges[4], tmp_643, tmp_973);
    signal tmp_645[3] <== CMulAdd()(challenges[4], tmp_644, tmp_978);
    signal tmp_646[3] <== CMul()(evals[19], evals[0]);
    signal tmp_647[3] <== CMulAdd()(evals[58], tmp_979, tmp_646);
    signal tmp_648[3] <== CMulAdd()(evals[56], evals[2], tmp_647);
    signal tmp_649[3] <== CMulAdd()(evals[57], evals[3], tmp_648);
    signal tmp_650[3] <== [tmp_649[0] + evals[60][0], tmp_649[1] + evals[60][1], tmp_649[2] + evals[60][2]];
    signal tmp_651[3] <== [tmp_650[0] - evals[14][0], tmp_650[1] - evals[14][1], tmp_650[2] - evals[14][2]];
    signal tmp_652[3] <== CMulAdd()(challenges[4], tmp_645, tmp_651);
    signal tmp_653[3] <== CMul()(evals[19], evals[4]);
    signal tmp_654[3] <== CMulAdd()(evals[58], tmp_980, tmp_653);
    signal tmp_655[3] <== CMulAdd()(evals[56], evals[5], tmp_654);
    signal tmp_656[3] <== CMulAdd()(evals[57], evals[6], tmp_655);
    signal tmp_657[3] <== [tmp_656[0] + evals[60][0], tmp_656[1] + evals[60][1], tmp_656[2] + evals[60][2]];
    signal tmp_658[3] <== [tmp_657[0] - evals[16][0], tmp_657[1] - evals[16][1], tmp_657[2] - evals[16][2]];
    signal tmp_659[3] <== CMulAdd()(challenges[4], tmp_652, tmp_658);
    signal tmp_660[3] <== CMul()(evals[45], evals[7]);
    signal tmp_661[3] <== CMulAdd()(evals[88], tmp_981, tmp_660);
    signal tmp_662[3] <== CMulAdd()(evals[52], evals[8], tmp_661);
    signal tmp_663[3] <== CMulAdd()(evals[54], evals[9], tmp_662);
    signal tmp_664[3] <== [tmp_663[0] + evals[46][0], tmp_663[1] + evals[46][1], tmp_663[2] + evals[46][2]];
    signal tmp_665[3] <== [tmp_664[0] - evals[17][0], tmp_664[1] - evals[17][1], tmp_664[2] - evals[17][2]];
    signal tmp_666[3] <== CMulAdd()(challenges[4], tmp_659, tmp_665);
    signal tmp_667[3] <== CMul()(evals[45], evals[10]);
    signal tmp_668[3] <== CMulAdd()(evals[88], tmp_982, tmp_667);
    signal tmp_669[3] <== CMulAdd()(evals[52], evals[11], tmp_668);
    signal tmp_670[3] <== CMulAdd()(evals[54], evals[12], tmp_669);
    signal tmp_671[3] <== [tmp_670[0] + evals[46][0], tmp_670[1] + evals[46][1], tmp_670[2] + evals[46][2]];
    signal tmp_672[3] <== [tmp_671[0] - evals[18][0], tmp_671[1] - evals[18][1], tmp_671[2] - evals[18][2]];
    signal tmp_673[3] <== CMulAdd()(challenges[4], tmp_666, tmp_672);
    signal tmp_674[3] <== CMul()(tmp_922, tmp_922);
    signal tmp_675[3] <== [tmp_674[0] - evals[71][0], tmp_674[1] - evals[71][1], tmp_674[2] - evals[71][2]];
    signal tmp_676[3] <== CMulAdd()(challenges[4], tmp_673, tmp_675);
    signal tmp_677[3] <== CMul()(tmp_983, evals[71]);
    signal tmp_678[3] <== [tmp_677[0] - evals[20][0], tmp_677[1] - evals[20][1], tmp_677[2] - evals[20][2]];
    signal tmp_679[3] <== CMulAdd()(challenges[4], tmp_676, tmp_678);
    signal tmp_680[3] <== CMul()(tmp_984, tmp_984);
    signal tmp_681[3] <== [tmp_680[0] - evals[72][0], tmp_680[1] - evals[72][1], tmp_680[2] - evals[72][2]];
    signal tmp_682[3] <== CMulAdd()(challenges[4], tmp_679, tmp_681);
    signal tmp_683[3] <== CMul()(tmp_985, evals[72]);
    signal tmp_684[3] <== [tmp_683[0] - evals[73][0], tmp_683[1] - evals[73][1], tmp_683[2] - evals[73][2]];
    signal tmp_685[3] <== CMulAdd()(challenges[4], tmp_682, tmp_684);
    signal tmp_686[3] <== [tmp_984[0] - tmp_986[0], tmp_984[1] - tmp_986[1], tmp_984[2] - tmp_986[2]];
    signal tmp_687[3] <== CMulAdd()(evals[118], tmp_686, tmp_986);
    signal tmp_688[3] <== [tmp_687[0] - evals[21][0], tmp_687[1] - evals[21][1], tmp_687[2] - evals[21][2]];
    signal tmp_689[3] <== CMulAdd()(challenges[4], tmp_685, tmp_688);
    signal tmp_690[3] <== CMul()(tmp_987, tmp_987);
    signal tmp_691[3] <== [tmp_690[0] - evals[74][0], tmp_690[1] - evals[74][1], tmp_690[2] - evals[74][2]];
    signal tmp_692[3] <== CMulAdd()(challenges[4], tmp_689, tmp_691);
    signal tmp_693[3] <== CMul()(tmp_988, evals[74]);
    signal tmp_694[3] <== [tmp_693[0] - evals[75][0], tmp_693[1] - evals[75][1], tmp_693[2] - evals[75][2]];
    signal tmp_695[3] <== CMulAdd()(challenges[4], tmp_692, tmp_694);
    signal tmp_696[3] <== [tmp_987[0] - tmp_989[0], tmp_987[1] - tmp_989[1], tmp_987[2] - tmp_989[2]];
    signal tmp_697[3] <== CMulAdd()(evals[118], tmp_696, tmp_989);
    signal tmp_698[3] <== [tmp_697[0] - evals[22][0], tmp_697[1] - evals[22][1], tmp_697[2] - evals[22][2]];
    signal tmp_699[3] <== CMulAdd()(challenges[4], tmp_695, tmp_698);
    signal tmp_700[3] <== CMul()(tmp_990, tmp_990);
    signal tmp_701[3] <== [tmp_700[0] - evals[76][0], tmp_700[1] - evals[76][1], tmp_700[2] - evals[76][2]];
    signal tmp_702[3] <== CMulAdd()(challenges[4], tmp_699, tmp_701);
    signal tmp_703[3] <== CMul()(tmp_991, evals[76]);
    signal tmp_704[3] <== [tmp_703[0] - evals[77][0], tmp_703[1] - evals[77][1], tmp_703[2] - evals[77][2]];
    signal tmp_705[3] <== CMulAdd()(challenges[4], tmp_702, tmp_704);
    signal tmp_706[3] <== [tmp_990[0] - tmp_992[0], tmp_990[1] - tmp_992[1], tmp_990[2] - tmp_992[2]];
    signal tmp_707[3] <== CMulAdd()(evals[118], tmp_706, tmp_992);
    signal tmp_708[3] <== [tmp_707[0] - evals[23][0], tmp_707[1] - evals[23][1], tmp_707[2] - evals[23][2]];
    signal tmp_709[3] <== CMulAdd()(challenges[4], tmp_705, tmp_708);
    signal tmp_710[3] <== CMul()(tmp_993, tmp_993);
    signal tmp_711[3] <== [tmp_710[0] - evals[78][0], tmp_710[1] - evals[78][1], tmp_710[2] - evals[78][2]];
    signal tmp_712[3] <== CMulAdd()(challenges[4], tmp_709, tmp_711);
    signal tmp_713[3] <== CMul()(tmp_994, evals[78]);
    signal tmp_714[3] <== [tmp_713[0] - evals[79][0], tmp_713[1] - evals[79][1], tmp_713[2] - evals[79][2]];
    signal tmp_715[3] <== CMulAdd()(challenges[4], tmp_712, tmp_714);
    signal tmp_716[3] <== [tmp_993[0] - tmp_995[0], tmp_993[1] - tmp_995[1], tmp_993[2] - tmp_995[2]];
    signal tmp_717[3] <== CMulAdd()(evals[118], tmp_716, tmp_995);
    signal tmp_718[3] <== [tmp_717[0] - evals[24][0], tmp_717[1] - evals[24][1], tmp_717[2] - evals[24][2]];
    signal tmp_719[3] <== CMulAdd()(challenges[4], tmp_715, tmp_718);
    signal tmp_720[3] <== CMul()(tmp_996, tmp_996);
    signal tmp_721[3] <== [tmp_720[0] - evals[80][0], tmp_720[1] - evals[80][1], tmp_720[2] - evals[80][2]];
    signal tmp_722[3] <== CMulAdd()(challenges[4], tmp_719, tmp_721);
    signal tmp_723[3] <== CMul()(tmp_997, evals[80]);
    signal tmp_724[3] <== [tmp_723[0] - evals[81][0], tmp_723[1] - evals[81][1], tmp_723[2] - evals[81][2]];
    signal tmp_725[3] <== CMulAdd()(challenges[4], tmp_722, tmp_724);
    signal tmp_726[3] <== [tmp_996[0] - tmp_998[0], tmp_996[1] - tmp_998[1], tmp_996[2] - tmp_998[2]];
    signal tmp_727[3] <== CMulAdd()(evals[118], tmp_726, tmp_998);
    signal tmp_728[3] <== [tmp_727[0] - evals[25][0], tmp_727[1] - evals[25][1], tmp_727[2] - evals[25][2]];
    signal tmp_729[3] <== CMulAdd()(challenges[4], tmp_725, tmp_728);
    signal tmp_730[3] <== CMul()(tmp_999, tmp_999);
    signal tmp_731[3] <== [tmp_730[0] - evals[82][0], tmp_730[1] - evals[82][1], tmp_730[2] - evals[82][2]];
    signal tmp_732[3] <== CMulAdd()(challenges[4], tmp_729, tmp_731);
    signal tmp_733[3] <== CMul()(tmp_1000, evals[82]);
    signal tmp_734[3] <== [tmp_733[0] - evals[83][0], tmp_733[1] - evals[83][1], tmp_733[2] - evals[83][2]];
    signal tmp_735[3] <== CMulAdd()(challenges[4], tmp_732, tmp_734);
    signal tmp_736[3] <== [tmp_999[0] - tmp_1001[0], tmp_999[1] - tmp_1001[1], tmp_999[2] - tmp_1001[2]];
    signal tmp_737[3] <== CMulAdd()(evals[118], tmp_736, tmp_1001);
    signal tmp_738[3] <== [tmp_737[0] - evals[26][0], tmp_737[1] - evals[26][1], tmp_737[2] - evals[26][2]];
    signal tmp_739[3] <== CMulAdd()(challenges[4], tmp_735, tmp_738);
    signal tmp_740[3] <== CMul()(tmp_1002, tmp_1002);
    signal tmp_741[3] <== [tmp_740[0] - evals[84][0], tmp_740[1] - evals[84][1], tmp_740[2] - evals[84][2]];
    signal tmp_742[3] <== CMulAdd()(challenges[4], tmp_739, tmp_741);
    signal tmp_743[3] <== CMul()(tmp_1003, evals[84]);
    signal tmp_744[3] <== [tmp_743[0] - evals[85][0], tmp_743[1] - evals[85][1], tmp_743[2] - evals[85][2]];
    signal tmp_745[3] <== CMulAdd()(challenges[4], tmp_742, tmp_744);
    signal tmp_746[3] <== [tmp_1002[0] - tmp_1004[0], tmp_1002[1] - tmp_1004[1], tmp_1002[2] - tmp_1004[2]];
    signal tmp_747[3] <== CMulAdd()(evals[118], tmp_746, tmp_1004);
    signal tmp_748[3] <== [tmp_747[0] - evals[27][0], tmp_747[1] - evals[27][1], tmp_747[2] - evals[27][2]];
    signal tmp_749[3] <== CMulAdd()(challenges[4], tmp_745, tmp_748);
    signal tmp_750[3] <== CMul()(tmp_1005, tmp_1005);
    signal tmp_751[3] <== [tmp_750[0] - evals[86][0], tmp_750[1] - evals[86][1], tmp_750[2] - evals[86][2]];
    signal tmp_752[3] <== CMulAdd()(challenges[4], tmp_749, tmp_751);
    signal tmp_753[3] <== CMul()(tmp_1006, evals[86]);
    signal tmp_754[3] <== [tmp_753[0] - evals[87][0], tmp_753[1] - evals[87][1], tmp_753[2] - evals[87][2]];
    signal tmp_755[3] <== CMulAdd()(challenges[4], tmp_752, tmp_754);
    signal tmp_756[3] <== [tmp_1005[0] - tmp_1007[0], tmp_1005[1] - tmp_1007[1], tmp_1005[2] - tmp_1007[2]];
    signal tmp_757[3] <== CMulAdd()(evals[118], tmp_756, tmp_1007);
    signal tmp_758[3] <== [tmp_757[0] - evals[28][0], tmp_757[1] - evals[28][1], tmp_757[2] - evals[28][2]];
    signal tmp_759[3] <== CMulAdd()(challenges[4], tmp_755, tmp_758);
    signal tmp_760[3] <== CMul()(tmp_1008, tmp_1008);
    signal tmp_761[3] <== [tmp_760[0] - evals[89][0], tmp_760[1] - evals[89][1], tmp_760[2] - evals[89][2]];
    signal tmp_762[3] <== CMulAdd()(challenges[4], tmp_759, tmp_761);
    signal tmp_763[3] <== CMul()(tmp_1009, evals[89]);
    signal tmp_764[3] <== [tmp_763[0] - evals[90][0], tmp_763[1] - evals[90][1], tmp_763[2] - evals[90][2]];
    signal tmp_765[3] <== CMulAdd()(challenges[4], tmp_762, tmp_764);
    signal tmp_766[3] <== [tmp_1008[0] - tmp_1010[0], tmp_1008[1] - tmp_1010[1], tmp_1008[2] - tmp_1010[2]];
    signal tmp_767[3] <== CMulAdd()(evals[118], tmp_766, tmp_1010);
    signal tmp_768[3] <== [tmp_767[0] - evals[29][0], tmp_767[1] - evals[29][1], tmp_767[2] - evals[29][2]];
    signal tmp_769[3] <== CMulAdd()(challenges[4], tmp_765, tmp_768);
    signal tmp_770[3] <== CMul()(tmp_1011, tmp_1011);
    signal tmp_771[3] <== [tmp_770[0] - evals[91][0], tmp_770[1] - evals[91][1], tmp_770[2] - evals[91][2]];
    signal tmp_772[3] <== CMulAdd()(challenges[4], tmp_769, tmp_771);
    signal tmp_773[3] <== CMul()(tmp_1012, evals[91]);
    signal tmp_774[3] <== [tmp_773[0] - evals[92][0], tmp_773[1] - evals[92][1], tmp_773[2] - evals[92][2]];
    signal tmp_775[3] <== CMulAdd()(challenges[4], tmp_772, tmp_774);
    signal tmp_776[3] <== [tmp_1011[0] - tmp_1013[0], tmp_1011[1] - tmp_1013[1], tmp_1011[2] - tmp_1013[2]];
    signal tmp_777[3] <== CMulAdd()(evals[118], tmp_776, tmp_1013);
    signal tmp_778[3] <== [tmp_777[0] - evals[30][0], tmp_777[1] - evals[30][1], tmp_777[2] - evals[30][2]];
    signal tmp_779[3] <== CMulAdd()(challenges[4], tmp_775, tmp_778);
    signal tmp_780[3] <== CMul()(tmp_1014, tmp_1014);
    signal tmp_781[3] <== [tmp_780[0] - evals[94][0], tmp_780[1] - evals[94][1], tmp_780[2] - evals[94][2]];
    signal tmp_782[3] <== CMulAdd()(challenges[4], tmp_779, tmp_781);
    signal tmp_783[3] <== CMul()(tmp_1015, evals[94]);
    signal tmp_784[3] <== [tmp_783[0] - evals[95][0], tmp_783[1] - evals[95][1], tmp_783[2] - evals[95][2]];
    signal tmp_785[3] <== CMulAdd()(challenges[4], tmp_782, tmp_784);
    signal tmp_786[3] <== [tmp_1014[0] - tmp_1016[0], tmp_1014[1] - tmp_1016[1], tmp_1014[2] - tmp_1016[2]];
    signal tmp_787[3] <== CMulAdd()(evals[118], tmp_786, tmp_1016);
    signal tmp_788[3] <== [tmp_787[0] - evals[31][0], tmp_787[1] - evals[31][1], tmp_787[2] - evals[31][2]];
    signal tmp_789[3] <== CMulAdd()(challenges[4], tmp_785, tmp_788);
    signal tmp_790[3] <== [tmp_1017[0] + tmp_1018[0], tmp_1017[1] + tmp_1018[1], tmp_1017[2] + tmp_1018[2]];
    signal tmp_791[3] <== [tmp_1019[0] + tmp_1020[0], tmp_1019[1] + tmp_1020[1], tmp_1019[2] + tmp_1020[2]];
    signal tmp_792[3] <== CMul()(tmp_790, tmp_791);
    signal tmp_793[3] <== [tmp_792[0] - evals[53][0], tmp_792[1] - evals[53][1], tmp_792[2] - evals[53][2]];
    signal tmp_794[3] <== CMulAdd()(challenges[4], tmp_789, tmp_793);
    signal tmp_795[3] <== [tmp_1017[0] + tmp_1021[0], tmp_1017[1] + tmp_1021[1], tmp_1017[2] + tmp_1021[2]];
    signal tmp_796[3] <== [tmp_1019[0] + tmp_1022[0], tmp_1019[1] + tmp_1022[1], tmp_1019[2] + tmp_1022[2]];
    signal tmp_797[3] <== CMul()(tmp_795, tmp_796);
    signal tmp_798[3] <== [tmp_797[0] - evals[55][0], tmp_797[1] - evals[55][1], tmp_797[2] - evals[55][2]];
    signal tmp_799[3] <== CMulAdd()(challenges[4], tmp_794, tmp_798);
    signal tmp_800[3] <== [tmp_1018[0] + tmp_1021[0], tmp_1018[1] + tmp_1021[1], tmp_1018[2] + tmp_1021[2]];
    signal tmp_801[3] <== [tmp_1020[0] + tmp_1022[0], tmp_1020[1] + tmp_1022[1], tmp_1020[2] + tmp_1022[2]];
    signal tmp_802[3] <== CMul()(tmp_800, tmp_801);
    signal tmp_803[3] <== [tmp_802[0] - evals[47][0], tmp_802[1] - evals[47][1], tmp_802[2] - evals[47][2]];
    signal tmp_804[3] <== CMulAdd()(challenges[4], tmp_799, tmp_803);
    signal tmp_805[3] <== CMul()(tmp_1017, tmp_1019);
    signal tmp_806[3] <== [tmp_805[0] - evals[48][0], tmp_805[1] - evals[48][1], tmp_805[2] - evals[48][2]];
    signal tmp_807[3] <== CMulAdd()(challenges[4], tmp_804, tmp_806);
    signal tmp_808[3] <== CMul()(tmp_1018, tmp_1020);
    signal tmp_809[3] <== [tmp_808[0] - evals[49][0], tmp_808[1] - evals[49][1], tmp_808[2] - evals[49][2]];
    signal tmp_810[3] <== CMulAdd()(challenges[4], tmp_807, tmp_809);
    signal tmp_811[3] <== CMul()(tmp_1021, tmp_1022);
    signal tmp_812[3] <== [tmp_811[0] - evals[50][0], tmp_811[1] - evals[50][1], tmp_811[2] - evals[50][2]];
    signal tmp_813[3] <== CMulAdd()(challenges[4], tmp_810, tmp_812);
    signal tmp_814[3] <== [tmp_1030[0] + tmp_1033[0], tmp_1030[1] + tmp_1033[1], tmp_1030[2] + tmp_1033[2]];
    signal tmp_815[3] <== [tmp_814[0] - tmp_1034[0], tmp_814[1] - tmp_1034[1], tmp_814[2] - tmp_1034[2]];
    signal tmp_816[3] <== [tmp_815[0] - tmp_1035[0], tmp_815[1] - tmp_1035[1], tmp_815[2] - tmp_1035[2]];
    signal tmp_817[3] <== [tmp_816[0] + evals[7][0], tmp_816[1] + evals[7][1], tmp_816[2] + evals[7][2]];
    signal tmp_818[3] <== [tmp_817[0] - evals[96][0], tmp_817[1] - evals[96][1], tmp_817[2] - evals[96][2]];
    signal tmp_819[3] <== CMulAdd()(challenges[4], tmp_813, tmp_818);
    signal tmp_820[3] <== [tmp_1036[0] + tmp_1030[0], tmp_1036[1] + tmp_1030[1], tmp_1036[2] + tmp_1030[2]];
    signal tmp_821[3] <== [2 * tmp_1034[0], 2 * tmp_1034[1], 2 * tmp_1034[2]];
    signal tmp_822[3] <== [tmp_820[0] - tmp_821[0], tmp_820[1] - tmp_821[1], tmp_820[2] - tmp_821[2]];
    signal tmp_823[3] <== [tmp_822[0] - tmp_1033[0], tmp_822[1] - tmp_1033[1], tmp_822[2] - tmp_1033[2]];
    signal tmp_824[3] <== [tmp_823[0] + evals[8][0], tmp_823[1] + evals[8][1], tmp_823[2] + evals[8][2]];
    signal tmp_825[3] <== [tmp_824[0] - evals[97][0], tmp_824[1] - evals[97][1], tmp_824[2] - evals[97][2]];
    signal tmp_826[3] <== CMulAdd()(challenges[4], tmp_819, tmp_825);
    signal tmp_827[3] <== [tmp_1037[0] - tmp_1033[0], tmp_1037[1] - tmp_1033[1], tmp_1037[2] - tmp_1033[2]];
    signal tmp_828[3] <== [tmp_827[0] + tmp_1034[0], tmp_827[1] + tmp_1034[1], tmp_827[2] + tmp_1034[2]];
    signal tmp_829[3] <== [tmp_828[0] + evals[9][0], tmp_828[1] + evals[9][1], tmp_828[2] + evals[9][2]];
    signal tmp_830[3] <== [tmp_829[0] - evals[98][0], tmp_829[1] - evals[98][1], tmp_829[2] - evals[98][2]];
    signal tmp_831[3] <== CMulAdd()(challenges[4], tmp_826, tmp_830);
    signal tmp_832[3] <== [tmp_1045[0] + tmp_1048[0], tmp_1045[1] + tmp_1048[1], tmp_1045[2] + tmp_1048[2]];
    signal tmp_833[3] <== [tmp_832[0] - tmp_1049[0], tmp_832[1] - tmp_1049[1], tmp_832[2] - tmp_1049[2]];
    signal tmp_834[3] <== [tmp_833[0] - tmp_1050[0], tmp_833[1] - tmp_1050[1], tmp_833[2] - tmp_1050[2]];
    signal tmp_835[3] <== [tmp_834[0] + evals[0][0], tmp_834[1] + evals[0][1], tmp_834[2] + evals[0][2]];
    signal tmp_836[3] <== [tmp_835[0] - evals[62][0], tmp_835[1] - evals[62][1], tmp_835[2] - evals[62][2]];
    signal tmp_837[3] <== CMulAdd()(challenges[4], tmp_831, tmp_836);
    signal tmp_838[3] <== [tmp_1051[0] + tmp_1045[0], tmp_1051[1] + tmp_1045[1], tmp_1051[2] + tmp_1045[2]];
    signal tmp_839[3] <== [2 * tmp_1049[0], 2 * tmp_1049[1], 2 * tmp_1049[2]];
    signal tmp_840[3] <== [tmp_838[0] - tmp_839[0], tmp_838[1] - tmp_839[1], tmp_838[2] - tmp_839[2]];
    signal tmp_841[3] <== [tmp_840[0] - tmp_1048[0], tmp_840[1] - tmp_1048[1], tmp_840[2] - tmp_1048[2]];
    signal tmp_842[3] <== [tmp_841[0] + evals[2][0], tmp_841[1] + evals[2][1], tmp_841[2] + evals[2][2]];
    signal tmp_843[3] <== [tmp_842[0] - evals[64][0], tmp_842[1] - evals[64][1], tmp_842[2] - evals[64][2]];
    signal tmp_844[3] <== CMulAdd()(challenges[4], tmp_837, tmp_843);
    signal tmp_845[3] <== [tmp_1052[0] - tmp_1048[0], tmp_1052[1] - tmp_1048[1], tmp_1052[2] - tmp_1048[2]];
    signal tmp_846[3] <== [tmp_845[0] + tmp_1049[0], tmp_845[1] + tmp_1049[1], tmp_845[2] + tmp_1049[2]];
    signal tmp_847[3] <== [tmp_846[0] + evals[3][0], tmp_846[1] + evals[3][1], tmp_846[2] + evals[3][2]];
    signal tmp_848[3] <== [tmp_847[0] - evals[65][0], tmp_847[1] - evals[65][1], tmp_847[2] - evals[65][2]];
    signal tmp_849[3] <== CMulAdd()(challenges[4], tmp_844, tmp_848);
    signal tmp_850[3] <== [challenges[3][0] * 4756475762779100925, challenges[3][1] * 4756475762779100925, challenges[3][2] * 4756475762779100925];
    signal tmp_851[3] <== CMulAdd()(tmp_850, challenges[7], tmp_1057);
    signal tmp_852[3] <== [tmp_851[0] + challenges[2][0], tmp_851[1] + challenges[2][1], tmp_851[2] + challenges[2][2]];
    signal tmp_853[3] <== CMul()(tmp_1056, tmp_852);
    signal tmp_854[3] <== [tmp_853[0] - evals[102][0], tmp_853[1] - evals[102][1], tmp_853[2] - evals[102][2]];
    signal tmp_855[3] <== CMulAdd()(challenges[4], tmp_849, tmp_854);
    signal tmp_856[3] <== CMulAdd()(challenges[3], tmp_1062, tmp_1057);
    signal tmp_857[3] <== [tmp_856[0] + challenges[2][0], tmp_856[1] + challenges[2][1], tmp_856[2] + challenges[2][2]];
    signal tmp_858[3] <== CMul()(tmp_1061, tmp_857);
    signal tmp_859[3] <== [tmp_858[0] - evals[104][0], tmp_858[1] - evals[104][1], tmp_858[2] - evals[104][2]];
    signal tmp_860[3] <== CMulAdd()(challenges[4], tmp_855, tmp_859);
    signal tmp_861[3] <== [challenges[3][0] * 8312008622371998338, challenges[3][1] * 8312008622371998338, challenges[3][2] * 8312008622371998338];
    signal tmp_862[3] <== CMulAdd()(tmp_861, challenges[7], tmp_1065);
    signal tmp_863[3] <== [tmp_862[0] + challenges[2][0], tmp_862[1] + challenges[2][1], tmp_862[2] + challenges[2][2]];
    signal tmp_864[3] <== CMul()(tmp_1064, tmp_863);
    signal tmp_865[3] <== [tmp_864[0] - evals[106][0], tmp_864[1] - evals[106][1], tmp_864[2] - evals[106][2]];
    signal tmp_866[3] <== CMulAdd()(challenges[4], tmp_860, tmp_865);
    signal tmp_867[3] <== CMulAdd()(challenges[3], tmp_1068, tmp_1065);
    signal tmp_868[3] <== [tmp_867[0] + challenges[2][0], tmp_867[1] + challenges[2][1], tmp_867[2] + challenges[2][2]];
    signal tmp_869[3] <== CMul()(tmp_1067, tmp_868);
    signal tmp_870[3] <== [tmp_869[0] - evals[108][0], tmp_869[1] - evals[108][1], tmp_869[2] - evals[108][2]];
    signal tmp_871[3] <== CMulAdd()(challenges[4], tmp_866, tmp_870);
    signal tmp_872[3] <== [challenges[3][0] * 11302600489504509467, challenges[3][1] * 11302600489504509467, challenges[3][2] * 11302600489504509467];
    signal tmp_873[3] <== CMulAdd()(tmp_872, challenges[7], tmp_1071);
    signal tmp_874[3] <== [tmp_873[0] + challenges[2][0], tmp_873[1] + challenges[2][1], tmp_873[2] + challenges[2][2]];
    signal tmp_875[3] <== CMul()(tmp_1070, tmp_874);
    signal tmp_876[3] <== [tmp_875[0] - evals[110][0], tmp_875[1] - evals[110][1], tmp_875[2] - evals[110][2]];
    signal tmp_877[3] <== CMulAdd()(challenges[4], tmp_871, tmp_876);
    signal tmp_878[3] <== CMulAdd()(challenges[3], tmp_1074, tmp_1071);
    signal tmp_879[3] <== [tmp_878[0] + challenges[2][0], tmp_878[1] + challenges[2][1], tmp_878[2] + challenges[2][2]];
    signal tmp_880[3] <== CMul()(tmp_1073, tmp_879);
    signal tmp_881[3] <== [tmp_880[0] - evals[112][0], tmp_880[1] - evals[112][1], tmp_880[2] - evals[112][2]];
    signal tmp_882[3] <== CMulAdd()(challenges[4], tmp_877, tmp_881);
    signal tmp_883[3] <== [challenges[3][0] * 3688660304411827445, challenges[3][1] * 3688660304411827445, challenges[3][2] * 3688660304411827445];
    signal tmp_884[3] <== CMulAdd()(tmp_883, challenges[7], tmp_1077);
    signal tmp_885[3] <== [tmp_884[0] + challenges[2][0], tmp_884[1] + challenges[2][1], tmp_884[2] + challenges[2][2]];
    signal tmp_886[3] <== CMul()(tmp_1076, tmp_885);
    signal tmp_887[3] <== [tmp_886[0] - evals[114][0], tmp_886[1] - evals[114][1], tmp_886[2] - evals[114][2]];
    signal tmp_888[3] <== CMulAdd()(challenges[4], tmp_882, tmp_887);
    signal tmp_889[3] <== CMulAdd()(challenges[3], tmp_1080, tmp_1077);
    signal tmp_890[3] <== [tmp_889[0] + challenges[2][0], tmp_889[1] + challenges[2][1], tmp_889[2] + challenges[2][2]];
    signal tmp_891[3] <== CMul()(tmp_1079, tmp_890);
    signal tmp_892[3] <== [tmp_891[0] - evals[116][0], tmp_891[1] - evals[116][1], tmp_891[2] - evals[116][2]];
    signal tmp_893[3] <== CMulAdd()(challenges[4], tmp_888, tmp_892);
    signal tmp_894[3] <== [challenges[3][0] * 16538725463549498621, challenges[3][1] * 16538725463549498621, challenges[3][2] * 16538725463549498621];
    signal tmp_895[3] <== CMulAdd()(tmp_894, challenges[7], tmp_1083);
    signal tmp_896[3] <== [tmp_895[0] + challenges[2][0], tmp_895[1] + challenges[2][1], tmp_895[2] + challenges[2][2]];
    signal tmp_897[3] <== CMul()(tmp_1082, tmp_896);
    signal tmp_898[3] <== [tmp_897[0] - evals[69][0], tmp_897[1] - evals[69][1], tmp_897[2] - evals[69][2]];
    signal tmp_899[3] <== CMulAdd()(challenges[4], tmp_893, tmp_898);
    signal tmp_900[3] <== CMulAdd()(challenges[3], tmp_1086, tmp_1083);
    signal tmp_901[3] <== [tmp_900[0] + challenges[2][0], tmp_900[1] + challenges[2][1], tmp_900[2] + challenges[2][2]];
    signal tmp_902[3] <== CMul()(tmp_1085, tmp_901);
    signal tmp_903[3] <== [tmp_902[0] - evals[68][0], tmp_902[1] - evals[68][1], tmp_902[2] - evals[68][2]];
    signal tmp_1087[3] <== CMulAdd()(challenges[4], tmp_899, tmp_903);
    signal xN[3] <== zMul[16];

    signal xAcc[2][3];
    signal qStep[1][3];
    signal qAcc[2][3];
    for (var i=0; i< 2; i++) {
        if (i==0) {
            xAcc[0] <== [1, 0, 0];
            qAcc[0] <== evals[119+i];
        } else {
            xAcc[i] <== CMul()(xAcc[i-1], xN);
            qStep[i-1] <== CMul()(xAcc[i], evals[119+i]);

            qAcc[i][0] <== qAcc[i-1][0] + qStep[i-1][0];
            qAcc[i][1] <== qAcc[i-1][1] + qStep[i-1][1];
            qAcc[i][2] <== qAcc[i-1][2] + qStep[i-1][2];
        }
    }
    signal qZ[3] <== CMul()(qAcc[1], Z);

// Final Verification
    enable * (tmp_1087[0] - qZ[0]) === 0;
    enable * (tmp_1087[1] - qZ[1]) === 0;
    enable * (tmp_1087[2] - qZ[2]) === 0;
}
        
template parallel VerifyQuery() {
    signal input ys[18];
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
    signal xacc[18];
    xacc[0] <== ys[0]*(49 * roots(18)-49) + 49;
    for (var i=1; i<18; i++ ) {
        xacc[i] <== xacc[i-1] * ( ys[i]*(roots(18 - i) - 1) +1);
    }
    component den1inv = CInv();
    den1inv.in[0] <== xacc[17] - challenges[7][0];
    den1inv.in[1] <== -challenges[7][1];
    den1inv.in[2] <== -challenges[7][2];
    signal xDivXSubXi[3];
    xDivXSubXi[0] <== xacc[17] * den1inv.out[0];
    xDivXSubXi[1] <== xacc[17] * den1inv.out[1];
    xDivXSubXi[2] <== xacc[17] * den1inv.out[2];
    
    component den2inv = CInv();
    den2inv.in[0] <== xacc[17] - roots(17)*challenges[7][0];
    den2inv.in[1] <== -roots(17)*challenges[7][1];
    den2inv.in[2] <== -roots(17)*challenges[7][2];
    signal xDivXSubWXi[3];
    xDivXSubWXi[0] <== xacc[17] * den2inv.out[0];
    xDivXSubWXi[1] <== xacc[17] * den2inv.out[1];
    xDivXSubWXi[2] <== xacc[17] * den2inv.out[2];
    
    signal tmp_0[3] <== [challenges[5][0] * mapValues.tree1_0 + mapValues.tree1_1, challenges[5][1] * mapValues.tree1_0, challenges[5][2] * mapValues.tree1_0];
    signal tmp_1[3] <== CMulAdd()(challenges[5], tmp_0, [mapValues.tree1_2, 0, 0]);
    signal tmp_2[3] <== CMulAdd()(challenges[5], tmp_1, [mapValues.tree1_3, 0, 0]);
    signal tmp_3[3] <== CMulAdd()(challenges[5], tmp_2, [mapValues.tree1_4, 0, 0]);
    signal tmp_4[3] <== CMulAdd()(challenges[5], tmp_3, [mapValues.tree1_5, 0, 0]);
    signal tmp_5[3] <== CMulAdd()(challenges[5], tmp_4, [mapValues.tree1_6, 0, 0]);
    signal tmp_6[3] <== CMulAdd()(challenges[5], tmp_5, [mapValues.tree1_7, 0, 0]);
    signal tmp_7[3] <== CMulAdd()(challenges[5], tmp_6, [mapValues.tree1_8, 0, 0]);
    signal tmp_8[3] <== CMulAdd()(challenges[5], tmp_7, [mapValues.tree1_9, 0, 0]);
    signal tmp_9[3] <== CMulAdd()(challenges[5], tmp_8, [mapValues.tree1_10, 0, 0]);
    signal tmp_10[3] <== CMulAdd()(challenges[5], tmp_9, [mapValues.tree1_11, 0, 0]);
    signal tmp_11[3] <== CMulAdd()(challenges[5], tmp_10, mapValues.tree3_0);
    signal tmp_12[3] <== CMulAdd()(challenges[5], tmp_11, [mapValues.tree3_1, 0, 0]);
    signal tmp_13[3] <== CMulAdd()(challenges[5], tmp_12, [mapValues.tree3_2, 0, 0]);
    signal tmp_14[3] <== CMulAdd()(challenges[5], tmp_13, [mapValues.tree3_3, 0, 0]);
    signal tmp_15[3] <== CMulAdd()(challenges[5], tmp_14, [mapValues.tree3_4, 0, 0]);
    signal tmp_16[3] <== CMulAdd()(challenges[5], tmp_15, [mapValues.tree3_5, 0, 0]);
    signal tmp_17[3] <== CMulAdd()(challenges[5], tmp_16, [mapValues.tree3_6, 0, 0]);
    signal tmp_18[3] <== CMulAdd()(challenges[5], tmp_17, [mapValues.tree3_7, 0, 0]);
    signal tmp_19[3] <== CMulAdd()(challenges[5], tmp_18, [mapValues.tree3_8, 0, 0]);
    signal tmp_20[3] <== CMulAdd()(challenges[5], tmp_19, [mapValues.tree3_9, 0, 0]);
    signal tmp_21[3] <== CMulAdd()(challenges[5], tmp_20, [mapValues.tree3_10, 0, 0]);
    signal tmp_22[3] <== CMulAdd()(challenges[5], tmp_21, [mapValues.tree3_11, 0, 0]);
    signal tmp_23[3] <== CMulAdd()(challenges[5], tmp_22, [mapValues.tree3_12, 0, 0]);
    signal tmp_24[3] <== CMulAdd()(challenges[5], tmp_23, [mapValues.tree3_13, 0, 0]);
    signal tmp_25[3] <== CMulAdd()(challenges[5], tmp_24, [mapValues.tree3_14, 0, 0]);
    signal tmp_26[3] <== CMulAdd()(challenges[5], tmp_25, [mapValues.tree3_15, 0, 0]);
    signal tmp_27[3] <== CMulAdd()(challenges[5], tmp_26, [mapValues.tree3_16, 0, 0]);
    signal tmp_28[3] <== CMulAdd()(challenges[5], tmp_27, [mapValues.tree3_17, 0, 0]);
    signal tmp_29[3] <== CMulAdd()(challenges[5], tmp_28, [mapValues.tree3_18, 0, 0]);
    signal tmp_30[3] <== CMulAdd()(challenges[5], tmp_29, [mapValues.tree3_19, 0, 0]);
    signal tmp_31[3] <== CMulAdd()(challenges[5], tmp_30, [mapValues.tree3_20, 0, 0]);
    signal tmp_32[3] <== CMulAdd()(challenges[5], tmp_31, [mapValues.tree3_21, 0, 0]);
    signal tmp_33[3] <== CMulAdd()(challenges[5], tmp_32, [mapValues.tree3_22, 0, 0]);
    signal tmp_34[3] <== CMulAdd()(challenges[5], tmp_33, [mapValues.tree3_23, 0, 0]);
    signal tmp_35[3] <== CMulAdd()(challenges[5], tmp_34, [mapValues.tree3_24, 0, 0]);
    signal tmp_36[3] <== CMulAdd()(challenges[5], tmp_35, [mapValues.tree3_25, 0, 0]);
    signal tmp_37[3] <== CMulAdd()(challenges[5], tmp_36, [mapValues.tree3_26, 0, 0]);
    signal tmp_38[3] <== CMulAdd()(challenges[5], tmp_37, [mapValues.tree3_27, 0, 0]);
    signal tmp_39[3] <== CMulAdd()(challenges[5], tmp_38, [mapValues.tree3_28, 0, 0]);
    signal tmp_40[3] <== CMulAdd()(challenges[5], tmp_39, [mapValues.tree3_29, 0, 0]);
    signal tmp_41[3] <== CMulAdd()(challenges[5], tmp_40, [mapValues.tree3_30, 0, 0]);
    signal tmp_42[3] <== CMulAdd()(challenges[5], tmp_41, [mapValues.tree3_31, 0, 0]);
    signal tmp_43[3] <== CMulAdd()(challenges[5], tmp_42, [mapValues.tree3_32, 0, 0]);
    signal tmp_44[3] <== CMulAdd()(challenges[5], tmp_43, [mapValues.tree3_33, 0, 0]);
    signal tmp_45[3] <== CMulAdd()(challenges[5], tmp_44, [mapValues.tree3_34, 0, 0]);
    signal tmp_46[3] <== CMulAdd()(challenges[5], tmp_45, [mapValues.tree3_35, 0, 0]);
    signal tmp_47[3] <== CMulAdd()(challenges[5], tmp_46, [mapValues.tree3_36, 0, 0]);
    signal tmp_48[3] <== CMulAdd()(challenges[5], tmp_47, [mapValues.tree3_37, 0, 0]);
    signal tmp_49[3] <== CMulAdd()(challenges[5], tmp_48, [mapValues.tree3_38, 0, 0]);
    signal tmp_50[3] <== CMulAdd()(challenges[5], tmp_49, [mapValues.tree3_39, 0, 0]);
    signal tmp_51[3] <== CMulAdd()(challenges[5], tmp_50, [mapValues.tree3_40, 0, 0]);
    signal tmp_52[3] <== CMulAdd()(challenges[5], tmp_51, [mapValues.tree3_41, 0, 0]);
    signal tmp_53[3] <== CMulAdd()(challenges[5], tmp_52, [mapValues.tree3_42, 0, 0]);
    signal tmp_54[3] <== CMulAdd()(challenges[5], tmp_53, [mapValues.tree3_43, 0, 0]);
    signal tmp_55[3] <== CMulAdd()(challenges[5], tmp_54, [mapValues.tree3_44, 0, 0]);
    signal tmp_56[3] <== CMulAdd()(challenges[5], tmp_55, [mapValues.tree3_45, 0, 0]);
    signal tmp_57[3] <== CMulAdd()(challenges[5], tmp_56, [mapValues.tree3_46, 0, 0]);
    signal tmp_58[3] <== CMulAdd()(challenges[5], tmp_57, [mapValues.tree3_47, 0, 0]);
    signal tmp_59[3] <== CMulAdd()(challenges[5], tmp_58, [mapValues.tree3_48, 0, 0]);
    signal tmp_60[3] <== CMulAdd()(challenges[5], tmp_59, [mapValues.tree3_49, 0, 0]);
    signal tmp_61[3] <== CMulAdd()(challenges[5], tmp_60, [mapValues.tree3_50, 0, 0]);
    signal tmp_62[3] <== CMulAdd()(challenges[5], tmp_61, [mapValues.tree3_51, 0, 0]);
    signal tmp_63[3] <== CMulAdd()(challenges[5], tmp_62, mapValues.tree3_52);
    signal tmp_64[3] <== CMulAdd()(challenges[5], tmp_63, mapValues.tree3_53);
    signal tmp_65[3] <== CMulAdd()(challenges[5], tmp_64, mapValues.tree3_54);
    signal tmp_66[3] <== CMulAdd()(challenges[5], tmp_65, mapValues.tree3_55);
    signal tmp_67[3] <== CMulAdd()(challenges[5], tmp_66, mapValues.tree3_56);
    signal tmp_68[3] <== CMulAdd()(challenges[5], tmp_67, mapValues.tree3_57);
    signal tmp_69[3] <== CMulAdd()(challenges[5], tmp_68, mapValues.tree3_58);
    signal tmp_70[3] <== CMulAdd()(challenges[5], tmp_69, mapValues.tree3_59);
    signal tmp_71[3] <== CMulAdd()(challenges[5], tmp_70, mapValues.tree3_60);
    signal tmp_72[3] <== CMulAdd()(challenges[5], tmp_71, mapValues.tree3_61);
    signal tmp_73[3] <== CMulAdd()(challenges[5], tmp_72, mapValues.tree4_0);
    signal tmp_74[3] <== CMulAdd()(challenges[5], tmp_73, mapValues.tree4_1);
    signal tmp_75[3] <== [mapValues.tree1_0 - evals[0][0], -evals[0][1], -evals[0][2]];
    signal tmp_76[3] <== [consts[0] - evals[1][0], -evals[1][1], -evals[1][2]];
    signal tmp_77[3] <== CMulAdd()(tmp_75, challenges[6], tmp_76);
    signal tmp_78[3] <== [mapValues.tree1_1 - evals[2][0], -evals[2][1], -evals[2][2]];
    signal tmp_79[3] <== CMulAdd()(tmp_77, challenges[6], tmp_78);
    signal tmp_80[3] <== [mapValues.tree1_2 - evals[3][0], -evals[3][1], -evals[3][2]];
    signal tmp_81[3] <== CMulAdd()(tmp_79, challenges[6], tmp_80);
    signal tmp_82[3] <== [mapValues.tree1_3 - evals[4][0], -evals[4][1], -evals[4][2]];
    signal tmp_83[3] <== CMulAdd()(tmp_81, challenges[6], tmp_82);
    signal tmp_84[3] <== [mapValues.tree1_4 - evals[5][0], -evals[5][1], -evals[5][2]];
    signal tmp_85[3] <== CMulAdd()(tmp_83, challenges[6], tmp_84);
    signal tmp_86[3] <== [mapValues.tree1_5 - evals[6][0], -evals[6][1], -evals[6][2]];
    signal tmp_87[3] <== CMulAdd()(tmp_85, challenges[6], tmp_86);
    signal tmp_88[3] <== [mapValues.tree1_6 - evals[7][0], -evals[7][1], -evals[7][2]];
    signal tmp_89[3] <== CMulAdd()(tmp_87, challenges[6], tmp_88);
    signal tmp_90[3] <== [mapValues.tree1_7 - evals[8][0], -evals[8][1], -evals[8][2]];
    signal tmp_91[3] <== CMulAdd()(tmp_89, challenges[6], tmp_90);
    signal tmp_92[3] <== [mapValues.tree1_8 - evals[9][0], -evals[9][1], -evals[9][2]];
    signal tmp_93[3] <== CMulAdd()(tmp_91, challenges[6], tmp_92);
    signal tmp_94[3] <== [mapValues.tree1_9 - evals[10][0], -evals[10][1], -evals[10][2]];
    signal tmp_95[3] <== CMulAdd()(tmp_93, challenges[6], tmp_94);
    signal tmp_96[3] <== [mapValues.tree1_10 - evals[11][0], -evals[11][1], -evals[11][2]];
    signal tmp_97[3] <== CMulAdd()(tmp_95, challenges[6], tmp_96);
    signal tmp_98[3] <== [mapValues.tree1_11 - evals[12][0], -evals[12][1], -evals[12][2]];
    signal tmp_99[3] <== CMulAdd()(tmp_97, challenges[6], tmp_98);
    signal tmp_100[3] <== [consts[1] - evals[13][0], -evals[13][1], -evals[13][2]];
    signal tmp_101[3] <== CMulAdd()(tmp_99, challenges[6], tmp_100);
    signal tmp_102[3] <== [mapValues.tree3_1 - evals[14][0], -evals[14][1], -evals[14][2]];
    signal tmp_103[3] <== CMulAdd()(tmp_101, challenges[6], tmp_102);
    signal tmp_104[3] <== [consts[28] - evals[15][0], -evals[15][1], -evals[15][2]];
    signal tmp_105[3] <== CMulAdd()(tmp_103, challenges[6], tmp_104);
    signal tmp_106[3] <== [mapValues.tree3_2 - evals[16][0], -evals[16][1], -evals[16][2]];
    signal tmp_107[3] <== CMulAdd()(tmp_105, challenges[6], tmp_106);
    signal tmp_108[3] <== [mapValues.tree3_3 - evals[17][0], -evals[17][1], -evals[17][2]];
    signal tmp_109[3] <== CMulAdd()(tmp_107, challenges[6], tmp_108);
    signal tmp_110[3] <== [mapValues.tree3_4 - evals[18][0], -evals[18][1], -evals[18][2]];
    signal tmp_111[3] <== CMulAdd()(tmp_109, challenges[6], tmp_110);
    signal tmp_112[3] <== [consts[14] - evals[19][0], -evals[19][1], -evals[19][2]];
    signal tmp_113[3] <== CMulAdd()(tmp_111, challenges[6], tmp_112);
    signal tmp_114[3] <== [mapValues.tree3_6 - evals[20][0], -evals[20][1], -evals[20][2]];
    signal tmp_115[3] <== CMulAdd()(tmp_113, challenges[6], tmp_114);
    signal tmp_116[3] <== [mapValues.tree3_9 - evals[21][0], -evals[21][1], -evals[21][2]];
    signal tmp_117[3] <== CMulAdd()(tmp_115, challenges[6], tmp_116);
    signal tmp_118[3] <== [mapValues.tree3_12 - evals[22][0], -evals[22][1], -evals[22][2]];
    signal tmp_119[3] <== CMulAdd()(tmp_117, challenges[6], tmp_118);
    signal tmp_120[3] <== [mapValues.tree3_15 - evals[23][0], -evals[23][1], -evals[23][2]];
    signal tmp_121[3] <== CMulAdd()(tmp_119, challenges[6], tmp_120);
    signal tmp_122[3] <== [mapValues.tree3_18 - evals[24][0], -evals[24][1], -evals[24][2]];
    signal tmp_123[3] <== CMulAdd()(tmp_121, challenges[6], tmp_122);
    signal tmp_124[3] <== [mapValues.tree3_21 - evals[25][0], -evals[25][1], -evals[25][2]];
    signal tmp_125[3] <== CMulAdd()(tmp_123, challenges[6], tmp_124);
    signal tmp_126[3] <== [mapValues.tree3_24 - evals[26][0], -evals[26][1], -evals[26][2]];
    signal tmp_127[3] <== CMulAdd()(tmp_125, challenges[6], tmp_126);
    signal tmp_128[3] <== [mapValues.tree3_27 - evals[27][0], -evals[27][1], -evals[27][2]];
    signal tmp_129[3] <== CMulAdd()(tmp_127, challenges[6], tmp_128);
    signal tmp_130[3] <== [mapValues.tree3_30 - evals[28][0], -evals[28][1], -evals[28][2]];
    signal tmp_131[3] <== CMulAdd()(tmp_129, challenges[6], tmp_130);
    signal tmp_132[3] <== [mapValues.tree3_33 - evals[29][0], -evals[29][1], -evals[29][2]];
    signal tmp_133[3] <== CMulAdd()(tmp_131, challenges[6], tmp_132);
    signal tmp_134[3] <== [mapValues.tree3_36 - evals[30][0], -evals[30][1], -evals[30][2]];
    signal tmp_135[3] <== CMulAdd()(tmp_133, challenges[6], tmp_134);
    signal tmp_136[3] <== [mapValues.tree3_39 - evals[31][0], -evals[31][1], -evals[31][2]];
    signal tmp_137[3] <== CMulAdd()(tmp_135, challenges[6], tmp_136);
    signal tmp_138[3] <== [consts[27] - evals[33][0], -evals[33][1], -evals[33][2]];
    signal tmp_139[3] <== CMulAdd()(tmp_137, challenges[6], tmp_138);
    signal tmp_140[3] <== [consts[20] - evals[45][0], -evals[45][1], -evals[45][2]];
    signal tmp_141[3] <== CMulAdd()(tmp_139, challenges[6], tmp_140);
    signal tmp_142[3] <== [consts[24] - evals[46][0], -evals[46][1], -evals[46][2]];
    signal tmp_143[3] <== CMulAdd()(tmp_141, challenges[6], tmp_142);
    signal tmp_144[3] <== [mapValues.tree3_42 - evals[47][0], -evals[47][1], -evals[47][2]];
    signal tmp_145[3] <== CMulAdd()(tmp_143, challenges[6], tmp_144);
    signal tmp_146[3] <== [mapValues.tree3_43 - evals[48][0], -evals[48][1], -evals[48][2]];
    signal tmp_147[3] <== CMulAdd()(tmp_145, challenges[6], tmp_146);
    signal tmp_148[3] <== [mapValues.tree3_44 - evals[49][0], -evals[49][1], -evals[49][2]];
    signal tmp_149[3] <== CMulAdd()(tmp_147, challenges[6], tmp_148);
    signal tmp_150[3] <== [mapValues.tree3_45 - evals[50][0], -evals[50][1], -evals[50][2]];
    signal tmp_151[3] <== CMulAdd()(tmp_149, challenges[6], tmp_150);
    signal tmp_152[3] <== [consts[29] - evals[51][0], -evals[51][1], -evals[51][2]];
    signal tmp_153[3] <== CMulAdd()(tmp_151, challenges[6], tmp_152);
    signal tmp_154[3] <== [consts[21] - evals[52][0], -evals[52][1], -evals[52][2]];
    signal tmp_155[3] <== CMulAdd()(tmp_153, challenges[6], tmp_154);
    signal tmp_156[3] <== [mapValues.tree3_40 - evals[53][0], -evals[53][1], -evals[53][2]];
    signal tmp_157[3] <== CMulAdd()(tmp_155, challenges[6], tmp_156);
    signal tmp_158[3] <== [consts[22] - evals[54][0], -evals[54][1], -evals[54][2]];
    signal tmp_159[3] <== CMulAdd()(tmp_157, challenges[6], tmp_158);
    signal tmp_160[3] <== [mapValues.tree3_41 - evals[55][0], -evals[55][1], -evals[55][2]];
    signal tmp_161[3] <== CMulAdd()(tmp_159, challenges[6], tmp_160);
    signal tmp_162[3] <== [consts[15] - evals[56][0], -evals[56][1], -evals[56][2]];
    signal tmp_163[3] <== CMulAdd()(tmp_161, challenges[6], tmp_162);
    signal tmp_164[3] <== [consts[16] - evals[57][0], -evals[57][1], -evals[57][2]];
    signal tmp_165[3] <== CMulAdd()(tmp_163, challenges[6], tmp_164);
    signal tmp_166[3] <== [consts[17] - evals[58][0], -evals[58][1], -evals[58][2]];
    signal tmp_167[3] <== CMulAdd()(tmp_165, challenges[6], tmp_166);
    signal tmp_168[3] <== [consts[31] - evals[59][0], -evals[59][1], -evals[59][2]];
    signal tmp_169[3] <== CMulAdd()(tmp_167, challenges[6], tmp_168);
    signal tmp_170[3] <== [consts[18] - evals[60][0], -evals[60][1], -evals[60][2]];
    signal tmp_171[3] <== CMulAdd()(tmp_169, challenges[6], tmp_170);
    signal tmp_172[3] <== [consts[19] - evals[61][0], -evals[61][1], -evals[61][2]];
    signal tmp_173[3] <== CMulAdd()(tmp_171, challenges[6], tmp_172);
    signal tmp_174[3] <== [mapValues.tree3_49 - evals[62][0], -evals[62][1], -evals[62][2]];
    signal tmp_175[3] <== CMulAdd()(tmp_173, challenges[6], tmp_174);
    signal tmp_176[3] <== [consts[30] - evals[63][0], -evals[63][1], -evals[63][2]];
    signal tmp_177[3] <== CMulAdd()(tmp_175, challenges[6], tmp_176);
    signal tmp_178[3] <== [mapValues.tree3_50 - evals[64][0], -evals[64][1], -evals[64][2]];
    signal tmp_179[3] <== CMulAdd()(tmp_177, challenges[6], tmp_178);
    signal tmp_180[3] <== [mapValues.tree3_51 - evals[65][0], -evals[65][1], -evals[65][2]];
    signal tmp_181[3] <== CMulAdd()(tmp_179, challenges[6], tmp_180);
    signal tmp_182[3] <== [mapValues.tree3_0[0] - evals[66][0], mapValues.tree3_0[1] - evals[66][1], mapValues.tree3_0[2] - evals[66][2]];
    signal tmp_183[3] <== CMulAdd()(tmp_181, challenges[6], tmp_182);
    signal tmp_184[3] <== [consts[13] - evals[67][0], -evals[67][1], -evals[67][2]];
    signal tmp_185[3] <== CMulAdd()(tmp_183, challenges[6], tmp_184);
    signal tmp_186[3] <== [mapValues.tree3_61[0] - evals[68][0], mapValues.tree3_61[1] - evals[68][1], mapValues.tree3_61[2] - evals[68][2]];
    signal tmp_187[3] <== CMulAdd()(tmp_185, challenges[6], tmp_186);
    signal tmp_188[3] <== [mapValues.tree3_60[0] - evals[69][0], mapValues.tree3_60[1] - evals[69][1], mapValues.tree3_60[2] - evals[69][2]];
    signal tmp_189[3] <== CMulAdd()(tmp_187, challenges[6], tmp_188);
    signal tmp_190[3] <== [mapValues.tree3_5 - evals[71][0], -evals[71][1], -evals[71][2]];
    signal tmp_191[3] <== CMulAdd()(tmp_189, challenges[6], tmp_190);
    signal tmp_192[3] <== [mapValues.tree3_7 - evals[72][0], -evals[72][1], -evals[72][2]];
    signal tmp_193[3] <== CMulAdd()(tmp_191, challenges[6], tmp_192);
    signal tmp_194[3] <== [mapValues.tree3_8 - evals[73][0], -evals[73][1], -evals[73][2]];
    signal tmp_195[3] <== CMulAdd()(tmp_193, challenges[6], tmp_194);
    signal tmp_196[3] <== [mapValues.tree3_10 - evals[74][0], -evals[74][1], -evals[74][2]];
    signal tmp_197[3] <== CMulAdd()(tmp_195, challenges[6], tmp_196);
    signal tmp_198[3] <== [mapValues.tree3_11 - evals[75][0], -evals[75][1], -evals[75][2]];
    signal tmp_199[3] <== CMulAdd()(tmp_197, challenges[6], tmp_198);
    signal tmp_200[3] <== [mapValues.tree3_13 - evals[76][0], -evals[76][1], -evals[76][2]];
    signal tmp_201[3] <== CMulAdd()(tmp_199, challenges[6], tmp_200);
    signal tmp_202[3] <== [mapValues.tree3_14 - evals[77][0], -evals[77][1], -evals[77][2]];
    signal tmp_203[3] <== CMulAdd()(tmp_201, challenges[6], tmp_202);
    signal tmp_204[3] <== [mapValues.tree3_16 - evals[78][0], -evals[78][1], -evals[78][2]];
    signal tmp_205[3] <== CMulAdd()(tmp_203, challenges[6], tmp_204);
    signal tmp_206[3] <== [mapValues.tree3_17 - evals[79][0], -evals[79][1], -evals[79][2]];
    signal tmp_207[3] <== CMulAdd()(tmp_205, challenges[6], tmp_206);
    signal tmp_208[3] <== [mapValues.tree3_19 - evals[80][0], -evals[80][1], -evals[80][2]];
    signal tmp_209[3] <== CMulAdd()(tmp_207, challenges[6], tmp_208);
    signal tmp_210[3] <== [mapValues.tree3_20 - evals[81][0], -evals[81][1], -evals[81][2]];
    signal tmp_211[3] <== CMulAdd()(tmp_209, challenges[6], tmp_210);
    signal tmp_212[3] <== [mapValues.tree3_22 - evals[82][0], -evals[82][1], -evals[82][2]];
    signal tmp_213[3] <== CMulAdd()(tmp_211, challenges[6], tmp_212);
    signal tmp_214[3] <== [mapValues.tree3_23 - evals[83][0], -evals[83][1], -evals[83][2]];
    signal tmp_215[3] <== CMulAdd()(tmp_213, challenges[6], tmp_214);
    signal tmp_216[3] <== [mapValues.tree3_25 - evals[84][0], -evals[84][1], -evals[84][2]];
    signal tmp_217[3] <== CMulAdd()(tmp_215, challenges[6], tmp_216);
    signal tmp_218[3] <== [mapValues.tree3_26 - evals[85][0], -evals[85][1], -evals[85][2]];
    signal tmp_219[3] <== CMulAdd()(tmp_217, challenges[6], tmp_218);
    signal tmp_220[3] <== [mapValues.tree3_28 - evals[86][0], -evals[86][1], -evals[86][2]];
    signal tmp_221[3] <== CMulAdd()(tmp_219, challenges[6], tmp_220);
    signal tmp_222[3] <== [mapValues.tree3_29 - evals[87][0], -evals[87][1], -evals[87][2]];
    signal tmp_223[3] <== CMulAdd()(tmp_221, challenges[6], tmp_222);
    signal tmp_224[3] <== [consts[23] - evals[88][0], -evals[88][1], -evals[88][2]];
    signal tmp_225[3] <== CMulAdd()(tmp_223, challenges[6], tmp_224);
    signal tmp_226[3] <== [mapValues.tree3_31 - evals[89][0], -evals[89][1], -evals[89][2]];
    signal tmp_227[3] <== CMulAdd()(tmp_225, challenges[6], tmp_226);
    signal tmp_228[3] <== [mapValues.tree3_32 - evals[90][0], -evals[90][1], -evals[90][2]];
    signal tmp_229[3] <== CMulAdd()(tmp_227, challenges[6], tmp_228);
    signal tmp_230[3] <== [mapValues.tree3_34 - evals[91][0], -evals[91][1], -evals[91][2]];
    signal tmp_231[3] <== CMulAdd()(tmp_229, challenges[6], tmp_230);
    signal tmp_232[3] <== [mapValues.tree3_35 - evals[92][0], -evals[92][1], -evals[92][2]];
    signal tmp_233[3] <== CMulAdd()(tmp_231, challenges[6], tmp_232);
    signal tmp_234[3] <== [consts[25] - evals[93][0], -evals[93][1], -evals[93][2]];
    signal tmp_235[3] <== CMulAdd()(tmp_233, challenges[6], tmp_234);
    signal tmp_236[3] <== [mapValues.tree3_37 - evals[94][0], -evals[94][1], -evals[94][2]];
    signal tmp_237[3] <== CMulAdd()(tmp_235, challenges[6], tmp_236);
    signal tmp_238[3] <== [mapValues.tree3_38 - evals[95][0], -evals[95][1], -evals[95][2]];
    signal tmp_239[3] <== CMulAdd()(tmp_237, challenges[6], tmp_238);
    signal tmp_240[3] <== [mapValues.tree3_46 - evals[96][0], -evals[96][1], -evals[96][2]];
    signal tmp_241[3] <== CMulAdd()(tmp_239, challenges[6], tmp_240);
    signal tmp_242[3] <== [mapValues.tree3_47 - evals[97][0], -evals[97][1], -evals[97][2]];
    signal tmp_243[3] <== CMulAdd()(tmp_241, challenges[6], tmp_242);
    signal tmp_244[3] <== [mapValues.tree3_48 - evals[98][0], -evals[98][1], -evals[98][2]];
    signal tmp_245[3] <== CMulAdd()(tmp_243, challenges[6], tmp_244);
    signal tmp_246[3] <== [consts[2] - evals[99][0], -evals[99][1], -evals[99][2]];
    signal tmp_247[3] <== CMulAdd()(tmp_245, challenges[6], tmp_246);
    signal tmp_248[3] <== [consts[3] - evals[100][0], -evals[100][1], -evals[100][2]];
    signal tmp_249[3] <== CMulAdd()(tmp_247, challenges[6], tmp_248);
    signal tmp_250[3] <== [consts[4] - evals[101][0], -evals[101][1], -evals[101][2]];
    signal tmp_251[3] <== CMulAdd()(tmp_249, challenges[6], tmp_250);
    signal tmp_252[3] <== [mapValues.tree3_52[0] - evals[102][0], mapValues.tree3_52[1] - evals[102][1], mapValues.tree3_52[2] - evals[102][2]];
    signal tmp_253[3] <== CMulAdd()(tmp_251, challenges[6], tmp_252);
    signal tmp_254[3] <== [consts[5] - evals[103][0], -evals[103][1], -evals[103][2]];
    signal tmp_255[3] <== CMulAdd()(tmp_253, challenges[6], tmp_254);
    signal tmp_256[3] <== [mapValues.tree3_53[0] - evals[104][0], mapValues.tree3_53[1] - evals[104][1], mapValues.tree3_53[2] - evals[104][2]];
    signal tmp_257[3] <== CMulAdd()(tmp_255, challenges[6], tmp_256);
    signal tmp_258[3] <== [consts[6] - evals[105][0], -evals[105][1], -evals[105][2]];
    signal tmp_259[3] <== CMulAdd()(tmp_257, challenges[6], tmp_258);
    signal tmp_260[3] <== [mapValues.tree3_54[0] - evals[106][0], mapValues.tree3_54[1] - evals[106][1], mapValues.tree3_54[2] - evals[106][2]];
    signal tmp_261[3] <== CMulAdd()(tmp_259, challenges[6], tmp_260);
    signal tmp_262[3] <== [consts[7] - evals[107][0], -evals[107][1], -evals[107][2]];
    signal tmp_263[3] <== CMulAdd()(tmp_261, challenges[6], tmp_262);
    signal tmp_264[3] <== [mapValues.tree3_55[0] - evals[108][0], mapValues.tree3_55[1] - evals[108][1], mapValues.tree3_55[2] - evals[108][2]];
    signal tmp_265[3] <== CMulAdd()(tmp_263, challenges[6], tmp_264);
    signal tmp_266[3] <== [consts[8] - evals[109][0], -evals[109][1], -evals[109][2]];
    signal tmp_267[3] <== CMulAdd()(tmp_265, challenges[6], tmp_266);
    signal tmp_268[3] <== [mapValues.tree3_56[0] - evals[110][0], mapValues.tree3_56[1] - evals[110][1], mapValues.tree3_56[2] - evals[110][2]];
    signal tmp_269[3] <== CMulAdd()(tmp_267, challenges[6], tmp_268);
    signal tmp_270[3] <== [consts[9] - evals[111][0], -evals[111][1], -evals[111][2]];
    signal tmp_271[3] <== CMulAdd()(tmp_269, challenges[6], tmp_270);
    signal tmp_272[3] <== [mapValues.tree3_57[0] - evals[112][0], mapValues.tree3_57[1] - evals[112][1], mapValues.tree3_57[2] - evals[112][2]];
    signal tmp_273[3] <== CMulAdd()(tmp_271, challenges[6], tmp_272);
    signal tmp_274[3] <== [consts[10] - evals[113][0], -evals[113][1], -evals[113][2]];
    signal tmp_275[3] <== CMulAdd()(tmp_273, challenges[6], tmp_274);
    signal tmp_276[3] <== [mapValues.tree3_58[0] - evals[114][0], mapValues.tree3_58[1] - evals[114][1], mapValues.tree3_58[2] - evals[114][2]];
    signal tmp_277[3] <== CMulAdd()(tmp_275, challenges[6], tmp_276);
    signal tmp_278[3] <== [consts[11] - evals[115][0], -evals[115][1], -evals[115][2]];
    signal tmp_279[3] <== CMulAdd()(tmp_277, challenges[6], tmp_278);
    signal tmp_280[3] <== [mapValues.tree3_59[0] - evals[116][0], mapValues.tree3_59[1] - evals[116][1], mapValues.tree3_59[2] - evals[116][2]];
    signal tmp_281[3] <== CMulAdd()(tmp_279, challenges[6], tmp_280);
    signal tmp_282[3] <== [consts[12] - evals[117][0], -evals[117][1], -evals[117][2]];
    signal tmp_283[3] <== CMulAdd()(tmp_281, challenges[6], tmp_282);
    signal tmp_284[3] <== [consts[26] - evals[118][0], -evals[118][1], -evals[118][2]];
    signal tmp_285[3] <== CMulAdd()(tmp_283, challenges[6], tmp_284);
    signal tmp_286[3] <== [mapValues.tree4_0[0] - evals[119][0], mapValues.tree4_0[1] - evals[119][1], mapValues.tree4_0[2] - evals[119][2]];
    signal tmp_287[3] <== CMulAdd()(tmp_285, challenges[6], tmp_286);
    signal tmp_288[3] <== [mapValues.tree4_1[0] - evals[120][0], mapValues.tree4_1[1] - evals[120][1], mapValues.tree4_1[2] - evals[120][2]];
    signal tmp_289[3] <== CMulAdd()(tmp_287, challenges[6], tmp_288);
    signal tmp_290[3] <== CMul()(tmp_289, xDivXSubXi);
    signal tmp_291[3] <== CMulAdd()(challenges[5], tmp_74, tmp_290);
    signal tmp_292[3] <== [mapValues.tree1_0 - evals[32][0], -evals[32][1], -evals[32][2]];
    signal tmp_293[3] <== [mapValues.tree1_1 - evals[34][0], -evals[34][1], -evals[34][2]];
    signal tmp_294[3] <== CMulAdd()(tmp_292, challenges[6], tmp_293);
    signal tmp_295[3] <== [mapValues.tree1_2 - evals[35][0], -evals[35][1], -evals[35][2]];
    signal tmp_296[3] <== CMulAdd()(tmp_294, challenges[6], tmp_295);
    signal tmp_297[3] <== [mapValues.tree1_3 - evals[36][0], -evals[36][1], -evals[36][2]];
    signal tmp_298[3] <== CMulAdd()(tmp_296, challenges[6], tmp_297);
    signal tmp_299[3] <== [mapValues.tree1_4 - evals[37][0], -evals[37][1], -evals[37][2]];
    signal tmp_300[3] <== CMulAdd()(tmp_298, challenges[6], tmp_299);
    signal tmp_301[3] <== [mapValues.tree1_5 - evals[38][0], -evals[38][1], -evals[38][2]];
    signal tmp_302[3] <== CMulAdd()(tmp_300, challenges[6], tmp_301);
    signal tmp_303[3] <== [mapValues.tree1_6 - evals[39][0], -evals[39][1], -evals[39][2]];
    signal tmp_304[3] <== CMulAdd()(tmp_302, challenges[6], tmp_303);
    signal tmp_305[3] <== [mapValues.tree1_7 - evals[40][0], -evals[40][1], -evals[40][2]];
    signal tmp_306[3] <== CMulAdd()(tmp_304, challenges[6], tmp_305);
    signal tmp_307[3] <== [mapValues.tree1_8 - evals[41][0], -evals[41][1], -evals[41][2]];
    signal tmp_308[3] <== CMulAdd()(tmp_306, challenges[6], tmp_307);
    signal tmp_309[3] <== [mapValues.tree1_9 - evals[42][0], -evals[42][1], -evals[42][2]];
    signal tmp_310[3] <== CMulAdd()(tmp_308, challenges[6], tmp_309);
    signal tmp_311[3] <== [mapValues.tree1_10 - evals[43][0], -evals[43][1], -evals[43][2]];
    signal tmp_312[3] <== CMulAdd()(tmp_310, challenges[6], tmp_311);
    signal tmp_313[3] <== [mapValues.tree1_11 - evals[44][0], -evals[44][1], -evals[44][2]];
    signal tmp_314[3] <== CMulAdd()(tmp_312, challenges[6], tmp_313);
    signal tmp_315[3] <== [mapValues.tree3_0[0] - evals[70][0], mapValues.tree3_0[1] - evals[70][1], mapValues.tree3_0[2] - evals[70][2]];
    signal tmp_316[3] <== CMulAdd()(tmp_314, challenges[6], tmp_315);
    signal tmp_317[3] <== CMul()(tmp_316, xDivXSubWXi);
    signal tmp_318[3] <== CMulAdd()(challenges[5], tmp_291, tmp_317);
    out[0] <== tmp_318[0];
    out[1] <== tmp_318[1];
    out[2] <== tmp_318[2];
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
    signal input publics[14];
    signal input root1[4];
    signal input root2[4];
    signal input root3[4];
    signal input root4[4];

    signal input rootC[4];

    signal input evals[121][3];
    signal input s0_vals1[6][12];
    
    signal input s0_vals3[6][84];
        
    signal input s0_vals4[6][6];
    signal input s0_valsC[6][32];
    signal input s0_siblings1[6][18][4];

    signal input s0_siblings3[6][18][4];
        
    signal input s0_siblings4[6][18][4];
    signal input s0_siblingsC[6][18][4];
        
    signal input s1_root[4];
        
    signal input s2_root[4];
        
    signal input s3_root[4];
        
    signal input s1_vals[6][384];
    signal input s1_siblings[6][11][4];
        
    signal input s2_vals[6][48];
    signal input s2_siblings[6][7][4];
        
    signal input s3_vals[6][24];
    signal input s3_siblings[6][4][4];
        
    signal input finalPol[16][3];
    
    signal enable;
    enable <== 1;
    
    signal challenges[8][3];
    
    signal s0_specialX[3];
    
    signal s1_specialX[3];
    
    signal s2_specialX[3];
    
    signal s3_specialX[3];
    
    signal ys[6][18];
        signal tcHahs_0[12] <== Poseidon(12)([publics[0],publics[1],publics[2],publics[3],publics[4],publics[5],publics[6],publics[7]], [0,0,0,0]);
    signal tcHahs_1[12] <== Poseidon(12)([publics[8],publics[9],publics[10],publics[11],publics[12],publics[13],root1[0],root1[1]], [tcHahs_0[0],tcHahs_0[1],tcHahs_0[2],tcHahs_0[3]]);
    signal tcHahs_2[12] <==  Poseidon(12)([root1[2],root1[3],0,0,0,0,0,0], [tcHahs_1[0],tcHahs_1[1],tcHahs_1[2],tcHahs_1[3]]);
    challenges[0][0] <== tcHahs_2[0];
    challenges[0][1] <== tcHahs_2[1];
    challenges[0][2] <== tcHahs_2[2];
    challenges[1][0] <== tcHahs_2[3];
    challenges[1][1] <== tcHahs_2[4];
    challenges[1][2] <== tcHahs_2[5];
    signal tcHahs_3[12] <==  Poseidon(12)([root2[0],root2[1],root2[2],root2[3],0,0,0,0], [tcHahs_2[0],tcHahs_2[1],tcHahs_2[2],tcHahs_2[3]]);
    challenges[2][0] <== tcHahs_3[0];
    challenges[2][1] <== tcHahs_3[1];
    challenges[2][2] <== tcHahs_3[2];
    challenges[3][0] <== tcHahs_3[3];
    challenges[3][1] <== tcHahs_3[4];
    challenges[3][2] <== tcHahs_3[5];
    signal tcHahs_4[12] <==  Poseidon(12)([root3[0],root3[1],root3[2],root3[3],0,0,0,0], [tcHahs_3[0],tcHahs_3[1],tcHahs_3[2],tcHahs_3[3]]);
    challenges[4][0] <== tcHahs_4[0];
    challenges[4][1] <== tcHahs_4[1];
    challenges[4][2] <== tcHahs_4[2];
    signal tcHahs_5[12] <==  Poseidon(12)([root4[0],root4[1],root4[2],root4[3],0,0,0,0], [tcHahs_4[0],tcHahs_4[1],tcHahs_4[2],tcHahs_4[3]]);
    challenges[7][0] <== tcHahs_5[0];
    challenges[7][1] <== tcHahs_5[1];
    challenges[7][2] <== tcHahs_5[2];
    signal tcHahs_6[12] <== Poseidon(12)([evals[0][0],evals[0][1],evals[0][2],evals[1][0],evals[1][1],evals[1][2],evals[2][0],evals[2][1]], [tcHahs_5[0],tcHahs_5[1],tcHahs_5[2],tcHahs_5[3]]);
    signal tcHahs_7[12] <== Poseidon(12)([evals[2][2],evals[3][0],evals[3][1],evals[3][2],evals[4][0],evals[4][1],evals[4][2],evals[5][0]], [tcHahs_6[0],tcHahs_6[1],tcHahs_6[2],tcHahs_6[3]]);
    signal tcHahs_8[12] <== Poseidon(12)([evals[5][1],evals[5][2],evals[6][0],evals[6][1],evals[6][2],evals[7][0],evals[7][1],evals[7][2]], [tcHahs_7[0],tcHahs_7[1],tcHahs_7[2],tcHahs_7[3]]);
    signal tcHahs_9[12] <== Poseidon(12)([evals[8][0],evals[8][1],evals[8][2],evals[9][0],evals[9][1],evals[9][2],evals[10][0],evals[10][1]], [tcHahs_8[0],tcHahs_8[1],tcHahs_8[2],tcHahs_8[3]]);
    signal tcHahs_10[12] <== Poseidon(12)([evals[10][2],evals[11][0],evals[11][1],evals[11][2],evals[12][0],evals[12][1],evals[12][2],evals[13][0]], [tcHahs_9[0],tcHahs_9[1],tcHahs_9[2],tcHahs_9[3]]);
    signal tcHahs_11[12] <== Poseidon(12)([evals[13][1],evals[13][2],evals[14][0],evals[14][1],evals[14][2],evals[15][0],evals[15][1],evals[15][2]], [tcHahs_10[0],tcHahs_10[1],tcHahs_10[2],tcHahs_10[3]]);
    signal tcHahs_12[12] <== Poseidon(12)([evals[16][0],evals[16][1],evals[16][2],evals[17][0],evals[17][1],evals[17][2],evals[18][0],evals[18][1]], [tcHahs_11[0],tcHahs_11[1],tcHahs_11[2],tcHahs_11[3]]);
    signal tcHahs_13[12] <== Poseidon(12)([evals[18][2],evals[19][0],evals[19][1],evals[19][2],evals[20][0],evals[20][1],evals[20][2],evals[21][0]], [tcHahs_12[0],tcHahs_12[1],tcHahs_12[2],tcHahs_12[3]]);
    signal tcHahs_14[12] <== Poseidon(12)([evals[21][1],evals[21][2],evals[22][0],evals[22][1],evals[22][2],evals[23][0],evals[23][1],evals[23][2]], [tcHahs_13[0],tcHahs_13[1],tcHahs_13[2],tcHahs_13[3]]);
    signal tcHahs_15[12] <== Poseidon(12)([evals[24][0],evals[24][1],evals[24][2],evals[25][0],evals[25][1],evals[25][2],evals[26][0],evals[26][1]], [tcHahs_14[0],tcHahs_14[1],tcHahs_14[2],tcHahs_14[3]]);
    signal tcHahs_16[12] <== Poseidon(12)([evals[26][2],evals[27][0],evals[27][1],evals[27][2],evals[28][0],evals[28][1],evals[28][2],evals[29][0]], [tcHahs_15[0],tcHahs_15[1],tcHahs_15[2],tcHahs_15[3]]);
    signal tcHahs_17[12] <== Poseidon(12)([evals[29][1],evals[29][2],evals[30][0],evals[30][1],evals[30][2],evals[31][0],evals[31][1],evals[31][2]], [tcHahs_16[0],tcHahs_16[1],tcHahs_16[2],tcHahs_16[3]]);
    signal tcHahs_18[12] <== Poseidon(12)([evals[32][0],evals[32][1],evals[32][2],evals[33][0],evals[33][1],evals[33][2],evals[34][0],evals[34][1]], [tcHahs_17[0],tcHahs_17[1],tcHahs_17[2],tcHahs_17[3]]);
    signal tcHahs_19[12] <== Poseidon(12)([evals[34][2],evals[35][0],evals[35][1],evals[35][2],evals[36][0],evals[36][1],evals[36][2],evals[37][0]], [tcHahs_18[0],tcHahs_18[1],tcHahs_18[2],tcHahs_18[3]]);
    signal tcHahs_20[12] <== Poseidon(12)([evals[37][1],evals[37][2],evals[38][0],evals[38][1],evals[38][2],evals[39][0],evals[39][1],evals[39][2]], [tcHahs_19[0],tcHahs_19[1],tcHahs_19[2],tcHahs_19[3]]);
    signal tcHahs_21[12] <== Poseidon(12)([evals[40][0],evals[40][1],evals[40][2],evals[41][0],evals[41][1],evals[41][2],evals[42][0],evals[42][1]], [tcHahs_20[0],tcHahs_20[1],tcHahs_20[2],tcHahs_20[3]]);
    signal tcHahs_22[12] <== Poseidon(12)([evals[42][2],evals[43][0],evals[43][1],evals[43][2],evals[44][0],evals[44][1],evals[44][2],evals[45][0]], [tcHahs_21[0],tcHahs_21[1],tcHahs_21[2],tcHahs_21[3]]);
    signal tcHahs_23[12] <== Poseidon(12)([evals[45][1],evals[45][2],evals[46][0],evals[46][1],evals[46][2],evals[47][0],evals[47][1],evals[47][2]], [tcHahs_22[0],tcHahs_22[1],tcHahs_22[2],tcHahs_22[3]]);
    signal tcHahs_24[12] <== Poseidon(12)([evals[48][0],evals[48][1],evals[48][2],evals[49][0],evals[49][1],evals[49][2],evals[50][0],evals[50][1]], [tcHahs_23[0],tcHahs_23[1],tcHahs_23[2],tcHahs_23[3]]);
    signal tcHahs_25[12] <== Poseidon(12)([evals[50][2],evals[51][0],evals[51][1],evals[51][2],evals[52][0],evals[52][1],evals[52][2],evals[53][0]], [tcHahs_24[0],tcHahs_24[1],tcHahs_24[2],tcHahs_24[3]]);
    signal tcHahs_26[12] <== Poseidon(12)([evals[53][1],evals[53][2],evals[54][0],evals[54][1],evals[54][2],evals[55][0],evals[55][1],evals[55][2]], [tcHahs_25[0],tcHahs_25[1],tcHahs_25[2],tcHahs_25[3]]);
    signal tcHahs_27[12] <== Poseidon(12)([evals[56][0],evals[56][1],evals[56][2],evals[57][0],evals[57][1],evals[57][2],evals[58][0],evals[58][1]], [tcHahs_26[0],tcHahs_26[1],tcHahs_26[2],tcHahs_26[3]]);
    signal tcHahs_28[12] <== Poseidon(12)([evals[58][2],evals[59][0],evals[59][1],evals[59][2],evals[60][0],evals[60][1],evals[60][2],evals[61][0]], [tcHahs_27[0],tcHahs_27[1],tcHahs_27[2],tcHahs_27[3]]);
    signal tcHahs_29[12] <== Poseidon(12)([evals[61][1],evals[61][2],evals[62][0],evals[62][1],evals[62][2],evals[63][0],evals[63][1],evals[63][2]], [tcHahs_28[0],tcHahs_28[1],tcHahs_28[2],tcHahs_28[3]]);
    signal tcHahs_30[12] <== Poseidon(12)([evals[64][0],evals[64][1],evals[64][2],evals[65][0],evals[65][1],evals[65][2],evals[66][0],evals[66][1]], [tcHahs_29[0],tcHahs_29[1],tcHahs_29[2],tcHahs_29[3]]);
    signal tcHahs_31[12] <== Poseidon(12)([evals[66][2],evals[67][0],evals[67][1],evals[67][2],evals[68][0],evals[68][1],evals[68][2],evals[69][0]], [tcHahs_30[0],tcHahs_30[1],tcHahs_30[2],tcHahs_30[3]]);
    signal tcHahs_32[12] <== Poseidon(12)([evals[69][1],evals[69][2],evals[70][0],evals[70][1],evals[70][2],evals[71][0],evals[71][1],evals[71][2]], [tcHahs_31[0],tcHahs_31[1],tcHahs_31[2],tcHahs_31[3]]);
    signal tcHahs_33[12] <== Poseidon(12)([evals[72][0],evals[72][1],evals[72][2],evals[73][0],evals[73][1],evals[73][2],evals[74][0],evals[74][1]], [tcHahs_32[0],tcHahs_32[1],tcHahs_32[2],tcHahs_32[3]]);
    signal tcHahs_34[12] <== Poseidon(12)([evals[74][2],evals[75][0],evals[75][1],evals[75][2],evals[76][0],evals[76][1],evals[76][2],evals[77][0]], [tcHahs_33[0],tcHahs_33[1],tcHahs_33[2],tcHahs_33[3]]);
    signal tcHahs_35[12] <== Poseidon(12)([evals[77][1],evals[77][2],evals[78][0],evals[78][1],evals[78][2],evals[79][0],evals[79][1],evals[79][2]], [tcHahs_34[0],tcHahs_34[1],tcHahs_34[2],tcHahs_34[3]]);
    signal tcHahs_36[12] <== Poseidon(12)([evals[80][0],evals[80][1],evals[80][2],evals[81][0],evals[81][1],evals[81][2],evals[82][0],evals[82][1]], [tcHahs_35[0],tcHahs_35[1],tcHahs_35[2],tcHahs_35[3]]);
    signal tcHahs_37[12] <== Poseidon(12)([evals[82][2],evals[83][0],evals[83][1],evals[83][2],evals[84][0],evals[84][1],evals[84][2],evals[85][0]], [tcHahs_36[0],tcHahs_36[1],tcHahs_36[2],tcHahs_36[3]]);
    signal tcHahs_38[12] <== Poseidon(12)([evals[85][1],evals[85][2],evals[86][0],evals[86][1],evals[86][2],evals[87][0],evals[87][1],evals[87][2]], [tcHahs_37[0],tcHahs_37[1],tcHahs_37[2],tcHahs_37[3]]);
    signal tcHahs_39[12] <== Poseidon(12)([evals[88][0],evals[88][1],evals[88][2],evals[89][0],evals[89][1],evals[89][2],evals[90][0],evals[90][1]], [tcHahs_38[0],tcHahs_38[1],tcHahs_38[2],tcHahs_38[3]]);
    signal tcHahs_40[12] <== Poseidon(12)([evals[90][2],evals[91][0],evals[91][1],evals[91][2],evals[92][0],evals[92][1],evals[92][2],evals[93][0]], [tcHahs_39[0],tcHahs_39[1],tcHahs_39[2],tcHahs_39[3]]);
    signal tcHahs_41[12] <== Poseidon(12)([evals[93][1],evals[93][2],evals[94][0],evals[94][1],evals[94][2],evals[95][0],evals[95][1],evals[95][2]], [tcHahs_40[0],tcHahs_40[1],tcHahs_40[2],tcHahs_40[3]]);
    signal tcHahs_42[12] <== Poseidon(12)([evals[96][0],evals[96][1],evals[96][2],evals[97][0],evals[97][1],evals[97][2],evals[98][0],evals[98][1]], [tcHahs_41[0],tcHahs_41[1],tcHahs_41[2],tcHahs_41[3]]);
    signal tcHahs_43[12] <== Poseidon(12)([evals[98][2],evals[99][0],evals[99][1],evals[99][2],evals[100][0],evals[100][1],evals[100][2],evals[101][0]], [tcHahs_42[0],tcHahs_42[1],tcHahs_42[2],tcHahs_42[3]]);
    signal tcHahs_44[12] <== Poseidon(12)([evals[101][1],evals[101][2],evals[102][0],evals[102][1],evals[102][2],evals[103][0],evals[103][1],evals[103][2]], [tcHahs_43[0],tcHahs_43[1],tcHahs_43[2],tcHahs_43[3]]);
    signal tcHahs_45[12] <== Poseidon(12)([evals[104][0],evals[104][1],evals[104][2],evals[105][0],evals[105][1],evals[105][2],evals[106][0],evals[106][1]], [tcHahs_44[0],tcHahs_44[1],tcHahs_44[2],tcHahs_44[3]]);
    signal tcHahs_46[12] <== Poseidon(12)([evals[106][2],evals[107][0],evals[107][1],evals[107][2],evals[108][0],evals[108][1],evals[108][2],evals[109][0]], [tcHahs_45[0],tcHahs_45[1],tcHahs_45[2],tcHahs_45[3]]);
    signal tcHahs_47[12] <== Poseidon(12)([evals[109][1],evals[109][2],evals[110][0],evals[110][1],evals[110][2],evals[111][0],evals[111][1],evals[111][2]], [tcHahs_46[0],tcHahs_46[1],tcHahs_46[2],tcHahs_46[3]]);
    signal tcHahs_48[12] <== Poseidon(12)([evals[112][0],evals[112][1],evals[112][2],evals[113][0],evals[113][1],evals[113][2],evals[114][0],evals[114][1]], [tcHahs_47[0],tcHahs_47[1],tcHahs_47[2],tcHahs_47[3]]);
    signal tcHahs_49[12] <== Poseidon(12)([evals[114][2],evals[115][0],evals[115][1],evals[115][2],evals[116][0],evals[116][1],evals[116][2],evals[117][0]], [tcHahs_48[0],tcHahs_48[1],tcHahs_48[2],tcHahs_48[3]]);
    signal tcHahs_50[12] <== Poseidon(12)([evals[117][1],evals[117][2],evals[118][0],evals[118][1],evals[118][2],evals[119][0],evals[119][1],evals[119][2]], [tcHahs_49[0],tcHahs_49[1],tcHahs_49[2],tcHahs_49[3]]);
    signal tcHahs_51[12] <==  Poseidon(12)([evals[120][0],evals[120][1],evals[120][2],0,0,0,0,0], [tcHahs_50[0],tcHahs_50[1],tcHahs_50[2],tcHahs_50[3]]);
    challenges[5][0] <== tcHahs_51[0];
    challenges[5][1] <== tcHahs_51[1];
    challenges[5][2] <== tcHahs_51[2];
    challenges[6][0] <== tcHahs_51[3];
    challenges[6][1] <== tcHahs_51[4];
    challenges[6][2] <== tcHahs_51[5];
    s0_specialX[0] <== tcHahs_51[6];
    s0_specialX[1] <== tcHahs_51[7];
    s0_specialX[2] <== tcHahs_51[8];
    signal tcHahs_52[12] <==  Poseidon(12)([s1_root[0],s1_root[1],s1_root[2],s1_root[3],0,0,0,0], [tcHahs_51[0],tcHahs_51[1],tcHahs_51[2],tcHahs_51[3]]);
    s1_specialX[0] <== tcHahs_52[0];
    s1_specialX[1] <== tcHahs_52[1];
    s1_specialX[2] <== tcHahs_52[2];
    signal tcHahs_53[12] <==  Poseidon(12)([s2_root[0],s2_root[1],s2_root[2],s2_root[3],0,0,0,0], [tcHahs_52[0],tcHahs_52[1],tcHahs_52[2],tcHahs_52[3]]);
    s2_specialX[0] <== tcHahs_53[0];
    s2_specialX[1] <== tcHahs_53[1];
    s2_specialX[2] <== tcHahs_53[2];
    signal tcHahs_54[12] <==  Poseidon(12)([s3_root[0],s3_root[1],s3_root[2],s3_root[3],0,0,0,0], [tcHahs_53[0],tcHahs_53[1],tcHahs_53[2],tcHahs_53[3]]);
    s3_specialX[0] <== tcHahs_54[0];
    s3_specialX[1] <== tcHahs_54[1];
    s3_specialX[2] <== tcHahs_54[2];
    signal tcHahs_55[12] <== Poseidon(12)([finalPol[0][0],finalPol[0][1],finalPol[0][2],finalPol[1][0],finalPol[1][1],finalPol[1][2],finalPol[2][0],finalPol[2][1]], [tcHahs_54[0],tcHahs_54[1],tcHahs_54[2],tcHahs_54[3]]);
    signal tcHahs_56[12] <== Poseidon(12)([finalPol[2][2],finalPol[3][0],finalPol[3][1],finalPol[3][2],finalPol[4][0],finalPol[4][1],finalPol[4][2],finalPol[5][0]], [tcHahs_55[0],tcHahs_55[1],tcHahs_55[2],tcHahs_55[3]]);
    signal tcHahs_57[12] <== Poseidon(12)([finalPol[5][1],finalPol[5][2],finalPol[6][0],finalPol[6][1],finalPol[6][2],finalPol[7][0],finalPol[7][1],finalPol[7][2]], [tcHahs_56[0],tcHahs_56[1],tcHahs_56[2],tcHahs_56[3]]);
    signal tcHahs_58[12] <== Poseidon(12)([finalPol[8][0],finalPol[8][1],finalPol[8][2],finalPol[9][0],finalPol[9][1],finalPol[9][2],finalPol[10][0],finalPol[10][1]], [tcHahs_57[0],tcHahs_57[1],tcHahs_57[2],tcHahs_57[3]]);
    signal tcHahs_59[12] <== Poseidon(12)([finalPol[10][2],finalPol[11][0],finalPol[11][1],finalPol[11][2],finalPol[12][0],finalPol[12][1],finalPol[12][2],finalPol[13][0]], [tcHahs_58[0],tcHahs_58[1],tcHahs_58[2],tcHahs_58[3]]);
    signal tcHahs_60[12] <== Poseidon(12)([finalPol[13][1],finalPol[13][2],finalPol[14][0],finalPol[14][1],finalPol[14][2],finalPol[15][0],finalPol[15][1],finalPol[15][2]], [tcHahs_59[0],tcHahs_59[1],tcHahs_59[2],tcHahs_59[3]]);
    component tcN2b_0 = Num2Bits_strict();
    tcN2b_0.in <== tcHahs_60[0];
    component tcN2b_1 = Num2Bits_strict();
    tcN2b_1.in <== tcHahs_60[1];
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
    ys[0][17] <== tcN2b_0.out[17];
    ys[1][0] <== tcN2b_0.out[18];
    ys[1][1] <== tcN2b_0.out[19];
    ys[1][2] <== tcN2b_0.out[20];
    ys[1][3] <== tcN2b_0.out[21];
    ys[1][4] <== tcN2b_0.out[22];
    ys[1][5] <== tcN2b_0.out[23];
    ys[1][6] <== tcN2b_0.out[24];
    ys[1][7] <== tcN2b_0.out[25];
    ys[1][8] <== tcN2b_0.out[26];
    ys[1][9] <== tcN2b_0.out[27];
    ys[1][10] <== tcN2b_0.out[28];
    ys[1][11] <== tcN2b_0.out[29];
    ys[1][12] <== tcN2b_0.out[30];
    ys[1][13] <== tcN2b_0.out[31];
    ys[1][14] <== tcN2b_0.out[32];
    ys[1][15] <== tcN2b_0.out[33];
    ys[1][16] <== tcN2b_0.out[34];
    ys[1][17] <== tcN2b_0.out[35];
    ys[2][0] <== tcN2b_0.out[36];
    ys[2][1] <== tcN2b_0.out[37];
    ys[2][2] <== tcN2b_0.out[38];
    ys[2][3] <== tcN2b_0.out[39];
    ys[2][4] <== tcN2b_0.out[40];
    ys[2][5] <== tcN2b_0.out[41];
    ys[2][6] <== tcN2b_0.out[42];
    ys[2][7] <== tcN2b_0.out[43];
    ys[2][8] <== tcN2b_0.out[44];
    ys[2][9] <== tcN2b_0.out[45];
    ys[2][10] <== tcN2b_0.out[46];
    ys[2][11] <== tcN2b_0.out[47];
    ys[2][12] <== tcN2b_0.out[48];
    ys[2][13] <== tcN2b_0.out[49];
    ys[2][14] <== tcN2b_0.out[50];
    ys[2][15] <== tcN2b_0.out[51];
    ys[2][16] <== tcN2b_0.out[52];
    ys[2][17] <== tcN2b_0.out[53];
    ys[3][0] <== tcN2b_0.out[54];
    ys[3][1] <== tcN2b_0.out[55];
    ys[3][2] <== tcN2b_0.out[56];
    ys[3][3] <== tcN2b_0.out[57];
    ys[3][4] <== tcN2b_0.out[58];
    ys[3][5] <== tcN2b_0.out[59];
    ys[3][6] <== tcN2b_0.out[60];
    ys[3][7] <== tcN2b_0.out[61];
    ys[3][8] <== tcN2b_0.out[62];
    ys[3][9] <== tcN2b_1.out[0];
    ys[3][10] <== tcN2b_1.out[1];
    ys[3][11] <== tcN2b_1.out[2];
    ys[3][12] <== tcN2b_1.out[3];
    ys[3][13] <== tcN2b_1.out[4];
    ys[3][14] <== tcN2b_1.out[5];
    ys[3][15] <== tcN2b_1.out[6];
    ys[3][16] <== tcN2b_1.out[7];
    ys[3][17] <== tcN2b_1.out[8];
    ys[4][0] <== tcN2b_1.out[9];
    ys[4][1] <== tcN2b_1.out[10];
    ys[4][2] <== tcN2b_1.out[11];
    ys[4][3] <== tcN2b_1.out[12];
    ys[4][4] <== tcN2b_1.out[13];
    ys[4][5] <== tcN2b_1.out[14];
    ys[4][6] <== tcN2b_1.out[15];
    ys[4][7] <== tcN2b_1.out[16];
    ys[4][8] <== tcN2b_1.out[17];
    ys[4][9] <== tcN2b_1.out[18];
    ys[4][10] <== tcN2b_1.out[19];
    ys[4][11] <== tcN2b_1.out[20];
    ys[4][12] <== tcN2b_1.out[21];
    ys[4][13] <== tcN2b_1.out[22];
    ys[4][14] <== tcN2b_1.out[23];
    ys[4][15] <== tcN2b_1.out[24];
    ys[4][16] <== tcN2b_1.out[25];
    ys[4][17] <== tcN2b_1.out[26];
    ys[5][0] <== tcN2b_1.out[27];
    ys[5][1] <== tcN2b_1.out[28];
    ys[5][2] <== tcN2b_1.out[29];
    ys[5][3] <== tcN2b_1.out[30];
    ys[5][4] <== tcN2b_1.out[31];
    ys[5][5] <== tcN2b_1.out[32];
    ys[5][6] <== tcN2b_1.out[33];
    ys[5][7] <== tcN2b_1.out[34];
    ys[5][8] <== tcN2b_1.out[35];
    ys[5][9] <== tcN2b_1.out[36];
    ys[5][10] <== tcN2b_1.out[37];
    ys[5][11] <== tcN2b_1.out[38];
    ys[5][12] <== tcN2b_1.out[39];
    ys[5][13] <== tcN2b_1.out[40];
    ys[5][14] <== tcN2b_1.out[41];
    ys[5][15] <== tcN2b_1.out[42];
    ys[5][16] <== tcN2b_1.out[43];
    ys[5][17] <== tcN2b_1.out[44];
    component verifyEvaluations = VerifyEvaluations();
    verifyEvaluations.enable <== enable;
    for (var i=0; i<8; i++) {
        for (var k=0; k<3; k++) {
            verifyEvaluations.challenges[i][k] <== challenges[i][k];
        }
    }
    for (var i=0; i<14; i++) {
        verifyEvaluations.publics[i] <== publics[i];
    }
    for (var i=0; i<121; i++) {
        for (var k=0; k<3; k++) {
            verifyEvaluations.evals[i][k] <== evals[i][k];
        }
    }
    
    component verifyQueries[6];
    component s0_merkle1[6];
    
    component s0_merkle3[6];
    
    component s0_merkle4[6];
    component s0_merkleC[6];
    component s0_lowValues[6];
    
    for (var q=0; q<6; q++) {
        verifyQueries[q] = VerifyQuery();
        s0_merkle1[q] = MerkleHash(1, 12, 262144);
    
        s0_merkle3[q] = MerkleHash(1, 84, 262144);
    
        s0_merkle4[q] = MerkleHash(1, 6, 262144);
        s0_merkleC[q] = MerkleHash(1, 32, 262144);
        s0_lowValues[q] = TreeSelector(7, 3) ;
    
        for (var i=0; i<18; i++ ) {
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
        for (var i=0; i<18;i++) {
            for (var j=0; j<4; j++) {
                s0_merkle1[q].siblings[i][j] <== s0_siblings1[q][i][j];
    
                s0_merkle3[q].siblings[i][j] <== s0_siblings3[q][i][j];
        
                s0_merkle4[q].siblings[i][j] <== s0_siblings4[q][i][j];
                s0_merkleC[q].siblings[i][j] <== s0_siblingsC[q][i][j];
            }
        }
        
        for (var i=0; i<128; i++) {
            for (var e=0; e<3; e++) {
                s0_lowValues[q].values[i][e] <== s1_vals[q][i*3+e];
            }
        }
        for (var i=0; i<7; i++) {
            s0_lowValues[q].key[i] <== ys[q][i + 11];
        }
        
    }
        
    component s1_merkle[6];
    component s1_fft[6];
    component s1_evalPol[6];
    component s1_lowValues[6];
    signal s1_sx[6][11];
        
    for (var q=0; q<6; q++) {
        s1_merkle[q] = MerkleHash(3, 128, 2048);
        s1_fft[q] = FFT(7, 3, 1);
        s1_evalPol[q] = EvalPol(128);
        s1_lowValues[q] = TreeSelector(4, 3) ;
        for (var i=0; i< 128; i++) {
            for (var e=0; e<3; e++) {
                s1_merkle[q].values[i][e] <== s1_vals[q][i*3+e];
                s1_fft[q].in[i][e] <== s1_vals[q][i*3+e];
            }
        }
        
        for (var i=0; i<11; i++) {
            for (var j=0; j<4; j++) {
                s1_merkle[q].siblings[i][j] <== s1_siblings[q][i][j];
            }
            s1_merkle[q].key[i] <== ys[q][i];
        }
        s1_sx[q][0] <==  5646962470228954384 *  ( ys[q][0] * 2513567076326282709 +1);
        for (var i=1; i<11; i++) {
            s1_sx[q][i] <== s1_sx[q][i-1] *  ( ys[q][i] * ((1/roots(18 -i)) -1) +1);
        }
        for (var i=0; i< 128; i++) {
            for (var e=0; e<3; e++) {
                s1_evalPol[q].pol[i][e] <== s1_fft[q].out[i][e];
            }
        }
        for (var e=0; e<3; e++) {
            s1_evalPol[q].x[e] <== s1_specialX[e] *  s1_sx[q][10];
        }
        
        for (var i=0; i<16; i++) {
            for (var e=0; e<3; e++) {
                s1_lowValues[q].values[i][e] <== s2_vals[q][i*3+e];
            }
        }
        for (var i=0; i<4; i++) {
            s1_lowValues[q].key[i] <== ys[q][i + 7];
        }
        
        for(var q = 0; q < 6; q ++) {
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
        
    component s2_merkle[6];
    component s2_fft[6];
    component s2_evalPol[6];
    component s2_lowValues[6];
    signal s2_sx[6][7];
        
    for (var q=0; q<6; q++) {
        s2_merkle[q] = MerkleHash(3, 16, 128);
        s2_fft[q] = FFT(4, 3, 1);
        s2_evalPol[q] = EvalPol(16);
        s2_lowValues[q] = TreeSelector(3, 3) ;
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s2_merkle[q].values[i][e] <== s2_vals[q][i*3+e];
                s2_fft[q].in[i][e] <== s2_vals[q][i*3+e];
            }
        }
        
        for (var i=0; i<7; i++) {
            for (var j=0; j<4; j++) {
                s2_merkle[q].siblings[i][j] <== s2_siblings[q][i][j];
            }
            s2_merkle[q].key[i] <== ys[q][i];
        }
        s2_sx[q][0] <==  16627473974463641638 *  ( ys[q][0] * 8548973421900915980 +1);
        for (var i=1; i<7; i++) {
            s2_sx[q][i] <== s2_sx[q][i-1] *  ( ys[q][i] * ((1/roots(11 -i)) -1) +1);
        }
        for (var i=0; i< 16; i++) {
            for (var e=0; e<3; e++) {
                s2_evalPol[q].pol[i][e] <== s2_fft[q].out[i][e];
            }
        }
        for (var e=0; e<3; e++) {
            s2_evalPol[q].x[e] <== s2_specialX[e] *  s2_sx[q][6];
        }
        
        for (var i=0; i<8; i++) {
            for (var e=0; e<3; e++) {
                s2_lowValues[q].values[i][e] <== s3_vals[q][i*3+e];
            }
        }
        for (var i=0; i<3; i++) {
            s2_lowValues[q].key[i] <== ys[q][i + 4];
        }
        
        for(var q = 0; q < 6; q ++) {
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
        
    component s3_merkle[6];
    component s3_fft[6];
    component s3_evalPol[6];
    component s3_lowValues[6];
    signal s3_sx[6][4];
        
    for (var q=0; q<6; q++) {
        s3_merkle[q] = MerkleHash(3, 8, 16);
        s3_fft[q] = FFT(3, 3, 1);
        s3_evalPol[q] = EvalPol(8);
        s3_lowValues[q] = TreeSelector(4, 3) ;
        for (var i=0; i< 8; i++) {
            for (var e=0; e<3; e++) {
                s3_merkle[q].values[i][e] <== s3_vals[q][i*3+e];
                s3_fft[q].in[i][e] <== s3_vals[q][i*3+e];
            }
        }
        
        for (var i=0; i<4; i++) {
            for (var j=0; j<4; j++) {
                s3_merkle[q].siblings[i][j] <== s3_siblings[q][i][j];
            }
            s3_merkle[q].key[i] <== ys[q][i];
        }
        s3_sx[q][0] <==  140704680260498080 *  ( ys[q][0] * 18442240469787213840 +1);
        for (var i=1; i<4; i++) {
            s3_sx[q][i] <== s3_sx[q][i-1] *  ( ys[q][i] * ((1/roots(7 -i)) -1) +1);
        }
        for (var i=0; i< 8; i++) {
            for (var e=0; e<3; e++) {
                s3_evalPol[q].pol[i][e] <== s3_fft[q].out[i][e];
            }
        }
        for (var e=0; e<3; e++) {
            s3_evalPol[q].x[e] <== s3_specialX[e] *  s3_sx[q][3];
        }
        
        for (var i=0; i<16; i++) {
            for (var e=0; e<3; e++) {
                s3_lowValues[q].values[i][e] <== finalPol[i][e];
            }
        }
        for (var i=0; i<4; i++) {
            s3_lowValues[q].key[i] <== ys[q][i];
        }
        
        for(var q = 0; q < 6; q ++) {
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
            enable * (s3_lowValues[q].out[e] - s3_evalPol[q].out[e]) === 0;
        }

        enable * (s3_merkle[q].root[0] - s3_root[0]) === 0;
        enable * (s3_merkle[q].root[1] - s3_root[1]) === 0;
        enable * (s3_merkle[q].root[2] - s3_root[2]) === 0;
        enable * (s3_merkle[q].root[3] - s3_root[3]) === 0;
    }
        
    component lastIFFT = FFT(4, 3, 1);

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


component main {public [publics, rootC]}= StarkVerifier();
    