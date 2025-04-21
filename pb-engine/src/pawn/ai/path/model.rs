#[allow(
    unused_parens,
    non_snake_case,
    non_upper_case_globals,
    clippy::let_and_return,
    clippy::just_underscores_and_digits
)]
pub fn main_graph(onnx__Gemm_0: [[f32; 12usize]; 1usize]) -> [[f32; 6usize]; 1usize] {
    const onnx__Min_35: [f32; 1usize] = [20f32];
    const onnx__Max_34: [f32; 1usize] = [-20f32];
    const pi_net_mlp_0_bias: [f32; 6usize] = [
        0.09904972f32,
        0.19744186f32,
        -0.23079434f32,
        -0.16105899f32,
        -0.2073257f32,
        -0.08183505f32,
    ];
    const pi_net_mlp_0_weight: [[f32; 32usize]; 6usize] = [
        [
            0.026228642f32,
            -0.04249461f32,
            -0.038958807f32,
            -0.16636722f32,
            0.054370333f32,
            0.18617682f32,
            -0.11522295f32,
            0.06340435f32,
            0.009780183f32,
            -0.24136934f32,
            0.16375856f32,
            0.12569515f32,
            0.062704615f32,
            -0.043076277f32,
            0.08356108f32,
            -0.010219619f32,
            0.114541516f32,
            0.2557455f32,
            -0.028916225f32,
            0.13096137f32,
            -0.0011189367f32,
            0.09814569f32,
            -0.2490566f32,
            -0.049174428f32,
            0.009863162f32,
            -0.020054942f32,
            -0.13224955f32,
            -0.10160485f32,
            0.14154194f32,
            0.19130893f32,
            -0.025412519f32,
            0.020289758f32,
        ],
        [
            -0.31038913f32,
            0.17262061f32,
            -0.077710584f32,
            0.1129491f32,
            -0.077357724f32,
            0.18840301f32,
            0.21561432f32,
            -0.05140524f32,
            0.15602079f32,
            0.32283255f32,
            0.28787273f32,
            0.15517136f32,
            -0.015524709f32,
            0.50965893f32,
            -0.024645012f32,
            0.13235156f32,
            -0.0859134f32,
            -0.055797663f32,
            -0.09327571f32,
            -0.22147827f32,
            -0.025929507f32,
            0.21064551f32,
            -0.12258819f32,
            -0.25676778f32,
            -0.27064258f32,
            0.39454636f32,
            0.09408303f32,
            -0.054109845f32,
            -0.08020709f32,
            0.30743173f32,
            0.33919546f32,
            -0.13958627f32,
        ],
        [
            -0.2703266f32,
            -0.91153985f32,
            -0.06495949f32,
            -0.25060174f32,
            0.5387419f32,
            0.6769567f32,
            -0.1072416f32,
            0.14896907f32,
            0.07437907f32,
            0.11771106f32,
            1.1110208f32,
            -0.11419489f32,
            -0.06186992f32,
            -0.70481175f32,
            0.08387294f32,
            -0.17298412f32,
            0.034974366f32,
            0.20990257f32,
            0.068934515f32,
            0.010531561f32,
            -0.34338507f32,
            -0.15475309f32,
            -0.074133165f32,
            -0.36024424f32,
            -0.05417443f32,
            0.115333974f32,
            -0.20312837f32,
            0.3765595f32,
            -0.058108754f32,
            0.069725215f32,
            0.0384945f32,
            -0.029270632f32,
        ],
        [
            0.065869585f32,
            0.17964956f32,
            0.24905454f32,
            0.23754574f32,
            0.5139034f32,
            0.8462649f32,
            -0.11522665f32,
            -0.23992634f32,
            0.5895959f32,
            0.7649377f32,
            0.8612118f32,
            0.73650485f32,
            0.46517947f32,
            1.4725527f32,
            0.25750107f32,
            -0.14943688f32,
            0.011463689f32,
            0.29339802f32,
            1.5913436f32,
            -1.0584669f32,
            0.948492f32,
            0.036331005f32,
            0.49729016f32,
            0.15641604f32,
            -0.07650898f32,
            0.48143142f32,
            0.5713218f32,
            -0.2943998f32,
            0.30746964f32,
            0.56636655f32,
            1.1669813f32,
            -0.16769363f32,
        ],
        [
            -0.4351662f32,
            0.13480216f32,
            0.2967752f32,
            0.4017081f32,
            0.3418284f32,
            0.76857436f32,
            -0.22143199f32,
            -0.25022924f32,
            0.15334807f32,
            0.64118254f32,
            0.03610747f32,
            0.35475346f32,
            0.22820047f32,
            0.9898543f32,
            -0.054948162f32,
            0.124046594f32,
            0.032864705f32,
            0.13040674f32,
            0.4658446f32,
            -0.8742938f32,
            0.30679137f32,
            0.14925304f32,
            0.2545397f32,
            -0.12936553f32,
            0.18169513f32,
            0.22131005f32,
            0.17138074f32,
            0.10295186f32,
            0.12610851f32,
            0.6213002f32,
            0.9952174f32,
            -0.21889623f32,
        ],
        [
            -0.79727477f32,
            0.16319194f32,
            0.16745114f32,
            0.06438625f32,
            0.39075488f32,
            0.43835038f32,
            0.100587346f32,
            -0.19894499f32,
            0.17560352f32,
            0.43785352f32,
            0.3267078f32,
            0.2690737f32,
            0.003651234f32,
            0.41457734f32,
            0.03154949f32,
            0.19634442f32,
            0.18942636f32,
            0.050222963f32,
            0.3711428f32,
            -0.4667829f32,
            0.1964785f32,
            0.04926955f32,
            -0.015150642f32,
            0.0772736f32,
            -0.17424981f32,
            0.14478156f32,
            0.22487164f32,
            0.039099924f32,
            0.2593365f32,
            0.2846004f32,
            0.70178056f32,
            -0.1032208f32,
        ],
    ];
    const encoder_encoder_net_mlp_0_bias: [f32; 32usize] = [
        -0.3709562f32,
        0.121312745f32,
        -1.1464529f32,
        1.431062f32,
        -0.08139087f32,
        0.23320991f32,
        0.3830448f32,
        -1.393317f32,
        0.52290636f32,
        -0.91740835f32,
        0.47256434f32,
        0.06934679f32,
        0.657231f32,
        0.60129404f32,
        -2.0968714f32,
        -0.31230164f32,
        -0.98971415f32,
        0.22438791f32,
        -5.2129407f32,
        0.03155438f32,
        0.86000025f32,
        -0.63364154f32,
        0.19118695f32,
        0.5957195f32,
        -1.350958f32,
        0.99819815f32,
        -0.787864f32,
        -0.10985659f32,
        0.20659739f32,
        -0.10023127f32,
        0.0127197085f32,
        -2.0770962f32,
    ];
    const encoder_encoder_net_mlp_0_weight: [[f32; 12usize]; 32usize] = [
        [
            -0.021630833f32,
            4.0579243f32,
            0.08554912f32,
            -1.8163422f32,
            -0.3573207f32,
            -0.016053079f32,
            -2.3264325f32,
            -0.288594f32,
            0.15355219f32,
            0.118254006f32,
            -0.09101314f32,
            0.16223878f32,
        ],
        [
            -0.013376245f32,
            -4.203264f32,
            -0.0024339212f32,
            -2.4837704f32,
            0.87463623f32,
            -0.03712158f32,
            0.16728348f32,
            -0.20296822f32,
            -0.014276516f32,
            0.13079554f32,
            -0.13731565f32,
            0.2503577f32,
        ],
        [
            0.90065753f32,
            -0.10618598f32,
            -0.9105602f32,
            -2.4975564f32,
            -1.8632767f32,
            0.36675426f32,
            0.23813191f32,
            1.2289834f32,
            -1.969689f32,
            -0.0065841973f32,
            -1.7364286f32,
            -0.26040918f32,
        ],
        [
            -2.8270829f32,
            3.4081454f32,
            0.23372193f32,
            2.9448936f32,
            -0.028037766f32,
            0.51766974f32,
            4.983473f32,
            0.61283064f32,
            -0.046868656f32,
            0.21045682f32,
            1.2639271f32,
            -0.28547442f32,
        ],
        [
            -0.35702997f32,
            0.43213078f32,
            -0.16708887f32,
            0.099824905f32,
            0.5623996f32,
            -0.08956843f32,
            0.14474063f32,
            -0.24019876f32,
            0.08190978f32,
            0.07085797f32,
            -0.44305322f32,
            0.19529358f32,
        ],
        [
            0.5416819f32,
            -2.1996248f32,
            0.10336325f32,
            2.278582f32,
            0.9611302f32,
            -0.329319f32,
            2.3313441f32,
            1.1269293f32,
            0.3988436f32,
            -0.11277388f32,
            0.04579334f32,
            -0.26009881f32,
        ],
        [
            2.3451507f32,
            3.45919f32,
            0.1253783f32,
            -3.8727434f32,
            1.9382135f32,
            -0.02842157f32,
            4.438892f32,
            0.6453156f32,
            -0.2777326f32,
            -0.09311032f32,
            0.6332789f32,
            0.1637696f32,
        ],
        [
            -0.46819443f32,
            5.116291f32,
            -0.001010107f32,
            1.2455181f32,
            1.3415543f32,
            -0.28375575f32,
            1.6174121f32,
            0.038156785f32,
            -0.6572915f32,
            0.0075945556f32,
            -1.4792467f32,
            0.12219778f32,
        ],
        [
            -1.3710424f32,
            3.782478f32,
            -2.9740763f32,
            5.231007f32,
            0.6282765f32,
            0.051202107f32,
            0.0944641f32,
            -0.23555367f32,
            -0.05800449f32,
            -0.022903562f32,
            -0.91173977f32,
            -0.15382822f32,
        ],
        [
            0.027932987f32,
            4.9361396f32,
            -0.1244674f32,
            -2.8921354f32,
            1.814508f32,
            -0.30583718f32,
            -0.21789995f32,
            -0.6535031f32,
            -0.7683049f32,
            -0.038432866f32,
            -0.9595568f32,
            -0.20259568f32,
        ],
        [
            -4.11565f32,
            -1.9698967f32,
            -0.059935324f32,
            4.467095f32,
            0.11427647f32,
            -0.16830038f32,
            -0.14923954f32,
            0.25472286f32,
            0.19335626f32,
            0.24357885f32,
            -0.8073825f32,
            0.13006386f32,
        ],
        [
            0.036376502f32,
            2.988587f32,
            -0.072661795f32,
            0.37025988f32,
            -3.2611425f32,
            -0.033040464f32,
            -0.28495505f32,
            0.033881962f32,
            -0.11956097f32,
            -0.035679758f32,
            0.056896023f32,
            0.005882412f32,
        ],
        [
            -2.4145098f32,
            -7.544837f32,
            -0.2669876f32,
            -0.66168237f32,
            -1.5448519f32,
            -0.08576425f32,
            -2.5645027f32,
            0.5131891f32,
            0.039751783f32,
            -0.14477633f32,
            0.49760675f32,
            -0.107277155f32,
        ],
        [
            4.3304315f32,
            -0.5979115f32,
            0.008906094f32,
            -2.2519991f32,
            -0.3021386f32,
            0.056046776f32,
            -0.03769197f32,
            -0.085222f32,
            0.09732842f32,
            0.122038335f32,
            -0.7714199f32,
            -0.059193343f32,
        ],
        [
            -0.3235705f32,
            1.1030697f32,
            0.021169698f32,
            6.7018027f32,
            -0.25411966f32,
            -0.39992103f32,
            -0.051571593f32,
            0.07368752f32,
            -0.29768577f32,
            0.25432163f32,
            0.15629399f32,
            -0.15606956f32,
        ],
        [
            -0.46765873f32,
            4.6704926f32,
            -1.5669348f32,
            -0.029257718f32,
            2.2885518f32,
            -0.26179144f32,
            -0.15047142f32,
            -0.07091049f32,
            -0.0413522f32,
            -0.12787199f32,
            0.07265122f32,
            0.27504808f32,
        ],
        [
            -2.8103154f32,
            3.4307106f32,
            -0.14017048f32,
            1.7725974f32,
            -3.3127654f32,
            -0.51695555f32,
            -3.1089687f32,
            -2.194134f32,
            0.15683058f32,
            0.13898435f32,
            -1.0357343f32,
            0.24824363f32,
        ],
        [
            0.38932303f32,
            3.3759174f32,
            0.0686649f32,
            7.2051826f32,
            0.3287887f32,
            0.059642237f32,
            0.11650449f32,
            -0.46117133f32,
            -0.19505431f32,
            -0.06938128f32,
            -0.73676324f32,
            -0.25011006f32,
        ],
        [
            0.092809916f32,
            -2.3924243f32,
            -0.011794277f32,
            -3.3621173f32,
            0.74618477f32,
            -0.12641332f32,
            -0.042508442f32,
            -0.067742705f32,
            0.00331415f32,
            -0.25425753f32,
            -1.4962668f32,
            0.26843637f32,
        ],
        [
            0.05703611f32,
            -2.4667428f32,
            -0.02696302f32,
            -0.62293375f32,
            0.074961245f32,
            -0.09232301f32,
            -5.4827895f32,
            0.24228327f32,
            0.016697444f32,
            0.15314227f32,
            0.22541378f32,
            0.22122186f32,
        ],
        [
            -6.374359f32,
            -0.34911335f32,
            -0.022798022f32,
            -1.4164382f32,
            -0.6638672f32,
            0.26395804f32,
            -0.1287246f32,
            -0.2402824f32,
            -0.16256393f32,
            0.15474385f32,
            0.5667508f32,
            -0.25181967f32,
        ],
        [
            -1.563199f32,
            2.9466546f32,
            0.35800448f32,
            1.1354383f32,
            2.9679735f32,
            0.0012003735f32,
            0.13224638f32,
            -0.3458568f32,
            -0.5668679f32,
            -0.22324485f32,
            -0.878492f32,
            -0.04983917f32,
        ],
        [
            0.9791999f32,
            -2.0458636f32,
            -0.24601284f32,
            -1.5425811f32,
            -3.3841503f32,
            -1.1144136f32,
            -6.3875794f32,
            -0.29009995f32,
            -1.0885624f32,
            0.07243934f32,
            0.017741771f32,
            -0.048704267f32,
        ],
        [
            -0.026287008f32,
            -7.2740827f32,
            0.13118167f32,
            -0.26923463f32,
            -0.5002737f32,
            0.23405777f32,
            0.019296382f32,
            1.0546397f32,
            1.112258f32,
            0.25904566f32,
            0.4700843f32,
            -0.27280614f32,
        ],
        [
            -1.1169821f32,
            -3.3701272f32,
            -1.073988f32,
            3.6244366f32,
            -2.8314893f32,
            -0.44118634f32,
            -3.8937511f32,
            -1.703583f32,
            -0.658956f32,
            -0.0045361817f32,
            -1.6063732f32,
            -0.26321006f32,
        ],
        [
            -0.5305328f32,
            4.317297f32,
            0.1017416f32,
            -3.375526f32,
            -0.23565501f32,
            0.45103464f32,
            4.6379967f32,
            -2.1087103f32,
            -0.098045684f32,
            0.20514745f32,
            0.65423435f32,
            -0.23167944f32,
        ],
        [
            3.3792334f32,
            0.16170047f32,
            -0.0047714124f32,
            -4.1998825f32,
            0.46120992f32,
            -0.11038556f32,
            0.083187625f32,
            0.11705894f32,
            -0.06571186f32,
            -0.092690974f32,
            -2.80127f32,
            -0.009588748f32,
        ],
        [
            -3.1193557f32,
            -5.903519f32,
            0.17398141f32,
            0.61431223f32,
            -1.5004985f32,
            0.11484221f32,
            -1.6240264f32,
            -0.89400727f32,
            0.34098926f32,
            0.030264378f32,
            -0.10150336f32,
            0.24339354f32,
        ],
        [
            -0.054651298f32,
            4.3871675f32,
            1.9429532f32,
            0.011724731f32,
            2.462211f32,
            -0.09029052f32,
            -0.26434013f32,
            -0.72215855f32,
            0.0018871359f32,
            0.07630867f32,
            -0.24780416f32,
            -0.18858449f32,
        ],
        [
            0.21075933f32,
            -3.2993786f32,
            -0.07811117f32,
            -2.1837838f32,
            -3.1331966f32,
            -0.040180482f32,
            -2.6774578f32,
            -1.4481167f32,
            0.037515733f32,
            -0.22080019f32,
            0.34939018f32,
            -0.21446271f32,
        ],
        [
            -0.014471012f32,
            -0.5510818f32,
            0.0011198726f32,
            0.114267826f32,
            1.8479254f32,
            0.12721157f32,
            5.1692777f32,
            0.267011f32,
            -0.1106836f32,
            0.07290742f32,
            -0.3808616f32,
            0.21092072f32,
        ],
        [
            3.3731232f32,
            -1.5938519f32,
            -0.52854043f32,
            -4.411308f32,
            -0.4389791f32,
            0.046547018f32,
            -0.06467266f32,
            0.5802531f32,
            0.1582064f32,
            0.056348473f32,
            -0.37048298f32,
            0.28426826f32,
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
                        * encoder_encoder_net_mlp_0_weight[0usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[0usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[1usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[1usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[2usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[2usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[3usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[3usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[4usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[4usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[5usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[5usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[6usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[6usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[7usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[7usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[8usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[8usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[9usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[9usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[10usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[10usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[11usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[11usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[12usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[12usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[13usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[13usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[14usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[14usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[15usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[15usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[16usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[16usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[17usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[17usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[18usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[18usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[19usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[19usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[20usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[20usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[21usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[21usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[22usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[22usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[23usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[23usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[24usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[24usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[25usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[25usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[26usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[26usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[27usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[27usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[28usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[28usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[29usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[29usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[30usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[30usize][11usize])
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
                        * encoder_encoder_net_mlp_0_weight[31usize][9usize]
                    + onnx__Gemm_0[0usize][10usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][10usize]
                    + onnx__Gemm_0[0usize][11usize]
                        * encoder_encoder_net_mlp_0_weight[31usize][11usize])
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
