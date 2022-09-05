use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::shared_math::{b_field_element::BFieldElement, traits::ModPowU64};

pub const DIGEST_LENGTH: usize = 5;
pub const STATE_SIZE: usize = 16;
pub const CAPACITY: usize = 6;
pub const RATE: usize = 10;
pub const NUM_ROUNDS: usize = 8;

pub const ALPHA: u64 = 7;
pub const ALPHA_INV: u64 = 10540996611094048183;

pub const MDS: [u64; STATE_SIZE * STATE_SIZE] = [
    5910257123858819639,
    3449115226714951713,
    16770055338049327985,
    610399731775780810,
    7363016345531076300,
    16174724756564259629,
    8736587794472183152,
    12699016954477470956,
    13948112026909862966,
    18015813124076612987,
    9568929147539067610,
    14859461777592116402,
    18169364738825153183,
    18221568702798258352,
    1524268296724555606,
    5538821761600,
    1649528676200182784,
    336497118937017052,
    15805000027048028625,
    15709375513998678646,
    14837031240173858084,
    11366298206428370494,
    15698532768527519720,
    5911577595727321095,
    16676030327621016157,
    16537624251746851423,
    13325141695736654367,
    9337952653454313447,
    9090375522091353302,
    5605636660979522224,
    6357222834896114791,
    7776871531164456679,
    8264739868177574620,
    12732288338686680125,
    13022293791945187811,
    17403057736098613442,
    2871266924987061743,
    13286707530570640459,
    9229362695439112266,
    815317759014579856,
    7447771153889267897,
    2209002535000750347,
    3280506473249596174,
    13756142018694965622,
    10518080861296830621,
    16578355848983066277,
    12732532221704648123,
    3426526797578099186,
    8563516248221808333,
    13079317959606236131,
    15645458946300428515,
    9958819147895829140,
    13028053188247480206,
    6789511720078828478,
    6583246594815170294,
    4423695887326249884,
    9751139665897711642,
    10039202025292797758,
    12208726994829996150,
    6238795140281096003,
    9113696057226188857,
    9898705245385052191,
    4213712701625520075,
    8038355032286280912,
    426685147605824917,
    7673465577918025498,
    8452867379070564008,
    10827610229277395180,
    16155539332955658546,
    1575428636717115288,
    8765972548498757598,
    8405996249707890526,
    14855028677418679455,
    17878170012428694685,
    16572621079016066883,
    5311046098447994501,
    10635376800783355348,
    14205668690430323921,
    1181422971831412672,
    4651053123208915543,
    12465667489477238576,
    7300129031676503132,
    13458544786180633209,
    8946801771555977477,
    14203890406114400141,
    8219081892380458635,
    6035067543134909245,
    15140374581570897616,
    4514006299509426029,
    16757530089801321524,
    13202061911440346802,
    11227558237427129334,
    315998614524336401,
    11280705904396606227,
    5798516367202621128,
    17154761698338453414,
    13574436947400004837,
    3126509266905053998,
    10740979484255925394,
    9273322683773825324,
    15349096509718845737,
    14694022445619674948,
    8733857890739087596,
    3198488337424282101,
    9521016570828679381,
    11267736037298472148,
    14825280481028844943,
    1326588754335738002,
    6200834522767914499,
    1070210996042416038,
    9140190343656907671,
    15531381283521001952,
    253143295675927354,
    11977331414401291539,
    13941376566367813256,
    469904915148256197,
    10873951860155749104,
    3939719938926157877,
    2271392376641547055,
    4725974756185387075,
    14827835543640648161,
    17663273767033351157,
    12440960700789890843,
    16589620022628590428,
    12838889473653138505,
    11170336581460183657,
    7583333056198317221,
    6006908286410425140,
    15648567098514276013,
    188901633101859949,
    12256163716419861419,
    17319784688409668747,
    9648971065289440425,
    11370683735445551679,
    11265203235776280908,
    1737672785338087677,
    5225587291780939578,
    4739055740469849012,
    1212344601223444182,
    12958616893209019599,
    7922060480554370635,
    14661420107595710445,
    11744359917257111592,
    9674559564931202709,
    8326110231976411065,
    16856751238353701757,
    7515652322254196544,
    2062531989536141174,
    3875321171362100965,
    1164854003752487518,
    3997098993859160292,
    4074090397542250057,
    3050858158567944540,
    4568245569065883863,
    14559440781022773799,
    5401845794552358815,
    6544584366002554176,
    2511522072283652847,
    9759884967674698659,
    16411672358681189856,
    11392578809073737776,
    8013631514034873271,
    11439549174997471674,
    6373021446442411366,
    12491600135569477757,
    1017093281401495736,
    663547836518863091,
    16157302719777897692,
    11208801522915446640,
    10058178191286215107,
    5521712058210208094,
    3611681474253815005,
    4864578569041337696,
    12270319000993569289,
    7347066511426336318,
    6696546239958933736,
    3335469193383486908,
    12719366334180058014,
    14123019207894489639,
    11418186023060178542,
    2042199956854124583,
    17539253100488345226,
    16240833881391672847,
    11712520063241304909,
    6456900719511754234,
    1819022137223501306,
    7371152900053879920,
    6521878675261223812,
    2050999666988944811,
    8262038465464898064,
    13303819303390508091,
    12657292926928303663,
    8794128680724662595,
    4068577832515945116,
    758247715040138478,
    5600369601992438532,
    3369463178350382224,
    13763645328734311418,
    9685701761982837416,
    2711119809520557835,
    11680482056777716424,
    10958223503056770518,
    4168390070510137163,
    10823375744683484459,
    5613197991565754677,
    11781942063118564684,
    9352512500813609723,
    15997830646514778986,
    7407352006524266457,
    15312663387608602775,
    3026364159907661789,
    5698531403379362946,
    2544271242593770624,
    13104502948897878458,
    7840062700088318710,
    6028743588538970215,
    6144415809411296980,
    468368941216390216,
    3638618405705274008,
    11105401941482704573,
    1850274872877725129,
    1011155312563349004,
    3234620948537841909,
    3818372677739507813,
    4863130691592118581,
    8942166964590283171,
    3639677194051371072,
    15477372418124081864,
    10322228711752830209,
    9139111778956611066,
    202171733050704358,
    11982413146686512577,
    11001000478006340870,
    5491471715020327065,
    6969114856449768266,
    11088492421847219924,
    12913509272810999025,
    17366506887360149369,
    7036328554328346102,
    11139255730689011050,
    2844974929907956457,
    6488525141985913483,
    2860098796699131680,
    10366343151884073105,
    844875652557703984,
    1053177270393416978,
    5189466196833763142,
    1024738234713107670,
    8846741799369572841,
    14490406830213564822,
    10577371742628912722,
    3276210642025060502,
    2605621719516949928,
    5417148926702080639,
    11100652475866543814,
    5247366835775169839,
];

