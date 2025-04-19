#[allow(
    unused_parens,
    non_snake_case,
    non_upper_case_globals,
    clippy::let_and_return,
    clippy::just_underscores_and_digits
)]
pub fn main_graph(onnx__Gemm_0: [[f32; 5usize]; 1usize]) -> [[f32; 4usize]; 1usize] {
    const onnx__Min_35: [f32; 1usize] = [20f32];
    const onnx__Max_34: [f32; 1usize] = [-20f32];
    const pi_net_mlp_0_bias: [f32; 4usize] = [
        -0.12265335f32,
        0.08864879f32,
        -1.0994271f32,
        -0.120217316f32,
    ];
    const pi_net_mlp_0_weight: [[f32; 24usize]; 4usize] = [
        [
            0.095763706f32,
            -0.15204892f32,
            0.067988686f32,
            -0.15699092f32,
            0.1172777f32,
            -0.036218006f32,
            -0.16975145f32,
            -0.22153349f32,
            0.21188676f32,
            -0.08376521f32,
            -0.07448557f32,
            -0.03774966f32,
            0.115933865f32,
            -0.35688043f32,
            0.063133396f32,
            0.15083678f32,
            -0.034037385f32,
            -0.12854417f32,
            0.005506937f32,
            -0.15822269f32,
            0.22799724f32,
            -0.20223808f32,
            -0.055247188f32,
            0.026026981f32,
        ],
        [
            0.14761499f32,
            -0.0027388309f32,
            -0.04371352f32,
            -0.16313675f32,
            -0.013285623f32,
            0.20647608f32,
            -0.16127717f32,
            -0.3048683f32,
            0.14974459f32,
            0.03779133f32,
            0.22453016f32,
            0.023837047f32,
            0.021018408f32,
            -0.4533f32,
            0.11897003f32,
            0.21264382f32,
            -0.11340284f32,
            0.19821164f32,
            0.27096882f32,
            -0.11178203f32,
            -0.038671147f32,
            -0.020838149f32,
            0.0054240185f32,
            -0.094149604f32,
        ],
        [
            -0.08477093f32,
            -0.48643926f32,
            -0.0521439f32,
            0.21543258f32,
            0.20188619f32,
            0.2917627f32,
            0.16029425f32,
            0.083553515f32,
            -0.34620872f32,
            0.18562508f32,
            0.385148f32,
            -0.29934436f32,
            -0.46308243f32,
            0.30906695f32,
            -0.08310972f32,
            -0.56975865f32,
            -0.53043824f32,
            0.07799138f32,
            -0.15866412f32,
            -0.022459464f32,
            0.40512276f32,
            -0.48579532f32,
            -0.71873266f32,
            -0.040902287f32,
        ],
        [
            -0.30347198f32,
            -0.11652301f32,
            -0.36719742f32,
            -0.33653575f32,
            0.085160315f32,
            0.22020571f32,
            0.09965733f32,
            -0.017104983f32,
            -0.08057594f32,
            0.25951886f32,
            0.33892447f32,
            -0.42257115f32,
            -0.04442882f32,
            0.18426014f32,
            -0.15338355f32,
            -0.26095098f32,
            -0.13179979f32,
            0.07256514f32,
            0.022576481f32,
            -0.13028249f32,
            0.09080381f32,
            -0.43323448f32,
            -0.28440523f32,
            -0.11467823f32,
        ],
    ];
    const encoder_encoder_net_mlp_0_bias: [f32; 24usize] = [
        -0.08179412f32,
        0.14003083f32,
        -0.025758628f32,
        -0.4993969f32,
        -0.08042895f32,
        0.32590687f32,
        0.6905366f32,
        -0.19564131f32,
        0.27112207f32,
        0.17538702f32,
        -0.06586764f32,
        -0.545697f32,
        0.27575615f32,
        -0.18028767f32,
        -0.24500859f32,
        1.1927072f32,
        0.37109986f32,
        0.3357222f32,
        -0.08010322f32,
        -0.2652743f32,
        -0.07978251f32,
        0.23402599f32,
        0.58128166f32,
        0.26565048f32,
    ];
    const encoder_encoder_net_mlp_0_weight: [[f32; 5usize]; 24usize] = [
        [0.3212241f32, -0.1868243f32, -0.099001355f32, -0.395067f32, 2.5592659f32],
        [-0.8396613f32, 0.048176907f32, 0.08955667f32, -1.1763908f32, 0.3188025f32],
        [-0.65511966f32, -0.10563256f32, 0.28650007f32, 0.7607929f32, 2.0292082f32],
        [-0.07678245f32, -0.17775962f32, 0.043110743f32, -0.10775218f32, 1.9622045f32],
        [1.4248495f32, -0.4941003f32, -0.2520133f32, -0.3290863f32, -0.662756f32],
        [0.019220622f32, 0.2014309f32, -0.22690156f32, 0.3331518f32, -2.322728f32],
        [0.49104962f32, -0.1615994f32, 0.12328523f32, 0.12582262f32, -1.226464f32],
        [0.3566375f32, 0.118798584f32, -0.048587207f32, -0.5195102f32, 1.5175358f32],
        [-0.24757472f32, 0.1501622f32, -0.15437657f32, 1.2224736f32, 0.8027524f32],
        [0.41796398f32, 0.018451847f32, 0.07493194f32, -0.8320421f32, -2.525788f32],
        [-0.28192428f32, 0.2542238f32, -0.09155613f32, 0.6746154f32, -2.300672f32],
        [-0.098584525f32, -0.34648117f32, 0.048000384f32, -0.107628636f32, 2.1467495f32],
        [-0.69951326f32, -0.34272587f32, 0.13704565f32, 0.72217077f32, 1.6585764f32],
        [-0.23382205f32, -0.3345843f32, 0.21421143f32, -0.47823358f32, -0.0691431f32],
        [0.55644286f32, -0.18417747f32, 0.14171149f32, -0.009355409f32, 2.1076405f32],
        [-0.15660094f32, 0.62942487f32, 0.18481615f32, 1.2936332f32, -0.061128646f32],
        [0.69495344f32, -0.5147595f32, -0.20177937f32, -0.6032085f32, 2.890391f32],
        [0.40109986f32, 0.14236501f32, 0.25202787f32, 0.24797258f32, -1.1704963f32],
        [1.2145776f32, 0.52726555f32, -0.3073517f32, 1.3240312f32, 0.5800936f32],
        [1.0450468f32, -0.57001954f32, 0.33016187f32, 0.16787641f32, 1.761104f32],
        [0.34338847f32, 0.21953057f32, 0.20756592f32, -1.1867118f32, -2.7180326f32],
        [0.2686445f32, 0.2614607f32, -0.2503932f32, -1.5038265f32, 0.2338269f32],
        [0.8123796f32, 1.5223893f32, 0.014508425f32, -0.99554425f32, -0.3609847f32],
        [-1.7839983f32, 0.45262936f32, 0.26208565f32, -0.5299674f32, 0.34125814f32],
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
                        * encoder_encoder_net_mlp_0_weight[0usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[1usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[2usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[3usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[4usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[5usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[6usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[7usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[8usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[9usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[10usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[11usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[12usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[13usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[14usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[15usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[16usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[17usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[18usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[19usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[20usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[21usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[22usize][4usize])
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
                        * encoder_encoder_net_mlp_0_weight[23usize][4usize])
                + 1f32 * encoder_encoder_net_mlp_0_bias[23usize],
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
                        * pi_net_mlp_0_weight[0usize][23usize])
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
                        * pi_net_mlp_0_weight[1usize][23usize])
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
                        * pi_net_mlp_0_weight[2usize][23usize])
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
                        * pi_net_mlp_0_weight[3usize][23usize])
                + 1f32 * pi_net_mlp_0_bias[3usize],
        ],
    ];
    const _pi_Constant_output_0: [i64; 1usize] = [-1i64];
    const _pi_Constant_1_output_0: [i64; 1usize] = [0i64];
    const _pi_Mul_output_0: [i64; 1usize] = [2i64];
    let (_pi_Slice_output_0) = [
        [
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][0usize],
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][1usize],
        ],
    ];
    const _pi_Mul_1_output_0: [i64; 1usize] = [4i64];
    let (_pi_Slice_1_output_0) = [
        [
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][2usize],
            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][3usize],
        ],
    ];
    let (_pi_Max_output_0) = [
        [
            _pi_Slice_1_output_0[0usize][0usize].max(onnx__Max_34[0usize]),
            _pi_Slice_1_output_0[0usize][1usize].max(onnx__Max_34[0usize]),
        ],
    ];
    let (_pi_Min_output_0) = [
        [
            _pi_Max_output_0[0usize][0usize].min(onnx__Min_35[0usize]),
            _pi_Max_output_0[0usize][1usize].min(onnx__Min_35[0usize]),
        ],
    ];
    let (_30) = [
        [
            _pi_Slice_output_0[0usize][0usize],
            _pi_Slice_output_0[0usize][1usize],
            _pi_Min_output_0[0usize][0usize],
            _pi_Min_output_0[0usize][1usize],
        ],
    ];
    _30
}
