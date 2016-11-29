
extern crate rustc_serialize;
pub mod full_extract {
	#[derive(RustcDecodable)]
	pub struct TPCRecord {
	    agir1_puf: f32,
	    efi_puf: f32,
	    eic_puf: f32,
	    preselect_puf: f32,
	    fded: f32,
	    year_puf: f32,
	    month_puf: f32,
	    numchd: f32,
	    form3800_puf: f32,
	    f6251_puf: f32,
	    f8582_puf: f32,
	    f8606_puf: f32,
	    ie_puf: f32,
	    mars: f32,
	    midr_puf: f32,
	    numhope_puf: f32,
	    nctc_puf: f32,
	    numll_puf: f32,
	    prep_puf: f32,
	    schb_puf: f32,
	    schcf_puf: f32,
	    sche_puf: f32,
	    state_puf: f32,
	    tform_puf: f32,
	    txst_puf: f32,
	    xfpt: f32,
	    xfst: f32,
	    homex: f32,
	    awayex: f32,
	    othrex: f32,
	    parex: f32,
	    xtot_puf: f32,
	    age65: f32,
	    soiflag: f32,
	    eligmom_cdctc: f32,
	    ncdctc: f32,
	    nctc: f32,
	    eitck_puf: f32,
	    ndep: f32,
	    chdlt17_chdep: f32,
	    chdlt13_chdep: f32,
	    chdlt17: f32,
	    chdlt13: f32,
	    heicelig_lapuf: f32,
	    heic_lapuf: f32,
	    ws: f32,
	    intinc: f32,
	    txexint: f32,
	    taxdiv: f32,
	    qualdiv: f32,
	    stlref: f32,
	    alimony: f32,
	    businc: f32,
	    capgain_puf: f32,
	    cg_1040: f32,
	    cpgsup: f32,
	    taxira: f32,
	    pentot: f32,
	    penagi: f32,
	    schede: f32,
	    farm: f32,
	    totunemp: f32,
	    ssbrec: f32,
	    ssbagi_puf: f32,
	    iraexp: f32,
	    studloan_puf: f32,
	    edexp: f32,
	    tuitfees_puf: f32,
	    secaded_puf: f32,
	    sehealth: f32,
	    dpad: f32,
	    hsaexp: f32,
	    keoexp: f32,
	    penalty: f32,
	    paidalim: f32,
	    agi_puf: f32,
	    totded_puf: f32,
	    fdexmp_puf: f32,
	    ti_puf: f32,
	    tax_puf: f32,
	    comptax_puf: f32,
	    taxb4_puf: f32,
	    income_puf: f32,
	    margbase_puf: f32,
	    taxgen_puf: f32,
	    amt_puf: f32,
	    cdctc_puf: f32,
	    eldcrd_puf: f32,
	    ctc_puf: f32,
	    edcrd_puf: f32,
	    savcrd_puf: f32,
	    energy_puf: f32,
	    forcrd_org: f32,
	    gnbcrd_puf: f32,
	    amtcrd_puf: f32,
	    othcrd: f32,
	    totcrd_puf: f32,
	    tottax_puf: f32,
	    taxaftr_puf: f32,
	    seca_puf: f32,
	    recapture: f32,
	    sstip: f32,
	    irapenalty: f32,
	    totliab_puf: f32,
	    withtax_puf: f32,
	    esttax_puf: f32,
	    eicern_puf: f32,
	    eicliab_puf: f32,
	    eicpay_puf: f32,
	    eicref_puf: f32,
	    ctcref_puf: f32,
	    paidext_puf: f32,
	    excessfica_puf: f32,
	    fuelcrd_puf: f32,
	    riccrd_puf: f32,
	    telecrd_puf: f32,
	    totpay_puf: f32,
	    refund_puf: f32,
	    elect_puf: f32,
	    taxpen_puf: f32,
	    medsum: f32,
	    txstat: f32,
	    slstax: f32,
	    txreal: f32,
	    totint_puf: f32,
	    cashcont_puf: f32,
	    contno50_puf: f32,
	    ncashcont_puf: f32,
	    totcont: f32,
	    busexp_puf: f32,
	    prepfees_puf: f32,
	    msctot: f32,
	    msclim_puf: f32,
	    castheft: f32,
	    pease_puf: f32,
	    c_netrec_puf: f32,
	    c_cstgoods_puf: f32,
	    c_othinc_puf: f32,
	    c_deprec_puf: f32,
	    c_insure_puf: f32,
	    c_mortint_puf: f32,
	    c_othint_puf: f32,
	    c_offexp_puf: f32,
	    c_netwages_puf: f32,
	    c_totded_puf: f32,
	    shrgain: f32,
	    longother_puf: f32,
	    cgdistd_puf: f32,
	    longgain: f32,
	    sec1250gn_puf: f32,
	    cglessinv_puf: f32,
	    cg28rate_puf: f32,
	    schedd5_puf: f32,
	    cgtaxnond_puf: f32,
	    schedd15_puf: f32,
	    schedd25_puf: f32,
	    schedd28_puf: f32,
	    renttot_puf: f32,
	    roytot_puf: f32,
	    rentmort_puf: f32,
	    rentothint_puf: f32,
	    roydep_puf: f32,
	    rentdep_puf: f32,
	    rntnet: f32,
	    roynet: f32,
	    rentloss_puf: f32,
	    rntroyinc_puf: f32,
	    rntroyloss_puf: f32,
	    prtpg_puf: f32,
	    prtnpg: f32,
	    prtpl_puf: f32,
	    prtnpl: f32,
	    prtded: f32,
	    sbcpg_puf: f32,
	    sbcnpg: f32,
	    sbcpl_puf: f32,
	    sbcnpl: f32,
	    prtsbc: f32,
	    sbcded: f32,
	    estgan: f32,
	    estlos: f32,
	    frmrnt: f32,
	    slfinc: f32,
	    seisec: f32,
	    chdexp: f32,
	    chdearn: f32,
	    invcrd_puf: f32,
	    gbcjob_puf: f32,
	    rescrd: f32,
	    gnbcrd_pre: f32,
	    invint4952_puf: f32,
	    invincelct_puf: f32,
	    amtothprf: f32,
	    amttotadj_puf: f32,
	    tiforamt_puf: f32,
	    amti_puf: f32,
	    amtforcrd_puf: f32,
	    amtschedd_puf: f32,
	    amt1250_puf: f32,
	    amtcg_puf: f32,
	    passinc_puf: f32,
	    passloss_puf: f32,
	    pslssallow_puf: f32,
	    amtcrd_carryf: f32,
	    frmelect_puf: f32,
	    frmtenttax_puf: f32,
	    frmacttax_puf: f32,
	    hope_exp_puf: f32,
	    hope_half_puf: f32,
	    hope_puf: f32,
	    ll_exp_puf: f32,
	    ll_lim_puf: f32,
	    ll_puf: f32,
	    pub rtrnid_puf: f32,
	    lapuf_wgt: f32,
	    smplecount_puf: f32,
	    popcount_puf: f32,
	    sample_puf: f32,
	    taxrate_puf: f32,
	    otherinc: f32,
	    longcry: f32,
	    itemres: f32,
	    atxprf: f32,
	    invint: f32,
	    homint: f32,
	    ind_age: f32,
	    depage_1: f32,
	    depage_2: f32,
	    depage_3: f32,
	    depage_4: f32,
	    depage_5: f32,
	    depage_6: f32,
	    depage_7: f32,
	    depage_8: f32,
	    depage_9: f32,
	    depage_10: f32,
	    ssi: f32,
	    tanf: f32,
	    workcomp: f32,
	    vetben: f32,
	    childsup: f32,
	    disabinc: f32,
	    socsec: f32,
	    homeown: f32,
	    wsplit: f32,
	    energy: f32,
	    fstamps: f32,
	    lunches: f32,
	    ind_race_1: f32,
	    ind_race_2: f32,
	    ind_orig_1: f32,
	    ind_orig_2: f32,
	    survben: f32,
	    cpsother: f32,
	    edassist: f32,
	    fincps: f32,
	    weight_mult: f32,
	    agi_cl: f32,
		eci_cl: f32, 
		eciadj_cl: f32, 
		cashinc_cl: f32, 
		cashincadj_cl: f32,
		taxout_1: f32,
		taxout_2: f32,
		taxout_3: f32,
		taxout_4: f32,
		taxout_5: f32,
		taxout_6: f32,
		taxout_7: f32,
		taxout_8: f32,
		taxout_9: f32,
		taxout_10: f32,
		taxout_11: f32,
		taxout_12: f32,
		taxout_13: f32,
		taxout_14: f32,
		taxout_15: f32,
		taxout_16: f32,
		taxout_17: f32,
		taxout_18: f32,
		taxout_19: f32,
		taxout_20: f32,
		taxout_21: f32,
		taxout_22: f32,
		taxout_23: f32,
		taxout_24: f32,
		taxout_25: f32,
		taxout_26: f32,
		taxout_27: f32,
		taxout_28: f32,
		taxout_29: f32,
		taxout_30: f32,
		taxout_31: f32,
		taxout_32: f32,
		taxout_33: f32,
		taxout_34: f32,
		taxout_35: f32,
		taxout_36: f32,
		taxout_37: f32,
		taxout_38: f32,
		taxout_39: f32,
		taxout_40: f32,
		taxout_41: f32,
		taxout_42: f32,
		taxout_43: f32,
		taxout_44: f32,
		taxout_45: f32,
		taxout_46: f32,
		taxout_47: f32,
		taxout_48: f32,
		taxout_49: f32,
		taxout_50: f32,
		taxout_51: f32,
		taxout_52: f32,
		taxout_53: f32,
		taxout_54: f32,
		taxout_55: f32,
		taxout_56: f32,
		taxout_57: f32,
		taxout_58: f32,
		taxout_59: f32,
		taxout_60: f32,
		taxout_61: f32,
		taxout_62: f32,
		taxout_63: f32,
		taxout_64: f32,
		taxout_65: f32,
		taxout_66: f32,
		taxout_67: f32,
		taxout_68: f32,
		taxout_69: f32,
		taxout_70: f32,
		taxout_71: f32,
		taxout_72: f32,
		taxout_73: f32,
		taxout_74: f32,
		taxout_75: f32,
		taxout_76: f32,
		taxout_77: f32,
		taxout_78: f32,
		taxout_79: f32,
		taxout_80: f32,
		taxout_81: f32,
		taxout_82: f32,
		taxout_83: f32,
		taxout_84: f32,
		taxout_85: f32,
		taxout_86: f32,
		taxout_87: f32,
		taxout_88: f32,
		taxout_89: f32,
		taxout_90: f32,
		taxout_91: f32,
		taxout_92: f32,
		taxout_93: f32,
		taxout_94: f32,
		taxout_95: f32,
		taxout_96: f32,
		taxout_97: f32,
		taxout_98: f32,
		taxout_99: f32,
		taxout_100: f32,
		taxout_101: f32,
		taxout_102: f32,
		taxout_103: f32,
		taxout_104: f32,
		taxout_105: f32,
		taxout_106: f32,
		taxout_107: f32,
		taxout_108: f32,
		taxout_109: f32,
		taxout_110: f32,
		taxout_111: f32,
		taxout_112: f32,
		taxout_113: f32,
		taxout_114: f32,
		taxout_115: f32,
		taxout_116: f32,
		taxout_117: f32,
		taxout_118: f32,
		taxout_119: f32,
		taxout_120: f32,
		taxout_121: f32,
		taxout_122: f32,
		taxout_123: f32,
		taxout_124: f32,
		taxout_125: f32,
	}
}