pub const MDS_INV: [u64; STATE_SIZE * STATE_SIZE] = [
    1572742562154761373,
    11904188991461183391,
    16702037635100780588,
    10395027733616703929,
    8130016957979279389,
    12091057987196709719,
    14570460902390750822,
    13452497170858892918,
    7302470671584418296,
    12930709087691977410,
    6940810864055149191,
    15479085069460687984,
    15273989414499187903,
    8742532579937987008,
    78143684950290654,
    10454925311792498315,
    7789818152192856725,
    3486011543032592030,
    17188770042768805161,
    10490412495468775616,
    298640180115056798,
    12895819509602002088,
    1755013598313843104,
    17242416429764373372,
    993835663551930043,
    17604339535769584753,
    17954116481891390155,
    332811330083846624,
    14730023810555747819,
    435413210797820565,
    1781261080337413422,
    4148505421656051973,
    980199695323775177,
    4706730905557535223,
    12734714246714791746,
    14273996233795959868,
    7921735635146743134,
    14772166129594741813,
    2171393332099124215,
    11431591906353698662,
    1968460689143086961,
    12435956952300281356,
    18203712123938736914,
    13226878153002754824,
    4722189513468037980,
    14552059159516237140,
    2186026037853355566,
    11286141841507813990,
    565856028734827369,
    13655906686104936396,
    8559867348362880285,
    2797343365604350633,
    4465794635391355875,
    10602340776590577912,
    6532765362293732644,
    9971594382705594993,
    8246981798349136173,
    4260734168634971109,
    3096607081570771,
    823237991393038853,
    17532689952600815755,
    12134755733102166916,
    10570439735096051664,
    18403803913856082900,
    13128404168847275462,
    16663835358650929116,
    16546671721888068220,
    4685011688485137218,
    1959001578540316019,
    16340711608595843821,
    9460495021221259854,
    3858517940845573321,
    9427670160758976948,
    18064975260450261693,
    4905506444249847758,
    15986418616213903133,
    9282818778268010424,
    9769107232941785010,
    8521948467436343364,
    7419602577337727529,
    5926710664024036226,
    11667040483862285999,
    12291037072726747355,
    12257844845576909578,
    5216888292865522221,
    4949589496388892504,
    6571373688631618567,
    10091372984903831417,
    6240610640427541397,
    6328690792776976228,
    11836184983048970818,
    12710419323566440454,
    10374451385652807364,
    8254232795575550118,
    9866490979395302091,
    12991014125893242232,
    1063347186953727863,
    2952135743830082310,
    17315974856538709017,
    14554512349953922358,
    14134347382797855179,
    17882046380988406016,
    17463193400175360824,
    3726957756828900632,
    17604631050958608669,
    7585987025945897953,
    14470977033142357695,
    10643295498661723800,
    8871197056529643534,
    8384208064507509379,
    9280566467635869786,
    87319369282683875,
    1100172740622998121,
    622721254307916221,
    16843330035110191506,
    13024130485811341782,
    12334996107415540952,
    461552745543935046,
    8140793910765831499,
    9008477689109468885,
    17409910369122253035,
    1804565454784197696,
    5310948951638903141,
    12531953612536647976,
    6147853502869470889,
    1125351356112285953,
    6467901683012265601,
    16792548587138841945,
    14092833521360698433,
    13651748079341829335,
    10688258556205752814,
    1823953496327460008,
    2558053704584850519,
    13269131806718310421,
    4608410977522599149,
    9221187654763620553,
    4611978991500182874,
    8855429001286425455,
    5696709580182222832,
    17579496245625003067,
    5267934104348282564,
    1835676094870249003,
    3542280417783105151,
    11824126253481498070,
    9504622962336320170,
    17887320494921151801,
    6574518722274623914,
    16658124633332643846,
    13808019273382263890,
    13092903038683672100,
    501471167473345282,
    11161560208140424921,
    13001827442679699140,
    14739684132127818993,
    2868223407847949089,
    1726410909424820290,
    6794531346610991076,
    6698331109000773276,
    3680934785728193940,
    8875468921351982841,
    5477651765997654015,
    12280771278642823764,
    3619998794343148112,
    6883119128428826230,
    13512760119042878827,
    3675597821767844913,
    5414638790278102151,
    3587251244316549755,
    17100313981528550060,
    11048426899172804713,
    1396562484529002856,
    2252873797267794672,
    14201526079271439737,
    16618356769072634008,
    144564843743666734,
    11912794688498369701,
    10937102025343594422,
    15432144252435329607,
    2221546737981282133,
    6015808993571140081,
    7447996510907844453,
    7039231904611782781,
    2218118803134364409,
    9472427559993341443,
    11066826455107746221,
    6223571389973384864,
    13615228926415811268,
    10241352486499609335,
    12605380114102527595,
    11403123666082872720,
    9771232158486004346,
    11862860570670038891,
    10489319728736503343,
    588166220336712628,
    524399652036013851,
    2215268375273320892,
    1424724725807107497,
    2223952838426612865,
    1901666565705039600,
    14666084855112001547,
    16529527081633002035,
    3475787534446449190,
    17395838083455569055,
    10036301139275236437,
    5830062976180250577,
    6201110308815839738,
    3908827014617539568,
    13269427316630307104,
    1104974093011983663,
    335137437077264843,
    13411663683768112565,
    7907493007733959147,
    17240291213488173803,
    6357405277112016289,
    7875258449007392338,
    16100900298327085499,
    13542432207857463387,
    9466802464896264825,
    9221606791343926561,
    10417300838622453849,
    13201838829839066427,
    9833345239958202067,
    16688814355354359676,
    13315432437333533951,
    378443609734580293,
    14654525144709164243,
    1967217494445269914,
    16045947041840686058,
    18049263629128746044,
    1957063364541610677,
    16123386013589472221,
    5923137592664329389,
    12399617421793397670,
    3403518680407886401,
    6416516714555000604,
    13286977196258324106,
    17641011370212535641,
    14823578540420219384,
    11909888788340877523,
    11040604022089158722,
    14682783085930648838,
    7896655986299558210,
    9328642557612914244,
    6213125364180629684,
    16259136970573308007,
    12025260496935037210,
    1512031407150257270,
    1295709332547428576,
    13851880110872460625,
    6734559515296147531,
    17720805166223714561,
    11264121550751120724,
    7210341680607060660,
    17759718475616004694,
    610155440804635364,
    3209025413915748371,
];

