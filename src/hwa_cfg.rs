#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hwaccreg1: Hwaccreg1,
    hwaccreg2: Hwaccreg2,
    hwaccreg3: Hwaccreg3,
    hwaccreg4: Hwaccreg4,
    hwaccreg5: Hwaccreg5,
    hwaccreg6: Hwaccreg6,
    hwaccreg7: Hwaccreg7,
    hwaccreg8: Hwaccreg8,
    hwaccreg11: Hwaccreg11,
    hwaccreg12: Hwaccreg12,
    hwaccreg13: Hwaccreg13,
    hwaccreg14: Hwaccreg14,
    hwaccreg15: Hwaccreg15,
    cfar_det_thr: CfarDetThr,
    max1value: Max1value,
    max1index: Max1index,
    isum1lsb: Isum1lsb,
    isum1msb: Isum1msb,
    qsum1lsb: Qsum1lsb,
    qsum1msb: Qsum1msb,
    max2value: Max2value,
    max2index: Max2index,
    isum2lsb: Isum2lsb,
    isum2msb: Isum2msb,
    qsum2lsb: Qsum2lsb,
    qsum2msb: Qsum2msb,
    max3value: Max3value,
    max3index: Max3index,
    isum3lsb: Isum3lsb,
    isum3msb: Isum3msb,
    qsum3lsb: Qsum3lsb,
    qsum3msb: Qsum3msb,
    max4value: Max4value,
    max4index: Max4index,
    isum4lsb: Isum4lsb,
    isum4msb: Isum4msb,
    qsum4lsb: Qsum4lsb,
    qsum4msb: Qsum4msb,
    cfartest: Cfartest,
    rdstatus: Rdstatus,
    sigdmach1done: Sigdmach1done,
    sigdmach2done: Sigdmach2done,
    sigdmach3done: Sigdmach3done,
    sigdmach4done: Sigdmach4done,
    sigdmach5done: Sigdmach5done,
    sigdmach6done: Sigdmach6done,
    sigdmach7done: Sigdmach7done,
    sigdmach8done: Sigdmach8done,
    sigdmach9done: Sigdmach9done,
    sigdmach10done: Sigdmach10done,
    sigdmach11done: Sigdmach11done,
    sigdmach12done: Sigdmach12done,
    sigdmach13done: Sigdmach13done,
    sigdmach14done: Sigdmach14done,
    sigdmach15done: Sigdmach15done,
    sigdmach16done: Sigdmach16done,
    memaccesserr: Memaccesserr,
    fftclip: Fftclip,
    fftpeakcnt: Fftpeakcnt,
    hwaccreg1rd: Hwaccreg1rd,
    hwaccreg2rd: Hwaccreg2rd,
    hwaccreg3rd: Hwaccreg3rd,
    cmp_ege_k0123: CmpEgeK0123,
    cmp_ege_k4567: CmpEgeK4567,
    hwa_safety_enable: HwaSafetyEnable,
    meminit: Meminit,
    meminitdone: Meminitdone,
    hwa_safety_win_ram_err_loc: HwaSafetyWinRamErrLoc,
    hwa_safety_param_ram_err_loc: HwaSafetyParamRamErrLoc,
    hwa_safety_iping_err_loc: HwaSafetyIpingErrLoc,
    hwa_safety_ipong_err_loc: HwaSafetyIpongErrLoc,
    hwa_safety_oping_err_loc: HwaSafetyOpingErrLoc,
    hwa_safety_opong_err_loc: HwaSafetyOpongErrLoc,
    fftintmemwrdata: Fftintmemwrdata,
    fftintmemrddata: Fftintmemrddata,
    hwaccreg16: Hwaccreg16,
    dcest1i_sw: Dcest1iSw,
    dcest2i_sw: Dcest2iSw,
    dcest3i_sw: Dcest3iSw,
    dcest4i_sw: Dcest4iSw,
    dcest5i_sw: Dcest5iSw,
    dcest6i_sw: Dcest6iSw,
    dcest1i: Dcest1i,
    dcest2i: Dcest2i,
    dcest3i: Dcest3i,
    dcest4i: Dcest4i,
    dcest5i: Dcest5i,
    dcest6i: Dcest6i,
    dc_acc1i_lsb: DcAcc1iLsb,
    dc_acc1i_msb: DcAcc1iMsb,
    dc_acc2i_lsb: DcAcc2iLsb,
    dc_acc2i_msb: DcAcc2iMsb,
    dc_acc3i_lsb: DcAcc3iLsb,
    dc_acc3i_msb: DcAcc3iMsb,
    dc_acc4i_lsb: DcAcc4iLsb,
    dc_acc4i_msb: DcAcc4iMsb,
    dc_acc5i_lsb: DcAcc5iLsb,
    dc_acc5i_msb: DcAcc5iMsb,
    dc_acc6i_lsb: DcAcc6iLsb,
    dc_acc6i_msb: DcAcc6iMsb,
    dcest1q_sw: Dcest1qSw,
    dcest2q_sw: Dcest2qSw,
    dcest3q_sw: Dcest3qSw,
    dcest4q_sw: Dcest4qSw,
    dcest5q_sw: Dcest5qSw,
    dcest6q_sw: Dcest6qSw,
    dcest1q: Dcest1q,
    dcest2q: Dcest2q,
    dcest3q: Dcest3q,
    dcest4q: Dcest4q,
    dcest5q: Dcest5q,
    dcest6q: Dcest6q,
    dc_acc1q_lsb: DcAcc1qLsb,
    dc_acc1q_msb: DcAcc1qMsb,
    dc_acc2q_lsb: DcAcc2qLsb,
    dc_acc2q_msb: DcAcc2qMsb,
    dc_acc3q_lsb: DcAcc3qLsb,
    dc_acc3q_msb: DcAcc3qMsb,
    dc_acc4q_lsb: DcAcc4qLsb,
    dc_acc4q_msb: DcAcc4qMsb,
    dc_acc5q_lsb: DcAcc5qLsb,
    dc_acc5q_msb: DcAcc5qMsb,
    dc_acc6q_lsb: DcAcc6qLsb,
    dc_acc6q_msb: DcAcc6qMsb,
    dcacc1_clip: Dcacc1Clip,
    dcacc2_clip: Dcacc2Clip,
    dcacc3_clip: Dcacc3Clip,
    dcacc4_clip: Dcacc4Clip,
    dcacc5_clip: Dcacc5Clip,
    dcacc6_clip: Dcacc6Clip,
    dcest1_clip: Dcest1Clip,
    dcest2_clip: Dcest2Clip,
    dcest3_clip: Dcest3Clip,
    dcest4_clip: Dcest4Clip,
    dcest5_clip: Dcest5Clip,
    dcest6_clip: Dcest6Clip,
    dcsub1_clip: Dcsub1Clip,
    dcsub2_clip: Dcsub2Clip,
    dcsub3_clip: Dcsub3Clip,
    dcsub4_clip: Dcsub4Clip,
    dcsub5_clip: Dcsub5Clip,
    dcsub6_clip: Dcsub6Clip,
    dcest_shift: DcestShift,
    dcest_scale: DcestScale,
    intf_mag_scale: IntfMagScale,
    intf_mag_shift: IntfMagShift,
    intf_magdiff_scale: IntfMagdiffScale,
    intf_magdiff_shift: IntfMagdiffShift,
    intf_frame_zerocount: IntfFrameZerocount,
    intf_chirp_zerocount: IntfChirpZerocount,
    intf_magthresh1_sw: IntfMagthresh1Sw,
    intf_magthresh2_sw: IntfMagthresh2Sw,
    intf_magthresh3_sw: IntfMagthresh3Sw,
    intf_magthresh4_sw: IntfMagthresh4Sw,
    intf_magthresh5_sw: IntfMagthresh5Sw,
    intf_magthresh6_sw: IntfMagthresh6Sw,
    intf_magdiffthresh1_sw: IntfMagdiffthresh1Sw,
    intf_magdiffthresh2_sw: IntfMagdiffthresh2Sw,
    intf_magdiffthresh3_sw: IntfMagdiffthresh3Sw,
    intf_magdiffthresh4_sw: IntfMagdiffthresh4Sw,
    intf_magdiffthresh5_sw: IntfMagdiffthresh5Sw,
    intf_magdiffthresh6_sw: IntfMagdiffthresh6Sw,
    intf_magacc1_lsb: IntfMagacc1Lsb,
    intf_magacc1_msb: IntfMagacc1Msb,
    intf_magacc2_lsb: IntfMagacc2Lsb,
    intf_magacc2_msb: IntfMagacc2Msb,
    intf_magacc3_lsb: IntfMagacc3Lsb,
    intf_magacc3_msb: IntfMagacc3Msb,
    intf_magacc4_lsb: IntfMagacc4Lsb,
    intf_magacc4_msb: IntfMagacc4Msb,
    intf_magacc5_lsb: IntfMagacc5Lsb,
    intf_magacc5_msb: IntfMagacc5Msb,
    intf_magacc6_lsb: IntfMagacc6Lsb,
    intf_magacc6_msb: IntfMagacc6Msb,
    intf_magdiffacc1_lsb: IntfMagdiffacc1Lsb,
    intf_magdiffacc1_msb: IntfMagdiffacc1Msb,
    intf_magdiffacc2_lsb: IntfMagdiffacc2Lsb,
    intf_magdiffacc2_msb: IntfMagdiffacc2Msb,
    intf_magdiffacc3_lsb: IntfMagdiffacc3Lsb,
    intf_magdiffacc3_msb: IntfMagdiffacc3Msb,
    intf_magdiffacc4_lsb: IntfMagdiffacc4Lsb,
    intf_magdiffacc4_msb: IntfMagdiffacc4Msb,
    intf_magdiffacc5_lsb: IntfMagdiffacc5Lsb,
    intf_magdiffacc5_msb: IntfMagdiffacc5Msb,
    intf_magdiffacc6_lsb: IntfMagdiffacc6Lsb,
    intf_magdiffacc6_msb: IntfMagdiffacc6Msb,
    intf_magacc1_clip: IntfMagacc1Clip,
    intf_magacc2_clip: IntfMagacc2Clip,
    intf_magacc3_clip: IntfMagacc3Clip,
    intf_magacc4_clip: IntfMagacc4Clip,
    intf_magacc5_clip: IntfMagacc5Clip,
    intf_magacc6_clip: IntfMagacc6Clip,
    intf_magdiffacc1_clip: IntfMagdiffacc1Clip,
    intf_magdiffacc2_clip: IntfMagdiffacc2Clip,
    intf_magdiffacc3_clip: IntfMagdiffacc3Clip,
    intf_magdiffacc4_clip: IntfMagdiffacc4Clip,
    intf_magdiffacc5_clip: IntfMagdiffacc5Clip,
    intf_magdiffacc6_clip: IntfMagdiffacc6Clip,
    intf_magthresh1: IntfMagthresh1,
    intf_magthresh2: IntfMagthresh2,
    intf_magthresh3: IntfMagthresh3,
    intf_magthresh4: IntfMagthresh4,
    intf_magthresh5: IntfMagthresh5,
    intf_magthresh6: IntfMagthresh6,
    intf_magdiffthresh1: IntfMagdiffthresh1,
    intf_magdiffthresh2: IntfMagdiffthresh2,
    intf_magdiffthresh3: IntfMagdiffthresh3,
    intf_magdiffthresh4: IntfMagdiffthresh4,
    intf_magdiffthresh5: IntfMagdiffthresh5,
    intf_magdiffthresh6: IntfMagdiffthresh6,
    intf_summagthresh: IntfSummagthresh,
    intf_summagdiffthresh: IntfSummagdiffthresh,
    intf_summagthresh_clip: IntfSummagthreshClip,
    intf_summagdiffthresh_clip: IntfSummagdiffthreshClip,
    cmultscale1i: Cmultscale1i,
    cmultscale2i: Cmultscale2i,
    cmultscale3i: Cmultscale3i,
    cmultscale4i: Cmultscale4i,
    cmultscale5i: Cmultscale5i,
    cmultscale6i: Cmultscale6i,
    cmultscale1q: Cmultscale1q,
    cmultscale2q: Cmultscale2q,
    cmultscale3q: Cmultscale3q,
    cmultscale4q: Cmultscale4q,
    cmultscale5q: Cmultscale5q,
    cmultscale6q: Cmultscale6q,
    clr_misc_clip: ClrMiscClip,
    fftintmemaddr: Fftintmemaddr,
    intf_stats_reset_sw: IntfStatsResetSw,
    dcest_reset_sw: DcestResetSw,
    ip_op_formatter_clip_status: IpOpFormatterClipStatus,
    intf_magthresh1_clip: IntfMagthresh1Clip,
    intf_magthresh2_clip: IntfMagthresh2Clip,
    intf_magthresh3_clip: IntfMagthresh3Clip,
    intf_magthresh4_clip: IntfMagthresh4Clip,
    intf_magthresh5_clip: IntfMagthresh5Clip,
    intf_magthresh6_clip: IntfMagthresh6Clip,
    intf_magdiffthresh1_clip: IntfMagdiffthresh1Clip,
    intf_magdiffthresh2_clip: IntfMagdiffthresh2Clip,
    intf_magdiffthresh3_clip: IntfMagdiffthresh3Clip,
    intf_magdiffthresh4_clip: IntfMagdiffthresh4Clip,
    intf_magdiffthresh5_clip: IntfMagdiffthresh5Clip,
    intf_magdiffthresh6_clip: IntfMagdiffthresh6Clip,
    hwa_safety_err_mask: HwaSafetyErrMask,
    hwa_safety_err_status: HwaSafetyErrStatus,
    hwa_safety_err_status_raw: HwaSafetyErrStatusRaw,
}
impl RegisterBlock {
    #[doc = "0x00 - HWACCREG1"]
    #[inline(always)]
    pub const fn hwaccreg1(&self) -> &Hwaccreg1 {
        &self.hwaccreg1
    }
    #[doc = "0x04 - HWACCREG2"]
    #[inline(always)]
    pub const fn hwaccreg2(&self) -> &Hwaccreg2 {
        &self.hwaccreg2
    }
    #[doc = "0x08 - HWACCREG3"]
    #[inline(always)]
    pub const fn hwaccreg3(&self) -> &Hwaccreg3 {
        &self.hwaccreg3
    }
    #[doc = "0x0c - HWACCREG4"]
    #[inline(always)]
    pub const fn hwaccreg4(&self) -> &Hwaccreg4 {
        &self.hwaccreg4
    }
    #[doc = "0x10 - HWACCREG5"]
    #[inline(always)]
    pub const fn hwaccreg5(&self) -> &Hwaccreg5 {
        &self.hwaccreg5
    }
    #[doc = "0x14 - HWACCREG6"]
    #[inline(always)]
    pub const fn hwaccreg6(&self) -> &Hwaccreg6 {
        &self.hwaccreg6
    }
    #[doc = "0x18 - HWACCREG7"]
    #[inline(always)]
    pub const fn hwaccreg7(&self) -> &Hwaccreg7 {
        &self.hwaccreg7
    }
    #[doc = "0x1c - HWACCREG8"]
    #[inline(always)]
    pub const fn hwaccreg8(&self) -> &Hwaccreg8 {
        &self.hwaccreg8
    }
    #[doc = "0x20 - HWACCREG11"]
    #[inline(always)]
    pub const fn hwaccreg11(&self) -> &Hwaccreg11 {
        &self.hwaccreg11
    }
    #[doc = "0x24 - HWACCREG12"]
    #[inline(always)]
    pub const fn hwaccreg12(&self) -> &Hwaccreg12 {
        &self.hwaccreg12
    }
    #[doc = "0x28 - HWACCREG13"]
    #[inline(always)]
    pub const fn hwaccreg13(&self) -> &Hwaccreg13 {
        &self.hwaccreg13
    }
    #[doc = "0x2c - HWACCREG14"]
    #[inline(always)]
    pub const fn hwaccreg14(&self) -> &Hwaccreg14 {
        &self.hwaccreg14
    }
    #[doc = "0x30 - HWACCREG15"]
    #[inline(always)]
    pub const fn hwaccreg15(&self) -> &Hwaccreg15 {
        &self.hwaccreg15
    }
    #[doc = "0x34 - CFAR_DET_THR"]
    #[inline(always)]
    pub const fn cfar_det_thr(&self) -> &CfarDetThr {
        &self.cfar_det_thr
    }
    #[doc = "0x38 - MAX1VALUE"]
    #[inline(always)]
    pub const fn max1value(&self) -> &Max1value {
        &self.max1value
    }
    #[doc = "0x3c - MAX1INDEX"]
    #[inline(always)]
    pub const fn max1index(&self) -> &Max1index {
        &self.max1index
    }
    #[doc = "0x40 - ISUM1LSB"]
    #[inline(always)]
    pub const fn isum1lsb(&self) -> &Isum1lsb {
        &self.isum1lsb
    }
    #[doc = "0x44 - ISUM1MSB"]
    #[inline(always)]
    pub const fn isum1msb(&self) -> &Isum1msb {
        &self.isum1msb
    }
    #[doc = "0x48 - QSUM1LSB"]
    #[inline(always)]
    pub const fn qsum1lsb(&self) -> &Qsum1lsb {
        &self.qsum1lsb
    }
    #[doc = "0x4c - QSUM1MSB"]
    #[inline(always)]
    pub const fn qsum1msb(&self) -> &Qsum1msb {
        &self.qsum1msb
    }
    #[doc = "0x50 - MAX2VALUE"]
    #[inline(always)]
    pub const fn max2value(&self) -> &Max2value {
        &self.max2value
    }
    #[doc = "0x54 - MAX2INDEX"]
    #[inline(always)]
    pub const fn max2index(&self) -> &Max2index {
        &self.max2index
    }
    #[doc = "0x58 - ISUM2LSB"]
    #[inline(always)]
    pub const fn isum2lsb(&self) -> &Isum2lsb {
        &self.isum2lsb
    }
    #[doc = "0x5c - ISUM2MSB"]
    #[inline(always)]
    pub const fn isum2msb(&self) -> &Isum2msb {
        &self.isum2msb
    }
    #[doc = "0x60 - QSUM2LSB"]
    #[inline(always)]
    pub const fn qsum2lsb(&self) -> &Qsum2lsb {
        &self.qsum2lsb
    }
    #[doc = "0x64 - QSUM2MSB"]
    #[inline(always)]
    pub const fn qsum2msb(&self) -> &Qsum2msb {
        &self.qsum2msb
    }
    #[doc = "0x68 - MAX3VALUE"]
    #[inline(always)]
    pub const fn max3value(&self) -> &Max3value {
        &self.max3value
    }
    #[doc = "0x6c - MAX3INDEX"]
    #[inline(always)]
    pub const fn max3index(&self) -> &Max3index {
        &self.max3index
    }
    #[doc = "0x70 - ISUM3LSB"]
    #[inline(always)]
    pub const fn isum3lsb(&self) -> &Isum3lsb {
        &self.isum3lsb
    }
    #[doc = "0x74 - ISUM3MSB"]
    #[inline(always)]
    pub const fn isum3msb(&self) -> &Isum3msb {
        &self.isum3msb
    }
    #[doc = "0x78 - QSUM3LSB"]
    #[inline(always)]
    pub const fn qsum3lsb(&self) -> &Qsum3lsb {
        &self.qsum3lsb
    }
    #[doc = "0x7c - QSUM3MSB"]
    #[inline(always)]
    pub const fn qsum3msb(&self) -> &Qsum3msb {
        &self.qsum3msb
    }
    #[doc = "0x80 - MAX4VALUE"]
    #[inline(always)]
    pub const fn max4value(&self) -> &Max4value {
        &self.max4value
    }
    #[doc = "0x84 - MAX4INDEX"]
    #[inline(always)]
    pub const fn max4index(&self) -> &Max4index {
        &self.max4index
    }
    #[doc = "0x88 - ISUM4LSB"]
    #[inline(always)]
    pub const fn isum4lsb(&self) -> &Isum4lsb {
        &self.isum4lsb
    }
    #[doc = "0x8c - ISUM4MSB"]
    #[inline(always)]
    pub const fn isum4msb(&self) -> &Isum4msb {
        &self.isum4msb
    }
    #[doc = "0x90 - QSUM4LSB"]
    #[inline(always)]
    pub const fn qsum4lsb(&self) -> &Qsum4lsb {
        &self.qsum4lsb
    }
    #[doc = "0x94 - QSUM4MSB"]
    #[inline(always)]
    pub const fn qsum4msb(&self) -> &Qsum4msb {
        &self.qsum4msb
    }
    #[doc = "0x98 - CFARTEST"]
    #[inline(always)]
    pub const fn cfartest(&self) -> &Cfartest {
        &self.cfartest
    }
    #[doc = "0x9c - RDSTATUS"]
    #[inline(always)]
    pub const fn rdstatus(&self) -> &Rdstatus {
        &self.rdstatus
    }
    #[doc = "0xa0 - SIGDMACH1DONE"]
    #[inline(always)]
    pub const fn sigdmach1done(&self) -> &Sigdmach1done {
        &self.sigdmach1done
    }
    #[doc = "0xa4 - SIGDMACH2DONE"]
    #[inline(always)]
    pub const fn sigdmach2done(&self) -> &Sigdmach2done {
        &self.sigdmach2done
    }
    #[doc = "0xa8 - SIGDMACH3DONE"]
    #[inline(always)]
    pub const fn sigdmach3done(&self) -> &Sigdmach3done {
        &self.sigdmach3done
    }
    #[doc = "0xac - SIGDMACH4DONE"]
    #[inline(always)]
    pub const fn sigdmach4done(&self) -> &Sigdmach4done {
        &self.sigdmach4done
    }
    #[doc = "0xb0 - SIGDMACH5DONE"]
    #[inline(always)]
    pub const fn sigdmach5done(&self) -> &Sigdmach5done {
        &self.sigdmach5done
    }
    #[doc = "0xb4 - SIGDMACH6DONE"]
    #[inline(always)]
    pub const fn sigdmach6done(&self) -> &Sigdmach6done {
        &self.sigdmach6done
    }
    #[doc = "0xb8 - SIGDMACH7DONE"]
    #[inline(always)]
    pub const fn sigdmach7done(&self) -> &Sigdmach7done {
        &self.sigdmach7done
    }
    #[doc = "0xbc - SIGDMACH8DONE"]
    #[inline(always)]
    pub const fn sigdmach8done(&self) -> &Sigdmach8done {
        &self.sigdmach8done
    }
    #[doc = "0xc0 - SIGDMACH9DONE"]
    #[inline(always)]
    pub const fn sigdmach9done(&self) -> &Sigdmach9done {
        &self.sigdmach9done
    }
    #[doc = "0xc4 - SIGDMACH10DONE"]
    #[inline(always)]
    pub const fn sigdmach10done(&self) -> &Sigdmach10done {
        &self.sigdmach10done
    }
    #[doc = "0xc8 - SIGDMACH11DONE"]
    #[inline(always)]
    pub const fn sigdmach11done(&self) -> &Sigdmach11done {
        &self.sigdmach11done
    }
    #[doc = "0xcc - SIGDMACH12DONE"]
    #[inline(always)]
    pub const fn sigdmach12done(&self) -> &Sigdmach12done {
        &self.sigdmach12done
    }
    #[doc = "0xd0 - SIGDMACH13DONE"]
    #[inline(always)]
    pub const fn sigdmach13done(&self) -> &Sigdmach13done {
        &self.sigdmach13done
    }
    #[doc = "0xd4 - SIGDMACH14DONE"]
    #[inline(always)]
    pub const fn sigdmach14done(&self) -> &Sigdmach14done {
        &self.sigdmach14done
    }
    #[doc = "0xd8 - SIGDMACH15DONE"]
    #[inline(always)]
    pub const fn sigdmach15done(&self) -> &Sigdmach15done {
        &self.sigdmach15done
    }
    #[doc = "0xdc - SIGDMACH16DONE"]
    #[inline(always)]
    pub const fn sigdmach16done(&self) -> &Sigdmach16done {
        &self.sigdmach16done
    }
    #[doc = "0xe0 - MEMACCESSERR"]
    #[inline(always)]
    pub const fn memaccesserr(&self) -> &Memaccesserr {
        &self.memaccesserr
    }
    #[doc = "0xe4 - FFTCLIP"]
    #[inline(always)]
    pub const fn fftclip(&self) -> &Fftclip {
        &self.fftclip
    }
    #[doc = "0xe8 - FFTPEAKCNT"]
    #[inline(always)]
    pub const fn fftpeakcnt(&self) -> &Fftpeakcnt {
        &self.fftpeakcnt
    }
    #[doc = "0xec - HWACCREG1RD"]
    #[inline(always)]
    pub const fn hwaccreg1rd(&self) -> &Hwaccreg1rd {
        &self.hwaccreg1rd
    }
    #[doc = "0xf0 - HWACCREG2RD"]
    #[inline(always)]
    pub const fn hwaccreg2rd(&self) -> &Hwaccreg2rd {
        &self.hwaccreg2rd
    }
    #[doc = "0xf4 - HWACCREG3RD"]
    #[inline(always)]
    pub const fn hwaccreg3rd(&self) -> &Hwaccreg3rd {
        &self.hwaccreg3rd
    }
    #[doc = "0xf8 - CMP_EGE_K0123"]
    #[inline(always)]
    pub const fn cmp_ege_k0123(&self) -> &CmpEgeK0123 {
        &self.cmp_ege_k0123
    }
    #[doc = "0xfc - CMP_EGE_K4567"]
    #[inline(always)]
    pub const fn cmp_ege_k4567(&self) -> &CmpEgeK4567 {
        &self.cmp_ege_k4567
    }
    #[doc = "0x100 - HWA_SAFETY_ENABLE"]
    #[inline(always)]
    pub const fn hwa_safety_enable(&self) -> &HwaSafetyEnable {
        &self.hwa_safety_enable
    }
    #[doc = "0x104 - MEMINIT"]
    #[inline(always)]
    pub const fn meminit(&self) -> &Meminit {
        &self.meminit
    }
    #[doc = "0x108 - MEMINITDONE"]
    #[inline(always)]
    pub const fn meminitdone(&self) -> &Meminitdone {
        &self.meminitdone
    }
    #[doc = "0x10c - HWA_SAFETY_WIN_RAM_ERR_LOC"]
    #[inline(always)]
    pub const fn hwa_safety_win_ram_err_loc(&self) -> &HwaSafetyWinRamErrLoc {
        &self.hwa_safety_win_ram_err_loc
    }
    #[doc = "0x110 - HWA_SAFETY_PARAM_RAM_ERR_LOC"]
    #[inline(always)]
    pub const fn hwa_safety_param_ram_err_loc(&self) -> &HwaSafetyParamRamErrLoc {
        &self.hwa_safety_param_ram_err_loc
    }
    #[doc = "0x114 - HWA_SAFETY_IPING_ERR_LOC"]
    #[inline(always)]
    pub const fn hwa_safety_iping_err_loc(&self) -> &HwaSafetyIpingErrLoc {
        &self.hwa_safety_iping_err_loc
    }
    #[doc = "0x118 - HWA_SAFETY_IPONG_ERR_LOC"]
    #[inline(always)]
    pub const fn hwa_safety_ipong_err_loc(&self) -> &HwaSafetyIpongErrLoc {
        &self.hwa_safety_ipong_err_loc
    }
    #[doc = "0x11c - HWA_SAFETY_OPING_ERR_LOC"]
    #[inline(always)]
    pub const fn hwa_safety_oping_err_loc(&self) -> &HwaSafetyOpingErrLoc {
        &self.hwa_safety_oping_err_loc
    }
    #[doc = "0x120 - HWA_SAFETY_OPONG_ERR_LOC"]
    #[inline(always)]
    pub const fn hwa_safety_opong_err_loc(&self) -> &HwaSafetyOpongErrLoc {
        &self.hwa_safety_opong_err_loc
    }
    #[doc = "0x124 - FFTINTMEMWRDATA"]
    #[inline(always)]
    pub const fn fftintmemwrdata(&self) -> &Fftintmemwrdata {
        &self.fftintmemwrdata
    }
    #[doc = "0x128 - FFTINTMEMRDDATA"]
    #[inline(always)]
    pub const fn fftintmemrddata(&self) -> &Fftintmemrddata {
        &self.fftintmemrddata
    }
    #[doc = "0x12c - HWACCREG16"]
    #[inline(always)]
    pub const fn hwaccreg16(&self) -> &Hwaccreg16 {
        &self.hwaccreg16
    }
    #[doc = "0x130 - DCEST1I_SW"]
    #[inline(always)]
    pub const fn dcest1i_sw(&self) -> &Dcest1iSw {
        &self.dcest1i_sw
    }
    #[doc = "0x134 - DCEST2I_SW"]
    #[inline(always)]
    pub const fn dcest2i_sw(&self) -> &Dcest2iSw {
        &self.dcest2i_sw
    }
    #[doc = "0x138 - DCEST3I_SW"]
    #[inline(always)]
    pub const fn dcest3i_sw(&self) -> &Dcest3iSw {
        &self.dcest3i_sw
    }
    #[doc = "0x13c - DCEST4I_SW"]
    #[inline(always)]
    pub const fn dcest4i_sw(&self) -> &Dcest4iSw {
        &self.dcest4i_sw
    }
    #[doc = "0x140 - DCEST5I_SW"]
    #[inline(always)]
    pub const fn dcest5i_sw(&self) -> &Dcest5iSw {
        &self.dcest5i_sw
    }
    #[doc = "0x144 - DCEST6I_SW"]
    #[inline(always)]
    pub const fn dcest6i_sw(&self) -> &Dcest6iSw {
        &self.dcest6i_sw
    }
    #[doc = "0x148 - DCEST1I"]
    #[inline(always)]
    pub const fn dcest1i(&self) -> &Dcest1i {
        &self.dcest1i
    }
    #[doc = "0x14c - DCEST2I"]
    #[inline(always)]
    pub const fn dcest2i(&self) -> &Dcest2i {
        &self.dcest2i
    }
    #[doc = "0x150 - DCEST3I"]
    #[inline(always)]
    pub const fn dcest3i(&self) -> &Dcest3i {
        &self.dcest3i
    }
    #[doc = "0x154 - DCEST4I"]
    #[inline(always)]
    pub const fn dcest4i(&self) -> &Dcest4i {
        &self.dcest4i
    }
    #[doc = "0x158 - DCEST5I"]
    #[inline(always)]
    pub const fn dcest5i(&self) -> &Dcest5i {
        &self.dcest5i
    }
    #[doc = "0x15c - DCEST6I"]
    #[inline(always)]
    pub const fn dcest6i(&self) -> &Dcest6i {
        &self.dcest6i
    }
    #[doc = "0x160 - DC_ACC1I_LSB"]
    #[inline(always)]
    pub const fn dc_acc1i_lsb(&self) -> &DcAcc1iLsb {
        &self.dc_acc1i_lsb
    }
    #[doc = "0x164 - DC_ACC1I_MSB"]
    #[inline(always)]
    pub const fn dc_acc1i_msb(&self) -> &DcAcc1iMsb {
        &self.dc_acc1i_msb
    }
    #[doc = "0x168 - DC_ACC2I_LSB"]
    #[inline(always)]
    pub const fn dc_acc2i_lsb(&self) -> &DcAcc2iLsb {
        &self.dc_acc2i_lsb
    }
    #[doc = "0x16c - DC_ACC2I_MSB"]
    #[inline(always)]
    pub const fn dc_acc2i_msb(&self) -> &DcAcc2iMsb {
        &self.dc_acc2i_msb
    }
    #[doc = "0x170 - DC_ACC3I_LSB"]
    #[inline(always)]
    pub const fn dc_acc3i_lsb(&self) -> &DcAcc3iLsb {
        &self.dc_acc3i_lsb
    }
    #[doc = "0x174 - DC_ACC3I_MSB"]
    #[inline(always)]
    pub const fn dc_acc3i_msb(&self) -> &DcAcc3iMsb {
        &self.dc_acc3i_msb
    }
    #[doc = "0x178 - DC_ACC4I_LSB"]
    #[inline(always)]
    pub const fn dc_acc4i_lsb(&self) -> &DcAcc4iLsb {
        &self.dc_acc4i_lsb
    }
    #[doc = "0x17c - DC_ACC4I_MSB"]
    #[inline(always)]
    pub const fn dc_acc4i_msb(&self) -> &DcAcc4iMsb {
        &self.dc_acc4i_msb
    }
    #[doc = "0x180 - DC_ACC5I_LSB"]
    #[inline(always)]
    pub const fn dc_acc5i_lsb(&self) -> &DcAcc5iLsb {
        &self.dc_acc5i_lsb
    }
    #[doc = "0x184 - DC_ACC5I_MSB"]
    #[inline(always)]
    pub const fn dc_acc5i_msb(&self) -> &DcAcc5iMsb {
        &self.dc_acc5i_msb
    }
    #[doc = "0x188 - DC_ACC6I_LSB"]
    #[inline(always)]
    pub const fn dc_acc6i_lsb(&self) -> &DcAcc6iLsb {
        &self.dc_acc6i_lsb
    }
    #[doc = "0x18c - DC_ACC6I_MSB"]
    #[inline(always)]
    pub const fn dc_acc6i_msb(&self) -> &DcAcc6iMsb {
        &self.dc_acc6i_msb
    }
    #[doc = "0x190 - DCEST1Q_SW"]
    #[inline(always)]
    pub const fn dcest1q_sw(&self) -> &Dcest1qSw {
        &self.dcest1q_sw
    }
    #[doc = "0x194 - DCEST2Q_SW"]
    #[inline(always)]
    pub const fn dcest2q_sw(&self) -> &Dcest2qSw {
        &self.dcest2q_sw
    }
    #[doc = "0x198 - DCEST3Q_SW"]
    #[inline(always)]
    pub const fn dcest3q_sw(&self) -> &Dcest3qSw {
        &self.dcest3q_sw
    }
    #[doc = "0x19c - DCEST4Q_SW"]
    #[inline(always)]
    pub const fn dcest4q_sw(&self) -> &Dcest4qSw {
        &self.dcest4q_sw
    }
    #[doc = "0x1a0 - DCEST5Q_SW"]
    #[inline(always)]
    pub const fn dcest5q_sw(&self) -> &Dcest5qSw {
        &self.dcest5q_sw
    }
    #[doc = "0x1a4 - DCEST6Q_SW"]
    #[inline(always)]
    pub const fn dcest6q_sw(&self) -> &Dcest6qSw {
        &self.dcest6q_sw
    }
    #[doc = "0x1a8 - DCEST1Q"]
    #[inline(always)]
    pub const fn dcest1q(&self) -> &Dcest1q {
        &self.dcest1q
    }
    #[doc = "0x1ac - DCEST2Q"]
    #[inline(always)]
    pub const fn dcest2q(&self) -> &Dcest2q {
        &self.dcest2q
    }
    #[doc = "0x1b0 - DCEST3Q"]
    #[inline(always)]
    pub const fn dcest3q(&self) -> &Dcest3q {
        &self.dcest3q
    }
    #[doc = "0x1b4 - DCEST4Q"]
    #[inline(always)]
    pub const fn dcest4q(&self) -> &Dcest4q {
        &self.dcest4q
    }
    #[doc = "0x1b8 - DCEST5Q"]
    #[inline(always)]
    pub const fn dcest5q(&self) -> &Dcest5q {
        &self.dcest5q
    }
    #[doc = "0x1bc - DCEST6Q"]
    #[inline(always)]
    pub const fn dcest6q(&self) -> &Dcest6q {
        &self.dcest6q
    }
    #[doc = "0x1c0 - DC_ACC1Q_LSB"]
    #[inline(always)]
    pub const fn dc_acc1q_lsb(&self) -> &DcAcc1qLsb {
        &self.dc_acc1q_lsb
    }
    #[doc = "0x1c4 - DC_ACC1Q_MSB"]
    #[inline(always)]
    pub const fn dc_acc1q_msb(&self) -> &DcAcc1qMsb {
        &self.dc_acc1q_msb
    }
    #[doc = "0x1c8 - DC_ACC2Q_LSB"]
    #[inline(always)]
    pub const fn dc_acc2q_lsb(&self) -> &DcAcc2qLsb {
        &self.dc_acc2q_lsb
    }
    #[doc = "0x1cc - DC_ACC2Q_MSB"]
    #[inline(always)]
    pub const fn dc_acc2q_msb(&self) -> &DcAcc2qMsb {
        &self.dc_acc2q_msb
    }
    #[doc = "0x1d0 - DC_ACC3Q_LSB"]
    #[inline(always)]
    pub const fn dc_acc3q_lsb(&self) -> &DcAcc3qLsb {
        &self.dc_acc3q_lsb
    }
    #[doc = "0x1d4 - DC_ACC3Q_MSB"]
    #[inline(always)]
    pub const fn dc_acc3q_msb(&self) -> &DcAcc3qMsb {
        &self.dc_acc3q_msb
    }
    #[doc = "0x1d8 - DC_ACC4Q_LSB"]
    #[inline(always)]
    pub const fn dc_acc4q_lsb(&self) -> &DcAcc4qLsb {
        &self.dc_acc4q_lsb
    }
    #[doc = "0x1dc - DC_ACC4Q_MSB"]
    #[inline(always)]
    pub const fn dc_acc4q_msb(&self) -> &DcAcc4qMsb {
        &self.dc_acc4q_msb
    }
    #[doc = "0x1e0 - DC_ACC5Q_LSB"]
    #[inline(always)]
    pub const fn dc_acc5q_lsb(&self) -> &DcAcc5qLsb {
        &self.dc_acc5q_lsb
    }
    #[doc = "0x1e4 - DC_ACC5Q_MSB"]
    #[inline(always)]
    pub const fn dc_acc5q_msb(&self) -> &DcAcc5qMsb {
        &self.dc_acc5q_msb
    }
    #[doc = "0x1e8 - DC_ACC6Q_LSB"]
    #[inline(always)]
    pub const fn dc_acc6q_lsb(&self) -> &DcAcc6qLsb {
        &self.dc_acc6q_lsb
    }
    #[doc = "0x1ec - DC_ACC6Q_MSB"]
    #[inline(always)]
    pub const fn dc_acc6q_msb(&self) -> &DcAcc6qMsb {
        &self.dc_acc6q_msb
    }
    #[doc = "0x1f0 - DCACC1_CLIP"]
    #[inline(always)]
    pub const fn dcacc1_clip(&self) -> &Dcacc1Clip {
        &self.dcacc1_clip
    }
    #[doc = "0x1f4 - DCACC2_CLIP"]
    #[inline(always)]
    pub const fn dcacc2_clip(&self) -> &Dcacc2Clip {
        &self.dcacc2_clip
    }
    #[doc = "0x1f8 - DCACC3_CLIP"]
    #[inline(always)]
    pub const fn dcacc3_clip(&self) -> &Dcacc3Clip {
        &self.dcacc3_clip
    }
    #[doc = "0x1fc - DCACC4_CLIP"]
    #[inline(always)]
    pub const fn dcacc4_clip(&self) -> &Dcacc4Clip {
        &self.dcacc4_clip
    }
    #[doc = "0x200 - DCACC5_CLIP"]
    #[inline(always)]
    pub const fn dcacc5_clip(&self) -> &Dcacc5Clip {
        &self.dcacc5_clip
    }
    #[doc = "0x204 - DCACC6_CLIP"]
    #[inline(always)]
    pub const fn dcacc6_clip(&self) -> &Dcacc6Clip {
        &self.dcacc6_clip
    }
    #[doc = "0x208 - DCEST1_CLIP"]
    #[inline(always)]
    pub const fn dcest1_clip(&self) -> &Dcest1Clip {
        &self.dcest1_clip
    }
    #[doc = "0x20c - DCEST2_CLIP"]
    #[inline(always)]
    pub const fn dcest2_clip(&self) -> &Dcest2Clip {
        &self.dcest2_clip
    }
    #[doc = "0x210 - DCEST3_CLIP"]
    #[inline(always)]
    pub const fn dcest3_clip(&self) -> &Dcest3Clip {
        &self.dcest3_clip
    }
    #[doc = "0x214 - DCEST4_CLIP"]
    #[inline(always)]
    pub const fn dcest4_clip(&self) -> &Dcest4Clip {
        &self.dcest4_clip
    }
    #[doc = "0x218 - DCEST5_CLIP"]
    #[inline(always)]
    pub const fn dcest5_clip(&self) -> &Dcest5Clip {
        &self.dcest5_clip
    }
    #[doc = "0x21c - DCEST6_CLIP"]
    #[inline(always)]
    pub const fn dcest6_clip(&self) -> &Dcest6Clip {
        &self.dcest6_clip
    }
    #[doc = "0x220 - DCSUB1_CLIP"]
    #[inline(always)]
    pub const fn dcsub1_clip(&self) -> &Dcsub1Clip {
        &self.dcsub1_clip
    }
    #[doc = "0x224 - DCSUB2_CLIP"]
    #[inline(always)]
    pub const fn dcsub2_clip(&self) -> &Dcsub2Clip {
        &self.dcsub2_clip
    }
    #[doc = "0x228 - DCSUB3_CLIP"]
    #[inline(always)]
    pub const fn dcsub3_clip(&self) -> &Dcsub3Clip {
        &self.dcsub3_clip
    }
    #[doc = "0x22c - DCSUB4_CLIP"]
    #[inline(always)]
    pub const fn dcsub4_clip(&self) -> &Dcsub4Clip {
        &self.dcsub4_clip
    }
    #[doc = "0x230 - DCSUB5_CLIP"]
    #[inline(always)]
    pub const fn dcsub5_clip(&self) -> &Dcsub5Clip {
        &self.dcsub5_clip
    }
    #[doc = "0x234 - DCSUB6_CLIP"]
    #[inline(always)]
    pub const fn dcsub6_clip(&self) -> &Dcsub6Clip {
        &self.dcsub6_clip
    }
    #[doc = "0x238 - DCEST_SHIFT"]
    #[inline(always)]
    pub const fn dcest_shift(&self) -> &DcestShift {
        &self.dcest_shift
    }
    #[doc = "0x23c - DCEST_SCALE"]
    #[inline(always)]
    pub const fn dcest_scale(&self) -> &DcestScale {
        &self.dcest_scale
    }
    #[doc = "0x240 - INTF_MAG_SCALE"]
    #[inline(always)]
    pub const fn intf_mag_scale(&self) -> &IntfMagScale {
        &self.intf_mag_scale
    }
    #[doc = "0x244 - INTF_MAG_SHIFT"]
    #[inline(always)]
    pub const fn intf_mag_shift(&self) -> &IntfMagShift {
        &self.intf_mag_shift
    }
    #[doc = "0x248 - INTF_MAGDIFF_SCALE"]
    #[inline(always)]
    pub const fn intf_magdiff_scale(&self) -> &IntfMagdiffScale {
        &self.intf_magdiff_scale
    }
    #[doc = "0x24c - INTF_MAGDIFF_SHIFT"]
    #[inline(always)]
    pub const fn intf_magdiff_shift(&self) -> &IntfMagdiffShift {
        &self.intf_magdiff_shift
    }
    #[doc = "0x250 - INTF_FRAME_ZEROCOUNT"]
    #[inline(always)]
    pub const fn intf_frame_zerocount(&self) -> &IntfFrameZerocount {
        &self.intf_frame_zerocount
    }
    #[doc = "0x254 - INTF_CHIRP_ZEROCOUNT"]
    #[inline(always)]
    pub const fn intf_chirp_zerocount(&self) -> &IntfChirpZerocount {
        &self.intf_chirp_zerocount
    }
    #[doc = "0x258 - INTF_MAGTHRESH1_SW"]
    #[inline(always)]
    pub const fn intf_magthresh1_sw(&self) -> &IntfMagthresh1Sw {
        &self.intf_magthresh1_sw
    }
    #[doc = "0x25c - INTF_MAGTHRESH2_SW"]
    #[inline(always)]
    pub const fn intf_magthresh2_sw(&self) -> &IntfMagthresh2Sw {
        &self.intf_magthresh2_sw
    }
    #[doc = "0x260 - INTF_MAGTHRESH3_SW"]
    #[inline(always)]
    pub const fn intf_magthresh3_sw(&self) -> &IntfMagthresh3Sw {
        &self.intf_magthresh3_sw
    }
    #[doc = "0x264 - INTF_MAGTHRESH4_SW"]
    #[inline(always)]
    pub const fn intf_magthresh4_sw(&self) -> &IntfMagthresh4Sw {
        &self.intf_magthresh4_sw
    }
    #[doc = "0x268 - INTF_MAGTHRESH5_SW"]
    #[inline(always)]
    pub const fn intf_magthresh5_sw(&self) -> &IntfMagthresh5Sw {
        &self.intf_magthresh5_sw
    }
    #[doc = "0x26c - INTF_MAGTHRESH6_SW"]
    #[inline(always)]
    pub const fn intf_magthresh6_sw(&self) -> &IntfMagthresh6Sw {
        &self.intf_magthresh6_sw
    }
    #[doc = "0x270 - INTF_MAGDIFFTHRESH1_SW"]
    #[inline(always)]
    pub const fn intf_magdiffthresh1_sw(&self) -> &IntfMagdiffthresh1Sw {
        &self.intf_magdiffthresh1_sw
    }
    #[doc = "0x274 - INTF_MAGDIFFTHRESH2_SW"]
    #[inline(always)]
    pub const fn intf_magdiffthresh2_sw(&self) -> &IntfMagdiffthresh2Sw {
        &self.intf_magdiffthresh2_sw
    }
    #[doc = "0x278 - INTF_MAGDIFFTHRESH3_SW"]
    #[inline(always)]
    pub const fn intf_magdiffthresh3_sw(&self) -> &IntfMagdiffthresh3Sw {
        &self.intf_magdiffthresh3_sw
    }
    #[doc = "0x27c - INTF_MAGDIFFTHRESH4_SW"]
    #[inline(always)]
    pub const fn intf_magdiffthresh4_sw(&self) -> &IntfMagdiffthresh4Sw {
        &self.intf_magdiffthresh4_sw
    }
    #[doc = "0x280 - INTF_MAGDIFFTHRESH5_SW"]
    #[inline(always)]
    pub const fn intf_magdiffthresh5_sw(&self) -> &IntfMagdiffthresh5Sw {
        &self.intf_magdiffthresh5_sw
    }
    #[doc = "0x284 - INTF_MAGDIFFTHRESH6_SW"]
    #[inline(always)]
    pub const fn intf_magdiffthresh6_sw(&self) -> &IntfMagdiffthresh6Sw {
        &self.intf_magdiffthresh6_sw
    }
    #[doc = "0x288 - INTF_MAGACC1_LSB"]
    #[inline(always)]
    pub const fn intf_magacc1_lsb(&self) -> &IntfMagacc1Lsb {
        &self.intf_magacc1_lsb
    }
    #[doc = "0x28c - INTF_MAGACC1_MSB"]
    #[inline(always)]
    pub const fn intf_magacc1_msb(&self) -> &IntfMagacc1Msb {
        &self.intf_magacc1_msb
    }
    #[doc = "0x290 - INTF_MAGACC2_LSB"]
    #[inline(always)]
    pub const fn intf_magacc2_lsb(&self) -> &IntfMagacc2Lsb {
        &self.intf_magacc2_lsb
    }
    #[doc = "0x294 - INTF_MAGACC2_MSB"]
    #[inline(always)]
    pub const fn intf_magacc2_msb(&self) -> &IntfMagacc2Msb {
        &self.intf_magacc2_msb
    }
    #[doc = "0x298 - INTF_MAGACC3_LSB"]
    #[inline(always)]
    pub const fn intf_magacc3_lsb(&self) -> &IntfMagacc3Lsb {
        &self.intf_magacc3_lsb
    }
    #[doc = "0x29c - INTF_MAGACC3_MSB"]
    #[inline(always)]
    pub const fn intf_magacc3_msb(&self) -> &IntfMagacc3Msb {
        &self.intf_magacc3_msb
    }
    #[doc = "0x2a0 - INTF_MAGACC4_LSB"]
    #[inline(always)]
    pub const fn intf_magacc4_lsb(&self) -> &IntfMagacc4Lsb {
        &self.intf_magacc4_lsb
    }
    #[doc = "0x2a4 - INTF_MAGACC4_MSB"]
    #[inline(always)]
    pub const fn intf_magacc4_msb(&self) -> &IntfMagacc4Msb {
        &self.intf_magacc4_msb
    }
    #[doc = "0x2a8 - INTF_MAGACC5_LSB"]
    #[inline(always)]
    pub const fn intf_magacc5_lsb(&self) -> &IntfMagacc5Lsb {
        &self.intf_magacc5_lsb
    }
    #[doc = "0x2ac - INTF_MAGACC5_MSB"]
    #[inline(always)]
    pub const fn intf_magacc5_msb(&self) -> &IntfMagacc5Msb {
        &self.intf_magacc5_msb
    }
    #[doc = "0x2b0 - INTF_MAGACC6_LSB"]
    #[inline(always)]
    pub const fn intf_magacc6_lsb(&self) -> &IntfMagacc6Lsb {
        &self.intf_magacc6_lsb
    }
    #[doc = "0x2b4 - INTF_MAGACC6_MSB"]
    #[inline(always)]
    pub const fn intf_magacc6_msb(&self) -> &IntfMagacc6Msb {
        &self.intf_magacc6_msb
    }
    #[doc = "0x2b8 - INTF_MAGDIFFACC1_LSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc1_lsb(&self) -> &IntfMagdiffacc1Lsb {
        &self.intf_magdiffacc1_lsb
    }
    #[doc = "0x2bc - INTF_MAGDIFFACC1_MSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc1_msb(&self) -> &IntfMagdiffacc1Msb {
        &self.intf_magdiffacc1_msb
    }
    #[doc = "0x2c0 - INTF_MAGDIFFACC2_LSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc2_lsb(&self) -> &IntfMagdiffacc2Lsb {
        &self.intf_magdiffacc2_lsb
    }
    #[doc = "0x2c4 - INTF_MAGDIFFACC2_MSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc2_msb(&self) -> &IntfMagdiffacc2Msb {
        &self.intf_magdiffacc2_msb
    }
    #[doc = "0x2c8 - INTF_MAGDIFFACC3_LSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc3_lsb(&self) -> &IntfMagdiffacc3Lsb {
        &self.intf_magdiffacc3_lsb
    }
    #[doc = "0x2cc - INTF_MAGDIFFACC3_MSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc3_msb(&self) -> &IntfMagdiffacc3Msb {
        &self.intf_magdiffacc3_msb
    }
    #[doc = "0x2d0 - INTF_MAGDIFFACC4_LSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc4_lsb(&self) -> &IntfMagdiffacc4Lsb {
        &self.intf_magdiffacc4_lsb
    }
    #[doc = "0x2d4 - INTF_MAGDIFFACC4_MSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc4_msb(&self) -> &IntfMagdiffacc4Msb {
        &self.intf_magdiffacc4_msb
    }
    #[doc = "0x2d8 - INTF_MAGDIFFACC5_LSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc5_lsb(&self) -> &IntfMagdiffacc5Lsb {
        &self.intf_magdiffacc5_lsb
    }
    #[doc = "0x2dc - INTF_MAGDIFFACC5_MSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc5_msb(&self) -> &IntfMagdiffacc5Msb {
        &self.intf_magdiffacc5_msb
    }
    #[doc = "0x2e0 - INTF_MAGDIFFACC6_LSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc6_lsb(&self) -> &IntfMagdiffacc6Lsb {
        &self.intf_magdiffacc6_lsb
    }
    #[doc = "0x2e4 - INTF_MAGDIFFACC6_MSB"]
    #[inline(always)]
    pub const fn intf_magdiffacc6_msb(&self) -> &IntfMagdiffacc6Msb {
        &self.intf_magdiffacc6_msb
    }
    #[doc = "0x2e8 - INTF_MAGACC1_CLIP"]
    #[inline(always)]
    pub const fn intf_magacc1_clip(&self) -> &IntfMagacc1Clip {
        &self.intf_magacc1_clip
    }
    #[doc = "0x2ec - INTF_MAGACC2_CLIP"]
    #[inline(always)]
    pub const fn intf_magacc2_clip(&self) -> &IntfMagacc2Clip {
        &self.intf_magacc2_clip
    }
    #[doc = "0x2f0 - INTF_MAGACC3_CLIP"]
    #[inline(always)]
    pub const fn intf_magacc3_clip(&self) -> &IntfMagacc3Clip {
        &self.intf_magacc3_clip
    }
    #[doc = "0x2f4 - INTF_MAGACC4_CLIP"]
    #[inline(always)]
    pub const fn intf_magacc4_clip(&self) -> &IntfMagacc4Clip {
        &self.intf_magacc4_clip
    }
    #[doc = "0x2f8 - INTF_MAGACC5_CLIP"]
    #[inline(always)]
    pub const fn intf_magacc5_clip(&self) -> &IntfMagacc5Clip {
        &self.intf_magacc5_clip
    }
    #[doc = "0x2fc - INTF_MAGACC6_CLIP"]
    #[inline(always)]
    pub const fn intf_magacc6_clip(&self) -> &IntfMagacc6Clip {
        &self.intf_magacc6_clip
    }
    #[doc = "0x300 - INTF_MAGDIFFACC1_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffacc1_clip(&self) -> &IntfMagdiffacc1Clip {
        &self.intf_magdiffacc1_clip
    }
    #[doc = "0x304 - INTF_MAGDIFFACC2_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffacc2_clip(&self) -> &IntfMagdiffacc2Clip {
        &self.intf_magdiffacc2_clip
    }
    #[doc = "0x308 - INTF_MAGDIFFACC3_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffacc3_clip(&self) -> &IntfMagdiffacc3Clip {
        &self.intf_magdiffacc3_clip
    }
    #[doc = "0x30c - INTF_MAGDIFFACC4_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffacc4_clip(&self) -> &IntfMagdiffacc4Clip {
        &self.intf_magdiffacc4_clip
    }
    #[doc = "0x310 - INTF_MAGDIFFACC5_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffacc5_clip(&self) -> &IntfMagdiffacc5Clip {
        &self.intf_magdiffacc5_clip
    }
    #[doc = "0x314 - INTF_MAGDIFFACC6_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffacc6_clip(&self) -> &IntfMagdiffacc6Clip {
        &self.intf_magdiffacc6_clip
    }
    #[doc = "0x318 - INTF_MAGTHRESH1"]
    #[inline(always)]
    pub const fn intf_magthresh1(&self) -> &IntfMagthresh1 {
        &self.intf_magthresh1
    }
    #[doc = "0x31c - INTF_MAGTHRESH2"]
    #[inline(always)]
    pub const fn intf_magthresh2(&self) -> &IntfMagthresh2 {
        &self.intf_magthresh2
    }
    #[doc = "0x320 - INTF_MAGTHRESH3"]
    #[inline(always)]
    pub const fn intf_magthresh3(&self) -> &IntfMagthresh3 {
        &self.intf_magthresh3
    }
    #[doc = "0x324 - INTF_MAGTHRESH4"]
    #[inline(always)]
    pub const fn intf_magthresh4(&self) -> &IntfMagthresh4 {
        &self.intf_magthresh4
    }
    #[doc = "0x328 - INTF_MAGTHRESH5"]
    #[inline(always)]
    pub const fn intf_magthresh5(&self) -> &IntfMagthresh5 {
        &self.intf_magthresh5
    }
    #[doc = "0x32c - INTF_MAGTHRESH6"]
    #[inline(always)]
    pub const fn intf_magthresh6(&self) -> &IntfMagthresh6 {
        &self.intf_magthresh6
    }
    #[doc = "0x330 - INTF_MAGDIFFTHRESH1"]
    #[inline(always)]
    pub const fn intf_magdiffthresh1(&self) -> &IntfMagdiffthresh1 {
        &self.intf_magdiffthresh1
    }
    #[doc = "0x334 - INTF_MAGDIFFTHRESH2"]
    #[inline(always)]
    pub const fn intf_magdiffthresh2(&self) -> &IntfMagdiffthresh2 {
        &self.intf_magdiffthresh2
    }
    #[doc = "0x338 - INTF_MAGDIFFTHRESH3"]
    #[inline(always)]
    pub const fn intf_magdiffthresh3(&self) -> &IntfMagdiffthresh3 {
        &self.intf_magdiffthresh3
    }
    #[doc = "0x33c - INTF_MAGDIFFTHRESH4"]
    #[inline(always)]
    pub const fn intf_magdiffthresh4(&self) -> &IntfMagdiffthresh4 {
        &self.intf_magdiffthresh4
    }
    #[doc = "0x340 - INTF_MAGDIFFTHRESH5"]
    #[inline(always)]
    pub const fn intf_magdiffthresh5(&self) -> &IntfMagdiffthresh5 {
        &self.intf_magdiffthresh5
    }
    #[doc = "0x344 - INTF_MAGDIFFTHRESH6"]
    #[inline(always)]
    pub const fn intf_magdiffthresh6(&self) -> &IntfMagdiffthresh6 {
        &self.intf_magdiffthresh6
    }
    #[doc = "0x348 - INTF_SUMMAGTHRESH"]
    #[inline(always)]
    pub const fn intf_summagthresh(&self) -> &IntfSummagthresh {
        &self.intf_summagthresh
    }
    #[doc = "0x34c - INTF_SUMMAGDIFFTHRESH"]
    #[inline(always)]
    pub const fn intf_summagdiffthresh(&self) -> &IntfSummagdiffthresh {
        &self.intf_summagdiffthresh
    }
    #[doc = "0x350 - INTF_SUMMAGTHRESH_CLIP"]
    #[inline(always)]
    pub const fn intf_summagthresh_clip(&self) -> &IntfSummagthreshClip {
        &self.intf_summagthresh_clip
    }
    #[doc = "0x354 - INTF_SUMMAGDIFFTHRESH_CLIP"]
    #[inline(always)]
    pub const fn intf_summagdiffthresh_clip(&self) -> &IntfSummagdiffthreshClip {
        &self.intf_summagdiffthresh_clip
    }
    #[doc = "0x358 - CMULTSCALE1I"]
    #[inline(always)]
    pub const fn cmultscale1i(&self) -> &Cmultscale1i {
        &self.cmultscale1i
    }
    #[doc = "0x35c - CMULTSCALE2I"]
    #[inline(always)]
    pub const fn cmultscale2i(&self) -> &Cmultscale2i {
        &self.cmultscale2i
    }
    #[doc = "0x360 - CMULTSCALE3I"]
    #[inline(always)]
    pub const fn cmultscale3i(&self) -> &Cmultscale3i {
        &self.cmultscale3i
    }
    #[doc = "0x364 - CMULTSCALE4I"]
    #[inline(always)]
    pub const fn cmultscale4i(&self) -> &Cmultscale4i {
        &self.cmultscale4i
    }
    #[doc = "0x368 - CMULTSCALE5I"]
    #[inline(always)]
    pub const fn cmultscale5i(&self) -> &Cmultscale5i {
        &self.cmultscale5i
    }
    #[doc = "0x36c - CMULTSCALE6I"]
    #[inline(always)]
    pub const fn cmultscale6i(&self) -> &Cmultscale6i {
        &self.cmultscale6i
    }
    #[doc = "0x370 - CMULTSCALE1Q"]
    #[inline(always)]
    pub const fn cmultscale1q(&self) -> &Cmultscale1q {
        &self.cmultscale1q
    }
    #[doc = "0x374 - CMULTSCALE2Q"]
    #[inline(always)]
    pub const fn cmultscale2q(&self) -> &Cmultscale2q {
        &self.cmultscale2q
    }
    #[doc = "0x378 - CMULTSCALE3Q"]
    #[inline(always)]
    pub const fn cmultscale3q(&self) -> &Cmultscale3q {
        &self.cmultscale3q
    }
    #[doc = "0x37c - CMULTSCALE4Q"]
    #[inline(always)]
    pub const fn cmultscale4q(&self) -> &Cmultscale4q {
        &self.cmultscale4q
    }
    #[doc = "0x380 - CMULTSCALE5Q"]
    #[inline(always)]
    pub const fn cmultscale5q(&self) -> &Cmultscale5q {
        &self.cmultscale5q
    }
    #[doc = "0x384 - CMULTSCALE6Q"]
    #[inline(always)]
    pub const fn cmultscale6q(&self) -> &Cmultscale6q {
        &self.cmultscale6q
    }
    #[doc = "0x388 - CLR_MISC_CLIP"]
    #[inline(always)]
    pub const fn clr_misc_clip(&self) -> &ClrMiscClip {
        &self.clr_misc_clip
    }
    #[doc = "0x38c - FFTINTMEMADDR"]
    #[inline(always)]
    pub const fn fftintmemaddr(&self) -> &Fftintmemaddr {
        &self.fftintmemaddr
    }
    #[doc = "0x390 - INTF_STATS_RESET_SW"]
    #[inline(always)]
    pub const fn intf_stats_reset_sw(&self) -> &IntfStatsResetSw {
        &self.intf_stats_reset_sw
    }
    #[doc = "0x394 - DCEST_RESET_SW"]
    #[inline(always)]
    pub const fn dcest_reset_sw(&self) -> &DcestResetSw {
        &self.dcest_reset_sw
    }
    #[doc = "0x398 - IP_OP_FORMATTER_CLIP_STATUS"]
    #[inline(always)]
    pub const fn ip_op_formatter_clip_status(&self) -> &IpOpFormatterClipStatus {
        &self.ip_op_formatter_clip_status
    }
    #[doc = "0x39c - INTF_MAGTHRESH1_CLIP"]
    #[inline(always)]
    pub const fn intf_magthresh1_clip(&self) -> &IntfMagthresh1Clip {
        &self.intf_magthresh1_clip
    }
    #[doc = "0x3a0 - INTF_MAGTHRESH2_CLIP"]
    #[inline(always)]
    pub const fn intf_magthresh2_clip(&self) -> &IntfMagthresh2Clip {
        &self.intf_magthresh2_clip
    }
    #[doc = "0x3a4 - INTF_MAGTHRESH3_CLIP"]
    #[inline(always)]
    pub const fn intf_magthresh3_clip(&self) -> &IntfMagthresh3Clip {
        &self.intf_magthresh3_clip
    }
    #[doc = "0x3a8 - INTF_MAGTHRESH4_CLIP"]
    #[inline(always)]
    pub const fn intf_magthresh4_clip(&self) -> &IntfMagthresh4Clip {
        &self.intf_magthresh4_clip
    }
    #[doc = "0x3ac - INTF_MAGTHRESH5_CLIP"]
    #[inline(always)]
    pub const fn intf_magthresh5_clip(&self) -> &IntfMagthresh5Clip {
        &self.intf_magthresh5_clip
    }
    #[doc = "0x3b0 - INTF_MAGTHRESH6_CLIP"]
    #[inline(always)]
    pub const fn intf_magthresh6_clip(&self) -> &IntfMagthresh6Clip {
        &self.intf_magthresh6_clip
    }
    #[doc = "0x3b4 - INTF_MAGDIFFTHRESH1_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffthresh1_clip(&self) -> &IntfMagdiffthresh1Clip {
        &self.intf_magdiffthresh1_clip
    }
    #[doc = "0x3b8 - INTF_MAGDIFFTHRESH2_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffthresh2_clip(&self) -> &IntfMagdiffthresh2Clip {
        &self.intf_magdiffthresh2_clip
    }
    #[doc = "0x3bc - INTF_MAGDIFFTHRESH3_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffthresh3_clip(&self) -> &IntfMagdiffthresh3Clip {
        &self.intf_magdiffthresh3_clip
    }
    #[doc = "0x3c0 - INTF_MAGDIFFTHRESH4_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffthresh4_clip(&self) -> &IntfMagdiffthresh4Clip {
        &self.intf_magdiffthresh4_clip
    }
    #[doc = "0x3c4 - INTF_MAGDIFFTHRESH5_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffthresh5_clip(&self) -> &IntfMagdiffthresh5Clip {
        &self.intf_magdiffthresh5_clip
    }
    #[doc = "0x3c8 - INTF_MAGDIFFTHRESH6_CLIP"]
    #[inline(always)]
    pub const fn intf_magdiffthresh6_clip(&self) -> &IntfMagdiffthresh6Clip {
        &self.intf_magdiffthresh6_clip
    }
    #[doc = "0x3cc - HWA_SAFETY_ERR_MASK"]
    #[inline(always)]
    pub const fn hwa_safety_err_mask(&self) -> &HwaSafetyErrMask {
        &self.hwa_safety_err_mask
    }
    #[doc = "0x3d0 - HWA_SAFETY_ERR_STATUS"]
    #[inline(always)]
    pub const fn hwa_safety_err_status(&self) -> &HwaSafetyErrStatus {
        &self.hwa_safety_err_status
    }
    #[doc = "0x3d4 - HWA_SAFETY_ERR_STATUS_RAW"]
    #[inline(always)]
    pub const fn hwa_safety_err_status_raw(&self) -> &HwaSafetyErrStatusRaw {
        &self.hwa_safety_err_status_raw
    }
}
#[doc = "HWACCREG1 (rw) register accessor: HWACCREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg1`]
module"]
#[doc(alias = "HWACCREG1")]
pub type Hwaccreg1 = crate::Reg<hwaccreg1::Hwaccreg1Spec>;
#[doc = "HWACCREG1"]
pub mod hwaccreg1;
#[doc = "HWACCREG2 (rw) register accessor: HWACCREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg2`]
module"]
#[doc(alias = "HWACCREG2")]
pub type Hwaccreg2 = crate::Reg<hwaccreg2::Hwaccreg2Spec>;
#[doc = "HWACCREG2"]
pub mod hwaccreg2;
#[doc = "HWACCREG3 (rw) register accessor: HWACCREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg3`]
module"]
#[doc(alias = "HWACCREG3")]
pub type Hwaccreg3 = crate::Reg<hwaccreg3::Hwaccreg3Spec>;
#[doc = "HWACCREG3"]
pub mod hwaccreg3;
#[doc = "HWACCREG4 (rw) register accessor: HWACCREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg4`]
module"]
#[doc(alias = "HWACCREG4")]
pub type Hwaccreg4 = crate::Reg<hwaccreg4::Hwaccreg4Spec>;
#[doc = "HWACCREG4"]
pub mod hwaccreg4;
#[doc = "HWACCREG5 (rw) register accessor: HWACCREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg5`]
module"]
#[doc(alias = "HWACCREG5")]
pub type Hwaccreg5 = crate::Reg<hwaccreg5::Hwaccreg5Spec>;
#[doc = "HWACCREG5"]
pub mod hwaccreg5;
#[doc = "HWACCREG6 (rw) register accessor: HWACCREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg6`]
module"]
#[doc(alias = "HWACCREG6")]
pub type Hwaccreg6 = crate::Reg<hwaccreg6::Hwaccreg6Spec>;
#[doc = "HWACCREG6"]
pub mod hwaccreg6;
#[doc = "HWACCREG7 (rw) register accessor: HWACCREG7\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg7`]
module"]
#[doc(alias = "HWACCREG7")]
pub type Hwaccreg7 = crate::Reg<hwaccreg7::Hwaccreg7Spec>;
#[doc = "HWACCREG7"]
pub mod hwaccreg7;
#[doc = "HWACCREG8 (rw) register accessor: HWACCREG8\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg8`]
module"]
#[doc(alias = "HWACCREG8")]
pub type Hwaccreg8 = crate::Reg<hwaccreg8::Hwaccreg8Spec>;
#[doc = "HWACCREG8"]
pub mod hwaccreg8;
#[doc = "HWACCREG11 (rw) register accessor: HWACCREG11\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg11`]
module"]
#[doc(alias = "HWACCREG11")]
pub type Hwaccreg11 = crate::Reg<hwaccreg11::Hwaccreg11Spec>;
#[doc = "HWACCREG11"]
pub mod hwaccreg11;
#[doc = "HWACCREG12 (rw) register accessor: HWACCREG12\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg12`]
module"]
#[doc(alias = "HWACCREG12")]
pub type Hwaccreg12 = crate::Reg<hwaccreg12::Hwaccreg12Spec>;
#[doc = "HWACCREG12"]
pub mod hwaccreg12;
#[doc = "HWACCREG13 (rw) register accessor: HWACCREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg13`]
module"]
#[doc(alias = "HWACCREG13")]
pub type Hwaccreg13 = crate::Reg<hwaccreg13::Hwaccreg13Spec>;
#[doc = "HWACCREG13"]
pub mod hwaccreg13;
#[doc = "HWACCREG14 (rw) register accessor: HWACCREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg14`]
module"]
#[doc(alias = "HWACCREG14")]
pub type Hwaccreg14 = crate::Reg<hwaccreg14::Hwaccreg14Spec>;
#[doc = "HWACCREG14"]
pub mod hwaccreg14;
#[doc = "HWACCREG15 (rw) register accessor: HWACCREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg15`]
module"]
#[doc(alias = "HWACCREG15")]
pub type Hwaccreg15 = crate::Reg<hwaccreg15::Hwaccreg15Spec>;
#[doc = "HWACCREG15"]
pub mod hwaccreg15;
#[doc = "CFAR_DET_THR (rw) register accessor: CFAR_DET_THR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfar_det_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfar_det_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfar_det_thr`]
module"]
#[doc(alias = "CFAR_DET_THR")]
pub type CfarDetThr = crate::Reg<cfar_det_thr::CfarDetThrSpec>;
#[doc = "CFAR_DET_THR"]
pub mod cfar_det_thr;
#[doc = "MAX1VALUE (rw) register accessor: MAX1VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max1value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max1value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max1value`]
module"]
#[doc(alias = "MAX1VALUE")]
pub type Max1value = crate::Reg<max1value::Max1valueSpec>;
#[doc = "MAX1VALUE"]
pub mod max1value;
#[doc = "MAX1INDEX (rw) register accessor: MAX1INDEX\n\nYou can [`read`](crate::Reg::read) this register and get [`max1index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max1index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max1index`]
module"]
#[doc(alias = "MAX1INDEX")]
pub type Max1index = crate::Reg<max1index::Max1indexSpec>;
#[doc = "MAX1INDEX"]
pub mod max1index;
#[doc = "ISUM1LSB (rw) register accessor: ISUM1LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum1lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum1lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum1lsb`]
module"]
#[doc(alias = "ISUM1LSB")]
pub type Isum1lsb = crate::Reg<isum1lsb::Isum1lsbSpec>;
#[doc = "ISUM1LSB"]
pub mod isum1lsb;
#[doc = "ISUM1MSB (rw) register accessor: ISUM1MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum1msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum1msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum1msb`]
module"]
#[doc(alias = "ISUM1MSB")]
pub type Isum1msb = crate::Reg<isum1msb::Isum1msbSpec>;
#[doc = "ISUM1MSB"]
pub mod isum1msb;
#[doc = "QSUM1LSB (rw) register accessor: QSUM1LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum1lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum1lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum1lsb`]
module"]
#[doc(alias = "QSUM1LSB")]
pub type Qsum1lsb = crate::Reg<qsum1lsb::Qsum1lsbSpec>;
#[doc = "QSUM1LSB"]
pub mod qsum1lsb;
#[doc = "QSUM1MSB (rw) register accessor: QSUM1MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum1msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum1msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum1msb`]
module"]
#[doc(alias = "QSUM1MSB")]
pub type Qsum1msb = crate::Reg<qsum1msb::Qsum1msbSpec>;
#[doc = "QSUM1MSB"]
pub mod qsum1msb;
#[doc = "MAX2VALUE (rw) register accessor: MAX2VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max2value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max2value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max2value`]
module"]
#[doc(alias = "MAX2VALUE")]
pub type Max2value = crate::Reg<max2value::Max2valueSpec>;
#[doc = "MAX2VALUE"]
pub mod max2value;
#[doc = "MAX2INDEX (rw) register accessor: MAX2INDEX\n\nYou can [`read`](crate::Reg::read) this register and get [`max2index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max2index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max2index`]
module"]
#[doc(alias = "MAX2INDEX")]
pub type Max2index = crate::Reg<max2index::Max2indexSpec>;
#[doc = "MAX2INDEX"]
pub mod max2index;
#[doc = "ISUM2LSB (rw) register accessor: ISUM2LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum2lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum2lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum2lsb`]
module"]
#[doc(alias = "ISUM2LSB")]
pub type Isum2lsb = crate::Reg<isum2lsb::Isum2lsbSpec>;
#[doc = "ISUM2LSB"]
pub mod isum2lsb;
#[doc = "ISUM2MSB (rw) register accessor: ISUM2MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum2msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum2msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum2msb`]
module"]
#[doc(alias = "ISUM2MSB")]
pub type Isum2msb = crate::Reg<isum2msb::Isum2msbSpec>;
#[doc = "ISUM2MSB"]
pub mod isum2msb;
#[doc = "QSUM2LSB (rw) register accessor: QSUM2LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum2lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum2lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum2lsb`]
module"]
#[doc(alias = "QSUM2LSB")]
pub type Qsum2lsb = crate::Reg<qsum2lsb::Qsum2lsbSpec>;
#[doc = "QSUM2LSB"]
pub mod qsum2lsb;
#[doc = "QSUM2MSB (rw) register accessor: QSUM2MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum2msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum2msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum2msb`]
module"]
#[doc(alias = "QSUM2MSB")]
pub type Qsum2msb = crate::Reg<qsum2msb::Qsum2msbSpec>;
#[doc = "QSUM2MSB"]
pub mod qsum2msb;
#[doc = "MAX3VALUE (rw) register accessor: MAX3VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max3value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max3value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max3value`]
module"]
#[doc(alias = "MAX3VALUE")]
pub type Max3value = crate::Reg<max3value::Max3valueSpec>;
#[doc = "MAX3VALUE"]
pub mod max3value;
#[doc = "MAX3INDEX (rw) register accessor: MAX3INDEX\n\nYou can [`read`](crate::Reg::read) this register and get [`max3index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max3index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max3index`]
module"]
#[doc(alias = "MAX3INDEX")]
pub type Max3index = crate::Reg<max3index::Max3indexSpec>;
#[doc = "MAX3INDEX"]
pub mod max3index;
#[doc = "ISUM3LSB (rw) register accessor: ISUM3LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum3lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum3lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum3lsb`]
module"]
#[doc(alias = "ISUM3LSB")]
pub type Isum3lsb = crate::Reg<isum3lsb::Isum3lsbSpec>;
#[doc = "ISUM3LSB"]
pub mod isum3lsb;
#[doc = "ISUM3MSB (rw) register accessor: ISUM3MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum3msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum3msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum3msb`]
module"]
#[doc(alias = "ISUM3MSB")]
pub type Isum3msb = crate::Reg<isum3msb::Isum3msbSpec>;
#[doc = "ISUM3MSB"]
pub mod isum3msb;
#[doc = "QSUM3LSB (rw) register accessor: QSUM3LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum3lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum3lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum3lsb`]
module"]
#[doc(alias = "QSUM3LSB")]
pub type Qsum3lsb = crate::Reg<qsum3lsb::Qsum3lsbSpec>;
#[doc = "QSUM3LSB"]
pub mod qsum3lsb;
#[doc = "QSUM3MSB (rw) register accessor: QSUM3MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum3msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum3msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum3msb`]
module"]
#[doc(alias = "QSUM3MSB")]
pub type Qsum3msb = crate::Reg<qsum3msb::Qsum3msbSpec>;
#[doc = "QSUM3MSB"]
pub mod qsum3msb;
#[doc = "MAX4VALUE (rw) register accessor: MAX4VALUE\n\nYou can [`read`](crate::Reg::read) this register and get [`max4value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max4value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max4value`]
module"]
#[doc(alias = "MAX4VALUE")]
pub type Max4value = crate::Reg<max4value::Max4valueSpec>;
#[doc = "MAX4VALUE"]
pub mod max4value;
#[doc = "MAX4INDEX (rw) register accessor: MAX4INDEX\n\nYou can [`read`](crate::Reg::read) this register and get [`max4index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max4index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max4index`]
module"]
#[doc(alias = "MAX4INDEX")]
pub type Max4index = crate::Reg<max4index::Max4indexSpec>;
#[doc = "MAX4INDEX"]
pub mod max4index;
#[doc = "ISUM4LSB (rw) register accessor: ISUM4LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum4lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum4lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum4lsb`]
module"]
#[doc(alias = "ISUM4LSB")]
pub type Isum4lsb = crate::Reg<isum4lsb::Isum4lsbSpec>;
#[doc = "ISUM4LSB"]
pub mod isum4lsb;
#[doc = "ISUM4MSB (rw) register accessor: ISUM4MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum4msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum4msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isum4msb`]
module"]
#[doc(alias = "ISUM4MSB")]
pub type Isum4msb = crate::Reg<isum4msb::Isum4msbSpec>;
#[doc = "ISUM4MSB"]
pub mod isum4msb;
#[doc = "QSUM4LSB (rw) register accessor: QSUM4LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum4lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum4lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum4lsb`]
module"]
#[doc(alias = "QSUM4LSB")]
pub type Qsum4lsb = crate::Reg<qsum4lsb::Qsum4lsbSpec>;
#[doc = "QSUM4LSB"]
pub mod qsum4lsb;
#[doc = "QSUM4MSB (rw) register accessor: QSUM4MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum4msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum4msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qsum4msb`]
module"]
#[doc(alias = "QSUM4MSB")]
pub type Qsum4msb = crate::Reg<qsum4msb::Qsum4msbSpec>;
#[doc = "QSUM4MSB"]
pub mod qsum4msb;
#[doc = "CFARTEST (rw) register accessor: CFARTEST\n\nYou can [`read`](crate::Reg::read) this register and get [`cfartest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfartest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfartest`]
module"]
#[doc(alias = "CFARTEST")]
pub type Cfartest = crate::Reg<cfartest::CfartestSpec>;
#[doc = "CFARTEST"]
pub mod cfartest;
#[doc = "RDSTATUS (rw) register accessor: RDSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rdstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdstatus`]
module"]
#[doc(alias = "RDSTATUS")]
pub type Rdstatus = crate::Reg<rdstatus::RdstatusSpec>;
#[doc = "RDSTATUS"]
pub mod rdstatus;
#[doc = "SIGDMACH1DONE (rw) register accessor: SIGDMACH1DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach1done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach1done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach1done`]
module"]
#[doc(alias = "SIGDMACH1DONE")]
pub type Sigdmach1done = crate::Reg<sigdmach1done::Sigdmach1doneSpec>;
#[doc = "SIGDMACH1DONE"]
pub mod sigdmach1done;
#[doc = "SIGDMACH2DONE (rw) register accessor: SIGDMACH2DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach2done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach2done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach2done`]
module"]
#[doc(alias = "SIGDMACH2DONE")]
pub type Sigdmach2done = crate::Reg<sigdmach2done::Sigdmach2doneSpec>;
#[doc = "SIGDMACH2DONE"]
pub mod sigdmach2done;
#[doc = "SIGDMACH3DONE (rw) register accessor: SIGDMACH3DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach3done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach3done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach3done`]
module"]
#[doc(alias = "SIGDMACH3DONE")]
pub type Sigdmach3done = crate::Reg<sigdmach3done::Sigdmach3doneSpec>;
#[doc = "SIGDMACH3DONE"]
pub mod sigdmach3done;
#[doc = "SIGDMACH4DONE (rw) register accessor: SIGDMACH4DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach4done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach4done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach4done`]
module"]
#[doc(alias = "SIGDMACH4DONE")]
pub type Sigdmach4done = crate::Reg<sigdmach4done::Sigdmach4doneSpec>;
#[doc = "SIGDMACH4DONE"]
pub mod sigdmach4done;
#[doc = "SIGDMACH5DONE (rw) register accessor: SIGDMACH5DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach5done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach5done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach5done`]
module"]
#[doc(alias = "SIGDMACH5DONE")]
pub type Sigdmach5done = crate::Reg<sigdmach5done::Sigdmach5doneSpec>;
#[doc = "SIGDMACH5DONE"]
pub mod sigdmach5done;
#[doc = "SIGDMACH6DONE (rw) register accessor: SIGDMACH6DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach6done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach6done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach6done`]
module"]
#[doc(alias = "SIGDMACH6DONE")]
pub type Sigdmach6done = crate::Reg<sigdmach6done::Sigdmach6doneSpec>;
#[doc = "SIGDMACH6DONE"]
pub mod sigdmach6done;
#[doc = "SIGDMACH7DONE (rw) register accessor: SIGDMACH7DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach7done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach7done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach7done`]
module"]
#[doc(alias = "SIGDMACH7DONE")]
pub type Sigdmach7done = crate::Reg<sigdmach7done::Sigdmach7doneSpec>;
#[doc = "SIGDMACH7DONE"]
pub mod sigdmach7done;
#[doc = "SIGDMACH8DONE (rw) register accessor: SIGDMACH8DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach8done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach8done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach8done`]
module"]
#[doc(alias = "SIGDMACH8DONE")]
pub type Sigdmach8done = crate::Reg<sigdmach8done::Sigdmach8doneSpec>;
#[doc = "SIGDMACH8DONE"]
pub mod sigdmach8done;
#[doc = "SIGDMACH9DONE (rw) register accessor: SIGDMACH9DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach9done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach9done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach9done`]
module"]
#[doc(alias = "SIGDMACH9DONE")]
pub type Sigdmach9done = crate::Reg<sigdmach9done::Sigdmach9doneSpec>;
#[doc = "SIGDMACH9DONE"]
pub mod sigdmach9done;
#[doc = "SIGDMACH10DONE (rw) register accessor: SIGDMACH10DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach10done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach10done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach10done`]
module"]
#[doc(alias = "SIGDMACH10DONE")]
pub type Sigdmach10done = crate::Reg<sigdmach10done::Sigdmach10doneSpec>;
#[doc = "SIGDMACH10DONE"]
pub mod sigdmach10done;
#[doc = "SIGDMACH11DONE (rw) register accessor: SIGDMACH11DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach11done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach11done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach11done`]
module"]
#[doc(alias = "SIGDMACH11DONE")]
pub type Sigdmach11done = crate::Reg<sigdmach11done::Sigdmach11doneSpec>;
#[doc = "SIGDMACH11DONE"]
pub mod sigdmach11done;
#[doc = "SIGDMACH12DONE (rw) register accessor: SIGDMACH12DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach12done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach12done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach12done`]
module"]
#[doc(alias = "SIGDMACH12DONE")]
pub type Sigdmach12done = crate::Reg<sigdmach12done::Sigdmach12doneSpec>;
#[doc = "SIGDMACH12DONE"]
pub mod sigdmach12done;
#[doc = "SIGDMACH13DONE (rw) register accessor: SIGDMACH13DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach13done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach13done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach13done`]
module"]
#[doc(alias = "SIGDMACH13DONE")]
pub type Sigdmach13done = crate::Reg<sigdmach13done::Sigdmach13doneSpec>;
#[doc = "SIGDMACH13DONE"]
pub mod sigdmach13done;
#[doc = "SIGDMACH14DONE (rw) register accessor: SIGDMACH14DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach14done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach14done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach14done`]
module"]
#[doc(alias = "SIGDMACH14DONE")]
pub type Sigdmach14done = crate::Reg<sigdmach14done::Sigdmach14doneSpec>;
#[doc = "SIGDMACH14DONE"]
pub mod sigdmach14done;
#[doc = "SIGDMACH15DONE (rw) register accessor: SIGDMACH15DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach15done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach15done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach15done`]
module"]
#[doc(alias = "SIGDMACH15DONE")]
pub type Sigdmach15done = crate::Reg<sigdmach15done::Sigdmach15doneSpec>;
#[doc = "SIGDMACH15DONE"]
pub mod sigdmach15done;
#[doc = "SIGDMACH16DONE (rw) register accessor: SIGDMACH16DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach16done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach16done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigdmach16done`]
module"]
#[doc(alias = "SIGDMACH16DONE")]
pub type Sigdmach16done = crate::Reg<sigdmach16done::Sigdmach16doneSpec>;
#[doc = "SIGDMACH16DONE"]
pub mod sigdmach16done;
#[doc = "MEMACCESSERR (rw) register accessor: MEMACCESSERR\n\nYou can [`read`](crate::Reg::read) this register and get [`memaccesserr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memaccesserr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memaccesserr`]
module"]
#[doc(alias = "MEMACCESSERR")]
pub type Memaccesserr = crate::Reg<memaccesserr::MemaccesserrSpec>;
#[doc = "MEMACCESSERR"]
pub mod memaccesserr;
#[doc = "FFTCLIP (rw) register accessor: FFTCLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`fftclip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftclip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fftclip`]
module"]
#[doc(alias = "FFTCLIP")]
pub type Fftclip = crate::Reg<fftclip::FftclipSpec>;
#[doc = "FFTCLIP"]
pub mod fftclip;
#[doc = "FFTPEAKCNT (rw) register accessor: FFTPEAKCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`fftpeakcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftpeakcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fftpeakcnt`]
module"]
#[doc(alias = "FFTPEAKCNT")]
pub type Fftpeakcnt = crate::Reg<fftpeakcnt::FftpeakcntSpec>;
#[doc = "FFTPEAKCNT"]
pub mod fftpeakcnt;
#[doc = "HWACCREG1RD (rw) register accessor: HWACCREG1RD\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg1rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg1rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg1rd`]
module"]
#[doc(alias = "HWACCREG1RD")]
pub type Hwaccreg1rd = crate::Reg<hwaccreg1rd::Hwaccreg1rdSpec>;
#[doc = "HWACCREG1RD"]
pub mod hwaccreg1rd;
#[doc = "HWACCREG2RD (rw) register accessor: HWACCREG2RD\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg2rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg2rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg2rd`]
module"]
#[doc(alias = "HWACCREG2RD")]
pub type Hwaccreg2rd = crate::Reg<hwaccreg2rd::Hwaccreg2rdSpec>;
#[doc = "HWACCREG2RD"]
pub mod hwaccreg2rd;
#[doc = "HWACCREG3RD (rw) register accessor: HWACCREG3RD\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg3rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg3rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg3rd`]
module"]
#[doc(alias = "HWACCREG3RD")]
pub type Hwaccreg3rd = crate::Reg<hwaccreg3rd::Hwaccreg3rdSpec>;
#[doc = "HWACCREG3RD"]
pub mod hwaccreg3rd;
#[doc = "CMP_EGE_K0123 (rw) register accessor: CMP_EGE_K0123\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ege_k0123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ege_k0123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ege_k0123`]
module"]
#[doc(alias = "CMP_EGE_K0123")]
pub type CmpEgeK0123 = crate::Reg<cmp_ege_k0123::CmpEgeK0123Spec>;
#[doc = "CMP_EGE_K0123"]
pub mod cmp_ege_k0123;
#[doc = "CMP_EGE_K4567 (rw) register accessor: CMP_EGE_K4567\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ege_k4567::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ege_k4567::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ege_k4567`]
module"]
#[doc(alias = "CMP_EGE_K4567")]
pub type CmpEgeK4567 = crate::Reg<cmp_ege_k4567::CmpEgeK4567Spec>;
#[doc = "CMP_EGE_K4567"]
pub mod cmp_ege_k4567;
#[doc = "HWA_SAFETY_ENABLE (rw) register accessor: HWA_SAFETY_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_enable`]
module"]
#[doc(alias = "HWA_SAFETY_ENABLE")]
pub type HwaSafetyEnable = crate::Reg<hwa_safety_enable::HwaSafetyEnableSpec>;
#[doc = "HWA_SAFETY_ENABLE"]
pub mod hwa_safety_enable;
#[doc = "MEMINIT (rw) register accessor: MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`meminit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meminit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meminit`]
module"]
#[doc(alias = "MEMINIT")]
pub type Meminit = crate::Reg<meminit::MeminitSpec>;
#[doc = "MEMINIT"]
pub mod meminit;
#[doc = "MEMINITDONE (rw) register accessor: MEMINITDONE\n\nYou can [`read`](crate::Reg::read) this register and get [`meminitdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meminitdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meminitdone`]
module"]
#[doc(alias = "MEMINITDONE")]
pub type Meminitdone = crate::Reg<meminitdone::MeminitdoneSpec>;
#[doc = "MEMINITDONE"]
pub mod meminitdone;
#[doc = "HWA_SAFETY_WIN_RAM_ERR_LOC (rw) register accessor: HWA_SAFETY_WIN_RAM_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_win_ram_err_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_win_ram_err_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_win_ram_err_loc`]
module"]
#[doc(alias = "HWA_SAFETY_WIN_RAM_ERR_LOC")]
pub type HwaSafetyWinRamErrLoc = crate::Reg<hwa_safety_win_ram_err_loc::HwaSafetyWinRamErrLocSpec>;
#[doc = "HWA_SAFETY_WIN_RAM_ERR_LOC"]
pub mod hwa_safety_win_ram_err_loc;
#[doc = "HWA_SAFETY_PARAM_RAM_ERR_LOC (rw) register accessor: HWA_SAFETY_PARAM_RAM_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_param_ram_err_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_param_ram_err_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_param_ram_err_loc`]
module"]
#[doc(alias = "HWA_SAFETY_PARAM_RAM_ERR_LOC")]
pub type HwaSafetyParamRamErrLoc =
    crate::Reg<hwa_safety_param_ram_err_loc::HwaSafetyParamRamErrLocSpec>;
