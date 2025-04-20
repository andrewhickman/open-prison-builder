#[allow(
    unused_parens,
    non_snake_case,
    non_upper_case_globals,
    clippy::let_and_return,
    clippy::just_underscores_and_digits
)]
pub fn main_graph(onnx__Gemm_0: [[f32; 10usize]; 1usize]) -> [[f32; 6usize]; 1usize] {
    const onnx__Min_35: [f32; 1usize] = [20f32];
    const onnx__Max_34: [f32; 1usize] = [-20f32];
    const pi_net_mlp_0_bias: [f32; 6usize] = [
        -0.060714427f32,
        0.4477452f32,
        -0.050640818f32,
        0.7121882f32,
        0.42655712f32,
        0.65995026f32,
    ];
    const pi_net_mlp_0_weight: [[f32; 32usize]; 6usize] = [
        [
            -0.030867532f32,
            -0.033909515f32,
            0.12393102f32,
            0.025511317f32,
            -0.013900357f32,
            -0.09188698f32,
            0.013214458f32,
            -0.08004865f32,
            -0.052895762f32,
            0.051779002f32,
            -0.018187214f32,
            0.1790616f32,
            0.064578146f32,
            0.02404005f32,
            0.06731121f32,
            -0.10288602f32,
            0.008834287f32,
            -0.06938436f32,
            -0.009202447f32,
            0.053653076f32,
            -0.048191987f32,
            0.031227292f32,
            -0.031499755f32,
            0.0080007855f32,
            0.0091570765f32,
            -0.13448955f32,
            0.03032637f32,
            0.029780153f32,
            0.062484395f32,
            -0.010270318f32,
            0.14755976f32,
            -0.16148625f32,
        ],
        [
            -0.013025133f32,
            -0.13844918f32,
            0.17928748f32,
            0.0031853241f32,
            -0.049329244f32,
            -0.12862907f32,
            0.012502994f32,
            0.13517666f32,
            0.07544028f32,
            -0.20173807f32,
            -0.04848903f32,
            0.07186627f32,
            0.09862682f32,
            -0.063067354f32,
            0.05741889f32,
            -0.0601641f32,
            -0.1666429f32,
            -0.12647535f32,
            -0.04226924f32,
            0.036429f32,
            0.05450995f32,
            -0.026066314f32,
            0.1426288f32,
            0.05796462f32,
            -0.08324207f32,
            0.08321065f32,
            0.14222822f32,
            0.0241125f32,
            -0.39462605f32,
            0.0015721271f32,
            0.64485836f32,
            0.034050338f32,
        ],
        [
            -0.036150623f32,
            0.06139633f32,
            0.060617164f32,
            -0.09186547f32,
            -0.094924904f32,
            -0.063847624f32,
            -0.087088704f32,
            -0.11163441f32,
            0.03593839f32,
            -0.048630357f32,
            -0.023807997f32,
            0.22484122f32,
            0.1058984f32,
            -0.089074805f32,
            -0.00006213576f32,
            -0.046912935f32,
            0.021771003f32,
            0.049084865f32,
            0.07437188f32,
            -0.028439831f32,
            -0.06717762f32,
            0.06444496f32,
            -0.12207601f32,
            0.09246459f32,
            0.05472917f32,
            0.16704316f32,
            -0.0070009152f32,
            -0.43127114f32,
            0.090985045f32,
            0.028703146f32,
            0.22525935f32,
            -0.16659933f32,
        ],
        [
            0.5080846f32,
            0.11421875f32,
            -0.30017635f32,
            0.23162441f32,
            -0.08963078f32,
            0.0711446f32,
            0.48900247f32,
            0.10813566f32,
            -0.58229834f32,
            0.36212972f32,
            0.38283622f32,
            -0.3390413f32,
            -0.31243747f32,
            0.11714787f32,
            0.14992405f32,
            -0.011379102f32,
            0.2476069f32,
            0.094346315f32,
            0.035564717f32,
            -0.13297857f32,
            -0.15154016f32,
            0.2075717f32,
            -0.32396677f32,
            0.17672102f32,
            -0.15263534f32,
            -0.4999398f32,
            -0.028228227f32,
            0.07906351f32,
            -0.89264554f32,
            0.12358142f32,
            1.159499f32,
            0.4099068f32,
        ],
        [
            0.013737072f32,
            -0.108197644f32,
            -0.111137874f32,
            -0.009260796f32,
            -0.10448598f32,
            0.15189727f32,
            0.18115959f32,
            0.06787172f32,
            -0.32849702f32,
            -0.30816528f32,
            0.23474786f32,
            -0.17723915f32,
            -0.10651216f32,
            -0.0015841072f32,
            0.023143208f32,
            0.14126173f32,
            -0.021423737f32,
            0.04300878f32,
            0.025569158f32,
            -0.06925352f32,
            -0.08128757f32,
            0.1463835f32,
            -0.1194958f32,
            0.027271733f32,
            -0.026626498f32,
            -0.36165217f32,
            0.17711285f32,
            -0.043549154f32,
            -0.75675076f32,
            -0.032745413f32,
            0.55824405f32,
            0.32382405f32,
        ],
        [
            -0.057246715f32,
            -0.021297459f32,
            0.04572098f32,
            -0.06027609f32,
            0.0077651343f32,
            -0.15680234f32,
            0.5640638f32,
            -0.0026795904f32,
            -0.25223157f32,
            0.09923188f32,
            0.4994362f32,
            -0.0018513361f32,
            -0.09738868f32,
            0.08093732f32,
            -0.0590228f32,
            0.0072482843f32,
            -0.17272103f32,
            0.05687958f32,
            0.014047255f32,
            -0.059726518f32,
            -0.25459558f32,
            0.49946597f32,
            -0.20111236f32,
            0.1798541f32,
            -0.033238523f32,
            -0.6676269f32,
            0.40011644f32,
            -0.07128864f32,
            -0.7679617f32,
            -0.025659544f32,
            0.9446095f32,
            0.19896322f32,
        ],
    ];
    const encoder_encoder_net_mlp_0_bias: [f32; 32usize] = [
        1.3256533f32,
        -1.792489f32,
        2.2184026f32,
        -1.3389627f32,
        -1.0005652f32,
        0.67424035f32,
        0.45363775f32,
        2.054738f32,
        0.023817254f32,
        1.1016963f32,
        -0.7225638f32,
        1.0793098f32,
        1.1144345f32,
        -0.09834323f32,
        0.78645295f32,
        -1.8530939f32,
        -2.1402082f32,
        -2.4913068f32,
        -1.0549641f32,
        1.7659512f32,
        1.8491199f32,
        -1.4396595f32,
        3.0109127f32,
        -2.014669f32,
        -1.42435f32,
        1.4374205f32,
        -0.5855403f32,
        1.8653874f32,
        1.1099988f32,
        2.6643f32,
        -0.07820128f32,
        -0.16003028f32,
    ];
    const encoder_encoder_net_mlp_0_weight: [[f32; 10usize]; 32usize] = [
        [
            0.26960486f32,
            -0.21833354f32,
            0.033318933f32,
            -2.791178f32,
            -0.20342466f32,
            4.013683f32,
            -0.41247678f32,
            0.014583775f32,
            0.3826428f32,
            0.37799984f32,
        ],
        [
            -0.1016178f32,
            -1.0270823f32,
            -0.06869425f32,
            -0.1125625f32,
            1.2646292f32,
            0.7845422f32,
            -0.11104116f32,
            0.18337692f32,
            -6.799371f32,
            -7.242008f32,
        ],
        [
            1.3998047f32,
            -3.2627203f32,
            0.1220067f32,
            -0.5140423f32,
            1.1294773f32,
            0.17596798f32,
            0.53875196f32,
            -0.08300145f32,
            1.8052924f32,
            1.3944187f32,
        ],
        [
            0.05351805f32,
            1.5654435f32,
            0.066907346f32,
            1.7129514f32,
            -1.6185982f32,
            0.32970542f32,
            -0.2309766f32,
            -0.13982545f32,
            -0.37875977f32,
            -0.32094184f32,
        ],
        [
            0.12870303f32,
            -1.578006f32,
            0.023688925f32,
            -0.96798265f32,
            0.5284832f32,
            -0.81392545f32,
            0.051043995f32,
            0.011874001f32,
            -1.2298517f32,
            -1.1034784f32,
        ],
        [
            2.010955f32,
            -0.76050687f32,
            -0.04622287f32,
            -2.385058f32,
            -0.37598842f32,
            -0.36034504f32,
            0.016611233f32,
            -0.053943317f32,
            0.027290953f32,
            -0.1053522f32,
        ],
        [
            -0.7334447f32,
            0.21847272f32,
            -0.0150041655f32,
            1.7183408f32,
            0.19625098f32,
            1.7431684f32,
            0.15955113f32,
            0.14197566f32,
            -1.1762245f32,
            -1.3172197f32,
        ],
        [
            1.7112583f32,
            -1.5494623f32,
            -0.022028087f32,
            -2.1729124f32,
            -0.055105835f32,
            -0.19612604f32,
            -0.13459376f32,
            0.23584454f32,
            4.226687f32,
            4.152164f32,
        ],
        [
            -0.10910443f32,
            4.5930047f32,
            0.08265446f32,
            -0.62023467f32,
            -1.0611215f32,
            2.5868115f32,
            0.18622008f32,
            0.10849141f32,
            -1.821004f32,
            -1.7372473f32,
        ],
        [
            -0.10813457f32,
            0.1476032f32,
            -0.03908664f32,
            -0.4995006f32,
            -3.4555168f32,
            -0.18195388f32,
            0.012133674f32,
            0.0066062408f32,
            0.05828518f32,
            -0.03356049f32,
        ],
        [
            0.21839203f32,
            -0.30482525f32,
            0.066788636f32,
            -0.37659216f32,
            4.254059f32,
            0.010089645f32,
            0.13341069f32,
            0.01666616f32,
            0.19738305f32,
            -0.016043967f32,
        ],
        [
            0.14381088f32,
            -0.15848675f32,
            -0.008707432f32,
            1.2216825f32,
            -0.12058541f32,
            -0.27372202f32,
            -0.046853535f32,
            0.11474123f32,
            -0.53240895f32,
            -0.63497543f32,
        ],
        [
            -1.8646495f32,
            -0.69755715f32,
            -0.030949462f32,
            1.3934944f32,
            0.11834378f32,
            0.15964001f32,
            -0.0025515505f32,
            0.056845915f32,
            0.2535091f32,
            0.5084748f32,
        ],
        [
            0.10764375f32,
            -0.31745353f32,
            0.0073523154f32,
            1.0854663f32,
            0.002096627f32,
            -0.38941148f32,
            0.19986977f32,
            -0.19764388f32,
            -0.56148136f32,
            -0.5747756f32,
        ],
        [
            -1.3129902f32,
            -0.20070793f32,
            0.0012819658f32,
            2.3299727f32,
            -0.19913799f32,
            0.10666437f32,
            -0.18908426f32,
            0.08911278f32,
            -0.3690967f32,
            0.05534685f32,
        ],
        [
            -0.8167335f32,
            -1.577503f32,
            -0.0010654306f32,
            1.4690964f32,
            -2.0824277f32,
            1.0660495f32,
            -0.46275064f32,
            -0.003614213f32,
            0.5346354f32,
            0.060468838f32,
        ],
        [
            0.93860877f32,
            -0.33618456f32,
            0.09954571f32,
            1.3435906f32,
            -2.392592f32,
            -0.765591f32,
            -0.1811067f32,
            -0.08056895f32,
            -1.0890845f32,
            -0.6969924f32,
        ],
        [
            -1.1753134f32,
            1.7389086f32,
            -0.0051768036f32,
            0.8861548f32,
            -2.3858793f32,
            -1.2140696f32,
            0.08625475f32,
            -0.22759491f32,
            -1.0525707f32,
            -0.81298995f32,
        ],
        [
            0.67891854f32,
            -2.5031645f32,
            -0.14560676f32,
            1.0801387f32,
            -0.23872335f32,
            -0.17754406f32,
            -0.30549783f32,
            1.5737364f32,
            0.9660091f32,
            0.76634914f32,
        ],
        [
            1.3287143f32,
            -0.84938f32,
            0.065110356f32,
            1.2009572f32,
            2.4920337f32,
            -1.3508477f32,
            0.7730441f32,
            -0.073630236f32,
            1.040828f32,
            0.7784124f32,
        ],
        [
            -0.08358713f32,
            1.3133724f32,
            0.1225765f32,
            -2.898362f32,
            -0.73144335f32,
            -0.33306378f32,
            -0.087427326f32,
            -0.026720585f32,
            4.716788f32,
            4.8364882f32,
        ],
        [
            1.3192921f32,
            1.0414046f32,
            0.074079104f32,
            -1.0369565f32,
            0.45525214f32,
            -1.9944429f32,
            0.11446838f32,
            -0.076791376f32,
            0.15060478f32,
            0.074732564f32,
        ],
        [
            1.0041587f32,
            1.6586179f32,
            -0.040648647f32,
            -1.3751618f32,
            0.12192223f32,
            0.5275701f32,
            0.08435375f32,
            0.0152137065f32,
            2.2476573f32,
            1.9253888f32,
        ],
        [
            0.23837976f32,
            1.8638929f32,
            0.012555738f32,
            2.971491f32,
            0.070428416f32,
            -0.5281741f32,
            0.2827284f32,
            0.030897995f32,
            -1.4215686f32,
            -1.7612929f32,
        ],
        [
            1.4314691f32,
            0.26579514f32,
            0.11955528f32,
            -2.022086f32,
            0.19416435f32,
            0.7147884f32,
            0.07012204f32,
            -0.1093925f32,
            -3.5866437f32,
            -3.501442f32,
        ],
        [
            -0.006408437f32,
            1.2029384f32,
            -0.016761502f32,
            -1.4461243f32,
            -0.2557313f32,
            0.38626266f32,
            -0.0089261765f32,
            0.017621562f32,
            0.7735684f32,
            0.81961024f32,
        ],
        [
            0.700587f32,
            -0.64481264f32,
            -0.03664213f32,
            -0.28851765f32,
            2.5913668f32,
            -1.0785697f32,
            0.194891f32,
            -0.03849122f32,
            -0.46244684f32,
            -0.5484098f32,
        ],
        [
            -0.23473223f32,
            5.8657026f32,
            0.0009264512f32,
            -0.13914396f32,
            -0.88441914f32,
            0.9536101f32,
            0.12605059f32,
            -0.07622978f32,
            1.3481967f32,
            1.3264027f32,
        ],
        [
            0.46354118f32,
            0.50389946f32,
            0.1207917f32,
            -0.23025897f32,
            -2.147942f32,
            -3.5302694f32,
            -0.15372863f32,
            -0.08277059f32,
            0.31915832f32,
            0.37638056f32,
        ],
        [
            -2.9054868f32,
            0.7000128f32,
            -0.08646781f32,
            1.7636214f32,
            -0.16466574f32,
            -0.3794741f32,
            -0.34569344f32,
            0.051153097f32,
            4.9049635f32,
            4.858487f32,
        ],
        [
            -0.09618925f32,
            -0.14606933f32,
            -0.015271731f32,
            -0.0069033494f32,
            0.21694902f32,
            -0.4674408f32,
            -6.1520095f32,
            -0.005126354f32,
            0.32855308f32,
            0.27215308f32,
        ],
        [
            0.47229195f32,
            -1.4498165f32,
            0.05959991f32,
            -1.2953625f32,
            0.07278438f32,
            -0.1682618f32,
            0.057129823f32,
            0.03907829f32,
            -0.67129725f32,
            -0.24257033f32,
        ],
    ];
    let (_encoder_encoder_net_mlp_mlp_0_Gemm_output_0) = [
        [
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[0usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[0usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[1usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[1usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[2usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[2usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[3usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[3usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[4usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[4usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[5usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[5usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[6usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[6usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[7usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[7usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[8usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[8usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[9usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[9usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[10usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[10usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[11usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[11usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[12usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[12usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[13usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[13usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[14usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[14usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[15usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[15usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[16usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[16usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[17usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[17usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[18usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[18usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[19usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[19usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[20usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[20usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[21usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[21usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[22usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[22usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[23usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[23usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[24usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[24usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[25usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[25usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[26usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[26usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[27usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[27usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[28usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[28usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[29usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[29usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[30usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[30usize],
            1f32
                * (onnx__Gemm_0[0usize][0usize]
                    * encoder_encoder_net_mlp_0_weight[31usize][0usize]
                    + onnx__Gemm_0[0usize][1usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][1usize]
                    + onnx__Gemm_0[0usize][2usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][2usize]
                    + onnx__Gemm_0[0usize][3usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][3usize]
                    + onnx__Gemm_0[0usize][4usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][4usize]
                    + onnx__Gemm_0[0usize][5usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][5usize]
                    + onnx__Gemm_0[0usize][6usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][6usize]
                    + onnx__Gemm_0[0usize][7usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][7usize]
                    + onnx__Gemm_0[0usize][8usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][8usize]
                    + onnx__Gemm_0[0usize][9usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][9usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[31usize],
        ],
    ];
    let (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0) = [
        [
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][0usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][1usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][2usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][3usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][4usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][5usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][6usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][7usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][8usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][9usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][10usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][11usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][12usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][13usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][14usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][15usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][16usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][17usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][18usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][19usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][20usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][21usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][22usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][23usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][24usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][25usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][26usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][27usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][28usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][29usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][30usize].tanh(),
            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][31usize].tanh(),
        ],
    ];
    let (_pi_net_mlp_mlp_0_Gemm_output_0) = [
        [
            1f32
                * (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][0usize]
                    * pi_net_mlp_0_weight[0usize][0usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][1usize]
                        * pi_net_mlp_0_weight[0usize][1usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][2usize]
                        * pi_net_mlp_0_weight[0usize][2usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][3usize]
                        * pi_net_mlp_0_weight[0usize][3usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][4usize]
                        * pi_net_mlp_0_weight[0usize][4usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][5usize]
                        * pi_net_mlp_0_weight[0usize][5usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][6usize]
                        * pi_net_mlp_0_weight[0usize][6usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][7usize]
                        * pi_net_mlp_0_weight[0usize][7usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][8usize]
                        * pi_net_mlp_0_weight[0usize][8usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][9usize]
                        * pi_net_mlp_0_weight[0usize][9usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][10usize]
                        * pi_net_mlp_0_weight[0usize][10usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][11usize]
                        * pi_net_mlp_0_weight[0usize][11usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][12usize]
                        * pi_net_mlp_0_weight[0usize][12usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][13usize]
                        * pi_net_mlp_0_weight[0usize][13usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][14usize]
                        * pi_net_mlp_0_weight[0usize][14usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][15usize]
                        * pi_net_mlp_0_weight[0usize][15usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][16usize]
                        * pi_net_mlp_0_weight[0usize][16usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][17usize]
                        * pi_net_mlp_0_weight[0usize][17usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][18usize]
                        * pi_net_mlp_0_weight[0usize][18usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][19usize]
                        * pi_net_mlp_0_weight[0usize][19usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][20usize]
                        * pi_net_mlp_0_weight[0usize][20usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][21usize]
                        * pi_net_mlp_0_weight[0usize][21usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][22usize]
                        * pi_net_mlp_0_weight[0usize][22usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][23usize]
                        * pi_net_mlp_0_weight[0usize][23usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][24usize]
                        * pi_net_mlp_0_weight[0usize][24usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][25usize]
                        * pi_net_mlp_0_weight[0usize][25usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][26usize]
                        * pi_net_mlp_0_weight[0usize][26usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][27usize]
                        * pi_net_mlp_0_weight[0usize][27usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][28usize]
                        * pi_net_mlp_0_weight[0usize][28usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][29usize]
                        * pi_net_mlp_0_weight[0usize][29usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][30usize]
                        * pi_net_mlp_0_weight[0usize][30usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][31usize]
                        * pi_net_mlp_0_weight[0usize][31usize])
                + 1f32 * pi_net_mlp_0_bias[0usize],
            1f32
                * (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][0usize]
                    * pi_net_mlp_0_weight[1usize][0usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][1usize]
                        * pi_net_mlp_0_weight[1usize][1usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][2usize]
                        * pi_net_mlp_0_weight[1usize][2usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][3usize]
                        * pi_net_mlp_0_weight[1usize][3usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][4usize]
                        * pi_net_mlp_0_weight[1usize][4usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][5usize]
                        * pi_net_mlp_0_weight[1usize][5usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][6usize]
                        * pi_net_mlp_0_weight[1usize][6usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][7usize]
                        * pi_net_mlp_0_weight[1usize][7usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][8usize]
                        * pi_net_mlp_0_weight[1usize][8usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][9usize]
                        * pi_net_mlp_0_weight[1usize][9usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][10usize]
                        * pi_net_mlp_0_weight[1usize][10usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][11usize]
                        * pi_net_mlp_0_weight[1usize][11usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][12usize]
                        * pi_net_mlp_0_weight[1usize][12usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][13usize]
                        * pi_net_mlp_0_weight[1usize][13usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][14usize]
                        * pi_net_mlp_0_weight[1usize][14usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][15usize]
                        * pi_net_mlp_0_weight[1usize][15usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][16usize]
                        * pi_net_mlp_0_weight[1usize][16usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][17usize]
                        * pi_net_mlp_0_weight[1usize][17usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][18usize]
                        * pi_net_mlp_0_weight[1usize][18usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][19usize]
                        * pi_net_mlp_0_weight[1usize][19usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][20usize]
                        * pi_net_mlp_0_weight[1usize][20usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][21usize]
                        * pi_net_mlp_0_weight[1usize][21usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][22usize]
                        * pi_net_mlp_0_weight[1usize][22usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][23usize]
                        * pi_net_mlp_0_weight[1usize][23usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][24usize]
                        * pi_net_mlp_0_weight[1usize][24usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][25usize]
                        * pi_net_mlp_0_weight[1usize][25usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][26usize]
                        * pi_net_mlp_0_weight[1usize][26usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][27usize]
                        * pi_net_mlp_0_weight[1usize][27usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][28usize]
                        * pi_net_mlp_0_weight[1usize][28usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][29usize]
                        * pi_net_mlp_0_weight[1usize][29usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][30usize]
                        * pi_net_mlp_0_weight[1usize][30usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][31usize]
                        * pi_net_mlp_0_weight[1usize][31usize])
                + 1f32 * pi_net_mlp_0_bias[1usize],
            1f32
                * (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][0usize]
                    * pi_net_mlp_0_weight[2usize][0usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][1usize]
                        * pi_net_mlp_0_weight[2usize][1usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][2usize]
                        * pi_net_mlp_0_weight[2usize][2usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][3usize]
                        * pi_net_mlp_0_weight[2usize][3usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][4usize]
                        * pi_net_mlp_0_weight[2usize][4usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][5usize]
                        * pi_net_mlp_0_weight[2usize][5usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][6usize]
                        * pi_net_mlp_0_weight[2usize][6usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][7usize]
                        * pi_net_mlp_0_weight[2usize][7usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][8usize]
                        * pi_net_mlp_0_weight[2usize][8usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][9usize]
                        * pi_net_mlp_0_weight[2usize][9usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][10usize]
                        * pi_net_mlp_0_weight[2usize][10usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][11usize]
                        * pi_net_mlp_0_weight[2usize][11usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][12usize]
                        * pi_net_mlp_0_weight[2usize][12usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][13usize]
                        * pi_net_mlp_0_weight[2usize][13usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][14usize]
                        * pi_net_mlp_0_weight[2usize][14usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][15usize]
                        * pi_net_mlp_0_weight[2usize][15usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][16usize]
                        * pi_net_mlp_0_weight[2usize][16usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][17usize]
                        * pi_net_mlp_0_weight[2usize][17usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][18usize]
                        * pi_net_mlp_0_weight[2usize][18usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][19usize]
                        * pi_net_mlp_0_weight[2usize][19usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][20usize]
                        * pi_net_mlp_0_weight[2usize][20usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][21usize]
                        * pi_net_mlp_0_weight[2usize][21usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][22usize]
                        * pi_net_mlp_0_weight[2usize][22usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][23usize]
                        * pi_net_mlp_0_weight[2usize][23usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][24usize]
                        * pi_net_mlp_0_weight[2usize][24usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][25usize]
                        * pi_net_mlp_0_weight[2usize][25usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][26usize]
                        * pi_net_mlp_0_weight[2usize][26usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][27usize]
                        * pi_net_mlp_0_weight[2usize][27usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][28usize]
                        * pi_net_mlp_0_weight[2usize][28usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][29usize]
                        * pi_net_mlp_0_weight[2usize][29usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][30usize]
                        * pi_net_mlp_0_weight[2usize][30usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][31usize]
                        * pi_net_mlp_0_weight[2usize][31usize])
                + 1f32 * pi_net_mlp_0_bias[2usize],
            1f32
                * (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][0usize]
                    * pi_net_mlp_0_weight[3usize][0usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][1usize]
                        * pi_net_mlp_0_weight[3usize][1usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][2usize]
                        * pi_net_mlp_0_weight[3usize][2usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][3usize]
                        * pi_net_mlp_0_weight[3usize][3usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][4usize]
                        * pi_net_mlp_0_weight[3usize][4usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][5usize]
                        * pi_net_mlp_0_weight[3usize][5usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][6usize]
                        * pi_net_mlp_0_weight[3usize][6usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][7usize]
                        * pi_net_mlp_0_weight[3usize][7usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][8usize]
                        * pi_net_mlp_0_weight[3usize][8usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][9usize]
                        * pi_net_mlp_0_weight[3usize][9usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][10usize]
                        * pi_net_mlp_0_weight[3usize][10usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][11usize]
                        * pi_net_mlp_0_weight[3usize][11usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][12usize]
                        * pi_net_mlp_0_weight[3usize][12usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][13usize]
                        * pi_net_mlp_0_weight[3usize][13usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][14usize]
                        * pi_net_mlp_0_weight[3usize][14usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][15usize]
                        * pi_net_mlp_0_weight[3usize][15usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][16usize]
                        * pi_net_mlp_0_weight[3usize][16usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][17usize]
                        * pi_net_mlp_0_weight[3usize][17usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][18usize]
                        * pi_net_mlp_0_weight[3usize][18usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][19usize]
                        * pi_net_mlp_0_weight[3usize][19usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][20usize]
                        * pi_net_mlp_0_weight[3usize][20usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][21usize]
                        * pi_net_mlp_0_weight[3usize][21usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][22usize]
                        * pi_net_mlp_0_weight[3usize][22usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][23usize]
                        * pi_net_mlp_0_weight[3usize][23usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][24usize]
                        * pi_net_mlp_0_weight[3usize][24usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][25usize]
                        * pi_net_mlp_0_weight[3usize][25usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][26usize]
                        * pi_net_mlp_0_weight[3usize][26usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][27usize]
                        * pi_net_mlp_0_weight[3usize][27usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][28usize]
                        * pi_net_mlp_0_weight[3usize][28usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][29usize]
                        * pi_net_mlp_0_weight[3usize][29usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][30usize]
                        * pi_net_mlp_0_weight[3usize][30usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][31usize]
                        * pi_net_mlp_0_weight[3usize][31usize])
                + 1f32 * pi_net_mlp_0_bias[3usize],
            1f32
                * (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][0usize]
                    * pi_net_mlp_0_weight[4usize][0usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][1usize]
                        * pi_net_mlp_0_weight[4usize][1usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][2usize]
                        * pi_net_mlp_0_weight[4usize][2usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][3usize]
                        * pi_net_mlp_0_weight[4usize][3usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][4usize]
                        * pi_net_mlp_0_weight[4usize][4usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][5usize]
                        * pi_net_mlp_0_weight[4usize][5usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][6usize]
                        * pi_net_mlp_0_weight[4usize][6usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][7usize]
                        * pi_net_mlp_0_weight[4usize][7usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][8usize]
                        * pi_net_mlp_0_weight[4usize][8usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][9usize]
                        * pi_net_mlp_0_weight[4usize][9usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][10usize]
                        * pi_net_mlp_0_weight[4usize][10usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][11usize]
                        * pi_net_mlp_0_weight[4usize][11usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][12usize]
                        * pi_net_mlp_0_weight[4usize][12usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][13usize]
                        * pi_net_mlp_0_weight[4usize][13usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][14usize]
                        * pi_net_mlp_0_weight[4usize][14usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][15usize]
                        * pi_net_mlp_0_weight[4usize][15usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][16usize]
                        * pi_net_mlp_0_weight[4usize][16usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][17usize]
                        * pi_net_mlp_0_weight[4usize][17usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][18usize]
                        * pi_net_mlp_0_weight[4usize][18usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][19usize]
                        * pi_net_mlp_0_weight[4usize][19usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][20usize]
                        * pi_net_mlp_0_weight[4usize][20usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][21usize]
                        * pi_net_mlp_0_weight[4usize][21usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][22usize]
                        * pi_net_mlp_0_weight[4usize][22usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][23usize]
                        * pi_net_mlp_0_weight[4usize][23usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][24usize]
                        * pi_net_mlp_0_weight[4usize][24usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][25usize]
                        * pi_net_mlp_0_weight[4usize][25usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][26usize]
                        * pi_net_mlp_0_weight[4usize][26usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][27usize]
                        * pi_net_mlp_0_weight[4usize][27usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][28usize]
                        * pi_net_mlp_0_weight[4usize][28usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][29usize]
                        * pi_net_mlp_0_weight[4usize][29usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][30usize]
                        * pi_net_mlp_0_weight[4usize][30usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][31usize]
                        * pi_net_mlp_0_weight[4usize][31usize])
                + 1f32 * pi_net_mlp_0_bias[4usize],
            1f32
                * (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][0usize]
                    * pi_net_mlp_0_weight[5usize][0usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][1usize]
                        * pi_net_mlp_0_weight[5usize][1usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][2usize]
                        * pi_net_mlp_0_weight[5usize][2usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][3usize]
                        * pi_net_mlp_0_weight[5usize][3usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][4usize]
                        * pi_net_mlp_0_weight[5usize][4usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][5usize]
                        * pi_net_mlp_0_weight[5usize][5usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][6usize]
                        * pi_net_mlp_0_weight[5usize][6usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][7usize]
                        * pi_net_mlp_0_weight[5usize][7usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][8usize]
                        * pi_net_mlp_0_weight[5usize][8usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][9usize]
                        * pi_net_mlp_0_weight[5usize][9usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][10usize]
                        * pi_net_mlp_0_weight[5usize][10usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][11usize]
                        * pi_net_mlp_0_weight[5usize][11usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][12usize]
                        * pi_net_mlp_0_weight[5usize][12usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][13usize]
                        * pi_net_mlp_0_weight[5usize][13usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][14usize]
                        * pi_net_mlp_0_weight[5usize][14usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][15usize]
                        * pi_net_mlp_0_weight[5usize][15usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][16usize]
                        * pi_net_mlp_0_weight[5usize][16usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][17usize]
                        * pi_net_mlp_0_weight[5usize][17usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][18usize]
                        * pi_net_mlp_0_weight[5usize][18usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][19usize]
                        * pi_net_mlp_0_weight[5usize][19usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][20usize]
                        * pi_net_mlp_0_weight[5usize][20usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][21usize]
                        * pi_net_mlp_0_weight[5usize][21usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][22usize]
                        * pi_net_mlp_0_weight[5usize][22usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][23usize]
                        * pi_net_mlp_0_weight[5usize][23usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][24usize]
                        * pi_net_mlp_0_weight[5usize][24usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][25usize]
                        * pi_net_mlp_0_weight[5usize][25usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][26usize]
                        * pi_net_mlp_0_weight[5usize][26usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][27usize]
                        * pi_net_mlp_0_weight[5usize][27usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][28usize]
                        * pi_net_mlp_0_weight[5usize][28usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][29usize]
                        * pi_net_mlp_0_weight[5usize][29usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][30usize]
                        * pi_net_mlp_0_weight[5usize][30usize]
                    + _encoder_encoder_net_mlp_mlp_1_Tanh_output_0[0usize][31usize]
                        * pi_net_mlp_0_weight[5usize][31usize])
                + 1f32 * pi_net_mlp_0_bias[5usize],
        ],
    ];
    const _pi_Constant_output_0: [i64; 1usize] = [-1i64];
    const _pi_Constant_1_output_0: [i64; 1usize] = [0i64];
    const _pi_Mul_output_0: [i64; 1usize] = [3i64];
    let (_pi_Slice_output_0) = [
        [
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][0usize],
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][1usize],
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][2usize],
        ],
    ];
    const _pi_Mul_1_output_0: [i64; 1usize] = [6i64];
    let (_pi_Slice_1_output_0) = [
        [
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][3usize],
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][4usize],
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][5usize],
        ],
    ];
    let (_pi_Max_output_0) = [
        [
            _pi_Slice_1_output_0[0usize][0usize].max(onnx__Max_34[0usize]),
            _pi_Slice_1_output_0[0usize][1usize].max(onnx__Max_34[0usize]),
            _pi_Slice_1_output_0[0usize][2usize].max(onnx__Max_34[0usize]),
        ],
    ];
    let (_pi_Min_output_0) = [
        [
            _pi_Max_output_0[0usize][0usize].min(onnx__Min_35[0usize]),
            _pi_Max_output_0[0usize][1usize].min(onnx__Min_35[0usize]),
            _pi_Max_output_0[0usize][2usize].min(onnx__Min_35[0usize]),
        ],
    ];
    let (_30) = [
        [
            _pi_Slice_output_0[0usize][0usize],
            _pi_Slice_output_0[0usize][1usize],
            _pi_Slice_output_0[0usize][2usize],
            _pi_Min_output_0[0usize][0usize],
            _pi_Min_output_0[0usize][1usize],
            _pi_Min_output_0[0usize][2usize],
        ],
    ];
    _30
}