pub const ROUND_CONSTANTS: [u64; NUM_ROUNDS * STATE_SIZE * 2] = [
    3006656781416918236,
    4369161505641058227,
    6684374425476535479,
    15779820574306927140,
    9604497860052635077,
    6451419160553310210,
    16926195364602274076,
    6738541355147603274,
    13653823767463659393,
    16331310420018519380,
    10921208506902903237,
    5856388654420905056,
    180518533287168595,
    6394055120127805757,
    4624620449883041133,
    4245779370310492662,
    11436753067664141475,
    9565904130524743243,
    1795462928700216574,
    6069083569854718822,
    16847768509740167846,
    4958030292488314453,
    6638656158077421079,
    7387994719600814898,
    1380138540257684527,
    2756275326704598308,
    6162254851582803897,
    4357202747710082448,
    12150731779910470904,
    3121517886069239079,
    14951334357190345445,
    11174705360936334066,
    17619090104023680035,
    9879300494565649603,
    6833140673689496042,
    8026685634318089317,
    6481786893261067369,
    15148392398843394510,
    11231860157121869734,
    2645253741394956018,
    15345701758979398253,
    1715545688795694261,
    3419893440622363282,
    12314745080283886274,
    16173382637268011204,
    2012426895438224656,
    6886681868854518019,
    9323151312904004776,
    14061124303940833928,
    14720644192628944300,
    3643016909963520634,
    15164487940674916922,
    18095609311840631082,
    17450128049477479068,
    13770238146408051799,
    959547712344137104,
    12896174981045071755,
    15673600445734665670,
    5421724936277706559,
    15147580014608980436,
    10475549030802107253,
    9781768648599053415,
    12208559126136453589,
    14883846462224929329,
    4104889747365723917,
    748723978556009523,
    1227256388689532469,
    5479813539795083611,
    8771502115864637772,
    16732275956403307541,
    4416407293527364014,
    828170020209737786,
    12657110237330569793,
    6054985640939410036,
    4339925773473390539,
    12523290846763939879,
    6515670251745069817,
    3304839395869669984,
    13139364704983394567,
    7310284340158351735,
    10864373318031796808,
    17752126773383161797,
    1934077736434853411,
    12181011551355087129,
    16512655861290250275,
    17788869165454339633,
    12226346139665475316,
    521307319751404755,
    18194723210928015140,
    11017703779172233841,
    15109417014344088693,
    16118100307150379696,
    16104548432406078622,
    10637262801060241057,
    10146828954247700859,
    14927431817078997000,
    8849391379213793752,
    14873391436448856814,
    15301636286727658488,
    14600930856978269524,
    14900320206081752612,
    9439125422122803926,
    17731778886181971775,
    11364016993846997841,
    11610707911054206249,
    16438527050768899002,
    1230592087960588528,
    11390503834342845303,
    10608561066917009324,
    5454068995870010477,
    13783920070953012756,
    10807833173700567220,
    8597517374132535250,
    17631206339728520236,
    8083932512125088346,
    10460229397140806011,
    16904442127403184100,
    15806582425540851960,
    8002674967888750145,
    7088508235236416142,
    2774873684607752403,
    11519427263507311324,
    14949623981479468161,
    18169367272402768616,
    13279771425489376175,
    3437101568566296039,
    11820510872362664493,
    13649520728248893918,
    13432595021904865723,
    12153175375751103391,
    16459175915481931891,
    14698099486055505377,
    14962427686967561007,
    10825731681832829214,
    12562849212348892143,
    18054851842681741827,
    16866664833727482321,
    10485994783891875256,
    8074668712578030015,
    7502837771635714611,
    8326381174040960025,
    1299216707593490898,
    12092900834113479279,
    10147133736028577997,
    12103660182675227350,
    16088613802080804964,
    10323305955081440356,
    12814564542614394316,
    9653856919559060601,
    10390420172371317530,
    7831993942325060892,
    9568326819852151217,
    6299791178740935792,
    12692828392357621723,
    10331476541693143830,
    3115340436782501075,
    17456578083689713056,
    12924575652913558388,
    14365487216177868031,
    7211834371191912632,
    17610068359394967554,
    646302646073569086,
    12437378932700222679,
    2758591586601041336,
    10952396165876183059,
    8827205511644136726,
    17572216767879446421,
    12516044823385174395,
    6380048472179557105,
    1959389938825200414,
    257915527015303758,
    4942451629986849727,
    1698530521870297461,
    1802136667015215029,
    6353258543636931941,
    13791525219506237119,
    7093082295632492630,
    15409842367405634814,
    2090232819855225051,
    13926160661036606054,
    389467431021126699,
    4736917413147385608,
    6217341363393311211,
    4366302820407593918,
    12748238635329332117,
    7671680179984682360,
    17998193362025085453,
    432899318054332645,
    1973816396170253277,
    607886411884636526,
    15080416519109365682,
    13607062276466651973,
    2458254972975404730,
    15323169029557757131,
    10953434699543086460,
    13995946730291266219,
    12803971247555868632,
    3974568790603251423,
    10629169239281589943,
    2058261494620094806,
    15905212873859894286,
    11221574225004694137,
    15430295276730781380,
    10448646831319611878,
    7559293484620816204,
    15679753002507105741,
    6043747003590355195,
    3404573815097301491,
    13392826344874185313,
    6464466389567159772,
    8932733991045074013,
    6565970376680631168,
    7050411859293315754,
    9763347751680159247,
    3140014248604700259,
    5621238883761074228,
    12664766603293629079,
    6533276137502482405,
    914829860407409680,
    14599697497440353734,
    16400390478099648992,
    1619185634767959932,
    16420198681440130663,
    1331388886719756999,
    1430143015191336857,
    14618841684410509097,
    1870494251298489312,
    3783117677312763499,
    16164771504475705474,
    6996935044500625689,
    4356994160244918010,
    13579982029281680908,
    8835524728424198741,
    13281017722683773148,
    2669924686363521592,
    15020410046647566094,
    9534143832529454683,
    156263138519279564,
    17421879327900831752,
    9524879102847422379,
    5120021146470638642,
    9588770058331935449,
    1501841070476096181,
    5687728871183511192,
    16091855309800405887,
    17307425956518746505,
    1162636238106302518,
    8756478993690213481,
    6898084027896327288,
    8485261637658061794,
    4169208979833913382,
    7776158701576840241,
    13861841831073878156,
    4896983281306117497,
    6056805506026814259,
    15706891000994288769,
];

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RescuePrimeRegular {
    pub state: [BFieldElement; STATE_SIZE],
}