#[doc = "HWA_SAFETY_PARAM_RAM_ERR_LOC"]
pub mod hwa_safety_param_ram_err_loc;
#[doc = "HWA_SAFETY_IPING_ERR_LOC (rw) register accessor: HWA_SAFETY_IPING_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_iping_err_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_iping_err_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_iping_err_loc`]
module"]
#[doc(alias = "HWA_SAFETY_IPING_ERR_LOC")]
pub type HwaSafetyIpingErrLoc = crate::Reg<hwa_safety_iping_err_loc::HwaSafetyIpingErrLocSpec>;
#[doc = "HWA_SAFETY_IPING_ERR_LOC"]
pub mod hwa_safety_iping_err_loc;
#[doc = "HWA_SAFETY_IPONG_ERR_LOC (rw) register accessor: HWA_SAFETY_IPONG_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_ipong_err_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_ipong_err_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_ipong_err_loc`]
module"]
#[doc(alias = "HWA_SAFETY_IPONG_ERR_LOC")]
pub type HwaSafetyIpongErrLoc = crate::Reg<hwa_safety_ipong_err_loc::HwaSafetyIpongErrLocSpec>;
#[doc = "HWA_SAFETY_IPONG_ERR_LOC"]
pub mod hwa_safety_ipong_err_loc;
#[doc = "HWA_SAFETY_OPING_ERR_LOC (rw) register accessor: HWA_SAFETY_OPING_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_oping_err_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_oping_err_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_oping_err_loc`]
module"]
#[doc(alias = "HWA_SAFETY_OPING_ERR_LOC")]
pub type HwaSafetyOpingErrLoc = crate::Reg<hwa_safety_oping_err_loc::HwaSafetyOpingErrLocSpec>;
#[doc = "HWA_SAFETY_OPING_ERR_LOC"]
pub mod hwa_safety_oping_err_loc;
#[doc = "HWA_SAFETY_OPONG_ERR_LOC (rw) register accessor: HWA_SAFETY_OPONG_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_opong_err_loc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_opong_err_loc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_opong_err_loc`]
module"]
#[doc(alias = "HWA_SAFETY_OPONG_ERR_LOC")]
pub type HwaSafetyOpongErrLoc = crate::Reg<hwa_safety_opong_err_loc::HwaSafetyOpongErrLocSpec>;
#[doc = "HWA_SAFETY_OPONG_ERR_LOC"]
pub mod hwa_safety_opong_err_loc;
#[doc = "FFTINTMEMWRDATA (rw) register accessor: FFTINTMEMWRDATA\n\nYou can [`read`](crate::Reg::read) this register and get [`fftintmemwrdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftintmemwrdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fftintmemwrdata`]
module"]
#[doc(alias = "FFTINTMEMWRDATA")]
pub type Fftintmemwrdata = crate::Reg<fftintmemwrdata::FftintmemwrdataSpec>;
#[doc = "FFTINTMEMWRDATA"]
pub mod fftintmemwrdata;
#[doc = "FFTINTMEMRDDATA (rw) register accessor: FFTINTMEMRDDATA\n\nYou can [`read`](crate::Reg::read) this register and get [`fftintmemrddata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftintmemrddata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fftintmemrddata`]
module"]
#[doc(alias = "FFTINTMEMRDDATA")]
pub type Fftintmemrddata = crate::Reg<fftintmemrddata::FftintmemrddataSpec>;
#[doc = "FFTINTMEMRDDATA"]
pub mod fftintmemrddata;
#[doc = "HWACCREG16 (rw) register accessor: HWACCREG16\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwaccreg16`]
module"]
#[doc(alias = "HWACCREG16")]
pub type Hwaccreg16 = crate::Reg<hwaccreg16::Hwaccreg16Spec>;
#[doc = "HWACCREG16"]
pub mod hwaccreg16;
#[doc = "DCEST1I_SW (rw) register accessor: DCEST1I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest1i_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest1i_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest1i_sw`]
module"]
#[doc(alias = "DCEST1I_SW")]
pub type Dcest1iSw = crate::Reg<dcest1i_sw::Dcest1iSwSpec>;
#[doc = "DCEST1I_SW"]
pub mod dcest1i_sw;
#[doc = "DCEST2I_SW (rw) register accessor: DCEST2I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest2i_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest2i_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest2i_sw`]
module"]
#[doc(alias = "DCEST2I_SW")]
pub type Dcest2iSw = crate::Reg<dcest2i_sw::Dcest2iSwSpec>;
#[doc = "DCEST2I_SW"]
pub mod dcest2i_sw;
#[doc = "DCEST3I_SW (rw) register accessor: DCEST3I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest3i_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest3i_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest3i_sw`]
module"]
#[doc(alias = "DCEST3I_SW")]
pub type Dcest3iSw = crate::Reg<dcest3i_sw::Dcest3iSwSpec>;
#[doc = "DCEST3I_SW"]
pub mod dcest3i_sw;
#[doc = "DCEST4I_SW (rw) register accessor: DCEST4I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest4i_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest4i_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest4i_sw`]
module"]
#[doc(alias = "DCEST4I_SW")]
pub type Dcest4iSw = crate::Reg<dcest4i_sw::Dcest4iSwSpec>;
#[doc = "DCEST4I_SW"]
pub mod dcest4i_sw;
#[doc = "DCEST5I_SW (rw) register accessor: DCEST5I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest5i_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest5i_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest5i_sw`]
module"]
#[doc(alias = "DCEST5I_SW")]
pub type Dcest5iSw = crate::Reg<dcest5i_sw::Dcest5iSwSpec>;
#[doc = "DCEST5I_SW"]
pub mod dcest5i_sw;
#[doc = "DCEST6I_SW (rw) register accessor: DCEST6I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6i_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6i_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest6i_sw`]
module"]
#[doc(alias = "DCEST6I_SW")]
pub type Dcest6iSw = crate::Reg<dcest6i_sw::Dcest6iSwSpec>;
#[doc = "DCEST6I_SW"]
pub mod dcest6i_sw;
#[doc = "DCEST1I (rw) register accessor: DCEST1I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest1i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest1i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest1i`]
module"]
#[doc(alias = "DCEST1I")]
pub type Dcest1i = crate::Reg<dcest1i::Dcest1iSpec>;
#[doc = "DCEST1I"]
pub mod dcest1i;
#[doc = "DCEST2I (rw) register accessor: DCEST2I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest2i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest2i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest2i`]
module"]
#[doc(alias = "DCEST2I")]
pub type Dcest2i = crate::Reg<dcest2i::Dcest2iSpec>;
#[doc = "DCEST2I"]
pub mod dcest2i;
#[doc = "DCEST3I (rw) register accessor: DCEST3I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest3i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest3i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest3i`]
module"]
#[doc(alias = "DCEST3I")]
pub type Dcest3i = crate::Reg<dcest3i::Dcest3iSpec>;
#[doc = "DCEST3I"]
pub mod dcest3i;
#[doc = "DCEST4I (rw) register accessor: DCEST4I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest4i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest4i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest4i`]
module"]
#[doc(alias = "DCEST4I")]
pub type Dcest4i = crate::Reg<dcest4i::Dcest4iSpec>;
#[doc = "DCEST4I"]
pub mod dcest4i;
#[doc = "DCEST5I (rw) register accessor: DCEST5I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest5i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest5i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest5i`]
module"]
#[doc(alias = "DCEST5I")]
pub type Dcest5i = crate::Reg<dcest5i::Dcest5iSpec>;
#[doc = "DCEST5I"]
pub mod dcest5i;
#[doc = "DCEST6I (rw) register accessor: DCEST6I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest6i`]
module"]
#[doc(alias = "DCEST6I")]
pub type Dcest6i = crate::Reg<dcest6i::Dcest6iSpec>;
#[doc = "DCEST6I"]
pub mod dcest6i;
#[doc = "DC_ACC1I_LSB (rw) register accessor: DC_ACC1I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc1i_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc1i_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc1i_lsb`]
module"]
#[doc(alias = "DC_ACC1I_LSB")]
pub type DcAcc1iLsb = crate::Reg<dc_acc1i_lsb::DcAcc1iLsbSpec>;
#[doc = "DC_ACC1I_LSB"]
pub mod dc_acc1i_lsb;
#[doc = "DC_ACC1I_MSB (rw) register accessor: DC_ACC1I_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc1i_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc1i_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc1i_msb`]
module"]
#[doc(alias = "DC_ACC1I_MSB")]
pub type DcAcc1iMsb = crate::Reg<dc_acc1i_msb::DcAcc1iMsbSpec>;
#[doc = "DC_ACC1I_MSB"]
pub mod dc_acc1i_msb;
#[doc = "DC_ACC2I_LSB (rw) register accessor: DC_ACC2I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc2i_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc2i_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc2i_lsb`]
module"]
#[doc(alias = "DC_ACC2I_LSB")]
pub type DcAcc2iLsb = crate::Reg<dc_acc2i_lsb::DcAcc2iLsbSpec>;
#[doc = "DC_ACC2I_LSB"]
pub mod dc_acc2i_lsb;
#[doc = "DC_ACC2I_MSB (rw) register accessor: DC_ACC2I_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc2i_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc2i_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc2i_msb`]
module"]
#[doc(alias = "DC_ACC2I_MSB")]
pub type DcAcc2iMsb = crate::Reg<dc_acc2i_msb::DcAcc2iMsbSpec>;
#[doc = "DC_ACC2I_MSB"]
pub mod dc_acc2i_msb;
#[doc = "DC_ACC3I_LSB (rw) register accessor: DC_ACC3I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc3i_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc3i_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc3i_lsb`]
module"]
#[doc(alias = "DC_ACC3I_LSB")]
pub type DcAcc3iLsb = crate::Reg<dc_acc3i_lsb::DcAcc3iLsbSpec>;
#[doc = "DC_ACC3I_LSB"]
pub mod dc_acc3i_lsb;
#[doc = "DC_ACC3I_MSB (rw) register accessor: DC_ACC3I_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc3i_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc3i_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc3i_msb`]
module"]
#[doc(alias = "DC_ACC3I_MSB")]
pub type DcAcc3iMsb = crate::Reg<dc_acc3i_msb::DcAcc3iMsbSpec>;
#[doc = "DC_ACC3I_MSB"]
pub mod dc_acc3i_msb;
#[doc = "DC_ACC4I_LSB (rw) register accessor: DC_ACC4I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc4i_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc4i_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc4i_lsb`]
module"]
#[doc(alias = "DC_ACC4I_LSB")]
pub type DcAcc4iLsb = crate::Reg<dc_acc4i_lsb::DcAcc4iLsbSpec>;
#[doc = "DC_ACC4I_LSB"]
pub mod dc_acc4i_lsb;
#[doc = "DC_ACC4I_MSB (rw) register accessor: DC_ACC4I_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc4i_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc4i_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc4i_msb`]
module"]
#[doc(alias = "DC_ACC4I_MSB")]
pub type DcAcc4iMsb = crate::Reg<dc_acc4i_msb::DcAcc4iMsbSpec>;
#[doc = "DC_ACC4I_MSB"]
pub mod dc_acc4i_msb;
#[doc = "DC_ACC5I_LSB (rw) register accessor: DC_ACC5I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc5i_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc5i_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc5i_lsb`]
module"]
#[doc(alias = "DC_ACC5I_LSB")]
pub type DcAcc5iLsb = crate::Reg<dc_acc5i_lsb::DcAcc5iLsbSpec>;
#[doc = "DC_ACC5I_LSB"]
pub mod dc_acc5i_lsb;
#[doc = "DC_ACC5I_MSB (rw) register accessor: DC_ACC5I_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc5i_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc5i_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc5i_msb`]
module"]
#[doc(alias = "DC_ACC5I_MSB")]
pub type DcAcc5iMsb = crate::Reg<dc_acc5i_msb::DcAcc5iMsbSpec>;
#[doc = "DC_ACC5I_MSB"]
pub mod dc_acc5i_msb;
#[doc = "DC_ACC6I_LSB (rw) register accessor: DC_ACC6I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc6i_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc6i_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc6i_lsb`]
module"]
#[doc(alias = "DC_ACC6I_LSB")]
pub type DcAcc6iLsb = crate::Reg<dc_acc6i_lsb::DcAcc6iLsbSpec>;
#[doc = "DC_ACC6I_LSB"]
pub mod dc_acc6i_lsb;
#[doc = "DC_ACC6I_MSB (rw) register accessor: DC_ACC6I_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc6i_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc6i_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc6i_msb`]
module"]
#[doc(alias = "DC_ACC6I_MSB")]
pub type DcAcc6iMsb = crate::Reg<dc_acc6i_msb::DcAcc6iMsbSpec>;
#[doc = "DC_ACC6I_MSB"]
pub mod dc_acc6i_msb;
#[doc = "DCEST1Q_SW (rw) register accessor: DCEST1Q_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest1q_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest1q_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest1q_sw`]
module"]
#[doc(alias = "DCEST1Q_SW")]
pub type Dcest1qSw = crate::Reg<dcest1q_sw::Dcest1qSwSpec>;
#[doc = "DCEST1Q_SW"]
pub mod dcest1q_sw;
#[doc = "DCEST2Q_SW (rw) register accessor: DCEST2Q_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest2q_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest2q_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest2q_sw`]
module"]
#[doc(alias = "DCEST2Q_SW")]
pub type Dcest2qSw = crate::Reg<dcest2q_sw::Dcest2qSwSpec>;
#[doc = "DCEST2Q_SW"]
pub mod dcest2q_sw;
#[doc = "DCEST3Q_SW (rw) register accessor: DCEST3Q_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest3q_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest3q_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest3q_sw`]
module"]
#[doc(alias = "DCEST3Q_SW")]
pub type Dcest3qSw = crate::Reg<dcest3q_sw::Dcest3qSwSpec>;
#[doc = "DCEST3Q_SW"]
pub mod dcest3q_sw;
#[doc = "DCEST4Q_SW (rw) register accessor: DCEST4Q_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest4q_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest4q_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest4q_sw`]
module"]
#[doc(alias = "DCEST4Q_SW")]
pub type Dcest4qSw = crate::Reg<dcest4q_sw::Dcest4qSwSpec>;
#[doc = "DCEST4Q_SW"]
pub mod dcest4q_sw;
#[doc = "DCEST5Q_SW (rw) register accessor: DCEST5Q_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest5q_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest5q_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest5q_sw`]
module"]
#[doc(alias = "DCEST5Q_SW")]
pub type Dcest5qSw = crate::Reg<dcest5q_sw::Dcest5qSwSpec>;
#[doc = "DCEST5Q_SW"]
pub mod dcest5q_sw;
#[doc = "DCEST6Q_SW (rw) register accessor: DCEST6Q_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6q_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6q_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest6q_sw`]
module"]
#[doc(alias = "DCEST6Q_SW")]
pub type Dcest6qSw = crate::Reg<dcest6q_sw::Dcest6qSwSpec>;
#[doc = "DCEST6Q_SW"]
pub mod dcest6q_sw;
#[doc = "DCEST1Q (rw) register accessor: DCEST1Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest1q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest1q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest1q`]
module"]
#[doc(alias = "DCEST1Q")]
pub type Dcest1q = crate::Reg<dcest1q::Dcest1qSpec>;
#[doc = "DCEST1Q"]
pub mod dcest1q;
#[doc = "DCEST2Q (rw) register accessor: DCEST2Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest2q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest2q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest2q`]
module"]
#[doc(alias = "DCEST2Q")]
pub type Dcest2q = crate::Reg<dcest2q::Dcest2qSpec>;
#[doc = "DCEST2Q"]
pub mod dcest2q;
#[doc = "DCEST3Q (rw) register accessor: DCEST3Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest3q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest3q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest3q`]
module"]
#[doc(alias = "DCEST3Q")]
pub type Dcest3q = crate::Reg<dcest3q::Dcest3qSpec>;
#[doc = "DCEST3Q"]
pub mod dcest3q;
#[doc = "DCEST4Q (rw) register accessor: DCEST4Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest4q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest4q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest4q`]
module"]
#[doc(alias = "DCEST4Q")]
pub type Dcest4q = crate::Reg<dcest4q::Dcest4qSpec>;
#[doc = "DCEST4Q"]
pub mod dcest4q;
#[doc = "DCEST5Q (rw) register accessor: DCEST5Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest5q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest5q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest5q`]
module"]
#[doc(alias = "DCEST5Q")]
pub type Dcest5q = crate::Reg<dcest5q::Dcest5qSpec>;
#[doc = "DCEST5Q"]
pub mod dcest5q;
#[doc = "DCEST6Q (rw) register accessor: DCEST6Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest6q`]
module"]
#[doc(alias = "DCEST6Q")]
pub type Dcest6q = crate::Reg<dcest6q::Dcest6qSpec>;
#[doc = "DCEST6Q"]
pub mod dcest6q;
#[doc = "DC_ACC1Q_LSB (rw) register accessor: DC_ACC1Q_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc1q_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc1q_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc1q_lsb`]
module"]
#[doc(alias = "DC_ACC1Q_LSB")]
pub type DcAcc1qLsb = crate::Reg<dc_acc1q_lsb::DcAcc1qLsbSpec>;
#[doc = "DC_ACC1Q_LSB"]
pub mod dc_acc1q_lsb;
#[doc = "DC_ACC1Q_MSB (rw) register accessor: DC_ACC1Q_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc1q_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc1q_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc1q_msb`]
module"]
#[doc(alias = "DC_ACC1Q_MSB")]
pub type DcAcc1qMsb = crate::Reg<dc_acc1q_msb::DcAcc1qMsbSpec>;
#[doc = "DC_ACC1Q_MSB"]
pub mod dc_acc1q_msb;
#[doc = "DC_ACC2Q_LSB (rw) register accessor: DC_ACC2Q_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc2q_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc2q_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc2q_lsb`]
module"]
#[doc(alias = "DC_ACC2Q_LSB")]
pub type DcAcc2qLsb = crate::Reg<dc_acc2q_lsb::DcAcc2qLsbSpec>;
#[doc = "DC_ACC2Q_LSB"]
pub mod dc_acc2q_lsb;
#[doc = "DC_ACC2Q_MSB (rw) register accessor: DC_ACC2Q_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc2q_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc2q_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc2q_msb`]
module"]
#[doc(alias = "DC_ACC2Q_MSB")]
pub type DcAcc2qMsb = crate::Reg<dc_acc2q_msb::DcAcc2qMsbSpec>;
#[doc = "DC_ACC2Q_MSB"]
pub mod dc_acc2q_msb;
#[doc = "DC_ACC3Q_LSB (rw) register accessor: DC_ACC3Q_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc3q_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc3q_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc3q_lsb`]
module"]
#[doc(alias = "DC_ACC3Q_LSB")]
pub type DcAcc3qLsb = crate::Reg<dc_acc3q_lsb::DcAcc3qLsbSpec>;
#[doc = "DC_ACC3Q_LSB"]
pub mod dc_acc3q_lsb;
#[doc = "DC_ACC3Q_MSB (rw) register accessor: DC_ACC3Q_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc3q_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc3q_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc3q_msb`]
module"]
#[doc(alias = "DC_ACC3Q_MSB")]
pub type DcAcc3qMsb = crate::Reg<dc_acc3q_msb::DcAcc3qMsbSpec>;
#[doc = "DC_ACC3Q_MSB"]
pub mod dc_acc3q_msb;
#[doc = "DC_ACC4Q_LSB (rw) register accessor: DC_ACC4Q_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc4q_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc4q_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc4q_lsb`]
module"]
#[doc(alias = "DC_ACC4Q_LSB")]
pub type DcAcc4qLsb = crate::Reg<dc_acc4q_lsb::DcAcc4qLsbSpec>;
#[doc = "DC_ACC4Q_LSB"]
pub mod dc_acc4q_lsb;
#[doc = "DC_ACC4Q_MSB (rw) register accessor: DC_ACC4Q_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc4q_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc4q_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc4q_msb`]
module"]
#[doc(alias = "DC_ACC4Q_MSB")]
pub type DcAcc4qMsb = crate::Reg<dc_acc4q_msb::DcAcc4qMsbSpec>;
#[doc = "DC_ACC4Q_MSB"]
pub mod dc_acc4q_msb;
#[doc = "DC_ACC5Q_LSB (rw) register accessor: DC_ACC5Q_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc5q_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc5q_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc5q_lsb`]
module"]
#[doc(alias = "DC_ACC5Q_LSB")]
pub type DcAcc5qLsb = crate::Reg<dc_acc5q_lsb::DcAcc5qLsbSpec>;
#[doc = "DC_ACC5Q_LSB"]
pub mod dc_acc5q_lsb;
#[doc = "DC_ACC5Q_MSB (rw) register accessor: DC_ACC5Q_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc5q_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc5q_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc5q_msb`]
module"]
#[doc(alias = "DC_ACC5Q_MSB")]
pub type DcAcc5qMsb = crate::Reg<dc_acc5q_msb::DcAcc5qMsbSpec>;
#[doc = "DC_ACC5Q_MSB"]
pub mod dc_acc5q_msb;
#[doc = "DC_ACC6Q_LSB (rw) register accessor: DC_ACC6Q_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc6q_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc6q_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc6q_lsb`]
module"]
#[doc(alias = "DC_ACC6Q_LSB")]
pub type DcAcc6qLsb = crate::Reg<dc_acc6q_lsb::DcAcc6qLsbSpec>;
#[doc = "DC_ACC6Q_LSB"]
pub mod dc_acc6q_lsb;
#[doc = "DC_ACC6Q_MSB (rw) register accessor: DC_ACC6Q_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc6q_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc6q_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc_acc6q_msb`]
module"]
#[doc(alias = "DC_ACC6Q_MSB")]
pub type DcAcc6qMsb = crate::Reg<dc_acc6q_msb::DcAcc6qMsbSpec>;
#[doc = "DC_ACC6Q_MSB"]
pub mod dc_acc6q_msb;
#[doc = "DCACC1_CLIP (rw) register accessor: DCACC1_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcacc1_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcacc1_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcacc1_clip`]
module"]
#[doc(alias = "DCACC1_CLIP")]
pub type Dcacc1Clip = crate::Reg<dcacc1_clip::Dcacc1ClipSpec>;
#[doc = "DCACC1_CLIP"]
pub mod dcacc1_clip;
#[doc = "DCACC2_CLIP (rw) register accessor: DCACC2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcacc2_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcacc2_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcacc2_clip`]
module"]
#[doc(alias = "DCACC2_CLIP")]
pub type Dcacc2Clip = crate::Reg<dcacc2_clip::Dcacc2ClipSpec>;
#[doc = "DCACC2_CLIP"]
pub mod dcacc2_clip;
#[doc = "DCACC3_CLIP (rw) register accessor: DCACC3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcacc3_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcacc3_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcacc3_clip`]
module"]
#[doc(alias = "DCACC3_CLIP")]
pub type Dcacc3Clip = crate::Reg<dcacc3_clip::Dcacc3ClipSpec>;
#[doc = "DCACC3_CLIP"]
pub mod dcacc3_clip;
#[doc = "DCACC4_CLIP (rw) register accessor: DCACC4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcacc4_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcacc4_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcacc4_clip`]
module"]
#[doc(alias = "DCACC4_CLIP")]
pub type Dcacc4Clip = crate::Reg<dcacc4_clip::Dcacc4ClipSpec>;
#[doc = "DCACC4_CLIP"]
pub mod dcacc4_clip;
#[doc = "DCACC5_CLIP (rw) register accessor: DCACC5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcacc5_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcacc5_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcacc5_clip`]
module"]
#[doc(alias = "DCACC5_CLIP")]
pub type Dcacc5Clip = crate::Reg<dcacc5_clip::Dcacc5ClipSpec>;
#[doc = "DCACC5_CLIP"]
pub mod dcacc5_clip;
#[doc = "DCACC6_CLIP (rw) register accessor: DCACC6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcacc6_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcacc6_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcacc6_clip`]
module"]
#[doc(alias = "DCACC6_CLIP")]
pub type Dcacc6Clip = crate::Reg<dcacc6_clip::Dcacc6ClipSpec>;
#[doc = "DCACC6_CLIP"]
pub mod dcacc6_clip;
#[doc = "DCEST1_CLIP (rw) register accessor: DCEST1_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest1_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest1_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest1_clip`]
module"]
#[doc(alias = "DCEST1_CLIP")]
pub type Dcest1Clip = crate::Reg<dcest1_clip::Dcest1ClipSpec>;
#[doc = "DCEST1_CLIP"]
pub mod dcest1_clip;
#[doc = "DCEST2_CLIP (rw) register accessor: DCEST2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest2_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest2_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest2_clip`]
module"]
#[doc(alias = "DCEST2_CLIP")]
pub type Dcest2Clip = crate::Reg<dcest2_clip::Dcest2ClipSpec>;
#[doc = "DCEST2_CLIP"]
pub mod dcest2_clip;
#[doc = "DCEST3_CLIP (rw) register accessor: DCEST3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest3_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest3_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest3_clip`]
module"]
#[doc(alias = "DCEST3_CLIP")]
pub type Dcest3Clip = crate::Reg<dcest3_clip::Dcest3ClipSpec>;
#[doc = "DCEST3_CLIP"]
pub mod dcest3_clip;
#[doc = "DCEST4_CLIP (rw) register accessor: DCEST4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest4_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest4_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest4_clip`]
module"]
#[doc(alias = "DCEST4_CLIP")]
pub type Dcest4Clip = crate::Reg<dcest4_clip::Dcest4ClipSpec>;
#[doc = "DCEST4_CLIP"]
pub mod dcest4_clip;
#[doc = "DCEST5_CLIP (rw) register accessor: DCEST5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest5_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest5_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest5_clip`]
module"]
#[doc(alias = "DCEST5_CLIP")]
pub type Dcest5Clip = crate::Reg<dcest5_clip::Dcest5ClipSpec>;
#[doc = "DCEST5_CLIP"]
pub mod dcest5_clip;
#[doc = "DCEST6_CLIP (rw) register accessor: DCEST6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest6_clip`]
module"]
#[doc(alias = "DCEST6_CLIP")]
pub type Dcest6Clip = crate::Reg<dcest6_clip::Dcest6ClipSpec>;
#[doc = "DCEST6_CLIP"]
pub mod dcest6_clip;
#[doc = "DCSUB1_CLIP (rw) register accessor: DCSUB1_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub1_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub1_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcsub1_clip`]
module"]
#[doc(alias = "DCSUB1_CLIP")]
pub type Dcsub1Clip = crate::Reg<dcsub1_clip::Dcsub1ClipSpec>;
#[doc = "DCSUB1_CLIP"]
pub mod dcsub1_clip;
#[doc = "DCSUB2_CLIP (rw) register accessor: DCSUB2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub2_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub2_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcsub2_clip`]
module"]
#[doc(alias = "DCSUB2_CLIP")]
pub type Dcsub2Clip = crate::Reg<dcsub2_clip::Dcsub2ClipSpec>;
#[doc = "DCSUB2_CLIP"]
pub mod dcsub2_clip;
#[doc = "DCSUB3_CLIP (rw) register accessor: DCSUB3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub3_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub3_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcsub3_clip`]
module"]
#[doc(alias = "DCSUB3_CLIP")]
pub type Dcsub3Clip = crate::Reg<dcsub3_clip::Dcsub3ClipSpec>;
#[doc = "DCSUB3_CLIP"]
pub mod dcsub3_clip;
#[doc = "DCSUB4_CLIP (rw) register accessor: DCSUB4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub4_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub4_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcsub4_clip`]
module"]
#[doc(alias = "DCSUB4_CLIP")]
pub type Dcsub4Clip = crate::Reg<dcsub4_clip::Dcsub4ClipSpec>;
#[doc = "DCSUB4_CLIP"]
pub mod dcsub4_clip;
#[doc = "DCSUB5_CLIP (rw) register accessor: DCSUB5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub5_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub5_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcsub5_clip`]
module"]
#[doc(alias = "DCSUB5_CLIP")]
pub type Dcsub5Clip = crate::Reg<dcsub5_clip::Dcsub5ClipSpec>;
#[doc = "DCSUB5_CLIP"]
pub mod dcsub5_clip;
#[doc = "DCSUB6_CLIP (rw) register accessor: DCSUB6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`dcsub6_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsub6_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcsub6_clip`]
module"]
#[doc(alias = "DCSUB6_CLIP")]
pub type Dcsub6Clip = crate::Reg<dcsub6_clip::Dcsub6ClipSpec>;
#[doc = "DCSUB6_CLIP"]
pub mod dcsub6_clip;
#[doc = "DCEST_SHIFT (rw) register accessor: DCEST_SHIFT\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest_shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest_shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest_shift`]
module"]
#[doc(alias = "DCEST_SHIFT")]
pub type DcestShift = crate::Reg<dcest_shift::DcestShiftSpec>;
#[doc = "DCEST_SHIFT"]
pub mod dcest_shift;
#[doc = "DCEST_SCALE (rw) register accessor: DCEST_SCALE\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest_scale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest_scale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest_scale`]
module"]
#[doc(alias = "DCEST_SCALE")]
pub type DcestScale = crate::Reg<dcest_scale::DcestScaleSpec>;
#[doc = "DCEST_SCALE"]
pub mod dcest_scale;
#[doc = "INTF_MAG_SCALE (rw) register accessor: INTF_MAG_SCALE\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mag_scale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mag_scale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mag_scale`]
module"]
#[doc(alias = "INTF_MAG_SCALE")]
pub type IntfMagScale = crate::Reg<intf_mag_scale::IntfMagScaleSpec>;
#[doc = "INTF_MAG_SCALE"]
pub mod intf_mag_scale;
#[doc = "INTF_MAG_SHIFT (rw) register accessor: INTF_MAG_SHIFT\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mag_shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mag_shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mag_shift`]
module"]
#[doc(alias = "INTF_MAG_SHIFT")]
pub type IntfMagShift = crate::Reg<intf_mag_shift::IntfMagShiftSpec>;
#[doc = "INTF_MAG_SHIFT"]
pub mod intf_mag_shift;
#[doc = "INTF_MAGDIFF_SCALE (rw) register accessor: INTF_MAGDIFF_SCALE\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiff_scale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiff_scale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiff_scale`]
module"]
#[doc(alias = "INTF_MAGDIFF_SCALE")]
pub type IntfMagdiffScale = crate::Reg<intf_magdiff_scale::IntfMagdiffScaleSpec>;
#[doc = "INTF_MAGDIFF_SCALE"]
pub mod intf_magdiff_scale;
#[doc = "INTF_MAGDIFF_SHIFT (rw) register accessor: INTF_MAGDIFF_SHIFT\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiff_shift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiff_shift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiff_shift`]
module"]
#[doc(alias = "INTF_MAGDIFF_SHIFT")]
pub type IntfMagdiffShift = crate::Reg<intf_magdiff_shift::IntfMagdiffShiftSpec>;
#[doc = "INTF_MAGDIFF_SHIFT"]
pub mod intf_magdiff_shift;
#[doc = "INTF_FRAME_ZEROCOUNT (rw) register accessor: INTF_FRAME_ZEROCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_frame_zerocount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_frame_zerocount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_frame_zerocount`]
module"]
#[doc(alias = "INTF_FRAME_ZEROCOUNT")]
pub type IntfFrameZerocount = crate::Reg<intf_frame_zerocount::IntfFrameZerocountSpec>;
#[doc = "INTF_FRAME_ZEROCOUNT"]
pub mod intf_frame_zerocount;
#[doc = "INTF_CHIRP_ZEROCOUNT (rw) register accessor: INTF_CHIRP_ZEROCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_chirp_zerocount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_chirp_zerocount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_chirp_zerocount`]
module"]
#[doc(alias = "INTF_CHIRP_ZEROCOUNT")]
pub type IntfChirpZerocount = crate::Reg<intf_chirp_zerocount::IntfChirpZerocountSpec>;
#[doc = "INTF_CHIRP_ZEROCOUNT"]
pub mod intf_chirp_zerocount;
#[doc = "INTF_MAGTHRESH1_SW (rw) register accessor: INTF_MAGTHRESH1_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh1_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh1_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh1_sw`]
module"]
#[doc(alias = "INTF_MAGTHRESH1_SW")]
pub type IntfMagthresh1Sw = crate::Reg<intf_magthresh1_sw::IntfMagthresh1SwSpec>;
#[doc = "INTF_MAGTHRESH1_SW"]
pub mod intf_magthresh1_sw;
#[doc = "INTF_MAGTHRESH2_SW (rw) register accessor: INTF_MAGTHRESH2_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh2_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh2_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh2_sw`]
module"]
#[doc(alias = "INTF_MAGTHRESH2_SW")]
pub type IntfMagthresh2Sw = crate::Reg<intf_magthresh2_sw::IntfMagthresh2SwSpec>;
#[doc = "INTF_MAGTHRESH2_SW"]
pub mod intf_magthresh2_sw;
#[doc = "INTF_MAGTHRESH3_SW (rw) register accessor: INTF_MAGTHRESH3_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh3_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh3_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh3_sw`]
module"]
#[doc(alias = "INTF_MAGTHRESH3_SW")]
pub type IntfMagthresh3Sw = crate::Reg<intf_magthresh3_sw::IntfMagthresh3SwSpec>;
#[doc = "INTF_MAGTHRESH3_SW"]
pub mod intf_magthresh3_sw;
#[doc = "INTF_MAGTHRESH4_SW (rw) register accessor: INTF_MAGTHRESH4_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh4_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh4_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh4_sw`]
module"]
#[doc(alias = "INTF_MAGTHRESH4_SW")]
pub type IntfMagthresh4Sw = crate::Reg<intf_magthresh4_sw::IntfMagthresh4SwSpec>;
#[doc = "INTF_MAGTHRESH4_SW"]
pub mod intf_magthresh4_sw;
#[doc = "INTF_MAGTHRESH5_SW (rw) register accessor: INTF_MAGTHRESH5_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh5_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh5_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh5_sw`]
module"]
#[doc(alias = "INTF_MAGTHRESH5_SW")]
pub type IntfMagthresh5Sw = crate::Reg<intf_magthresh5_sw::IntfMagthresh5SwSpec>;
#[doc = "INTF_MAGTHRESH5_SW"]
pub mod intf_magthresh5_sw;
#[doc = "INTF_MAGTHRESH6_SW (rw) register accessor: INTF_MAGTHRESH6_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh6_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh6_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh6_sw`]
module"]
#[doc(alias = "INTF_MAGTHRESH6_SW")]
pub type IntfMagthresh6Sw = crate::Reg<intf_magthresh6_sw::IntfMagthresh6SwSpec>;
#[doc = "INTF_MAGTHRESH6_SW"]
pub mod intf_magthresh6_sw;
#[doc = "INTF_MAGDIFFTHRESH1_SW (rw) register accessor: INTF_MAGDIFFTHRESH1_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh1_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh1_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh1_sw`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH1_SW")]
pub type IntfMagdiffthresh1Sw = crate::Reg<intf_magdiffthresh1_sw::IntfMagdiffthresh1SwSpec>;
#[doc = "INTF_MAGDIFFTHRESH1_SW"]
pub mod intf_magdiffthresh1_sw;
#[doc = "INTF_MAGDIFFTHRESH2_SW (rw) register accessor: INTF_MAGDIFFTHRESH2_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh2_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh2_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh2_sw`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH2_SW")]
pub type IntfMagdiffthresh2Sw = crate::Reg<intf_magdiffthresh2_sw::IntfMagdiffthresh2SwSpec>;
#[doc = "INTF_MAGDIFFTHRESH2_SW"]
pub mod intf_magdiffthresh2_sw;
#[doc = "INTF_MAGDIFFTHRESH3_SW (rw) register accessor: INTF_MAGDIFFTHRESH3_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh3_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh3_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh3_sw`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH3_SW")]
pub type IntfMagdiffthresh3Sw = crate::Reg<intf_magdiffthresh3_sw::IntfMagdiffthresh3SwSpec>;
#[doc = "INTF_MAGDIFFTHRESH3_SW"]
pub mod intf_magdiffthresh3_sw;
#[doc = "INTF_MAGDIFFTHRESH4_SW (rw) register accessor: INTF_MAGDIFFTHRESH4_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh4_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh4_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh4_sw`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH4_SW")]
pub type IntfMagdiffthresh4Sw = crate::Reg<intf_magdiffthresh4_sw::IntfMagdiffthresh4SwSpec>;
#[doc = "INTF_MAGDIFFTHRESH4_SW"]
pub mod intf_magdiffthresh4_sw;
#[doc = "INTF_MAGDIFFTHRESH5_SW (rw) register accessor: INTF_MAGDIFFTHRESH5_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh5_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh5_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh5_sw`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH5_SW")]
pub type IntfMagdiffthresh5Sw = crate::Reg<intf_magdiffthresh5_sw::IntfMagdiffthresh5SwSpec>;
#[doc = "INTF_MAGDIFFTHRESH5_SW"]
pub mod intf_magdiffthresh5_sw;
#[doc = "INTF_MAGDIFFTHRESH6_SW (rw) register accessor: INTF_MAGDIFFTHRESH6_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh6_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh6_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh6_sw`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH6_SW")]
pub type IntfMagdiffthresh6Sw = crate::Reg<intf_magdiffthresh6_sw::IntfMagdiffthresh6SwSpec>;
#[doc = "INTF_MAGDIFFTHRESH6_SW"]
pub mod intf_magdiffthresh6_sw;
#[doc = "INTF_MAGACC1_LSB (rw) register accessor: INTF_MAGACC1_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc1_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc1_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc1_lsb`]
module"]
#[doc(alias = "INTF_MAGACC1_LSB")]
pub type IntfMagacc1Lsb = crate::Reg<intf_magacc1_lsb::IntfMagacc1LsbSpec>;
#[doc = "INTF_MAGACC1_LSB"]
pub mod intf_magacc1_lsb;
#[doc = "INTF_MAGACC1_MSB (rw) register accessor: INTF_MAGACC1_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc1_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc1_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc1_msb`]
module"]
#[doc(alias = "INTF_MAGACC1_MSB")]
pub type IntfMagacc1Msb = crate::Reg<intf_magacc1_msb::IntfMagacc1MsbSpec>;
#[doc = "INTF_MAGACC1_MSB"]
pub mod intf_magacc1_msb;
#[doc = "INTF_MAGACC2_LSB (rw) register accessor: INTF_MAGACC2_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc2_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc2_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc2_lsb`]
module"]
#[doc(alias = "INTF_MAGACC2_LSB")]
pub type IntfMagacc2Lsb = crate::Reg<intf_magacc2_lsb::IntfMagacc2LsbSpec>;
#[doc = "INTF_MAGACC2_LSB"]
pub mod intf_magacc2_lsb;
#[doc = "INTF_MAGACC2_MSB (rw) register accessor: INTF_MAGACC2_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc2_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc2_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc2_msb`]
module"]
#[doc(alias = "INTF_MAGACC2_MSB")]
pub type IntfMagacc2Msb = crate::Reg<intf_magacc2_msb::IntfMagacc2MsbSpec>;
#[doc = "INTF_MAGACC2_MSB"]
pub mod intf_magacc2_msb;
#[doc = "INTF_MAGACC3_LSB (rw) register accessor: INTF_MAGACC3_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc3_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc3_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc3_lsb`]
module"]
#[doc(alias = "INTF_MAGACC3_LSB")]
pub type IntfMagacc3Lsb = crate::Reg<intf_magacc3_lsb::IntfMagacc3LsbSpec>;
#[doc = "INTF_MAGACC3_LSB"]
pub mod intf_magacc3_lsb;
#[doc = "INTF_MAGACC3_MSB (rw) register accessor: INTF_MAGACC3_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc3_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc3_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc3_msb`]
module"]
#[doc(alias = "INTF_MAGACC3_MSB")]
pub type IntfMagacc3Msb = crate::Reg<intf_magacc3_msb::IntfMagacc3MsbSpec>;
#[doc = "INTF_MAGACC3_MSB"]
pub mod intf_magacc3_msb;
#[doc = "INTF_MAGACC4_LSB (rw) register accessor: INTF_MAGACC4_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc4_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc4_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc4_lsb`]
module"]
#[doc(alias = "INTF_MAGACC4_LSB")]
pub type IntfMagacc4Lsb = crate::Reg<intf_magacc4_lsb::IntfMagacc4LsbSpec>;
#[doc = "INTF_MAGACC4_LSB"]
pub mod intf_magacc4_lsb;
#[doc = "INTF_MAGACC4_MSB (rw) register accessor: INTF_MAGACC4_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc4_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc4_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc4_msb`]
module"]
#[doc(alias = "INTF_MAGACC4_MSB")]
pub type IntfMagacc4Msb = crate::Reg<intf_magacc4_msb::IntfMagacc4MsbSpec>;
#[doc = "INTF_MAGACC4_MSB"]
pub mod intf_magacc4_msb;
#[doc = "INTF_MAGACC5_LSB (rw) register accessor: INTF_MAGACC5_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc5_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc5_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc5_lsb`]
module"]
#[doc(alias = "INTF_MAGACC5_LSB")]
pub type IntfMagacc5Lsb = crate::Reg<intf_magacc5_lsb::IntfMagacc5LsbSpec>;
#[doc = "INTF_MAGACC5_LSB"]
pub mod intf_magacc5_lsb;
#[doc = "INTF_MAGACC5_MSB (rw) register accessor: INTF_MAGACC5_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc5_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc5_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc5_msb`]
module"]
#[doc(alias = "INTF_MAGACC5_MSB")]
pub type IntfMagacc5Msb = crate::Reg<intf_magacc5_msb::IntfMagacc5MsbSpec>;
#[doc = "INTF_MAGACC5_MSB"]
pub mod intf_magacc5_msb;
#[doc = "INTF_MAGACC6_LSB (rw) register accessor: INTF_MAGACC6_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc6_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc6_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc6_lsb`]
module"]
#[doc(alias = "INTF_MAGACC6_LSB")]
pub type IntfMagacc6Lsb = crate::Reg<intf_magacc6_lsb::IntfMagacc6LsbSpec>;
#[doc = "INTF_MAGACC6_LSB"]
pub mod intf_magacc6_lsb;
#[doc = "INTF_MAGACC6_MSB (rw) register accessor: INTF_MAGACC6_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc6_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc6_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc6_msb`]
module"]
#[doc(alias = "INTF_MAGACC6_MSB")]
pub type IntfMagacc6Msb = crate::Reg<intf_magacc6_msb::IntfMagacc6MsbSpec>;
#[doc = "INTF_MAGACC6_MSB"]
pub mod intf_magacc6_msb;
#[doc = "INTF_MAGDIFFACC1_LSB (rw) register accessor: INTF_MAGDIFFACC1_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc1_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc1_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc1_lsb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC1_LSB")]
pub type IntfMagdiffacc1Lsb = crate::Reg<intf_magdiffacc1_lsb::IntfMagdiffacc1LsbSpec>;
#[doc = "INTF_MAGDIFFACC1_LSB"]
pub mod intf_magdiffacc1_lsb;
#[doc = "INTF_MAGDIFFACC1_MSB (rw) register accessor: INTF_MAGDIFFACC1_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc1_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc1_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc1_msb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC1_MSB")]
pub type IntfMagdiffacc1Msb = crate::Reg<intf_magdiffacc1_msb::IntfMagdiffacc1MsbSpec>;
#[doc = "INTF_MAGDIFFACC1_MSB"]
pub mod intf_magdiffacc1_msb;
#[doc = "INTF_MAGDIFFACC2_LSB (rw) register accessor: INTF_MAGDIFFACC2_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc2_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc2_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc2_lsb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC2_LSB")]
pub type IntfMagdiffacc2Lsb = crate::Reg<intf_magdiffacc2_lsb::IntfMagdiffacc2LsbSpec>;
#[doc = "INTF_MAGDIFFACC2_LSB"]
pub mod intf_magdiffacc2_lsb;
#[doc = "INTF_MAGDIFFACC2_MSB (rw) register accessor: INTF_MAGDIFFACC2_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc2_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc2_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc2_msb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC2_MSB")]
pub type IntfMagdiffacc2Msb = crate::Reg<intf_magdiffacc2_msb::IntfMagdiffacc2MsbSpec>;
#[doc = "INTF_MAGDIFFACC2_MSB"]
pub mod intf_magdiffacc2_msb;
#[doc = "INTF_MAGDIFFACC3_LSB (rw) register accessor: INTF_MAGDIFFACC3_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc3_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc3_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc3_lsb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC3_LSB")]
pub type IntfMagdiffacc3Lsb = crate::Reg<intf_magdiffacc3_lsb::IntfMagdiffacc3LsbSpec>;
#[doc = "INTF_MAGDIFFACC3_LSB"]
pub mod intf_magdiffacc3_lsb;
#[doc = "INTF_MAGDIFFACC3_MSB (rw) register accessor: INTF_MAGDIFFACC3_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc3_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc3_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc3_msb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC3_MSB")]
pub type IntfMagdiffacc3Msb = crate::Reg<intf_magdiffacc3_msb::IntfMagdiffacc3MsbSpec>;
#[doc = "INTF_MAGDIFFACC3_MSB"]
pub mod intf_magdiffacc3_msb;
#[doc = "INTF_MAGDIFFACC4_LSB (rw) register accessor: INTF_MAGDIFFACC4_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc4_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc4_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc4_lsb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC4_LSB")]
pub type IntfMagdiffacc4Lsb = crate::Reg<intf_magdiffacc4_lsb::IntfMagdiffacc4LsbSpec>;
#[doc = "INTF_MAGDIFFACC4_LSB"]
pub mod intf_magdiffacc4_lsb;
#[doc = "INTF_MAGDIFFACC4_MSB (rw) register accessor: INTF_MAGDIFFACC4_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc4_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc4_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc4_msb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC4_MSB")]
pub type IntfMagdiffacc4Msb = crate::Reg<intf_magdiffacc4_msb::IntfMagdiffacc4MsbSpec>;
#[doc = "INTF_MAGDIFFACC4_MSB"]
pub mod intf_magdiffacc4_msb;
#[doc = "INTF_MAGDIFFACC5_LSB (rw) register accessor: INTF_MAGDIFFACC5_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc5_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc5_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc5_lsb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC5_LSB")]
pub type IntfMagdiffacc5Lsb = crate::Reg<intf_magdiffacc5_lsb::IntfMagdiffacc5LsbSpec>;
#[doc = "INTF_MAGDIFFACC5_LSB"]
pub mod intf_magdiffacc5_lsb;
#[doc = "INTF_MAGDIFFACC5_MSB (rw) register accessor: INTF_MAGDIFFACC5_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc5_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc5_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc5_msb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC5_MSB")]
pub type IntfMagdiffacc5Msb = crate::Reg<intf_magdiffacc5_msb::IntfMagdiffacc5MsbSpec>;
#[doc = "INTF_MAGDIFFACC5_MSB"]
pub mod intf_magdiffacc5_msb;
#[doc = "INTF_MAGDIFFACC6_LSB (rw) register accessor: INTF_MAGDIFFACC6_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc6_lsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc6_lsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc6_lsb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC6_LSB")]
pub type IntfMagdiffacc6Lsb = crate::Reg<intf_magdiffacc6_lsb::IntfMagdiffacc6LsbSpec>;
#[doc = "INTF_MAGDIFFACC6_LSB"]
pub mod intf_magdiffacc6_lsb;
#[doc = "INTF_MAGDIFFACC6_MSB (rw) register accessor: INTF_MAGDIFFACC6_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc6_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc6_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc6_msb`]
module"]
#[doc(alias = "INTF_MAGDIFFACC6_MSB")]
pub type IntfMagdiffacc6Msb = crate::Reg<intf_magdiffacc6_msb::IntfMagdiffacc6MsbSpec>;
#[doc = "INTF_MAGDIFFACC6_MSB"]
pub mod intf_magdiffacc6_msb;
#[doc = "INTF_MAGACC1_CLIP (rw) register accessor: INTF_MAGACC1_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc1_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc1_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc1_clip`]
module"]
#[doc(alias = "INTF_MAGACC1_CLIP")]
pub type IntfMagacc1Clip = crate::Reg<intf_magacc1_clip::IntfMagacc1ClipSpec>;
#[doc = "INTF_MAGACC1_CLIP"]
pub mod intf_magacc1_clip;
#[doc = "INTF_MAGACC2_CLIP (rw) register accessor: INTF_MAGACC2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc2_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc2_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc2_clip`]
module"]
#[doc(alias = "INTF_MAGACC2_CLIP")]
pub type IntfMagacc2Clip = crate::Reg<intf_magacc2_clip::IntfMagacc2ClipSpec>;
#[doc = "INTF_MAGACC2_CLIP"]
pub mod intf_magacc2_clip;
#[doc = "INTF_MAGACC3_CLIP (rw) register accessor: INTF_MAGACC3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc3_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc3_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc3_clip`]
module"]
#[doc(alias = "INTF_MAGACC3_CLIP")]
pub type IntfMagacc3Clip = crate::Reg<intf_magacc3_clip::IntfMagacc3ClipSpec>;
#[doc = "INTF_MAGACC3_CLIP"]
pub mod intf_magacc3_clip;
#[doc = "INTF_MAGACC4_CLIP (rw) register accessor: INTF_MAGACC4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc4_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc4_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc4_clip`]
module"]
#[doc(alias = "INTF_MAGACC4_CLIP")]
pub type IntfMagacc4Clip = crate::Reg<intf_magacc4_clip::IntfMagacc4ClipSpec>;
#[doc = "INTF_MAGACC4_CLIP"]
pub mod intf_magacc4_clip;
#[doc = "INTF_MAGACC5_CLIP (rw) register accessor: INTF_MAGACC5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc5_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc5_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc5_clip`]
module"]
#[doc(alias = "INTF_MAGACC5_CLIP")]
pub type IntfMagacc5Clip = crate::Reg<intf_magacc5_clip::IntfMagacc5ClipSpec>;
#[doc = "INTF_MAGACC5_CLIP"]
pub mod intf_magacc5_clip;
#[doc = "INTF_MAGACC6_CLIP (rw) register accessor: INTF_MAGACC6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc6_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc6_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magacc6_clip`]
module"]
#[doc(alias = "INTF_MAGACC6_CLIP")]
pub type IntfMagacc6Clip = crate::Reg<intf_magacc6_clip::IntfMagacc6ClipSpec>;
#[doc = "INTF_MAGACC6_CLIP"]
pub mod intf_magacc6_clip;
#[doc = "INTF_MAGDIFFACC1_CLIP (rw) register accessor: INTF_MAGDIFFACC1_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc1_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc1_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc1_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFACC1_CLIP")]
pub type IntfMagdiffacc1Clip = crate::Reg<intf_magdiffacc1_clip::IntfMagdiffacc1ClipSpec>;
#[doc = "INTF_MAGDIFFACC1_CLIP"]
pub mod intf_magdiffacc1_clip;
#[doc = "INTF_MAGDIFFACC2_CLIP (rw) register accessor: INTF_MAGDIFFACC2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc2_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc2_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc2_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFACC2_CLIP")]
pub type IntfMagdiffacc2Clip = crate::Reg<intf_magdiffacc2_clip::IntfMagdiffacc2ClipSpec>;
#[doc = "INTF_MAGDIFFACC2_CLIP"]
pub mod intf_magdiffacc2_clip;
#[doc = "INTF_MAGDIFFACC3_CLIP (rw) register accessor: INTF_MAGDIFFACC3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc3_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc3_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc3_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFACC3_CLIP")]
pub type IntfMagdiffacc3Clip = crate::Reg<intf_magdiffacc3_clip::IntfMagdiffacc3ClipSpec>;
#[doc = "INTF_MAGDIFFACC3_CLIP"]
pub mod intf_magdiffacc3_clip;
#[doc = "INTF_MAGDIFFACC4_CLIP (rw) register accessor: INTF_MAGDIFFACC4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc4_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc4_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc4_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFACC4_CLIP")]
pub type IntfMagdiffacc4Clip = crate::Reg<intf_magdiffacc4_clip::IntfMagdiffacc4ClipSpec>;
#[doc = "INTF_MAGDIFFACC4_CLIP"]
pub mod intf_magdiffacc4_clip;
#[doc = "INTF_MAGDIFFACC5_CLIP (rw) register accessor: INTF_MAGDIFFACC5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc5_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc5_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc5_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFACC5_CLIP")]
pub type IntfMagdiffacc5Clip = crate::Reg<intf_magdiffacc5_clip::IntfMagdiffacc5ClipSpec>;
#[doc = "INTF_MAGDIFFACC5_CLIP"]
pub mod intf_magdiffacc5_clip;
#[doc = "INTF_MAGDIFFACC6_CLIP (rw) register accessor: INTF_MAGDIFFACC6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc6_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc6_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffacc6_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFACC6_CLIP")]
pub type IntfMagdiffacc6Clip = crate::Reg<intf_magdiffacc6_clip::IntfMagdiffacc6ClipSpec>;
#[doc = "INTF_MAGDIFFACC6_CLIP"]
pub mod intf_magdiffacc6_clip;
#[doc = "INTF_MAGTHRESH1 (rw) register accessor: INTF_MAGTHRESH1\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh1`]
module"]
#[doc(alias = "INTF_MAGTHRESH1")]
pub type IntfMagthresh1 = crate::Reg<intf_magthresh1::IntfMagthresh1Spec>;
#[doc = "INTF_MAGTHRESH1"]
pub mod intf_magthresh1;
#[doc = "INTF_MAGTHRESH2 (rw) register accessor: INTF_MAGTHRESH2\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh2`]
module"]
#[doc(alias = "INTF_MAGTHRESH2")]
pub type IntfMagthresh2 = crate::Reg<intf_magthresh2::IntfMagthresh2Spec>;
#[doc = "INTF_MAGTHRESH2"]
pub mod intf_magthresh2;
#[doc = "INTF_MAGTHRESH3 (rw) register accessor: INTF_MAGTHRESH3\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh3`]
module"]
#[doc(alias = "INTF_MAGTHRESH3")]
pub type IntfMagthresh3 = crate::Reg<intf_magthresh3::IntfMagthresh3Spec>;
#[doc = "INTF_MAGTHRESH3"]
pub mod intf_magthresh3;
#[doc = "INTF_MAGTHRESH4 (rw) register accessor: INTF_MAGTHRESH4\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh4`]
module"]
#[doc(alias = "INTF_MAGTHRESH4")]
pub type IntfMagthresh4 = crate::Reg<intf_magthresh4::IntfMagthresh4Spec>;
#[doc = "INTF_MAGTHRESH4"]
pub mod intf_magthresh4;
#[doc = "INTF_MAGTHRESH5 (rw) register accessor: INTF_MAGTHRESH5\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh5`]
module"]
#[doc(alias = "INTF_MAGTHRESH5")]
pub type IntfMagthresh5 = crate::Reg<intf_magthresh5::IntfMagthresh5Spec>;
#[doc = "INTF_MAGTHRESH5"]
pub mod intf_magthresh5;
#[doc = "INTF_MAGTHRESH6 (rw) register accessor: INTF_MAGTHRESH6\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh6`]
module"]
#[doc(alias = "INTF_MAGTHRESH6")]
pub type IntfMagthresh6 = crate::Reg<intf_magthresh6::IntfMagthresh6Spec>;
#[doc = "INTF_MAGTHRESH6"]
pub mod intf_magthresh6;
#[doc = "INTF_MAGDIFFTHRESH1 (rw) register accessor: INTF_MAGDIFFTHRESH1\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh1`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH1")]
pub type IntfMagdiffthresh1 = crate::Reg<intf_magdiffthresh1::IntfMagdiffthresh1Spec>;
#[doc = "INTF_MAGDIFFTHRESH1"]
pub mod intf_magdiffthresh1;
#[doc = "INTF_MAGDIFFTHRESH2 (rw) register accessor: INTF_MAGDIFFTHRESH2\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh2`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH2")]
pub type IntfMagdiffthresh2 = crate::Reg<intf_magdiffthresh2::IntfMagdiffthresh2Spec>;
#[doc = "INTF_MAGDIFFTHRESH2"]
pub mod intf_magdiffthresh2;
#[doc = "INTF_MAGDIFFTHRESH3 (rw) register accessor: INTF_MAGDIFFTHRESH3\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh3`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH3")]
pub type IntfMagdiffthresh3 = crate::Reg<intf_magdiffthresh3::IntfMagdiffthresh3Spec>;
#[doc = "INTF_MAGDIFFTHRESH3"]
pub mod intf_magdiffthresh3;
#[doc = "INTF_MAGDIFFTHRESH4 (rw) register accessor: INTF_MAGDIFFTHRESH4\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh4`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH4")]
pub type IntfMagdiffthresh4 = crate::Reg<intf_magdiffthresh4::IntfMagdiffthresh4Spec>;
#[doc = "INTF_MAGDIFFTHRESH4"]
pub mod intf_magdiffthresh4;
#[doc = "INTF_MAGDIFFTHRESH5 (rw) register accessor: INTF_MAGDIFFTHRESH5\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh5`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH5")]
pub type IntfMagdiffthresh5 = crate::Reg<intf_magdiffthresh5::IntfMagdiffthresh5Spec>;
#[doc = "INTF_MAGDIFFTHRESH5"]
pub mod intf_magdiffthresh5;
#[doc = "INTF_MAGDIFFTHRESH6 (rw) register accessor: INTF_MAGDIFFTHRESH6\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh6`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH6")]
pub type IntfMagdiffthresh6 = crate::Reg<intf_magdiffthresh6::IntfMagdiffthresh6Spec>;
#[doc = "INTF_MAGDIFFTHRESH6"]
pub mod intf_magdiffthresh6;
#[doc = "INTF_SUMMAGTHRESH (rw) register accessor: INTF_SUMMAGTHRESH\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_summagthresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_summagthresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_summagthresh`]
module"]
#[doc(alias = "INTF_SUMMAGTHRESH")]
pub type IntfSummagthresh = crate::Reg<intf_summagthresh::IntfSummagthreshSpec>;
#[doc = "INTF_SUMMAGTHRESH"]
pub mod intf_summagthresh;
#[doc = "INTF_SUMMAGDIFFTHRESH (rw) register accessor: INTF_SUMMAGDIFFTHRESH\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_summagdiffthresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_summagdiffthresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_summagdiffthresh`]
module"]
#[doc(alias = "INTF_SUMMAGDIFFTHRESH")]
pub type IntfSummagdiffthresh = crate::Reg<intf_summagdiffthresh::IntfSummagdiffthreshSpec>;
#[doc = "INTF_SUMMAGDIFFTHRESH"]
pub mod intf_summagdiffthresh;
#[doc = "INTF_SUMMAGTHRESH_CLIP (rw) register accessor: INTF_SUMMAGTHRESH_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_summagthresh_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_summagthresh_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_summagthresh_clip`]
module"]
#[doc(alias = "INTF_SUMMAGTHRESH_CLIP")]
pub type IntfSummagthreshClip = crate::Reg<intf_summagthresh_clip::IntfSummagthreshClipSpec>;
#[doc = "INTF_SUMMAGTHRESH_CLIP"]
pub mod intf_summagthresh_clip;
#[doc = "INTF_SUMMAGDIFFTHRESH_CLIP (rw) register accessor: INTF_SUMMAGDIFFTHRESH_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_summagdiffthresh_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_summagdiffthresh_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_summagdiffthresh_clip`]
module"]
#[doc(alias = "INTF_SUMMAGDIFFTHRESH_CLIP")]
pub type IntfSummagdiffthreshClip =
    crate::Reg<intf_summagdiffthresh_clip::IntfSummagdiffthreshClipSpec>;