impl RescuePrimeRegular {
    /// new
    /// Create a new sponge object. This function is used internally.
    #[allow(dead_code)]
    fn new() -> Self {
        RescuePrimeRegular {
            state: [BFieldElement::ring_zero(); STATE_SIZE],
        }
    }

    /// xlix_round
    /// Apply one round of the XLIX permutation.
    #[allow(dead_code)]
    fn xlix_round(&mut self, round_index: usize) {
        assert!(
            round_index < NUM_ROUNDS,
            "Cannot apply {}th round; only have {} in total.",
            round_index,
            NUM_ROUNDS
        );

        // S-box
        for i in 0..STATE_SIZE {
            self.state[i] = self.state[i].mod_pow_u64(ALPHA);
        }

        // MDS matrix
        let mut v: [BFieldElement; STATE_SIZE] = [BFieldElement::new(0u64); STATE_SIZE];
        for i in 0..STATE_SIZE {
            for j in 0..STATE_SIZE {
                v[i] += BFieldElement::new(MDS[i * STATE_SIZE + j]) * self.state[j];
            }
        }
        self.state = v;

        // round constants A
        for i in 0..STATE_SIZE {
            self.state[i] += BFieldElement::new(ROUND_CONSTANTS[round_index * STATE_SIZE * 2 + i]);
        }

        // Inverse S-box
        for i in 0..STATE_SIZE {
            self.state[i] = self.state[i].mod_pow_u64(ALPHA_INV);
        }
        for i in 0..STATE_SIZE {
            v[i] = BFieldElement::ring_zero();
            for j in 0..STATE_SIZE {
                v[i] += BFieldElement::new(MDS[i * STATE_SIZE + j]) * self.state[j];
            }
        }
        self.state = v;

        // round constants B
        for i in 0..STATE_SIZE {
            self.state[i] +=
                BFieldElement::new(ROUND_CONSTANTS[round_index * STATE_SIZE * 2 + STATE_SIZE + i]);
        }
    }

    /// xlix
    /// XLIX is the permutation defined by Rescue-Prime. This
    /// function applies XLIX to the state of a sponge.
    #[allow(dead_code)]
    fn xlix(&mut self) {
        for i in 0..NUM_ROUNDS {
            self.xlix_round(i);
        }
    }

    /// hash_10
    /// Hash 10 elements, or two digests. There is no padding because
    /// the input length is fixed.
    #[allow(dead_code)]
    fn hash_10(input: [BFieldElement; 10]) -> [BFieldElement; 5] {
        let mut sponge = Self::new();

        // absorb once
        sponge.state[..10].copy_from_slice(&input);

        // apply xlix
        sponge.xlix();

        // squeeze once
        sponge.state[..5].try_into().unwrap()
    }

    /// hash_varlen
    /// Hash an arbitrary number of field elements. Takes care of
    /// padding.
    #[allow(dead_code)]
    fn hash_varlen(input: &[BFieldElement]) -> [BFieldElement; 5] {
        let mut sponge = Self::new();

        // pad input
        let mut padded_input = input.to_vec();
        padded_input.push(BFieldElement::ring_one());
        while padded_input.len() % RATE != 0 {
            padded_input.push(BFieldElement::ring_zero());
        }

        // absorb
        while !padded_input.is_empty() {
            for (sponge_state_element, input_element) in sponge
                .state
                .iter_mut()
                .take(RATE)
                .zip_eq(padded_input.iter().take(RATE))
            {
                *sponge_state_element += input_element.to_owned();
            }
            padded_input = padded_input[RATE..].to_vec();
            sponge.xlix();
        }

        // squeeze once
        sponge.state[..5].try_into().unwrap()
    }
}