#[doc = "INTF_SUMMAGDIFFTHRESH_CLIP"]
pub mod intf_summagdiffthresh_clip;
#[doc = "CMULTSCALE1I (rw) register accessor: CMULTSCALE1I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale1i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale1i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale1i`]
module"]
#[doc(alias = "CMULTSCALE1I")]
pub type Cmultscale1i = crate::Reg<cmultscale1i::Cmultscale1iSpec>;
#[doc = "CMULTSCALE1I"]
pub mod cmultscale1i;
#[doc = "CMULTSCALE2I (rw) register accessor: CMULTSCALE2I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale2i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale2i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale2i`]
module"]
#[doc(alias = "CMULTSCALE2I")]
pub type Cmultscale2i = crate::Reg<cmultscale2i::Cmultscale2iSpec>;
#[doc = "CMULTSCALE2I"]
pub mod cmultscale2i;
#[doc = "CMULTSCALE3I (rw) register accessor: CMULTSCALE3I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale3i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale3i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale3i`]
module"]
#[doc(alias = "CMULTSCALE3I")]
pub type Cmultscale3i = crate::Reg<cmultscale3i::Cmultscale3iSpec>;
#[doc = "CMULTSCALE3I"]
pub mod cmultscale3i;
#[doc = "CMULTSCALE4I (rw) register accessor: CMULTSCALE4I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale4i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale4i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale4i`]
module"]
#[doc(alias = "CMULTSCALE4I")]
pub type Cmultscale4i = crate::Reg<cmultscale4i::Cmultscale4iSpec>;
#[doc = "CMULTSCALE4I"]
pub mod cmultscale4i;
#[doc = "CMULTSCALE5I (rw) register accessor: CMULTSCALE5I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale5i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale5i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale5i`]
module"]
#[doc(alias = "CMULTSCALE5I")]
pub type Cmultscale5i = crate::Reg<cmultscale5i::Cmultscale5iSpec>;
#[doc = "CMULTSCALE5I"]
pub mod cmultscale5i;
#[doc = "CMULTSCALE6I (rw) register accessor: CMULTSCALE6I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale6i::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale6i::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale6i`]
module"]
#[doc(alias = "CMULTSCALE6I")]
pub type Cmultscale6i = crate::Reg<cmultscale6i::Cmultscale6iSpec>;
#[doc = "CMULTSCALE6I"]
pub mod cmultscale6i;
#[doc = "CMULTSCALE1Q (rw) register accessor: CMULTSCALE1Q\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale1q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale1q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale1q`]
module"]
#[doc(alias = "CMULTSCALE1Q")]
pub type Cmultscale1q = crate::Reg<cmultscale1q::Cmultscale1qSpec>;
#[doc = "CMULTSCALE1Q"]
pub mod cmultscale1q;
#[doc = "CMULTSCALE2Q (rw) register accessor: CMULTSCALE2Q\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale2q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale2q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale2q`]
module"]
#[doc(alias = "CMULTSCALE2Q")]
pub type Cmultscale2q = crate::Reg<cmultscale2q::Cmultscale2qSpec>;
#[doc = "CMULTSCALE2Q"]
pub mod cmultscale2q;
#[doc = "CMULTSCALE3Q (rw) register accessor: CMULTSCALE3Q\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale3q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale3q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale3q`]
module"]
#[doc(alias = "CMULTSCALE3Q")]
pub type Cmultscale3q = crate::Reg<cmultscale3q::Cmultscale3qSpec>;
#[doc = "CMULTSCALE3Q"]
pub mod cmultscale3q;
#[doc = "CMULTSCALE4Q (rw) register accessor: CMULTSCALE4Q\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale4q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale4q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale4q`]
module"]
#[doc(alias = "CMULTSCALE4Q")]
pub type Cmultscale4q = crate::Reg<cmultscale4q::Cmultscale4qSpec>;
#[doc = "CMULTSCALE4Q"]
pub mod cmultscale4q;
#[doc = "CMULTSCALE5Q (rw) register accessor: CMULTSCALE5Q\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale5q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale5q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale5q`]
module"]
#[doc(alias = "CMULTSCALE5Q")]
pub type Cmultscale5q = crate::Reg<cmultscale5q::Cmultscale5qSpec>;
#[doc = "CMULTSCALE5Q"]
pub mod cmultscale5q;
#[doc = "CMULTSCALE6Q (rw) register accessor: CMULTSCALE6Q\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale6q::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale6q::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmultscale6q`]
module"]
#[doc(alias = "CMULTSCALE6Q")]
pub type Cmultscale6q = crate::Reg<cmultscale6q::Cmultscale6qSpec>;
#[doc = "CMULTSCALE6Q"]
pub mod cmultscale6q;
#[doc = "CLR_MISC_CLIP (rw) register accessor: CLR_MISC_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_misc_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_misc_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_misc_clip`]
module"]
#[doc(alias = "CLR_MISC_CLIP")]
pub type ClrMiscClip = crate::Reg<clr_misc_clip::ClrMiscClipSpec>;
#[doc = "CLR_MISC_CLIP"]
pub mod clr_misc_clip;
#[doc = "FFTINTMEMADDR (rw) register accessor: FFTINTMEMADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`fftintmemaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftintmemaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fftintmemaddr`]
module"]
#[doc(alias = "FFTINTMEMADDR")]
pub type Fftintmemaddr = crate::Reg<fftintmemaddr::FftintmemaddrSpec>;
#[doc = "FFTINTMEMADDR"]
pub mod fftintmemaddr;
#[doc = "INTF_STATS_RESET_SW (rw) register accessor: INTF_STATS_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_stats_reset_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_stats_reset_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_stats_reset_sw`]
module"]
#[doc(alias = "INTF_STATS_RESET_SW")]
pub type IntfStatsResetSw = crate::Reg<intf_stats_reset_sw::IntfStatsResetSwSpec>;
#[doc = "INTF_STATS_RESET_SW"]
pub mod intf_stats_reset_sw;
#[doc = "DCEST_RESET_SW (rw) register accessor: DCEST_RESET_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest_reset_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest_reset_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcest_reset_sw`]
module"]
#[doc(alias = "DCEST_RESET_SW")]
pub type DcestResetSw = crate::Reg<dcest_reset_sw::DcestResetSwSpec>;
#[doc = "DCEST_RESET_SW"]
pub mod dcest_reset_sw;
#[doc = "IP_OP_FORMATTER_CLIP_STATUS (rw) register accessor: IP_OP_FORMATTER_CLIP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_op_formatter_clip_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ip_op_formatter_clip_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ip_op_formatter_clip_status`]
module"]
#[doc(alias = "IP_OP_FORMATTER_CLIP_STATUS")]
pub type IpOpFormatterClipStatus =
    crate::Reg<ip_op_formatter_clip_status::IpOpFormatterClipStatusSpec>;