#[cfg(test)]
mod rescue_prime_regular_tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_compliance() {
        // hash 10, first batch
        let targets_first_batch: [[u64; 5]; 10] = [
            [
                3711666231709755894,
                17384520371490492612,
                12519181950482751223,
                8881538813396339421,
                11721401967599973822,
            ],
            [
                3925419690564839133,
                5931190319485970901,
                10741999842343541948,
                4379361927708357429,
                624425822377240257,
            ],
            [
                5331222247009081977,
                9629957264924751254,
                5472011874756883892,
                14344107133748284933,
                14970576837907603547,
            ],
            [
                14272762628928522800,
                15123724254351143775,
                4189549910834528442,
                18211802542628133906,
                10378470030067250620,
            ],
            [
                9028505522423420051,
                4548947560070025805,
                365873200364364602,
                13396630679584734738,
                2902158397921607703,
            ],
            [
                2728682124193801200,
                5029580824698741384,
                10966522651132571069,
                17005072713855867631,
                13173883832108679094,
            ],
            [
                9034061418505774715,
                10437510895109481550,
                9148840397973008160,
                13830213613005541037,
                7975937385060295448,
            ],
            [
                179231839474090644,
                11485838938749715951,
                11407963643270877283,
                2525018771010700841,
                6003326942652031452,
            ],
            [
                8886510033299586384,
                16641376996742108698,
                14336686901719852709,
                6393684032912544087,
                10756510615894243169,
            ],
            [
                3984287137806488239,
                12491581399256955922,
                17075276231967322881,
                14463990650613004419,
                1480614519044393603,
            ],
        ];
        let mut targets_bfe: Vec<Vec<BFieldElement>> = targets_first_batch
            .iter()
            .map(|l| l.iter().map(|e| BFieldElement::new(*e)).collect_vec())
            .collect_vec();
        let mut input = [BFieldElement::ring_zero(); 10];
        for i in 0..10 {
            input[input.len() - 1] = BFieldElement::new(i as u64);
            assert_eq!(targets_bfe[i], RescuePrimeRegular::hash_10(input).to_vec());
        }

        // hash 10, second batch
        let targets_second_batch: [[u64; 5]; 10] = [
            [
                9954340196098770044,
                16766478858550921719,
                14795358262939961687,
                5971715312175262159,
                10621735453321362721,
            ],
            [
                12391337510008436236,
                15657559504547420941,
                9377428313701566093,
                6455690240973939776,
                17925569643122616714,
            ],
            [
                14174674396431091177,
                10419196049487033389,
                13966773987063948344,
                483969843169798562,
                3639530542734092584,
            ],
            [
                14166842618688128156,
                17168550802342907985,
                3721514828700812032,
                4755976588553173055,
                5040960317306343383,
            ],
            [
                6393788816962046948,
                7027503643088443403,
                1123213526028638111,
                2111884675417454493,
                6407675412275457708,
            ],
            [
                7881221386634165564,
                17805016662820222439,
                12929479976497464999,
                7122243629396859500,
                2136259294370865427,
            ],
            [
                2759425216273856214,
                5365163625581141111,
                5562349150005538806,
                2022465106184614757,
                14545944666541784105,
            ],
            [
                4197295740525818,
                3710188887179138501,
                2520368773656332449,
                7135604343463999533,
                12275919319873645417,
            ],
            [
                5163999481497719848,
                14208882111473173488,
                6905551447652867406,
                5506325960735952984,
                13454019500172070006,
            ],
            [
                3925419690564839133,
                5931190319485970901,
                10741999842343541948,
                4379361927708357429,
                624425822377240257,
            ],
        ];
        targets_bfe = targets_second_batch
            .iter()
            .map(|l| l.iter().map(|e| BFieldElement::new(*e)).collect_vec())
            .collect_vec();
        input[input.len() - 1] = BFieldElement::ring_zero();
        for i in 0..10 {
            input[i] = BFieldElement::ring_one();
            assert_eq!(targets_bfe[i], RescuePrimeRegular::hash_10(input).to_vec());
            input[i] = BFieldElement::ring_zero();
        }