#[doc = "IP_OP_FORMATTER_CLIP_STATUS"]
pub mod ip_op_formatter_clip_status;
#[doc = "INTF_MAGTHRESH1_CLIP (rw) register accessor: INTF_MAGTHRESH1_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh1_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh1_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh1_clip`]
module"]
#[doc(alias = "INTF_MAGTHRESH1_CLIP")]
pub type IntfMagthresh1Clip = crate::Reg<intf_magthresh1_clip::IntfMagthresh1ClipSpec>;
#[doc = "INTF_MAGTHRESH1_CLIP"]
pub mod intf_magthresh1_clip;
#[doc = "INTF_MAGTHRESH2_CLIP (rw) register accessor: INTF_MAGTHRESH2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh2_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh2_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh2_clip`]
module"]
#[doc(alias = "INTF_MAGTHRESH2_CLIP")]
pub type IntfMagthresh2Clip = crate::Reg<intf_magthresh2_clip::IntfMagthresh2ClipSpec>;
#[doc = "INTF_MAGTHRESH2_CLIP"]
pub mod intf_magthresh2_clip;
#[doc = "INTF_MAGTHRESH3_CLIP (rw) register accessor: INTF_MAGTHRESH3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh3_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh3_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh3_clip`]
module"]
#[doc(alias = "INTF_MAGTHRESH3_CLIP")]
pub type IntfMagthresh3Clip = crate::Reg<intf_magthresh3_clip::IntfMagthresh3ClipSpec>;
#[doc = "INTF_MAGTHRESH3_CLIP"]
pub mod intf_magthresh3_clip;
#[doc = "INTF_MAGTHRESH4_CLIP (rw) register accessor: INTF_MAGTHRESH4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh4_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh4_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh4_clip`]
module"]
#[doc(alias = "INTF_MAGTHRESH4_CLIP")]
pub type IntfMagthresh4Clip = crate::Reg<intf_magthresh4_clip::IntfMagthresh4ClipSpec>;
#[doc = "INTF_MAGTHRESH4_CLIP"]
pub mod intf_magthresh4_clip;
#[doc = "INTF_MAGTHRESH5_CLIP (rw) register accessor: INTF_MAGTHRESH5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh5_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh5_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh5_clip`]
module"]
#[doc(alias = "INTF_MAGTHRESH5_CLIP")]
pub type IntfMagthresh5Clip = crate::Reg<intf_magthresh5_clip::IntfMagthresh5ClipSpec>;
#[doc = "INTF_MAGTHRESH5_CLIP"]
pub mod intf_magthresh5_clip;
#[doc = "INTF_MAGTHRESH6_CLIP (rw) register accessor: INTF_MAGTHRESH6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh6_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh6_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magthresh6_clip`]
module"]
#[doc(alias = "INTF_MAGTHRESH6_CLIP")]
pub type IntfMagthresh6Clip = crate::Reg<intf_magthresh6_clip::IntfMagthresh6ClipSpec>;
#[doc = "INTF_MAGTHRESH6_CLIP"]
pub mod intf_magthresh6_clip;
#[doc = "INTF_MAGDIFFTHRESH1_CLIP (rw) register accessor: INTF_MAGDIFFTHRESH1_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh1_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh1_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh1_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH1_CLIP")]
pub type IntfMagdiffthresh1Clip = crate::Reg<intf_magdiffthresh1_clip::IntfMagdiffthresh1ClipSpec>;
#[doc = "INTF_MAGDIFFTHRESH1_CLIP"]
pub mod intf_magdiffthresh1_clip;
#[doc = "INTF_MAGDIFFTHRESH2_CLIP (rw) register accessor: INTF_MAGDIFFTHRESH2_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh2_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh2_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh2_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH2_CLIP")]
pub type IntfMagdiffthresh2Clip = crate::Reg<intf_magdiffthresh2_clip::IntfMagdiffthresh2ClipSpec>;
#[doc = "INTF_MAGDIFFTHRESH2_CLIP"]
pub mod intf_magdiffthresh2_clip;
#[doc = "INTF_MAGDIFFTHRESH3_CLIP (rw) register accessor: INTF_MAGDIFFTHRESH3_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh3_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh3_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh3_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH3_CLIP")]
pub type IntfMagdiffthresh3Clip = crate::Reg<intf_magdiffthresh3_clip::IntfMagdiffthresh3ClipSpec>;
#[doc = "INTF_MAGDIFFTHRESH3_CLIP"]
pub mod intf_magdiffthresh3_clip;
#[doc = "INTF_MAGDIFFTHRESH4_CLIP (rw) register accessor: INTF_MAGDIFFTHRESH4_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh4_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh4_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh4_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH4_CLIP")]
pub type IntfMagdiffthresh4Clip = crate::Reg<intf_magdiffthresh4_clip::IntfMagdiffthresh4ClipSpec>;
#[doc = "INTF_MAGDIFFTHRESH4_CLIP"]
pub mod intf_magdiffthresh4_clip;
#[doc = "INTF_MAGDIFFTHRESH5_CLIP (rw) register accessor: INTF_MAGDIFFTHRESH5_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh5_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh5_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh5_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH5_CLIP")]
pub type IntfMagdiffthresh5Clip = crate::Reg<intf_magdiffthresh5_clip::IntfMagdiffthresh5ClipSpec>;
#[doc = "INTF_MAGDIFFTHRESH5_CLIP"]
pub mod intf_magdiffthresh5_clip;
#[doc = "INTF_MAGDIFFTHRESH6_CLIP (rw) register accessor: INTF_MAGDIFFTHRESH6_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh6_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh6_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_magdiffthresh6_clip`]
module"]
#[doc(alias = "INTF_MAGDIFFTHRESH6_CLIP")]
pub type IntfMagdiffthresh6Clip = crate::Reg<intf_magdiffthresh6_clip::IntfMagdiffthresh6ClipSpec>;
#[doc = "INTF_MAGDIFFTHRESH6_CLIP"]
pub mod intf_magdiffthresh6_clip;
#[doc = "HWA_SAFETY_ERR_MASK (rw) register accessor: HWA_SAFETY_ERR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_err_mask`]
module"]
#[doc(alias = "HWA_SAFETY_ERR_MASK")]
pub type HwaSafetyErrMask = crate::Reg<hwa_safety_err_mask::HwaSafetyErrMaskSpec>;
#[doc = "HWA_SAFETY_ERR_MASK"]
pub mod hwa_safety_err_mask;
#[doc = "HWA_SAFETY_ERR_STATUS (rw) register accessor: HWA_SAFETY_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_err_status`]
module"]
#[doc(alias = "HWA_SAFETY_ERR_STATUS")]
pub type HwaSafetyErrStatus = crate::Reg<hwa_safety_err_status::HwaSafetyErrStatusSpec>;
#[doc = "HWA_SAFETY_ERR_STATUS"]
pub mod hwa_safety_err_status;
#[doc = "HWA_SAFETY_ERR_STATUS_RAW (rw) register accessor: HWA_SAFETY_ERR_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_err_status_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_err_status_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwa_safety_err_status_raw`]
module"]
#[doc(alias = "HWA_SAFETY_ERR_STATUS_RAW")]
pub type HwaSafetyErrStatusRaw = crate::Reg<hwa_safety_err_status_raw::HwaSafetyErrStatusRawSpec>;
#[doc = "HWA_SAFETY_ERR_STATUS_RAW"]
pub mod hwa_safety_err_status_raw;