        // hash varlen, third batch
        let targets_third_batch: [[u64; 5]; 20] = [
            [
                9954340196098770044,
                16766478858550921719,
                14795358262939961687,
                5971715312175262159,
                10621735453321362721,
            ],
            [
                12391337510008436236,
                15657559504547420941,
                9377428313701566093,
                6455690240973939776,
                17925569643122616714,
            ],
            [
                14112140563534526715,
                12091338198119135732,
                16277751626976027823,
                4331384491863420413,
                15800084865512048249,
            ],
            [
                15410161320350300674,
                12862508375582878113,
                1871289024748006724,
                1120358582983879653,
                10608258519034552134,
            ],
            [
                1107344292722494872,
                17391364595468230070,
                4218215235563531160,
                7497689442794338714,
                3900922406630849053,
            ],
            [
                16822019589580184555,
                7989891526544888053,
                14569641731101827620,
                17919386380356552805,
                7463713352054333042,
            ],
            [
                674513153759557192,
                5885835165060751739,
                7545202825089012468,
                7455443267898983077,
                11460188487338022037,
            ],
            [
                1452905481212349824,
                4602871015292638258,
                16799505315703203495,
                15502476305285227202,
                14418240163510509007,
            ],
            [
                2169799351531593851,
                141901148303731658,
                12571509576917037512,
                2730951366471393395,
                10868840823954592153,
            ],
            [
                5004294054773159410,
                15035327361975356310,
                14190623520133446702,
                16843665251688123638,
                4543333205754908370,
            ],
            [
                16497508931324347828,
                10379016827033660777,
                5027352471010305075,
                15362732119758725484,
                13390969807239861733,
            ],
            [
                1743559568736995800,
                11815709956493259346,
                5763576938286686837,
                7541138447063081288,
                17969015713376415699,
            ],
            [
                10441678943133242957,
                15290592304889070108,
                18288160234755515065,
                3671382450876247307,
                3447450231474938402,
            ],
            [
                11057569330409963321,
                4984952761946312859,
                16529019269578375042,
                1908152979369527531,
                7121827819059879337,
            ],
            [
                17067972955397432517,
                2912062349216497629,
                15263972887304976204,
                9246522127607732383,
                17610927156233305697,
            ],
            [
                5980270367087450085,
                2990338491388854267,
                3198993023459349000,
                5035257001959372883,
                5260797048498744804,
            ],
            [
                8542899768037601505,
                5239516840302652488,
                2299137376555803866,
                952010414036958775,
                9717098700918296507,
            ],
            [
                8231024478155080292,
                9594681520895674398,
                191017068357133911,
                1512051294906340420,
                12055973608766483576,
            ],
            [
                16653142742451850722,
                9252945525340562222,
                4805241920959388929,
                937662086458078174,
                17775208482321191727,
            ],
            [
                14634923894397095166,
                1247948061415695017,
                3048493836613607105,
                2432649783604354905,
                7424726151688166928,
            ],
        ];
        targets_bfe = targets_third_batch
            .iter()
            .map(|l| l.iter().map(|e| BFieldElement::new(*e)).collect_vec())
            .collect_vec();
        for i in 0..20 {
            let var_input = (0..i).map(|e| BFieldElement::new(e as u64)).collect_vec();
            assert_eq!(
                RescuePrimeRegular::hash_varlen(&var_input).to_vec(),
                targets_bfe[i]
            );
        }
    }
}
