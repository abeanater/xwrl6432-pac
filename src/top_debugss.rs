#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    onemcu_apb_base: OnemcuApbBase,
    _reserved1: [u8; 0x0ff8],
    onemcu_apb_base_end: OnemcuApbBaseEnd,
    onemcu_cti_control: OnemcuCtiControl,
    _reserved3: [u8; 0x0c],
    onemcu_cti_intack: OnemcuCtiIntack,
    onemcu_cti_appset: OnemcuCtiAppset,
    onemcu_cti_appclear: OnemcuCtiAppclear,
    onemcu_cti_apppulse: OnemcuCtiApppulse,
    onemcu_cti_inen0: OnemcuCtiInen0,
    onemcu_cti_inen1: OnemcuCtiInen1,
    onemcu_cti_inen2: OnemcuCtiInen2,
    onemcu_cti_inen3: OnemcuCtiInen3,
    onemcu_cti_inen4: OnemcuCtiInen4,
    onemcu_cti_inen5: OnemcuCtiInen5,
    onemcu_cti_inen6: OnemcuCtiInen6,
    onemcu_cti_inen7: OnemcuCtiInen7,
    _reserved15: [u8; 0x60],
    onemcu_cti_outen0: OnemcuCtiOuten0,
    onemcu_cti_outen1: OnemcuCtiOuten1,
    onemcu_cti_outen2: OnemcuCtiOuten2,
    onemcu_cti_outen3: OnemcuCtiOuten3,
    onemcu_cti_outen4: OnemcuCtiOuten4,
    onemcu_cti_outen5: OnemcuCtiOuten5,
    onemcu_cti_outen6: OnemcuCtiOuten6,
    onemcu_cti_outen7: OnemcuCtiOuten7,
    _reserved23: [u8; 0x70],
    onemcu_cti_triginstatus: OnemcuCtiTriginstatus,
    onemcu_cti_trigoutstatus: OnemcuCtiTrigoutstatus,
    onemcu_cti_chinstatus: OnemcuCtiChinstatus,
    onemcu_cti_choutstatus: OnemcuCtiChoutstatus,
    onemcu_cti_gate: OnemcuCtiGate,
    onemcu_cti_asicctl: OnemcuCtiAsicctl,
    _reserved29: [u8; 0x0d94],
    onemcu_cti_itchinack: OnemcuCtiItchinack,
    onemcu_cti_ittriginack: OnemcuCtiIttriginack,
    onemcu_cti_itchout: OnemcuCtiItchout,
    onemcu_cti_ittrigout: OnemcuCtiIttrigout,
    onemcu_cti_itchoutack: OnemcuCtiItchoutack,
    onemcu_cti_ittrigoutack: OnemcuCtiIttrigoutack,
    onemcu_cti_itchin: OnemcuCtiItchin,
    onemcu_cti_ittrigin: OnemcuCtiIttrigin,
    _reserved37: [u8; 0x04],
    onemcu_cti_itctrl: OnemcuCtiItctrl,
    _reserved38: [u8; 0x9c],
    onemcu_cti_claim_tag_set: OnemcuCtiClaimTagSet,
    onemcu_cti_claim_tag_clear: OnemcuCtiClaimTagClear,
    _reserved40: [u8; 0x08],
    onemcu_cti_lock_access_register: OnemcuCtiLockAccessRegister,
    onemcu_cti_lock_status_register: OnemcuCtiLockStatusRegister,
    onemcu_cti_authentication_status: OnemcuCtiAuthenticationStatus,
    _reserved43: [u8; 0x0c],
    onemcu_cti_device_id: OnemcuCtiDeviceId,
    onemcu_cti_device_type_identifier: OnemcuCtiDeviceTypeIdentifier,
    onemcu_cti_peripheral_id4: OnemcuCtiPeripheralId4,
    onemcu_cti_peripheral_id5: OnemcuCtiPeripheralId5,
    onemcu_cti_peripheral_id6: OnemcuCtiPeripheralId6,
    onemcu_cti_peripheral_id7: OnemcuCtiPeripheralId7,
    onemcu_cti_peripheral_id0: OnemcuCtiPeripheralId0,
    onemcu_cti_peripheral_id1: OnemcuCtiPeripheralId1,
    onemcu_cti_peripheral_id2: OnemcuCtiPeripheralId2,
    onemcu_cti_peripheral_id3: OnemcuCtiPeripheralId3,
    onemcu_cti_component_id0: OnemcuCtiComponentId0,
    onemcu_cti_component_id1: OnemcuCtiComponentId1,
    onemcu_cti_component_id2: OnemcuCtiComponentId2,
    onemcu_cti_component_id3: OnemcuCtiComponentId3,
    onemcu_tpiu_sportsz: OnemcuTpiuSportsz,
    onemcu_tpiu_cportsz: OnemcuTpiuCportsz,
    _reserved59: [u8; 0xf8],
    onemcu_tpiu_strigm: OnemcuTpiuStrigm,
    onemcu_tpiu_trigcnt: OnemcuTpiuTrigcnt,
    onemcu_tpiu_trigmul: OnemcuTpiuTrigmul,
    _reserved62: [u8; 0xf4],
    onemcu_tpiu_ststptrn: OnemcuTpiuStstptrn,
    onemcu_tpiu_ctstptrn: OnemcuTpiuCtstptrn,
    onemcu_tpiu_tprcntr: OnemcuTpiuTprcntr,
    _reserved65: [u8; 0xf4],
    onemcu_tpiu_ffsts: OnemcuTpiuFfsts,
    onemcu_tpiu_ffctrl: OnemcuTpiuFfctrl,
    onemcu_tpiu_fscntr: OnemcuTpiuFscntr,
    _reserved68: [u8; 0xf4],
    onemcu_tpiu_exctlin: OnemcuTpiuExctlin,
    onemcu_tpiu_exctlout: OnemcuTpiuExctlout,
    _reserved70: [u8; 0x0adc],
    onemcu_tpiu_ittrflinack: OnemcuTpiuIttrflinack,
    onemcu_tpiu_ittrflin: OnemcuTpiuIttrflin,
    onemcu_tpiu_itatbdata0: OnemcuTpiuItatbdata0,
    onemcu_tpiu_itatbctr2: OnemcuTpiuItatbctr2,
    onemcu_tpiu_itatbctr1: OnemcuTpiuItatbctr1,
    onemcu_tpiu_itatbctr0: OnemcuTpiuItatbctr0,
    _reserved76: [u8; 0x04],
    onemcu_tpiu_itctrl: OnemcuTpiuItctrl,
    _reserved77: [u8; 0x9c],
    onemcu_tpiu_claimset: OnemcuTpiuClaimset,
    onemcu_tpiu_claimclr: OnemcuTpiuClaimclr,
    _reserved79: [u8; 0x08],
    onemcu_tpiu_lar: OnemcuTpiuLar,
    onemcu_tpiu_lsr: OnemcuTpiuLsr,
    onemcu_tpiu_authstatus: OnemcuTpiuAuthstatus,
    _reserved82: [u8; 0x0c],
    onemcu_tpiu_devid: OnemcuTpiuDevid,
    onemcu_tpiu_devtype: OnemcuTpiuDevtype,
    onemcu_tpiu_pidr4: OnemcuTpiuPidr4,
    onemcu_tpiu_pidr5: OnemcuTpiuPidr5,
    onemcu_tpiu_pidr6: OnemcuTpiuPidr6,
    onemcu_tpiu_pidr7: OnemcuTpiuPidr7,
    onemcu_tpiu_pidr0: OnemcuTpiuPidr0,
    onemcu_tpiu_pidr1: OnemcuTpiuPidr1,
    onemcu_tpiu_pidr2: OnemcuTpiuPidr2,
    onemcu_tpiu_pidr3: OnemcuTpiuPidr3,
    onemcu_tpiu_cidr0: OnemcuTpiuCidr0,
    onemcu_tpiu_cidr1: OnemcuTpiuCidr1,
    onemcu_tpiu_cidr2: OnemcuTpiuCidr2,
    onemcu_tpiu_cidr3: OnemcuTpiuCidr3,
    _reserved96: [u8; 0xd000],
    app_cm4_cti_control: AppCm4CtiControl,
    _reserved97: [u8; 0x0c],
    app_cm4_cti_intack: AppCm4CtiIntack,
    app_cm4_cti_appset: AppCm4CtiAppset,
    app_cm4_cti_appclear: AppCm4CtiAppclear,
    app_cm4_cti_apppulse: AppCm4CtiApppulse,
    app_cm4_cti_inen0: AppCm4CtiInen0,
    app_cm4_cti_inen1: AppCm4CtiInen1,
    app_cm4_cti_inen2: AppCm4CtiInen2,
    app_cm4_cti_inen3: AppCm4CtiInen3,
    app_cm4_cti_inen4: AppCm4CtiInen4,
    app_cm4_cti_inen5: AppCm4CtiInen5,
    app_cm4_cti_inen6: AppCm4CtiInen6,
    app_cm4_cti_inen7: AppCm4CtiInen7,
    _reserved109: [u8; 0x60],
    app_cm4_cti_outen0: AppCm4CtiOuten0,
    app_cm4_cti_outen1: AppCm4CtiOuten1,
    app_cm4_cti_outen2: AppCm4CtiOuten2,
    app_cm4_cti_outen3: AppCm4CtiOuten3,
    app_cm4_cti_outen4: AppCm4CtiOuten4,
    app_cm4_cti_outen5: AppCm4CtiOuten5,
    app_cm4_cti_outen6: AppCm4CtiOuten6,
    app_cm4_cti_outen7: AppCm4CtiOuten7,
    _reserved117: [u8; 0x70],
    app_cm4_cti_triginstatus: AppCm4CtiTriginstatus,
    app_cm4_cti_trigoutstatus: AppCm4CtiTrigoutstatus,
    app_cm4_cti_chinstatus: AppCm4CtiChinstatus,
    app_cm4_cti_choutstatus: AppCm4CtiChoutstatus,
    app_cm4_cti_gate: AppCm4CtiGate,
    app_cm4_cti_asicctl: AppCm4CtiAsicctl,
    _reserved123: [u8; 0x0d94],
    app_cm4_cti_itchinack: AppCm4CtiItchinack,
    app_cm4_cti_ittriginack: AppCm4CtiIttriginack,
    app_cm4_cti_itchout: AppCm4CtiItchout,
    app_cm4_cti_ittrigout: AppCm4CtiIttrigout,
    app_cm4_cti_itchoutack: AppCm4CtiItchoutack,
    app_cm4_cti_ittrigoutack: AppCm4CtiIttrigoutack,
    app_cm4_cti_itchin: AppCm4CtiItchin,
    app_cm4_cti_ittrigin: AppCm4CtiIttrigin,
    _reserved131: [u8; 0x04],
    app_cm4_cti_itctrl: AppCm4CtiItctrl,
    _reserved132: [u8; 0x9c],
    app_cm4_cti_claim_tag_set: AppCm4CtiClaimTagSet,
    app_cm4_cti_claim_tag_clear: AppCm4CtiClaimTagClear,
    _reserved134: [u8; 0x08],
    app_cm4_cti_lock_access_register: AppCm4CtiLockAccessRegister,
    app_cm4_cti_lock_status_register: AppCm4CtiLockStatusRegister,
    app_cm4_cti_authentication_status: AppCm4CtiAuthenticationStatus,
    _reserved137: [u8; 0x0c],
    app_cm4_cti_device_id: AppCm4CtiDeviceId,
    app_cm4_cti_device_type_identifier: AppCm4CtiDeviceTypeIdentifier,
    app_cm4_cti_peripheral_id4: AppCm4CtiPeripheralId4,
    app_cm4_cti_peripheral_id5: AppCm4CtiPeripheralId5,
    app_cm4_cti_peripheral_id6: AppCm4CtiPeripheralId6,
    app_cm4_cti_peripheral_id7: AppCm4CtiPeripheralId7,
    app_cm4_cti_peripheral_id0: AppCm4CtiPeripheralId0,
    app_cm4_cti_peripheral_id1: AppCm4CtiPeripheralId1,
    app_cm4_cti_peripheral_id2: AppCm4CtiPeripheralId2,
    app_cm4_cti_peripheral_id3: AppCm4CtiPeripheralId3,
    app_cm4_cti_component_id0: AppCm4CtiComponentId0,
    app_cm4_cti_component_id1: AppCm4CtiComponentId1,
    app_cm4_cti_component_id2: AppCm4CtiComponentId2,
    app_cm4_cti_component_id3: AppCm4CtiComponentId3,
    fec_cm3_cti_control: FecCm3CtiControl,
    _reserved152: [u8; 0x0c],
    fec_cm3_cti_intack: FecCm3CtiIntack,
    fec_cm3_cti_appset: FecCm3CtiAppset,
    fec_cm3_cti_appclear: FecCm3CtiAppclear,
    fec_cm3_cti_apppulse: FecCm3CtiApppulse,
    fec_cm3_cti_inen0: FecCm3CtiInen0,
    fec_cm3_cti_inen1: FecCm3CtiInen1,
    fec_cm3_cti_inen2: FecCm3CtiInen2,
    fec_cm3_cti_inen3: FecCm3CtiInen3,
    fec_cm3_cti_inen4: FecCm3CtiInen4,
    fec_cm3_cti_inen5: FecCm3CtiInen5,
    fec_cm3_cti_inen6: FecCm3CtiInen6,
    fec_cm3_cti_inen7: FecCm3CtiInen7,
    _reserved164: [u8; 0x60],
    fec_cm3_cti_outen0: FecCm3CtiOuten0,
    fec_cm3_cti_outen1: FecCm3CtiOuten1,
    fec_cm3_cti_outen2: FecCm3CtiOuten2,
    fec_cm3_cti_outen3: FecCm3CtiOuten3,
    fec_cm3_cti_outen4: FecCm3CtiOuten4,
    fec_cm3_cti_outen5: FecCm3CtiOuten5,
    fec_cm3_cti_outen6: FecCm3CtiOuten6,
    fec_cm3_cti_outen7: FecCm3CtiOuten7,
    _reserved172: [u8; 0x70],
    fec_cm3_cti_triginstatus: FecCm3CtiTriginstatus,
    fec_cm3_cti_trigoutstatus: FecCm3CtiTrigoutstatus,
    fec_cm3_cti_chinstatus: FecCm3CtiChinstatus,
    fec_cm3_cti_choutstatus: FecCm3CtiChoutstatus,
    fec_cm3_cti_gate: FecCm3CtiGate,
    fec_cm3_cti_asicctl: FecCm3CtiAsicctl,
    _reserved178: [u8; 0x0d94],
    fec_cm3_cti_itchinack: FecCm3CtiItchinack,
    fec_cm3_cti_ittriginack: FecCm3CtiIttriginack,
    fec_cm3_cti_itchout: FecCm3CtiItchout,
    fec_cm3_cti_ittrigout: FecCm3CtiIttrigout,
    fec_cm3_cti_itchoutack: FecCm3CtiItchoutack,
    fec_cm3_cti_ittrigoutack: FecCm3CtiIttrigoutack,
    fec_cm3_cti_itchin: FecCm3CtiItchin,
    fec_cm3_cti_ittrigin: FecCm3CtiIttrigin,
    _reserved186: [u8; 0x04],
    fec_cm3_cti_itctrl: FecCm3CtiItctrl,
    _reserved187: [u8; 0x9c],
    fec_cm3_cti_claim_tag_set: FecCm3CtiClaimTagSet,
    fec_cm3_cti_claim_tag_clear: FecCm3CtiClaimTagClear,
    _reserved189: [u8; 0x08],
    fec_cm3_cti_lock_access_register: FecCm3CtiLockAccessRegister,
    fec_cm3_cti_lock_status_register: FecCm3CtiLockStatusRegister,
    fec_cm3_cti_authentication_status: FecCm3CtiAuthenticationStatus,
    _reserved192: [u8; 0x0c],
    fec_cm3_cti_device_id: FecCm3CtiDeviceId,
    fec_cm3_cti_device_type_identifier: FecCm3CtiDeviceTypeIdentifier,
    fec_cm3_cti_peripheral_id4: FecCm3CtiPeripheralId4,
    fec_cm3_cti_peripheral_id5: FecCm3CtiPeripheralId5,
    fec_cm3_cti_peripheral_id6: FecCm3CtiPeripheralId6,
    fec_cm3_cti_peripheral_id7: FecCm3CtiPeripheralId7,
    fec_cm3_cti_peripheral_id0: FecCm3CtiPeripheralId0,
    fec_cm3_cti_peripheral_id1: FecCm3CtiPeripheralId1,
    fec_cm3_cti_peripheral_id2: FecCm3CtiPeripheralId2,
    fec_cm3_cti_peripheral_id3: FecCm3CtiPeripheralId3,
    fec_cm3_cti_component_id0: FecCm3CtiComponentId0,
    fec_cm3_cti_component_id1: FecCm3CtiComponentId1,
    fec_cm3_cti_component_id2: FecCm3CtiComponentId2,
    fec_cm3_cti_component_id3: FecCm3CtiComponentId3,
}
impl RegisterBlock {
    #[doc = "0x00 - Start Address of ROM Table"]
    #[inline(always)]
    pub const fn onemcu_apb_base(&self) -> &OnemcuApbBase {
        &self.onemcu_apb_base
    }
    #[doc = "0xffc - End Address of ROM Table"]
    #[inline(always)]
    pub const fn onemcu_apb_base_end(&self) -> &OnemcuApbBaseEnd {
        &self.onemcu_apb_base_end
    }
    #[doc = "0x1000 - http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html"]
    #[inline(always)]
    pub const fn onemcu_cti_control(&self) -> &OnemcuCtiControl {
        &self.onemcu_cti_control
    }
    #[doc = "0x1010 - ONEMCU_CTI_INTACK"]
    #[inline(always)]
    pub const fn onemcu_cti_intack(&self) -> &OnemcuCtiIntack {
        &self.onemcu_cti_intack
    }
    #[doc = "0x1014 - ONEMCU_CTI_APPSET"]
    #[inline(always)]
    pub const fn onemcu_cti_appset(&self) -> &OnemcuCtiAppset {
        &self.onemcu_cti_appset
    }
    #[doc = "0x1018 - ONEMCU_CTI_APPCLEAR"]
    #[inline(always)]
    pub const fn onemcu_cti_appclear(&self) -> &OnemcuCtiAppclear {
        &self.onemcu_cti_appclear
    }
    #[doc = "0x101c - ONEMCU_CTI_APPPULSE"]
    #[inline(always)]
    pub const fn onemcu_cti_apppulse(&self) -> &OnemcuCtiApppulse {
        &self.onemcu_cti_apppulse
    }
    #[doc = "0x1020 - ONEMCU_CTI_INEN0"]
    #[inline(always)]
    pub const fn onemcu_cti_inen0(&self) -> &OnemcuCtiInen0 {
        &self.onemcu_cti_inen0
    }
    #[doc = "0x1024 - ONEMCU_CTI_INEN1"]
    #[inline(always)]
    pub const fn onemcu_cti_inen1(&self) -> &OnemcuCtiInen1 {
        &self.onemcu_cti_inen1
    }
    #[doc = "0x1028 - ONEMCU_CTI_INEN2"]
    #[inline(always)]
    pub const fn onemcu_cti_inen2(&self) -> &OnemcuCtiInen2 {
        &self.onemcu_cti_inen2
    }
    #[doc = "0x102c - ONEMCU_CTI_INEN3"]
    #[inline(always)]
    pub const fn onemcu_cti_inen3(&self) -> &OnemcuCtiInen3 {
        &self.onemcu_cti_inen3
    }
    #[doc = "0x1030 - ONEMCU_CTI_INEN4"]
    #[inline(always)]
    pub const fn onemcu_cti_inen4(&self) -> &OnemcuCtiInen4 {
        &self.onemcu_cti_inen4
    }
    #[doc = "0x1034 - ONEMCU_CTI_INEN5"]
    #[inline(always)]
    pub const fn onemcu_cti_inen5(&self) -> &OnemcuCtiInen5 {
        &self.onemcu_cti_inen5
    }
    #[doc = "0x1038 - ONEMCU_CTI_INEN6"]
    #[inline(always)]
    pub const fn onemcu_cti_inen6(&self) -> &OnemcuCtiInen6 {
        &self.onemcu_cti_inen6
    }
    #[doc = "0x103c - ONEMCU_CTI_INEN7"]
    #[inline(always)]
    pub const fn onemcu_cti_inen7(&self) -> &OnemcuCtiInen7 {
        &self.onemcu_cti_inen7
    }
    #[doc = "0x10a0 - ONEMCU_CTI_OUTEN0"]
    #[inline(always)]
    pub const fn onemcu_cti_outen0(&self) -> &OnemcuCtiOuten0 {
        &self.onemcu_cti_outen0
    }
    #[doc = "0x10a4 - ONEMCU_CTI_OUTEN1"]
    #[inline(always)]
    pub const fn onemcu_cti_outen1(&self) -> &OnemcuCtiOuten1 {
        &self.onemcu_cti_outen1
    }
    #[doc = "0x10a8 - ONEMCU_CTI_OUTEN2"]
    #[inline(always)]
    pub const fn onemcu_cti_outen2(&self) -> &OnemcuCtiOuten2 {
        &self.onemcu_cti_outen2
    }
    #[doc = "0x10ac - ONEMCU_CTI_OUTEN3"]
    #[inline(always)]
    pub const fn onemcu_cti_outen3(&self) -> &OnemcuCtiOuten3 {
        &self.onemcu_cti_outen3
    }
    #[doc = "0x10b0 - ONEMCU_CTI_OUTEN4"]
    #[inline(always)]
    pub const fn onemcu_cti_outen4(&self) -> &OnemcuCtiOuten4 {
        &self.onemcu_cti_outen4
    }
    #[doc = "0x10b4 - ONEMCU_CTI_OUTEN5"]
    #[inline(always)]
    pub const fn onemcu_cti_outen5(&self) -> &OnemcuCtiOuten5 {
        &self.onemcu_cti_outen5
    }
    #[doc = "0x10b8 - ONEMCU_CTI_OUTEN6"]
    #[inline(always)]
    pub const fn onemcu_cti_outen6(&self) -> &OnemcuCtiOuten6 {
        &self.onemcu_cti_outen6
    }
    #[doc = "0x10bc - ONEMCU_CTI_OUTEN7"]
    #[inline(always)]
    pub const fn onemcu_cti_outen7(&self) -> &OnemcuCtiOuten7 {
        &self.onemcu_cti_outen7
    }
    #[doc = "0x1130 - ONEMCU_CTI_TRIGINSTATUS"]
    #[inline(always)]
    pub const fn onemcu_cti_triginstatus(&self) -> &OnemcuCtiTriginstatus {
        &self.onemcu_cti_triginstatus
    }
    #[doc = "0x1134 - ONEMCU_CTI_TRIGOUTSTATUS"]
    #[inline(always)]
    pub const fn onemcu_cti_trigoutstatus(&self) -> &OnemcuCtiTrigoutstatus {
        &self.onemcu_cti_trigoutstatus
    }
    #[doc = "0x1138 - ONEMCU_CTI_CHINSTATUS"]
    #[inline(always)]
    pub const fn onemcu_cti_chinstatus(&self) -> &OnemcuCtiChinstatus {
        &self.onemcu_cti_chinstatus
    }
    #[doc = "0x113c - ONEMCU_CTI_CHOUTSTATUS"]
    #[inline(always)]
    pub const fn onemcu_cti_choutstatus(&self) -> &OnemcuCtiChoutstatus {
        &self.onemcu_cti_choutstatus
    }
    #[doc = "0x1140 - ONEMCU_CTI_GATE"]
    #[inline(always)]
    pub const fn onemcu_cti_gate(&self) -> &OnemcuCtiGate {
        &self.onemcu_cti_gate
    }
    #[doc = "0x1144 - ONEMCU_CTI_ASICCTL"]
    #[inline(always)]
    pub const fn onemcu_cti_asicctl(&self) -> &OnemcuCtiAsicctl {
        &self.onemcu_cti_asicctl
    }
    #[doc = "0x1edc - ONEMCU_CTI_ITCHINACK"]
    #[inline(always)]
    pub const fn onemcu_cti_itchinack(&self) -> &OnemcuCtiItchinack {
        &self.onemcu_cti_itchinack
    }
    #[doc = "0x1ee0 - ONEMCU_CTI_ITTRIGINACK"]
    #[inline(always)]
    pub const fn onemcu_cti_ittriginack(&self) -> &OnemcuCtiIttriginack {
        &self.onemcu_cti_ittriginack
    }
    #[doc = "0x1ee4 - ONEMCU_CTI_ITCHOUT"]
    #[inline(always)]
    pub const fn onemcu_cti_itchout(&self) -> &OnemcuCtiItchout {
        &self.onemcu_cti_itchout
    }
    #[doc = "0x1ee8 - ONEMCU_CTI_ITTRIGOUT"]
    #[inline(always)]
    pub const fn onemcu_cti_ittrigout(&self) -> &OnemcuCtiIttrigout {
        &self.onemcu_cti_ittrigout
    }
    #[doc = "0x1eec - ONEMCU_CTI_ITCHOUTACK"]
    #[inline(always)]
    pub const fn onemcu_cti_itchoutack(&self) -> &OnemcuCtiItchoutack {
        &self.onemcu_cti_itchoutack
    }
    #[doc = "0x1ef0 - ONEMCU_CTI_ITTRIGOUTACK"]
    #[inline(always)]
    pub const fn onemcu_cti_ittrigoutack(&self) -> &OnemcuCtiIttrigoutack {
        &self.onemcu_cti_ittrigoutack
    }
    #[doc = "0x1ef4 - ONEMCU_CTI_ITCHIN"]
    #[inline(always)]
    pub const fn onemcu_cti_itchin(&self) -> &OnemcuCtiItchin {
        &self.onemcu_cti_itchin
    }
    #[doc = "0x1ef8 - ONEMCU_CTI_ITTRIGIN"]
    #[inline(always)]
    pub const fn onemcu_cti_ittrigin(&self) -> &OnemcuCtiIttrigin {
        &self.onemcu_cti_ittrigin
    }
    #[doc = "0x1f00 - ONEMCU_CTI_ITCTRL"]
    #[inline(always)]
    pub const fn onemcu_cti_itctrl(&self) -> &OnemcuCtiItctrl {
        &self.onemcu_cti_itctrl
    }
    #[doc = "0x1fa0 - ONEMCU_CTI_Claim_Tag_Set"]
    #[inline(always)]
    pub const fn onemcu_cti_claim_tag_set(&self) -> &OnemcuCtiClaimTagSet {
        &self.onemcu_cti_claim_tag_set
    }
    #[doc = "0x1fa4 - ONEMCU_CTI_Claim_Tag_Clear"]
    #[inline(always)]
    pub const fn onemcu_cti_claim_tag_clear(&self) -> &OnemcuCtiClaimTagClear {
        &self.onemcu_cti_claim_tag_clear
    }
    #[doc = "0x1fb0 - ONEMCU_CTI_Lock_Access_Register"]
    #[inline(always)]
    pub const fn onemcu_cti_lock_access_register(&self) -> &OnemcuCtiLockAccessRegister {
        &self.onemcu_cti_lock_access_register
    }
    #[doc = "0x1fb4 - ONEMCU_CTI_Lock_Status_Register"]
    #[inline(always)]
    pub const fn onemcu_cti_lock_status_register(&self) -> &OnemcuCtiLockStatusRegister {
        &self.onemcu_cti_lock_status_register
    }
    #[doc = "0x1fb8 - ONEMCU_CTI_Authentication_Status"]
    #[inline(always)]
    pub const fn onemcu_cti_authentication_status(&self) -> &OnemcuCtiAuthenticationStatus {
        &self.onemcu_cti_authentication_status
    }
    #[doc = "0x1fc8 - ONEMCU_CTI_Device_ID"]
    #[inline(always)]
    pub const fn onemcu_cti_device_id(&self) -> &OnemcuCtiDeviceId {
        &self.onemcu_cti_device_id
    }
    #[doc = "0x1fcc - ONEMCU_CTI_Device_Type_Identifier"]
    #[inline(always)]
    pub const fn onemcu_cti_device_type_identifier(&self) -> &OnemcuCtiDeviceTypeIdentifier {
        &self.onemcu_cti_device_type_identifier
    }
    #[doc = "0x1fd0 - ONEMCU_CTI_PeripheralID4"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id4(&self) -> &OnemcuCtiPeripheralId4 {
        &self.onemcu_cti_peripheral_id4
    }
    #[doc = "0x1fd4 - ONEMCU_CTI_PeripheralID5"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id5(&self) -> &OnemcuCtiPeripheralId5 {
        &self.onemcu_cti_peripheral_id5
    }
    #[doc = "0x1fd8 - ONEMCU_CTI_PeripheralID6"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id6(&self) -> &OnemcuCtiPeripheralId6 {
        &self.onemcu_cti_peripheral_id6
    }
    #[doc = "0x1fdc - ONEMCU_CTI_PeripheralID7"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id7(&self) -> &OnemcuCtiPeripheralId7 {
        &self.onemcu_cti_peripheral_id7
    }
    #[doc = "0x1fe0 - ONEMCU_CTI_PeripheralID0"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id0(&self) -> &OnemcuCtiPeripheralId0 {
        &self.onemcu_cti_peripheral_id0
    }
    #[doc = "0x1fe4 - ONEMCU_CTI_PeripheralID1"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id1(&self) -> &OnemcuCtiPeripheralId1 {
        &self.onemcu_cti_peripheral_id1
    }
    #[doc = "0x1fe8 - ONEMCU_CTI_PeripheralID2"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id2(&self) -> &OnemcuCtiPeripheralId2 {
        &self.onemcu_cti_peripheral_id2
    }
    #[doc = "0x1fec - ONEMCU_CTI_PeripheralID3"]
    #[inline(always)]
    pub const fn onemcu_cti_peripheral_id3(&self) -> &OnemcuCtiPeripheralId3 {
        &self.onemcu_cti_peripheral_id3
    }
    #[doc = "0x1ff0 - ONEMCU_CTI_Component_ID0"]
    #[inline(always)]
    pub const fn onemcu_cti_component_id0(&self) -> &OnemcuCtiComponentId0 {
        &self.onemcu_cti_component_id0
    }
    #[doc = "0x1ff4 - ONEMCU_CTI_Component_ID1"]
    #[inline(always)]
    pub const fn onemcu_cti_component_id1(&self) -> &OnemcuCtiComponentId1 {
        &self.onemcu_cti_component_id1
    }
    #[doc = "0x1ff8 - ONEMCU_CTI_Component_ID2"]
    #[inline(always)]
    pub const fn onemcu_cti_component_id2(&self) -> &OnemcuCtiComponentId2 {
        &self.onemcu_cti_component_id2
    }
    #[doc = "0x1ffc - ONEMCU_CTI_Component_ID3"]
    #[inline(always)]
    pub const fn onemcu_cti_component_id3(&self) -> &OnemcuCtiComponentId3 {
        &self.onemcu_cti_component_id3
    }
    #[doc = "0x2000 - Supported port sizes"]
    #[inline(always)]
    pub const fn onemcu_tpiu_sportsz(&self) -> &OnemcuTpiuSportsz {
        &self.onemcu_tpiu_sportsz
    }
    #[doc = "0x2004 - Current port size"]
    #[inline(always)]
    pub const fn onemcu_tpiu_cportsz(&self) -> &OnemcuTpiuCportsz {
        &self.onemcu_tpiu_cportsz
    }
    #[doc = "0x2100 - Supported trigger modes"]
    #[inline(always)]
    pub const fn onemcu_tpiu_strigm(&self) -> &OnemcuTpiuStrigm {
        &self.onemcu_tpiu_strigm
    }
    #[doc = "0x2104 - Trigger counter value"]
    #[inline(always)]
    pub const fn onemcu_tpiu_trigcnt(&self) -> &OnemcuTpiuTrigcnt {
        &self.onemcu_tpiu_trigcnt
    }
    #[doc = "0x2108 - Trigger multiplier"]
    #[inline(always)]
    pub const fn onemcu_tpiu_trigmul(&self) -> &OnemcuTpiuTrigmul {
        &self.onemcu_tpiu_trigmul
    }
    #[doc = "0x2200 - Supported test pattern/modes"]
    #[inline(always)]
    pub const fn onemcu_tpiu_ststptrn(&self) -> &OnemcuTpiuStstptrn {
        &self.onemcu_tpiu_ststptrn
    }
    #[doc = "0x2204 - Current test pattern/mode"]
    #[inline(always)]
    pub const fn onemcu_tpiu_ctstptrn(&self) -> &OnemcuTpiuCtstptrn {
        &self.onemcu_tpiu_ctstptrn
    }
    #[doc = "0x2208 - Test pattern repeat counter"]
    #[inline(always)]
    pub const fn onemcu_tpiu_tprcntr(&self) -> &OnemcuTpiuTprcntr {
        &self.onemcu_tpiu_tprcntr
    }
    #[doc = "0x2300 - Formatter and flush status"]
    #[inline(always)]
    pub const fn onemcu_tpiu_ffsts(&self) -> &OnemcuTpiuFfsts {
        &self.onemcu_tpiu_ffsts
    }
    #[doc = "0x2304 - Formatter and flush control"]
    #[inline(always)]
    pub const fn onemcu_tpiu_ffctrl(&self) -> &OnemcuTpiuFfctrl {
        &self.onemcu_tpiu_ffctrl
    }
    #[doc = "0x2308 - Formatter synchronization counter"]
    #[inline(always)]
    pub const fn onemcu_tpiu_fscntr(&self) -> &OnemcuTpiuFscntr {
        &self.onemcu_tpiu_fscntr
    }
    #[doc = "0x2400 - EXTCTL In Port"]
    #[inline(always)]
    pub const fn onemcu_tpiu_exctlin(&self) -> &OnemcuTpiuExctlin {
        &self.onemcu_tpiu_exctlin
    }
    #[doc = "0x2404 - EXTCTL Out Port"]
    #[inline(always)]
    pub const fn onemcu_tpiu_exctlout(&self) -> &OnemcuTpiuExctlout {
        &self.onemcu_tpiu_exctlout
    }
    #[doc = "0x2ee4 - Integration Register, ITTRFLINACK"]
    #[inline(always)]
    pub const fn onemcu_tpiu_ittrflinack(&self) -> &OnemcuTpiuIttrflinack {
        &self.onemcu_tpiu_ittrflinack
    }
    #[doc = "0x2ee8 - Integration Register, ITTRFLIN"]
    #[inline(always)]
    pub const fn onemcu_tpiu_ittrflin(&self) -> &OnemcuTpiuIttrflin {
        &self.onemcu_tpiu_ittrflin
    }
    #[doc = "0x2eec - Integration Register, ITATBDATA0"]
    #[inline(always)]
    pub const fn onemcu_tpiu_itatbdata0(&self) -> &OnemcuTpiuItatbdata0 {
        &self.onemcu_tpiu_itatbdata0
    }
    #[doc = "0x2ef0 - Integration Register, ITATBCTR2"]
    #[inline(always)]
    pub const fn onemcu_tpiu_itatbctr2(&self) -> &OnemcuTpiuItatbctr2 {
        &self.onemcu_tpiu_itatbctr2
    }
    #[doc = "0x2ef4 - Integration Register, ITATBCTR1"]
    #[inline(always)]
    pub const fn onemcu_tpiu_itatbctr1(&self) -> &OnemcuTpiuItatbctr1 {
        &self.onemcu_tpiu_itatbctr1
    }
    #[doc = "0x2ef8 - Integration Register, ITATBCTR0"]
    #[inline(always)]
    pub const fn onemcu_tpiu_itatbctr0(&self) -> &OnemcuTpiuItatbctr0 {
        &self.onemcu_tpiu_itatbctr0
    }
    #[doc = "0x2f00 - Integration Mode Control Register"]
    #[inline(always)]
    pub const fn onemcu_tpiu_itctrl(&self) -> &OnemcuTpiuItctrl {
        &self.onemcu_tpiu_itctrl
    }
    #[doc = "0x2fa0 - Claim Tag Set"]
    #[inline(always)]
    pub const fn onemcu_tpiu_claimset(&self) -> &OnemcuTpiuClaimset {
        &self.onemcu_tpiu_claimset
    }
    #[doc = "0x2fa4 - Claim Tag Clear"]
    #[inline(always)]
    pub const fn onemcu_tpiu_claimclr(&self) -> &OnemcuTpiuClaimclr {
        &self.onemcu_tpiu_claimclr
    }
    #[doc = "0x2fb0 - Lock status"]
    #[inline(always)]
    pub const fn onemcu_tpiu_lar(&self) -> &OnemcuTpiuLar {
        &self.onemcu_tpiu_lar
    }
    #[doc = "0x2fb4 - Lock Access"]
    #[inline(always)]
    pub const fn onemcu_tpiu_lsr(&self) -> &OnemcuTpiuLsr {
        &self.onemcu_tpiu_lsr
    }
    #[doc = "0x2fb8 - Authentication status"]
    #[inline(always)]
    pub const fn onemcu_tpiu_authstatus(&self) -> &OnemcuTpiuAuthstatus {
        &self.onemcu_tpiu_authstatus
    }
    #[doc = "0x2fc8 - Device ID"]
    #[inline(always)]
    pub const fn onemcu_tpiu_devid(&self) -> &OnemcuTpiuDevid {
        &self.onemcu_tpiu_devid
    }
    #[doc = "0x2fcc - Device type identifier"]
    #[inline(always)]
    pub const fn onemcu_tpiu_devtype(&self) -> &OnemcuTpiuDevtype {
        &self.onemcu_tpiu_devtype
    }
    #[doc = "0x2fd0 - Peripheral ID4"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr4(&self) -> &OnemcuTpiuPidr4 {
        &self.onemcu_tpiu_pidr4
    }
    #[doc = "0x2fd4 - Peripheral ID5"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr5(&self) -> &OnemcuTpiuPidr5 {
        &self.onemcu_tpiu_pidr5
    }
    #[doc = "0x2fd8 - Peripheral ID6"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr6(&self) -> &OnemcuTpiuPidr6 {
        &self.onemcu_tpiu_pidr6
    }
    #[doc = "0x2fdc - Peripheral ID7"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr7(&self) -> &OnemcuTpiuPidr7 {
        &self.onemcu_tpiu_pidr7
    }
    #[doc = "0x2fe0 - Peripheral ID0"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr0(&self) -> &OnemcuTpiuPidr0 {
        &self.onemcu_tpiu_pidr0
    }
    #[doc = "0x2fe4 - Peripheral ID1"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr1(&self) -> &OnemcuTpiuPidr1 {
        &self.onemcu_tpiu_pidr1
    }
    #[doc = "0x2fe8 - Peripheral ID2"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr2(&self) -> &OnemcuTpiuPidr2 {
        &self.onemcu_tpiu_pidr2
    }
    #[doc = "0x2fec - Peripheral ID3"]
    #[inline(always)]
    pub const fn onemcu_tpiu_pidr3(&self) -> &OnemcuTpiuPidr3 {
        &self.onemcu_tpiu_pidr3
    }
    #[doc = "0x2ff0 - Component ID0"]
    #[inline(always)]
    pub const fn onemcu_tpiu_cidr0(&self) -> &OnemcuTpiuCidr0 {
        &self.onemcu_tpiu_cidr0
    }
    #[doc = "0x2ff4 - Component ID1"]
    #[inline(always)]
    pub const fn onemcu_tpiu_cidr1(&self) -> &OnemcuTpiuCidr1 {
        &self.onemcu_tpiu_cidr1
    }
    #[doc = "0x2ff8 - Component ID2"]
    #[inline(always)]
    pub const fn onemcu_tpiu_cidr2(&self) -> &OnemcuTpiuCidr2 {
        &self.onemcu_tpiu_cidr2
    }
    #[doc = "0x2ffc - Component ID3"]
    #[inline(always)]
    pub const fn onemcu_tpiu_cidr3(&self) -> &OnemcuTpiuCidr3 {
        &self.onemcu_tpiu_cidr3
    }
    #[doc = "0x10000 - http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html"]
    #[inline(always)]
    pub const fn app_cm4_cti_control(&self) -> &AppCm4CtiControl {
        &self.app_cm4_cti_control
    }
    #[doc = "0x10010 - APP_CM4_CTI_INTACK"]
    #[inline(always)]
    pub const fn app_cm4_cti_intack(&self) -> &AppCm4CtiIntack {
        &self.app_cm4_cti_intack
    }
    #[doc = "0x10014 - APP_CM4_CTI_APPSET"]
    #[inline(always)]
    pub const fn app_cm4_cti_appset(&self) -> &AppCm4CtiAppset {
        &self.app_cm4_cti_appset
    }
    #[doc = "0x10018 - APP_CM4_CTI_APPCLEAR"]
    #[inline(always)]
    pub const fn app_cm4_cti_appclear(&self) -> &AppCm4CtiAppclear {
        &self.app_cm4_cti_appclear
    }
    #[doc = "0x1001c - APP_CM4_CTI_APPPULSE"]
    #[inline(always)]
    pub const fn app_cm4_cti_apppulse(&self) -> &AppCm4CtiApppulse {
        &self.app_cm4_cti_apppulse
    }
    #[doc = "0x10020 - APP_CM4_CTI_INEN0"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen0(&self) -> &AppCm4CtiInen0 {
        &self.app_cm4_cti_inen0
    }
    #[doc = "0x10024 - APP_CM4_CTI_INEN1"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen1(&self) -> &AppCm4CtiInen1 {
        &self.app_cm4_cti_inen1
    }
    #[doc = "0x10028 - APP_CM4_CTI_INEN2"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen2(&self) -> &AppCm4CtiInen2 {
        &self.app_cm4_cti_inen2
    }
    #[doc = "0x1002c - APP_CM4_CTI_INEN3"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen3(&self) -> &AppCm4CtiInen3 {
        &self.app_cm4_cti_inen3
    }
    #[doc = "0x10030 - APP_CM4_CTI_INEN4"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen4(&self) -> &AppCm4CtiInen4 {
        &self.app_cm4_cti_inen4
    }
    #[doc = "0x10034 - APP_CM4_CTI_INEN5"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen5(&self) -> &AppCm4CtiInen5 {
        &self.app_cm4_cti_inen5
    }
    #[doc = "0x10038 - APP_CM4_CTI_INEN6"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen6(&self) -> &AppCm4CtiInen6 {
        &self.app_cm4_cti_inen6
    }
    #[doc = "0x1003c - APP_CM4_CTI_INEN7"]
    #[inline(always)]
    pub const fn app_cm4_cti_inen7(&self) -> &AppCm4CtiInen7 {
        &self.app_cm4_cti_inen7
    }
    #[doc = "0x100a0 - APP_CM4_CTI_OUTEN0"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen0(&self) -> &AppCm4CtiOuten0 {
        &self.app_cm4_cti_outen0
    }
    #[doc = "0x100a4 - APP_CM4_CTI_OUTEN1"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen1(&self) -> &AppCm4CtiOuten1 {
        &self.app_cm4_cti_outen1
    }
    #[doc = "0x100a8 - APP_CM4_CTI_OUTEN2"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen2(&self) -> &AppCm4CtiOuten2 {
        &self.app_cm4_cti_outen2
    }
    #[doc = "0x100ac - APP_CM4_CTI_OUTEN3"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen3(&self) -> &AppCm4CtiOuten3 {
        &self.app_cm4_cti_outen3
    }
    #[doc = "0x100b0 - APP_CM4_CTI_OUTEN4"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen4(&self) -> &AppCm4CtiOuten4 {
        &self.app_cm4_cti_outen4
    }
    #[doc = "0x100b4 - APP_CM4_CTI_OUTEN5"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen5(&self) -> &AppCm4CtiOuten5 {
        &self.app_cm4_cti_outen5
    }
    #[doc = "0x100b8 - APP_CM4_CTI_OUTEN6"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen6(&self) -> &AppCm4CtiOuten6 {
        &self.app_cm4_cti_outen6
    }
    #[doc = "0x100bc - APP_CM4_CTI_OUTEN7"]
    #[inline(always)]
    pub const fn app_cm4_cti_outen7(&self) -> &AppCm4CtiOuten7 {
        &self.app_cm4_cti_outen7
    }
    #[doc = "0x10130 - APP_CM4_CTI_TRIGINSTATUS"]
    #[inline(always)]
    pub const fn app_cm4_cti_triginstatus(&self) -> &AppCm4CtiTriginstatus {
        &self.app_cm4_cti_triginstatus
    }
    #[doc = "0x10134 - APP_CM4_CTI_TRIGOUTSTATUS"]
    #[inline(always)]
    pub const fn app_cm4_cti_trigoutstatus(&self) -> &AppCm4CtiTrigoutstatus {
        &self.app_cm4_cti_trigoutstatus
    }
    #[doc = "0x10138 - APP_CM4_CTI_CHINSTATUS"]
    #[inline(always)]
    pub const fn app_cm4_cti_chinstatus(&self) -> &AppCm4CtiChinstatus {
        &self.app_cm4_cti_chinstatus
    }
    #[doc = "0x1013c - APP_CM4_CTI_CHOUTSTATUS"]
    #[inline(always)]
    pub const fn app_cm4_cti_choutstatus(&self) -> &AppCm4CtiChoutstatus {
        &self.app_cm4_cti_choutstatus
    }
    #[doc = "0x10140 - APP_CM4_CTI_GATE"]
    #[inline(always)]
    pub const fn app_cm4_cti_gate(&self) -> &AppCm4CtiGate {
        &self.app_cm4_cti_gate
    }
    #[doc = "0x10144 - APP_CM4_CTI_ASICCTL"]
    #[inline(always)]
    pub const fn app_cm4_cti_asicctl(&self) -> &AppCm4CtiAsicctl {
        &self.app_cm4_cti_asicctl
    }
    #[doc = "0x10edc - APP_CM4_CTI_ITCHINACK"]
    #[inline(always)]
    pub const fn app_cm4_cti_itchinack(&self) -> &AppCm4CtiItchinack {
        &self.app_cm4_cti_itchinack
    }
    #[doc = "0x10ee0 - APP_CM4_CTI_ITTRIGINACK"]
    #[inline(always)]
    pub const fn app_cm4_cti_ittriginack(&self) -> &AppCm4CtiIttriginack {
        &self.app_cm4_cti_ittriginack
    }
    #[doc = "0x10ee4 - APP_CM4_CTI_ITCHOUT"]
    #[inline(always)]
    pub const fn app_cm4_cti_itchout(&self) -> &AppCm4CtiItchout {
        &self.app_cm4_cti_itchout
    }
    #[doc = "0x10ee8 - APP_CM4_CTI_ITTRIGOUT"]
    #[inline(always)]
    pub const fn app_cm4_cti_ittrigout(&self) -> &AppCm4CtiIttrigout {
        &self.app_cm4_cti_ittrigout
    }
    #[doc = "0x10eec - APP_CM4_CTI_ITCHOUTACK"]
    #[inline(always)]
    pub const fn app_cm4_cti_itchoutack(&self) -> &AppCm4CtiItchoutack {
        &self.app_cm4_cti_itchoutack
    }
    #[doc = "0x10ef0 - APP_CM4_CTI_ITTRIGOUTACK"]
    #[inline(always)]
    pub const fn app_cm4_cti_ittrigoutack(&self) -> &AppCm4CtiIttrigoutack {
        &self.app_cm4_cti_ittrigoutack
    }
    #[doc = "0x10ef4 - APP_CM4_CTI_ITCHIN"]
    #[inline(always)]
    pub const fn app_cm4_cti_itchin(&self) -> &AppCm4CtiItchin {
        &self.app_cm4_cti_itchin
    }
    #[doc = "0x10ef8 - APP_CM4_CTI_ITTRIGIN"]
    #[inline(always)]
    pub const fn app_cm4_cti_ittrigin(&self) -> &AppCm4CtiIttrigin {
        &self.app_cm4_cti_ittrigin
    }
    #[doc = "0x10f00 - APP_CM4_CTI_ITCTRL"]
    #[inline(always)]
    pub const fn app_cm4_cti_itctrl(&self) -> &AppCm4CtiItctrl {
        &self.app_cm4_cti_itctrl
    }
    #[doc = "0x10fa0 - APP_CM4_CTI_Claim_Tag_Set"]
    #[inline(always)]
    pub const fn app_cm4_cti_claim_tag_set(&self) -> &AppCm4CtiClaimTagSet {
        &self.app_cm4_cti_claim_tag_set
    }
    #[doc = "0x10fa4 - APP_CM4_CTI_Claim_Tag_Clear"]
    #[inline(always)]
    pub const fn app_cm4_cti_claim_tag_clear(&self) -> &AppCm4CtiClaimTagClear {
        &self.app_cm4_cti_claim_tag_clear
    }
    #[doc = "0x10fb0 - APP_CM4_CTI_Lock_Access_Register"]
    #[inline(always)]
    pub const fn app_cm4_cti_lock_access_register(&self) -> &AppCm4CtiLockAccessRegister {
        &self.app_cm4_cti_lock_access_register
    }
    #[doc = "0x10fb4 - APP_CM4_CTI_Lock_Status_Register"]
    #[inline(always)]
    pub const fn app_cm4_cti_lock_status_register(&self) -> &AppCm4CtiLockStatusRegister {
        &self.app_cm4_cti_lock_status_register
    }
    #[doc = "0x10fb8 - APP_CM4_CTI_Authentication_Status"]
    #[inline(always)]
    pub const fn app_cm4_cti_authentication_status(&self) -> &AppCm4CtiAuthenticationStatus {
        &self.app_cm4_cti_authentication_status
    }
    #[doc = "0x10fc8 - APP_CM4_CTI_Device_ID"]
    #[inline(always)]
    pub const fn app_cm4_cti_device_id(&self) -> &AppCm4CtiDeviceId {
        &self.app_cm4_cti_device_id
    }
    #[doc = "0x10fcc - APP_CM4_CTI_Device_Type_Identifier"]
    #[inline(always)]
    pub const fn app_cm4_cti_device_type_identifier(&self) -> &AppCm4CtiDeviceTypeIdentifier {
        &self.app_cm4_cti_device_type_identifier
    }
    #[doc = "0x10fd0 - APP_CM4_CTI_PeripheralID4"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id4(&self) -> &AppCm4CtiPeripheralId4 {
        &self.app_cm4_cti_peripheral_id4
    }
    #[doc = "0x10fd4 - APP_CM4_CTI_PeripheralID5"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id5(&self) -> &AppCm4CtiPeripheralId5 {
        &self.app_cm4_cti_peripheral_id5
    }
    #[doc = "0x10fd8 - APP_CM4_CTI_PeripheralID6"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id6(&self) -> &AppCm4CtiPeripheralId6 {
        &self.app_cm4_cti_peripheral_id6
    }
    #[doc = "0x10fdc - APP_CM4_CTI_PeripheralID7"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id7(&self) -> &AppCm4CtiPeripheralId7 {
        &self.app_cm4_cti_peripheral_id7
    }
    #[doc = "0x10fe0 - APP_CM4_CTI_PeripheralID0"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id0(&self) -> &AppCm4CtiPeripheralId0 {
        &self.app_cm4_cti_peripheral_id0
    }
    #[doc = "0x10fe4 - APP_CM4_CTI_PeripheralID1"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id1(&self) -> &AppCm4CtiPeripheralId1 {
        &self.app_cm4_cti_peripheral_id1
    }
    #[doc = "0x10fe8 - APP_CM4_CTI_PeripheralID2"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id2(&self) -> &AppCm4CtiPeripheralId2 {
        &self.app_cm4_cti_peripheral_id2
    }
    #[doc = "0x10fec - APP_CM4_CTI_PeripheralID3"]
    #[inline(always)]
    pub const fn app_cm4_cti_peripheral_id3(&self) -> &AppCm4CtiPeripheralId3 {
        &self.app_cm4_cti_peripheral_id3
    }
    #[doc = "0x10ff0 - APP_CM4_CTI_Component_ID0"]
    #[inline(always)]
    pub const fn app_cm4_cti_component_id0(&self) -> &AppCm4CtiComponentId0 {
        &self.app_cm4_cti_component_id0
    }
    #[doc = "0x10ff4 - APP_CM4_CTI_Component_ID1"]
    #[inline(always)]
    pub const fn app_cm4_cti_component_id1(&self) -> &AppCm4CtiComponentId1 {
        &self.app_cm4_cti_component_id1
    }
    #[doc = "0x10ff8 - APP_CM4_CTI_Component_ID2"]
    #[inline(always)]
    pub const fn app_cm4_cti_component_id2(&self) -> &AppCm4CtiComponentId2 {
        &self.app_cm4_cti_component_id2
    }
    #[doc = "0x10ffc - APP_CM4_CTI_Component_ID3"]
    #[inline(always)]
    pub const fn app_cm4_cti_component_id3(&self) -> &AppCm4CtiComponentId3 {
        &self.app_cm4_cti_component_id3
    }
    #[doc = "0x11000 - http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html"]
    #[inline(always)]
    pub const fn fec_cm3_cti_control(&self) -> &FecCm3CtiControl {
        &self.fec_cm3_cti_control
    }
    #[doc = "0x11010 - FEC_CM3_CTI_INTACK"]
    #[inline(always)]
    pub const fn fec_cm3_cti_intack(&self) -> &FecCm3CtiIntack {
        &self.fec_cm3_cti_intack
    }
    #[doc = "0x11014 - FEC_CM3_CTI_APPSET"]
    #[inline(always)]
    pub const fn fec_cm3_cti_appset(&self) -> &FecCm3CtiAppset {
        &self.fec_cm3_cti_appset
    }
    #[doc = "0x11018 - FEC_CM3_CTI_APPCLEAR"]
    #[inline(always)]
    pub const fn fec_cm3_cti_appclear(&self) -> &FecCm3CtiAppclear {
        &self.fec_cm3_cti_appclear
    }
    #[doc = "0x1101c - FEC_CM3_CTI_APPPULSE"]
    #[inline(always)]
    pub const fn fec_cm3_cti_apppulse(&self) -> &FecCm3CtiApppulse {
        &self.fec_cm3_cti_apppulse
    }
    #[doc = "0x11020 - FEC_CM3_CTI_INEN0"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen0(&self) -> &FecCm3CtiInen0 {
        &self.fec_cm3_cti_inen0
    }
    #[doc = "0x11024 - FEC_CM3_CTI_INEN1"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen1(&self) -> &FecCm3CtiInen1 {
        &self.fec_cm3_cti_inen1
    }
    #[doc = "0x11028 - FEC_CM3_CTI_INEN2"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen2(&self) -> &FecCm3CtiInen2 {
        &self.fec_cm3_cti_inen2
    }
    #[doc = "0x1102c - FEC_CM3_CTI_INEN3"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen3(&self) -> &FecCm3CtiInen3 {
        &self.fec_cm3_cti_inen3
    }
    #[doc = "0x11030 - FEC_CM3_CTI_INEN4"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen4(&self) -> &FecCm3CtiInen4 {
        &self.fec_cm3_cti_inen4
    }
    #[doc = "0x11034 - FEC_CM3_CTI_INEN5"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen5(&self) -> &FecCm3CtiInen5 {
        &self.fec_cm3_cti_inen5
    }
    #[doc = "0x11038 - FEC_CM3_CTI_INEN6"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen6(&self) -> &FecCm3CtiInen6 {
        &self.fec_cm3_cti_inen6
    }
    #[doc = "0x1103c - FEC_CM3_CTI_INEN7"]
    #[inline(always)]
    pub const fn fec_cm3_cti_inen7(&self) -> &FecCm3CtiInen7 {
        &self.fec_cm3_cti_inen7
    }
    #[doc = "0x110a0 - FEC_CM3_CTI_OUTEN0"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen0(&self) -> &FecCm3CtiOuten0 {
        &self.fec_cm3_cti_outen0
    }
    #[doc = "0x110a4 - FEC_CM3_CTI_OUTEN1"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen1(&self) -> &FecCm3CtiOuten1 {
        &self.fec_cm3_cti_outen1
    }
    #[doc = "0x110a8 - FEC_CM3_CTI_OUTEN2"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen2(&self) -> &FecCm3CtiOuten2 {
        &self.fec_cm3_cti_outen2
    }
    #[doc = "0x110ac - FEC_CM3_CTI_OUTEN3"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen3(&self) -> &FecCm3CtiOuten3 {
        &self.fec_cm3_cti_outen3
    }
    #[doc = "0x110b0 - FEC_CM3_CTI_OUTEN4"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen4(&self) -> &FecCm3CtiOuten4 {
        &self.fec_cm3_cti_outen4
    }
    #[doc = "0x110b4 - FEC_CM3_CTI_OUTEN5"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen5(&self) -> &FecCm3CtiOuten5 {
        &self.fec_cm3_cti_outen5
    }
    #[doc = "0x110b8 - FEC_CM3_CTI_OUTEN6"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen6(&self) -> &FecCm3CtiOuten6 {
        &self.fec_cm3_cti_outen6
    }
    #[doc = "0x110bc - FEC_CM3_CTI_OUTEN7"]
    #[inline(always)]
    pub const fn fec_cm3_cti_outen7(&self) -> &FecCm3CtiOuten7 {
        &self.fec_cm3_cti_outen7
    }
    #[doc = "0x11130 - FEC_CM3_CTI_TRIGINSTATUS"]
    #[inline(always)]
    pub const fn fec_cm3_cti_triginstatus(&self) -> &FecCm3CtiTriginstatus {
        &self.fec_cm3_cti_triginstatus
    }
    #[doc = "0x11134 - FEC_CM3_CTI_TRIGOUTSTATUS"]
    #[inline(always)]
    pub const fn fec_cm3_cti_trigoutstatus(&self) -> &FecCm3CtiTrigoutstatus {
        &self.fec_cm3_cti_trigoutstatus
    }
    #[doc = "0x11138 - FEC_CM3_CTI_CHINSTATUS"]
    #[inline(always)]
    pub const fn fec_cm3_cti_chinstatus(&self) -> &FecCm3CtiChinstatus {
        &self.fec_cm3_cti_chinstatus
    }
    #[doc = "0x1113c - FEC_CM3_CTI_CHOUTSTATUS"]
    #[inline(always)]
    pub const fn fec_cm3_cti_choutstatus(&self) -> &FecCm3CtiChoutstatus {
        &self.fec_cm3_cti_choutstatus
    }
    #[doc = "0x11140 - FEC_CM3_CTI_GATE"]
    #[inline(always)]
    pub const fn fec_cm3_cti_gate(&self) -> &FecCm3CtiGate {
        &self.fec_cm3_cti_gate
    }
    #[doc = "0x11144 - FEC_CM3_CTI_ASICCTL"]
    #[inline(always)]
    pub const fn fec_cm3_cti_asicctl(&self) -> &FecCm3CtiAsicctl {
        &self.fec_cm3_cti_asicctl
    }
    #[doc = "0x11edc - FEC_CM3_CTI_ITCHINACK"]
    #[inline(always)]
    pub const fn fec_cm3_cti_itchinack(&self) -> &FecCm3CtiItchinack {
        &self.fec_cm3_cti_itchinack
    }
    #[doc = "0x11ee0 - FEC_CM3_CTI_ITTRIGINACK"]
    #[inline(always)]
    pub const fn fec_cm3_cti_ittriginack(&self) -> &FecCm3CtiIttriginack {
        &self.fec_cm3_cti_ittriginack
    }
    #[doc = "0x11ee4 - FEC_CM3_CTI_ITCHOUT"]
    #[inline(always)]
    pub const fn fec_cm3_cti_itchout(&self) -> &FecCm3CtiItchout {
        &self.fec_cm3_cti_itchout
    }
    #[doc = "0x11ee8 - FEC_CM3_CTI_ITTRIGOUT"]
    #[inline(always)]
    pub const fn fec_cm3_cti_ittrigout(&self) -> &FecCm3CtiIttrigout {
        &self.fec_cm3_cti_ittrigout
    }
    #[doc = "0x11eec - FEC_CM3_CTI_ITCHOUTACK"]
    #[inline(always)]
    pub const fn fec_cm3_cti_itchoutack(&self) -> &FecCm3CtiItchoutack {
        &self.fec_cm3_cti_itchoutack
    }
    #[doc = "0x11ef0 - FEC_CM3_CTI_ITTRIGOUTACK"]
    #[inline(always)]
    pub const fn fec_cm3_cti_ittrigoutack(&self) -> &FecCm3CtiIttrigoutack {
        &self.fec_cm3_cti_ittrigoutack
    }
    #[doc = "0x11ef4 - FEC_CM3_CTI_ITCHIN"]
    #[inline(always)]
    pub const fn fec_cm3_cti_itchin(&self) -> &FecCm3CtiItchin {
        &self.fec_cm3_cti_itchin
    }
    #[doc = "0x11ef8 - FEC_CM3_CTI_ITTRIGIN"]
    #[inline(always)]
    pub const fn fec_cm3_cti_ittrigin(&self) -> &FecCm3CtiIttrigin {
        &self.fec_cm3_cti_ittrigin
    }
    #[doc = "0x11f00 - FEC_CM3_CTI_ITCTRL"]
    #[inline(always)]
    pub const fn fec_cm3_cti_itctrl(&self) -> &FecCm3CtiItctrl {
        &self.fec_cm3_cti_itctrl
    }
    #[doc = "0x11fa0 - FEC_CM3_CTI_Claim_Tag_Set"]
    #[inline(always)]
    pub const fn fec_cm3_cti_claim_tag_set(&self) -> &FecCm3CtiClaimTagSet {
        &self.fec_cm3_cti_claim_tag_set
    }
    #[doc = "0x11fa4 - FEC_CM3_CTI_Claim_Tag_Clear"]
    #[inline(always)]
    pub const fn fec_cm3_cti_claim_tag_clear(&self) -> &FecCm3CtiClaimTagClear {
        &self.fec_cm3_cti_claim_tag_clear
    }
    #[doc = "0x11fb0 - FEC_CM3_CTI_Lock_Access_Register"]
    #[inline(always)]
    pub const fn fec_cm3_cti_lock_access_register(&self) -> &FecCm3CtiLockAccessRegister {
        &self.fec_cm3_cti_lock_access_register
    }
    #[doc = "0x11fb4 - FEC_CM3_CTI_Lock_Status_Register"]
    #[inline(always)]
    pub const fn fec_cm3_cti_lock_status_register(&self) -> &FecCm3CtiLockStatusRegister {
        &self.fec_cm3_cti_lock_status_register
    }
    #[doc = "0x11fb8 - FEC_CM3_CTI_Authentication_Status"]
    #[inline(always)]
    pub const fn fec_cm3_cti_authentication_status(&self) -> &FecCm3CtiAuthenticationStatus {
        &self.fec_cm3_cti_authentication_status
    }
    #[doc = "0x11fc8 - FEC_CM3_CTI_Device_ID"]
    #[inline(always)]
    pub const fn fec_cm3_cti_device_id(&self) -> &FecCm3CtiDeviceId {
        &self.fec_cm3_cti_device_id
    }
    #[doc = "0x11fcc - FEC_CM3_CTI_Device_Type_Identifier"]
    #[inline(always)]
    pub const fn fec_cm3_cti_device_type_identifier(&self) -> &FecCm3CtiDeviceTypeIdentifier {
        &self.fec_cm3_cti_device_type_identifier
    }
    #[doc = "0x11fd0 - FEC_CM3_CTI_PeripheralID4"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id4(&self) -> &FecCm3CtiPeripheralId4 {
        &self.fec_cm3_cti_peripheral_id4
    }
    #[doc = "0x11fd4 - FEC_CM3_CTI_PeripheralID5"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id5(&self) -> &FecCm3CtiPeripheralId5 {
        &self.fec_cm3_cti_peripheral_id5
    }
    #[doc = "0x11fd8 - FEC_CM3_CTI_PeripheralID6"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id6(&self) -> &FecCm3CtiPeripheralId6 {
        &self.fec_cm3_cti_peripheral_id6
    }
    #[doc = "0x11fdc - FEC_CM3_CTI_PeripheralID7"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id7(&self) -> &FecCm3CtiPeripheralId7 {
        &self.fec_cm3_cti_peripheral_id7
    }
    #[doc = "0x11fe0 - FEC_CM3_CTI_PeripheralID0"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id0(&self) -> &FecCm3CtiPeripheralId0 {
        &self.fec_cm3_cti_peripheral_id0
    }
    #[doc = "0x11fe4 - FEC_CM3_CTI_PeripheralID1"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id1(&self) -> &FecCm3CtiPeripheralId1 {
        &self.fec_cm3_cti_peripheral_id1
    }
    #[doc = "0x11fe8 - FEC_CM3_CTI_PeripheralID2"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id2(&self) -> &FecCm3CtiPeripheralId2 {
        &self.fec_cm3_cti_peripheral_id2
    }
    #[doc = "0x11fec - FEC_CM3_CTI_PeripheralID3"]
    #[inline(always)]
    pub const fn fec_cm3_cti_peripheral_id3(&self) -> &FecCm3CtiPeripheralId3 {
        &self.fec_cm3_cti_peripheral_id3
    }
    #[doc = "0x11ff0 - FEC_CM3_CTI_Component_ID0"]
    #[inline(always)]
    pub const fn fec_cm3_cti_component_id0(&self) -> &FecCm3CtiComponentId0 {
        &self.fec_cm3_cti_component_id0
    }
    #[doc = "0x11ff4 - FEC_CM3_CTI_Component_ID1"]
    #[inline(always)]
    pub const fn fec_cm3_cti_component_id1(&self) -> &FecCm3CtiComponentId1 {
        &self.fec_cm3_cti_component_id1
    }
    #[doc = "0x11ff8 - FEC_CM3_CTI_Component_ID2"]
    #[inline(always)]
    pub const fn fec_cm3_cti_component_id2(&self) -> &FecCm3CtiComponentId2 {
        &self.fec_cm3_cti_component_id2
    }
    #[doc = "0x11ffc - FEC_CM3_CTI_Component_ID3"]
    #[inline(always)]
    pub const fn fec_cm3_cti_component_id3(&self) -> &FecCm3CtiComponentId3 {
        &self.fec_cm3_cti_component_id3
    }
}
#[doc = "ONEMCU_APB_BASE (rw) register accessor: Start Address of ROM Table\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_apb_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_apb_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_apb_base`]
module"]
#[doc(alias = "ONEMCU_APB_BASE")]
pub type OnemcuApbBase = crate::Reg<onemcu_apb_base::OnemcuApbBaseSpec>;
#[doc = "Start Address of ROM Table"]
pub mod onemcu_apb_base;
#[doc = "ONEMCU_APB_BASE_END (rw) register accessor: End Address of ROM Table\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_apb_base_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_apb_base_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_apb_base_end`]
module"]
#[doc(alias = "ONEMCU_APB_BASE_END")]
pub type OnemcuApbBaseEnd = crate::Reg<onemcu_apb_base_end::OnemcuApbBaseEndSpec>;
#[doc = "End Address of ROM Table"]
pub mod onemcu_apb_base_end;
#[doc = "ONEMCU_CTI_CONTROL (rw) register accessor: http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_control`]
module"]
#[doc(alias = "ONEMCU_CTI_CONTROL")]
pub type OnemcuCtiControl = crate::Reg<onemcu_cti_control::OnemcuCtiControlSpec>;
#[doc = "http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html"]
pub mod onemcu_cti_control;
#[doc = "ONEMCU_CTI_INTACK (rw) register accessor: ONEMCU_CTI_INTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_intack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_intack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_intack`]
module"]
#[doc(alias = "ONEMCU_CTI_INTACK")]
pub type OnemcuCtiIntack = crate::Reg<onemcu_cti_intack::OnemcuCtiIntackSpec>;
#[doc = "ONEMCU_CTI_INTACK"]
pub mod onemcu_cti_intack;
#[doc = "ONEMCU_CTI_APPSET (rw) register accessor: ONEMCU_CTI_APPSET\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_appset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_appset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_appset`]
module"]
#[doc(alias = "ONEMCU_CTI_APPSET")]
pub type OnemcuCtiAppset = crate::Reg<onemcu_cti_appset::OnemcuCtiAppsetSpec>;
#[doc = "ONEMCU_CTI_APPSET"]
pub mod onemcu_cti_appset;
#[doc = "ONEMCU_CTI_APPCLEAR (rw) register accessor: ONEMCU_CTI_APPCLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_appclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_appclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_appclear`]
module"]
#[doc(alias = "ONEMCU_CTI_APPCLEAR")]
pub type OnemcuCtiAppclear = crate::Reg<onemcu_cti_appclear::OnemcuCtiAppclearSpec>;
#[doc = "ONEMCU_CTI_APPCLEAR"]
pub mod onemcu_cti_appclear;
#[doc = "ONEMCU_CTI_APPPULSE (rw) register accessor: ONEMCU_CTI_APPPULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_apppulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_apppulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_apppulse`]
module"]
#[doc(alias = "ONEMCU_CTI_APPPULSE")]
pub type OnemcuCtiApppulse = crate::Reg<onemcu_cti_apppulse::OnemcuCtiApppulseSpec>;
#[doc = "ONEMCU_CTI_APPPULSE"]
pub mod onemcu_cti_apppulse;
#[doc = "ONEMCU_CTI_INEN0 (rw) register accessor: ONEMCU_CTI_INEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen0`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN0")]
pub type OnemcuCtiInen0 = crate::Reg<onemcu_cti_inen0::OnemcuCtiInen0Spec>;
#[doc = "ONEMCU_CTI_INEN0"]
pub mod onemcu_cti_inen0;
#[doc = "ONEMCU_CTI_INEN1 (rw) register accessor: ONEMCU_CTI_INEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen1`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN1")]
pub type OnemcuCtiInen1 = crate::Reg<onemcu_cti_inen1::OnemcuCtiInen1Spec>;
#[doc = "ONEMCU_CTI_INEN1"]
pub mod onemcu_cti_inen1;
#[doc = "ONEMCU_CTI_INEN2 (rw) register accessor: ONEMCU_CTI_INEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen2`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN2")]
pub type OnemcuCtiInen2 = crate::Reg<onemcu_cti_inen2::OnemcuCtiInen2Spec>;
#[doc = "ONEMCU_CTI_INEN2"]
pub mod onemcu_cti_inen2;
#[doc = "ONEMCU_CTI_INEN3 (rw) register accessor: ONEMCU_CTI_INEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen3`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN3")]
pub type OnemcuCtiInen3 = crate::Reg<onemcu_cti_inen3::OnemcuCtiInen3Spec>;
#[doc = "ONEMCU_CTI_INEN3"]
pub mod onemcu_cti_inen3;
#[doc = "ONEMCU_CTI_INEN4 (rw) register accessor: ONEMCU_CTI_INEN4\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen4`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN4")]
pub type OnemcuCtiInen4 = crate::Reg<onemcu_cti_inen4::OnemcuCtiInen4Spec>;
#[doc = "ONEMCU_CTI_INEN4"]
pub mod onemcu_cti_inen4;
#[doc = "ONEMCU_CTI_INEN5 (rw) register accessor: ONEMCU_CTI_INEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen5`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN5")]
pub type OnemcuCtiInen5 = crate::Reg<onemcu_cti_inen5::OnemcuCtiInen5Spec>;
#[doc = "ONEMCU_CTI_INEN5"]
pub mod onemcu_cti_inen5;
#[doc = "ONEMCU_CTI_INEN6 (rw) register accessor: ONEMCU_CTI_INEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen6`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN6")]
pub type OnemcuCtiInen6 = crate::Reg<onemcu_cti_inen6::OnemcuCtiInen6Spec>;
#[doc = "ONEMCU_CTI_INEN6"]
pub mod onemcu_cti_inen6;
#[doc = "ONEMCU_CTI_INEN7 (rw) register accessor: ONEMCU_CTI_INEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_inen7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_inen7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_inen7`]
module"]
#[doc(alias = "ONEMCU_CTI_INEN7")]
pub type OnemcuCtiInen7 = crate::Reg<onemcu_cti_inen7::OnemcuCtiInen7Spec>;
#[doc = "ONEMCU_CTI_INEN7"]
pub mod onemcu_cti_inen7;
#[doc = "ONEMCU_CTI_OUTEN0 (rw) register accessor: ONEMCU_CTI_OUTEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen0`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN0")]
pub type OnemcuCtiOuten0 = crate::Reg<onemcu_cti_outen0::OnemcuCtiOuten0Spec>;
#[doc = "ONEMCU_CTI_OUTEN0"]
pub mod onemcu_cti_outen0;
#[doc = "ONEMCU_CTI_OUTEN1 (rw) register accessor: ONEMCU_CTI_OUTEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen1`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN1")]
pub type OnemcuCtiOuten1 = crate::Reg<onemcu_cti_outen1::OnemcuCtiOuten1Spec>;
#[doc = "ONEMCU_CTI_OUTEN1"]
pub mod onemcu_cti_outen1;
#[doc = "ONEMCU_CTI_OUTEN2 (rw) register accessor: ONEMCU_CTI_OUTEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen2`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN2")]
pub type OnemcuCtiOuten2 = crate::Reg<onemcu_cti_outen2::OnemcuCtiOuten2Spec>;
#[doc = "ONEMCU_CTI_OUTEN2"]
pub mod onemcu_cti_outen2;
#[doc = "ONEMCU_CTI_OUTEN3 (rw) register accessor: ONEMCU_CTI_OUTEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen3`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN3")]
pub type OnemcuCtiOuten3 = crate::Reg<onemcu_cti_outen3::OnemcuCtiOuten3Spec>;
#[doc = "ONEMCU_CTI_OUTEN3"]
pub mod onemcu_cti_outen3;
#[doc = "ONEMCU_CTI_OUTEN4 (rw) register accessor: ONEMCU_CTI_OUTEN4\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen4`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN4")]
pub type OnemcuCtiOuten4 = crate::Reg<onemcu_cti_outen4::OnemcuCtiOuten4Spec>;
#[doc = "ONEMCU_CTI_OUTEN4"]
pub mod onemcu_cti_outen4;
#[doc = "ONEMCU_CTI_OUTEN5 (rw) register accessor: ONEMCU_CTI_OUTEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen5`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN5")]
pub type OnemcuCtiOuten5 = crate::Reg<onemcu_cti_outen5::OnemcuCtiOuten5Spec>;
#[doc = "ONEMCU_CTI_OUTEN5"]
pub mod onemcu_cti_outen5;
#[doc = "ONEMCU_CTI_OUTEN6 (rw) register accessor: ONEMCU_CTI_OUTEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen6`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN6")]
pub type OnemcuCtiOuten6 = crate::Reg<onemcu_cti_outen6::OnemcuCtiOuten6Spec>;
#[doc = "ONEMCU_CTI_OUTEN6"]
pub mod onemcu_cti_outen6;
#[doc = "ONEMCU_CTI_OUTEN7 (rw) register accessor: ONEMCU_CTI_OUTEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_outen7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_outen7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_outen7`]
module"]
#[doc(alias = "ONEMCU_CTI_OUTEN7")]
pub type OnemcuCtiOuten7 = crate::Reg<onemcu_cti_outen7::OnemcuCtiOuten7Spec>;
#[doc = "ONEMCU_CTI_OUTEN7"]
pub mod onemcu_cti_outen7;
#[doc = "ONEMCU_CTI_TRIGINSTATUS (rw) register accessor: ONEMCU_CTI_TRIGINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_triginstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_triginstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_triginstatus`]
module"]
#[doc(alias = "ONEMCU_CTI_TRIGINSTATUS")]
pub type OnemcuCtiTriginstatus = crate::Reg<onemcu_cti_triginstatus::OnemcuCtiTriginstatusSpec>;
#[doc = "ONEMCU_CTI_TRIGINSTATUS"]
pub mod onemcu_cti_triginstatus;
#[doc = "ONEMCU_CTI_TRIGOUTSTATUS (rw) register accessor: ONEMCU_CTI_TRIGOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_trigoutstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_trigoutstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_trigoutstatus`]
module"]
#[doc(alias = "ONEMCU_CTI_TRIGOUTSTATUS")]
pub type OnemcuCtiTrigoutstatus = crate::Reg<onemcu_cti_trigoutstatus::OnemcuCtiTrigoutstatusSpec>;
#[doc = "ONEMCU_CTI_TRIGOUTSTATUS"]
pub mod onemcu_cti_trigoutstatus;
#[doc = "ONEMCU_CTI_CHINSTATUS (rw) register accessor: ONEMCU_CTI_CHINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_chinstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_chinstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_chinstatus`]
module"]
#[doc(alias = "ONEMCU_CTI_CHINSTATUS")]
pub type OnemcuCtiChinstatus = crate::Reg<onemcu_cti_chinstatus::OnemcuCtiChinstatusSpec>;
#[doc = "ONEMCU_CTI_CHINSTATUS"]
pub mod onemcu_cti_chinstatus;
#[doc = "ONEMCU_CTI_CHOUTSTATUS (rw) register accessor: ONEMCU_CTI_CHOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_choutstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_choutstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_choutstatus`]
module"]
#[doc(alias = "ONEMCU_CTI_CHOUTSTATUS")]
pub type OnemcuCtiChoutstatus = crate::Reg<onemcu_cti_choutstatus::OnemcuCtiChoutstatusSpec>;
#[doc = "ONEMCU_CTI_CHOUTSTATUS"]
pub mod onemcu_cti_choutstatus;
#[doc = "ONEMCU_CTI_GATE (rw) register accessor: ONEMCU_CTI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_gate`]
module"]
#[doc(alias = "ONEMCU_CTI_GATE")]
pub type OnemcuCtiGate = crate::Reg<onemcu_cti_gate::OnemcuCtiGateSpec>;
#[doc = "ONEMCU_CTI_GATE"]
pub mod onemcu_cti_gate;
#[doc = "ONEMCU_CTI_ASICCTL (rw) register accessor: ONEMCU_CTI_ASICCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_asicctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_asicctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_asicctl`]
module"]
#[doc(alias = "ONEMCU_CTI_ASICCTL")]
pub type OnemcuCtiAsicctl = crate::Reg<onemcu_cti_asicctl::OnemcuCtiAsicctlSpec>;
#[doc = "ONEMCU_CTI_ASICCTL"]
pub mod onemcu_cti_asicctl;
#[doc = "ONEMCU_CTI_ITCHINACK (rw) register accessor: ONEMCU_CTI_ITCHINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchinack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchinack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_itchinack`]
module"]
#[doc(alias = "ONEMCU_CTI_ITCHINACK")]
pub type OnemcuCtiItchinack = crate::Reg<onemcu_cti_itchinack::OnemcuCtiItchinackSpec>;
#[doc = "ONEMCU_CTI_ITCHINACK"]
pub mod onemcu_cti_itchinack;
#[doc = "ONEMCU_CTI_ITTRIGINACK (rw) register accessor: ONEMCU_CTI_ITTRIGINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittriginack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittriginack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_ittriginack`]
module"]
#[doc(alias = "ONEMCU_CTI_ITTRIGINACK")]
pub type OnemcuCtiIttriginack = crate::Reg<onemcu_cti_ittriginack::OnemcuCtiIttriginackSpec>;
#[doc = "ONEMCU_CTI_ITTRIGINACK"]
pub mod onemcu_cti_ittriginack;
#[doc = "ONEMCU_CTI_ITCHOUT (rw) register accessor: ONEMCU_CTI_ITCHOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_itchout`]
module"]
#[doc(alias = "ONEMCU_CTI_ITCHOUT")]
pub type OnemcuCtiItchout = crate::Reg<onemcu_cti_itchout::OnemcuCtiItchoutSpec>;
#[doc = "ONEMCU_CTI_ITCHOUT"]
pub mod onemcu_cti_itchout;
#[doc = "ONEMCU_CTI_ITTRIGOUT (rw) register accessor: ONEMCU_CTI_ITTRIGOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittrigout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittrigout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_ittrigout`]
module"]
#[doc(alias = "ONEMCU_CTI_ITTRIGOUT")]
pub type OnemcuCtiIttrigout = crate::Reg<onemcu_cti_ittrigout::OnemcuCtiIttrigoutSpec>;
#[doc = "ONEMCU_CTI_ITTRIGOUT"]
pub mod onemcu_cti_ittrigout;
#[doc = "ONEMCU_CTI_ITCHOUTACK (rw) register accessor: ONEMCU_CTI_ITCHOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchoutack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchoutack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_itchoutack`]
module"]
#[doc(alias = "ONEMCU_CTI_ITCHOUTACK")]
pub type OnemcuCtiItchoutack = crate::Reg<onemcu_cti_itchoutack::OnemcuCtiItchoutackSpec>;
#[doc = "ONEMCU_CTI_ITCHOUTACK"]
pub mod onemcu_cti_itchoutack;
#[doc = "ONEMCU_CTI_ITTRIGOUTACK (rw) register accessor: ONEMCU_CTI_ITTRIGOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittrigoutack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittrigoutack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_ittrigoutack`]
module"]
#[doc(alias = "ONEMCU_CTI_ITTRIGOUTACK")]
pub type OnemcuCtiIttrigoutack = crate::Reg<onemcu_cti_ittrigoutack::OnemcuCtiIttrigoutackSpec>;
#[doc = "ONEMCU_CTI_ITTRIGOUTACK"]
pub mod onemcu_cti_ittrigoutack;
#[doc = "ONEMCU_CTI_ITCHIN (rw) register accessor: ONEMCU_CTI_ITCHIN\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itchin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itchin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_itchin`]
module"]
#[doc(alias = "ONEMCU_CTI_ITCHIN")]
pub type OnemcuCtiItchin = crate::Reg<onemcu_cti_itchin::OnemcuCtiItchinSpec>;
#[doc = "ONEMCU_CTI_ITCHIN"]
pub mod onemcu_cti_itchin;
#[doc = "ONEMCU_CTI_ITTRIGIN (rw) register accessor: ONEMCU_CTI_ITTRIGIN\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_ittrigin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_ittrigin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_ittrigin`]
module"]
#[doc(alias = "ONEMCU_CTI_ITTRIGIN")]
pub type OnemcuCtiIttrigin = crate::Reg<onemcu_cti_ittrigin::OnemcuCtiIttriginSpec>;
#[doc = "ONEMCU_CTI_ITTRIGIN"]
pub mod onemcu_cti_ittrigin;
#[doc = "ONEMCU_CTI_ITCTRL (rw) register accessor: ONEMCU_CTI_ITCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_itctrl`]
module"]
#[doc(alias = "ONEMCU_CTI_ITCTRL")]
pub type OnemcuCtiItctrl = crate::Reg<onemcu_cti_itctrl::OnemcuCtiItctrlSpec>;
#[doc = "ONEMCU_CTI_ITCTRL"]
pub mod onemcu_cti_itctrl;
#[doc = "ONEMCU_CTI_Claim_Tag_Set (rw) register accessor: ONEMCU_CTI_Claim_Tag_Set\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_claim_tag_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_claim_tag_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_claim_tag_set`]
module"]
#[doc(alias = "ONEMCU_CTI_Claim_Tag_Set")]
pub type OnemcuCtiClaimTagSet = crate::Reg<onemcu_cti_claim_tag_set::OnemcuCtiClaimTagSetSpec>;
#[doc = "ONEMCU_CTI_Claim_Tag_Set"]
pub mod onemcu_cti_claim_tag_set;
#[doc = "ONEMCU_CTI_Claim_Tag_Clear (rw) register accessor: ONEMCU_CTI_Claim_Tag_Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_claim_tag_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_claim_tag_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_claim_tag_clear`]
module"]
#[doc(alias = "ONEMCU_CTI_Claim_Tag_Clear")]
pub type OnemcuCtiClaimTagClear =
    crate::Reg<onemcu_cti_claim_tag_clear::OnemcuCtiClaimTagClearSpec>;
#[doc = "ONEMCU_CTI_Claim_Tag_Clear"]
pub mod onemcu_cti_claim_tag_clear;
#[doc = "ONEMCU_CTI_Lock_Access_Register (rw) register accessor: ONEMCU_CTI_Lock_Access_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_lock_access_register::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_lock_access_register::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_lock_access_register`]
module"]
#[doc(alias = "ONEMCU_CTI_Lock_Access_Register")]
pub type OnemcuCtiLockAccessRegister =
    crate::Reg<onemcu_cti_lock_access_register::OnemcuCtiLockAccessRegisterSpec>;
#[doc = "ONEMCU_CTI_Lock_Access_Register"]
pub mod onemcu_cti_lock_access_register;
#[doc = "ONEMCU_CTI_Lock_Status_Register (rw) register accessor: ONEMCU_CTI_Lock_Status_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_lock_status_register::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_lock_status_register::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_lock_status_register`]
module"]
#[doc(alias = "ONEMCU_CTI_Lock_Status_Register")]
pub type OnemcuCtiLockStatusRegister =
    crate::Reg<onemcu_cti_lock_status_register::OnemcuCtiLockStatusRegisterSpec>;
#[doc = "ONEMCU_CTI_Lock_Status_Register"]
pub mod onemcu_cti_lock_status_register;
#[doc = "ONEMCU_CTI_Authentication_Status (rw) register accessor: ONEMCU_CTI_Authentication_Status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_authentication_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_authentication_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_authentication_status`]
module"]
#[doc(alias = "ONEMCU_CTI_Authentication_Status")]
pub type OnemcuCtiAuthenticationStatus =
    crate::Reg<onemcu_cti_authentication_status::OnemcuCtiAuthenticationStatusSpec>;
#[doc = "ONEMCU_CTI_Authentication_Status"]
pub mod onemcu_cti_authentication_status;
#[doc = "ONEMCU_CTI_Device_ID (rw) register accessor: ONEMCU_CTI_Device_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_device_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_device_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_device_id`]
module"]
#[doc(alias = "ONEMCU_CTI_Device_ID")]
pub type OnemcuCtiDeviceId = crate::Reg<onemcu_cti_device_id::OnemcuCtiDeviceIdSpec>;
#[doc = "ONEMCU_CTI_Device_ID"]
pub mod onemcu_cti_device_id;
#[doc = "ONEMCU_CTI_Device_Type_Identifier (rw) register accessor: ONEMCU_CTI_Device_Type_Identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_device_type_identifier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_device_type_identifier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_device_type_identifier`]
module"]
#[doc(alias = "ONEMCU_CTI_Device_Type_Identifier")]
pub type OnemcuCtiDeviceTypeIdentifier =
    crate::Reg<onemcu_cti_device_type_identifier::OnemcuCtiDeviceTypeIdentifierSpec>;
#[doc = "ONEMCU_CTI_Device_Type_Identifier"]
pub mod onemcu_cti_device_type_identifier;
#[doc = "ONEMCU_CTI_PeripheralID4 (rw) register accessor: ONEMCU_CTI_PeripheralID4\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id4`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID4")]
pub type OnemcuCtiPeripheralId4 = crate::Reg<onemcu_cti_peripheral_id4::OnemcuCtiPeripheralId4Spec>;
#[doc = "ONEMCU_CTI_PeripheralID4"]
pub mod onemcu_cti_peripheral_id4;
#[doc = "ONEMCU_CTI_PeripheralID5 (rw) register accessor: ONEMCU_CTI_PeripheralID5\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id5`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID5")]
pub type OnemcuCtiPeripheralId5 = crate::Reg<onemcu_cti_peripheral_id5::OnemcuCtiPeripheralId5Spec>;
#[doc = "ONEMCU_CTI_PeripheralID5"]
pub mod onemcu_cti_peripheral_id5;
#[doc = "ONEMCU_CTI_PeripheralID6 (rw) register accessor: ONEMCU_CTI_PeripheralID6\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id6`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID6")]
pub type OnemcuCtiPeripheralId6 = crate::Reg<onemcu_cti_peripheral_id6::OnemcuCtiPeripheralId6Spec>;
#[doc = "ONEMCU_CTI_PeripheralID6"]
pub mod onemcu_cti_peripheral_id6;
#[doc = "ONEMCU_CTI_PeripheralID7 (rw) register accessor: ONEMCU_CTI_PeripheralID7\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id7`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID7")]
pub type OnemcuCtiPeripheralId7 = crate::Reg<onemcu_cti_peripheral_id7::OnemcuCtiPeripheralId7Spec>;
#[doc = "ONEMCU_CTI_PeripheralID7"]
pub mod onemcu_cti_peripheral_id7;
#[doc = "ONEMCU_CTI_PeripheralID0 (rw) register accessor: ONEMCU_CTI_PeripheralID0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id0`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID0")]
pub type OnemcuCtiPeripheralId0 = crate::Reg<onemcu_cti_peripheral_id0::OnemcuCtiPeripheralId0Spec>;
#[doc = "ONEMCU_CTI_PeripheralID0"]
pub mod onemcu_cti_peripheral_id0;
#[doc = "ONEMCU_CTI_PeripheralID1 (rw) register accessor: ONEMCU_CTI_PeripheralID1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id1`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID1")]
pub type OnemcuCtiPeripheralId1 = crate::Reg<onemcu_cti_peripheral_id1::OnemcuCtiPeripheralId1Spec>;
#[doc = "ONEMCU_CTI_PeripheralID1"]
pub mod onemcu_cti_peripheral_id1;
#[doc = "ONEMCU_CTI_PeripheralID2 (rw) register accessor: ONEMCU_CTI_PeripheralID2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id2`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID2")]
pub type OnemcuCtiPeripheralId2 = crate::Reg<onemcu_cti_peripheral_id2::OnemcuCtiPeripheralId2Spec>;
#[doc = "ONEMCU_CTI_PeripheralID2"]
pub mod onemcu_cti_peripheral_id2;
#[doc = "ONEMCU_CTI_PeripheralID3 (rw) register accessor: ONEMCU_CTI_PeripheralID3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_peripheral_id3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_peripheral_id3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_peripheral_id3`]
module"]
#[doc(alias = "ONEMCU_CTI_PeripheralID3")]
pub type OnemcuCtiPeripheralId3 = crate::Reg<onemcu_cti_peripheral_id3::OnemcuCtiPeripheralId3Spec>;
#[doc = "ONEMCU_CTI_PeripheralID3"]
pub mod onemcu_cti_peripheral_id3;
#[doc = "ONEMCU_CTI_Component_ID0 (rw) register accessor: ONEMCU_CTI_Component_ID0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_component_id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_component_id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_component_id0`]
module"]
#[doc(alias = "ONEMCU_CTI_Component_ID0")]
pub type OnemcuCtiComponentId0 = crate::Reg<onemcu_cti_component_id0::OnemcuCtiComponentId0Spec>;
#[doc = "ONEMCU_CTI_Component_ID0"]
pub mod onemcu_cti_component_id0;
#[doc = "ONEMCU_CTI_Component_ID1 (rw) register accessor: ONEMCU_CTI_Component_ID1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_component_id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_component_id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_component_id1`]
module"]
#[doc(alias = "ONEMCU_CTI_Component_ID1")]
pub type OnemcuCtiComponentId1 = crate::Reg<onemcu_cti_component_id1::OnemcuCtiComponentId1Spec>;
#[doc = "ONEMCU_CTI_Component_ID1"]
pub mod onemcu_cti_component_id1;
#[doc = "ONEMCU_CTI_Component_ID2 (rw) register accessor: ONEMCU_CTI_Component_ID2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_component_id2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_component_id2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_component_id2`]
module"]
#[doc(alias = "ONEMCU_CTI_Component_ID2")]
pub type OnemcuCtiComponentId2 = crate::Reg<onemcu_cti_component_id2::OnemcuCtiComponentId2Spec>;
#[doc = "ONEMCU_CTI_Component_ID2"]
pub mod onemcu_cti_component_id2;
#[doc = "ONEMCU_CTI_Component_ID3 (rw) register accessor: ONEMCU_CTI_Component_ID3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_component_id3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_component_id3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_cti_component_id3`]
module"]
#[doc(alias = "ONEMCU_CTI_Component_ID3")]
pub type OnemcuCtiComponentId3 = crate::Reg<onemcu_cti_component_id3::OnemcuCtiComponentId3Spec>;
#[doc = "ONEMCU_CTI_Component_ID3"]
pub mod onemcu_cti_component_id3;
#[doc = "ONEMCU_TPIU_SPORTSZ (rw) register accessor: Supported port sizes\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_sportsz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_sportsz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_sportsz`]
module"]
#[doc(alias = "ONEMCU_TPIU_SPORTSZ")]
pub type OnemcuTpiuSportsz = crate::Reg<onemcu_tpiu_sportsz::OnemcuTpiuSportszSpec>;
#[doc = "Supported port sizes"]
pub mod onemcu_tpiu_sportsz;
#[doc = "ONEMCU_TPIU_CPORTSZ (rw) register accessor: Current port size\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cportsz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cportsz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_cportsz`]
module"]
#[doc(alias = "ONEMCU_TPIU_CPORTSZ")]
pub type OnemcuTpiuCportsz = crate::Reg<onemcu_tpiu_cportsz::OnemcuTpiuCportszSpec>;
#[doc = "Current port size"]
pub mod onemcu_tpiu_cportsz;
#[doc = "ONEMCU_TPIU_STRIGM (rw) register accessor: Supported trigger modes\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_strigm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_strigm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_strigm`]
module"]
#[doc(alias = "ONEMCU_TPIU_STRIGM")]
pub type OnemcuTpiuStrigm = crate::Reg<onemcu_tpiu_strigm::OnemcuTpiuStrigmSpec>;
#[doc = "Supported trigger modes"]
pub mod onemcu_tpiu_strigm;
#[doc = "ONEMCU_TPIU_TRIGCNT (rw) register accessor: Trigger counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_trigcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_trigcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_trigcnt`]
module"]
#[doc(alias = "ONEMCU_TPIU_TRIGCNT")]
pub type OnemcuTpiuTrigcnt = crate::Reg<onemcu_tpiu_trigcnt::OnemcuTpiuTrigcntSpec>;
#[doc = "Trigger counter value"]
pub mod onemcu_tpiu_trigcnt;
#[doc = "ONEMCU_TPIU_TRIGMUL (rw) register accessor: Trigger multiplier\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_trigmul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_trigmul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_trigmul`]
module"]
#[doc(alias = "ONEMCU_TPIU_TRIGMUL")]
pub type OnemcuTpiuTrigmul = crate::Reg<onemcu_tpiu_trigmul::OnemcuTpiuTrigmulSpec>;
#[doc = "Trigger multiplier"]
pub mod onemcu_tpiu_trigmul;
#[doc = "ONEMCU_TPIU_STSTPTRN (rw) register accessor: Supported test pattern/modes\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ststptrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ststptrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_ststptrn`]
module"]
#[doc(alias = "ONEMCU_TPIU_STSTPTRN")]
pub type OnemcuTpiuStstptrn = crate::Reg<onemcu_tpiu_ststptrn::OnemcuTpiuStstptrnSpec>;
#[doc = "Supported test pattern/modes"]
pub mod onemcu_tpiu_ststptrn;
#[doc = "ONEMCU_TPIU_CTSTPTRN (rw) register accessor: Current test pattern/mode\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ctstptrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ctstptrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_ctstptrn`]
module"]
#[doc(alias = "ONEMCU_TPIU_CTSTPTRN")]
pub type OnemcuTpiuCtstptrn = crate::Reg<onemcu_tpiu_ctstptrn::OnemcuTpiuCtstptrnSpec>;
#[doc = "Current test pattern/mode"]
pub mod onemcu_tpiu_ctstptrn;
#[doc = "ONEMCU_TPIU_TPRCNTR (rw) register accessor: Test pattern repeat counter\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_tprcntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_tprcntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_tprcntr`]
module"]
#[doc(alias = "ONEMCU_TPIU_TPRCNTR")]
pub type OnemcuTpiuTprcntr = crate::Reg<onemcu_tpiu_tprcntr::OnemcuTpiuTprcntrSpec>;
#[doc = "Test pattern repeat counter"]
pub mod onemcu_tpiu_tprcntr;
#[doc = "ONEMCU_TPIU_FFSTS (rw) register accessor: Formatter and flush status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ffsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ffsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_ffsts`]
module"]
#[doc(alias = "ONEMCU_TPIU_FFSTS")]
pub type OnemcuTpiuFfsts = crate::Reg<onemcu_tpiu_ffsts::OnemcuTpiuFfstsSpec>;
#[doc = "Formatter and flush status"]
pub mod onemcu_tpiu_ffsts;
#[doc = "ONEMCU_TPIU_FFCTRL (rw) register accessor: Formatter and flush control\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ffctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ffctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_ffctrl`]
module"]
#[doc(alias = "ONEMCU_TPIU_FFCTRL")]
pub type OnemcuTpiuFfctrl = crate::Reg<onemcu_tpiu_ffctrl::OnemcuTpiuFfctrlSpec>;
#[doc = "Formatter and flush control"]
pub mod onemcu_tpiu_ffctrl;
#[doc = "ONEMCU_TPIU_FSCNTR (rw) register accessor: Formatter synchronization counter\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_fscntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_fscntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_fscntr`]
module"]
#[doc(alias = "ONEMCU_TPIU_FSCNTR")]
pub type OnemcuTpiuFscntr = crate::Reg<onemcu_tpiu_fscntr::OnemcuTpiuFscntrSpec>;
#[doc = "Formatter synchronization counter"]
pub mod onemcu_tpiu_fscntr;
#[doc = "ONEMCU_TPIU_EXCTLIN (rw) register accessor: EXTCTL In Port\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_exctlin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_exctlin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_exctlin`]
module"]
#[doc(alias = "ONEMCU_TPIU_EXCTLIN")]
pub type OnemcuTpiuExctlin = crate::Reg<onemcu_tpiu_exctlin::OnemcuTpiuExctlinSpec>;
#[doc = "EXTCTL In Port"]
pub mod onemcu_tpiu_exctlin;
#[doc = "ONEMCU_TPIU_EXCTLOUT (rw) register accessor: EXTCTL Out Port\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_exctlout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_exctlout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_exctlout`]
module"]
#[doc(alias = "ONEMCU_TPIU_EXCTLOUT")]
pub type OnemcuTpiuExctlout = crate::Reg<onemcu_tpiu_exctlout::OnemcuTpiuExctloutSpec>;
#[doc = "EXTCTL Out Port"]
pub mod onemcu_tpiu_exctlout;
#[doc = "ONEMCU_TPIU_ITTRFLINACK (rw) register accessor: Integration Register, ITTRFLINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ittrflinack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ittrflinack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_ittrflinack`]
module"]
#[doc(alias = "ONEMCU_TPIU_ITTRFLINACK")]
pub type OnemcuTpiuIttrflinack = crate::Reg<onemcu_tpiu_ittrflinack::OnemcuTpiuIttrflinackSpec>;
#[doc = "Integration Register, ITTRFLINACK"]
pub mod onemcu_tpiu_ittrflinack;
#[doc = "ONEMCU_TPIU_ITTRFLIN (rw) register accessor: Integration Register, ITTRFLIN\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ittrflin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ittrflin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_ittrflin`]
module"]
#[doc(alias = "ONEMCU_TPIU_ITTRFLIN")]
pub type OnemcuTpiuIttrflin = crate::Reg<onemcu_tpiu_ittrflin::OnemcuTpiuIttrflinSpec>;
#[doc = "Integration Register, ITTRFLIN"]
pub mod onemcu_tpiu_ittrflin;
#[doc = "ONEMCU_TPIU_ITATBDATA0 (rw) register accessor: Integration Register, ITATBDATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_itatbdata0`]
module"]
#[doc(alias = "ONEMCU_TPIU_ITATBDATA0")]
pub type OnemcuTpiuItatbdata0 = crate::Reg<onemcu_tpiu_itatbdata0::OnemcuTpiuItatbdata0Spec>;
#[doc = "Integration Register, ITATBDATA0"]
pub mod onemcu_tpiu_itatbdata0;
#[doc = "ONEMCU_TPIU_ITATBCTR2 (rw) register accessor: Integration Register, ITATBCTR2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbctr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbctr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_itatbctr2`]
module"]
#[doc(alias = "ONEMCU_TPIU_ITATBCTR2")]
pub type OnemcuTpiuItatbctr2 = crate::Reg<onemcu_tpiu_itatbctr2::OnemcuTpiuItatbctr2Spec>;
#[doc = "Integration Register, ITATBCTR2"]
pub mod onemcu_tpiu_itatbctr2;
#[doc = "ONEMCU_TPIU_ITATBCTR1 (rw) register accessor: Integration Register, ITATBCTR1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbctr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbctr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_itatbctr1`]
module"]
#[doc(alias = "ONEMCU_TPIU_ITATBCTR1")]
pub type OnemcuTpiuItatbctr1 = crate::Reg<onemcu_tpiu_itatbctr1::OnemcuTpiuItatbctr1Spec>;
#[doc = "Integration Register, ITATBCTR1"]
pub mod onemcu_tpiu_itatbctr1;
#[doc = "ONEMCU_TPIU_ITATBCTR0 (rw) register accessor: Integration Register, ITATBCTR0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itatbctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itatbctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_itatbctr0`]
module"]
#[doc(alias = "ONEMCU_TPIU_ITATBCTR0")]
pub type OnemcuTpiuItatbctr0 = crate::Reg<onemcu_tpiu_itatbctr0::OnemcuTpiuItatbctr0Spec>;
#[doc = "Integration Register, ITATBCTR0"]
pub mod onemcu_tpiu_itatbctr0;
#[doc = "ONEMCU_TPIU_ITCTRL (rw) register accessor: Integration Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_itctrl`]
module"]
#[doc(alias = "ONEMCU_TPIU_ITCTRL")]
pub type OnemcuTpiuItctrl = crate::Reg<onemcu_tpiu_itctrl::OnemcuTpiuItctrlSpec>;
#[doc = "Integration Mode Control Register"]
pub mod onemcu_tpiu_itctrl;
#[doc = "ONEMCU_TPIU_CLAIMSET (rw) register accessor: Claim Tag Set\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_claimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_claimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_claimset`]
module"]
#[doc(alias = "ONEMCU_TPIU_CLAIMSET")]
pub type OnemcuTpiuClaimset = crate::Reg<onemcu_tpiu_claimset::OnemcuTpiuClaimsetSpec>;
#[doc = "Claim Tag Set"]
pub mod onemcu_tpiu_claimset;
#[doc = "ONEMCU_TPIU_CLAIMCLR (rw) register accessor: Claim Tag Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_claimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_claimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_claimclr`]
module"]
#[doc(alias = "ONEMCU_TPIU_CLAIMCLR")]
pub type OnemcuTpiuClaimclr = crate::Reg<onemcu_tpiu_claimclr::OnemcuTpiuClaimclrSpec>;
#[doc = "Claim Tag Clear"]
pub mod onemcu_tpiu_claimclr;
#[doc = "ONEMCU_TPIU_LAR (rw) register accessor: Lock status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_lar`]
module"]
#[doc(alias = "ONEMCU_TPIU_LAR")]
pub type OnemcuTpiuLar = crate::Reg<onemcu_tpiu_lar::OnemcuTpiuLarSpec>;
#[doc = "Lock status"]
pub mod onemcu_tpiu_lar;
#[doc = "ONEMCU_TPIU_LSR (rw) register accessor: Lock Access\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_lsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_lsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_lsr`]
module"]
#[doc(alias = "ONEMCU_TPIU_LSR")]
pub type OnemcuTpiuLsr = crate::Reg<onemcu_tpiu_lsr::OnemcuTpiuLsrSpec>;
#[doc = "Lock Access"]
pub mod onemcu_tpiu_lsr;
#[doc = "ONEMCU_TPIU_AUTHSTATUS (rw) register accessor: Authentication status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_authstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_authstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_authstatus`]
module"]
#[doc(alias = "ONEMCU_TPIU_AUTHSTATUS")]
pub type OnemcuTpiuAuthstatus = crate::Reg<onemcu_tpiu_authstatus::OnemcuTpiuAuthstatusSpec>;
#[doc = "Authentication status"]
pub mod onemcu_tpiu_authstatus;
#[doc = "ONEMCU_TPIU_DEVID (rw) register accessor: Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_devid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_devid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_devid`]
module"]
#[doc(alias = "ONEMCU_TPIU_DEVID")]
pub type OnemcuTpiuDevid = crate::Reg<onemcu_tpiu_devid::OnemcuTpiuDevidSpec>;
#[doc = "Device ID"]
pub mod onemcu_tpiu_devid;
#[doc = "ONEMCU_TPIU_DEVTYPE (rw) register accessor: Device type identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_devtype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_devtype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_devtype`]
module"]
#[doc(alias = "ONEMCU_TPIU_DEVTYPE")]
pub type OnemcuTpiuDevtype = crate::Reg<onemcu_tpiu_devtype::OnemcuTpiuDevtypeSpec>;
#[doc = "Device type identifier"]
pub mod onemcu_tpiu_devtype;
#[doc = "ONEMCU_TPIU_PIDR4 (rw) register accessor: Peripheral ID4\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr4`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR4")]
pub type OnemcuTpiuPidr4 = crate::Reg<onemcu_tpiu_pidr4::OnemcuTpiuPidr4Spec>;
#[doc = "Peripheral ID4"]
pub mod onemcu_tpiu_pidr4;
#[doc = "ONEMCU_TPIU_PIDR5 (rw) register accessor: Peripheral ID5\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr5`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR5")]
pub type OnemcuTpiuPidr5 = crate::Reg<onemcu_tpiu_pidr5::OnemcuTpiuPidr5Spec>;
#[doc = "Peripheral ID5"]
pub mod onemcu_tpiu_pidr5;
#[doc = "ONEMCU_TPIU_PIDR6 (rw) register accessor: Peripheral ID6\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr6`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR6")]
pub type OnemcuTpiuPidr6 = crate::Reg<onemcu_tpiu_pidr6::OnemcuTpiuPidr6Spec>;
#[doc = "Peripheral ID6"]
pub mod onemcu_tpiu_pidr6;
#[doc = "ONEMCU_TPIU_PIDR7 (rw) register accessor: Peripheral ID7\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr7`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR7")]
pub type OnemcuTpiuPidr7 = crate::Reg<onemcu_tpiu_pidr7::OnemcuTpiuPidr7Spec>;
#[doc = "Peripheral ID7"]
pub mod onemcu_tpiu_pidr7;
#[doc = "ONEMCU_TPIU_PIDR0 (rw) register accessor: Peripheral ID0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr0`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR0")]
pub type OnemcuTpiuPidr0 = crate::Reg<onemcu_tpiu_pidr0::OnemcuTpiuPidr0Spec>;
#[doc = "Peripheral ID0"]
pub mod onemcu_tpiu_pidr0;
#[doc = "ONEMCU_TPIU_PIDR1 (rw) register accessor: Peripheral ID1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr1`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR1")]
pub type OnemcuTpiuPidr1 = crate::Reg<onemcu_tpiu_pidr1::OnemcuTpiuPidr1Spec>;
#[doc = "Peripheral ID1"]
pub mod onemcu_tpiu_pidr1;
#[doc = "ONEMCU_TPIU_PIDR2 (rw) register accessor: Peripheral ID2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr2`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR2")]
pub type OnemcuTpiuPidr2 = crate::Reg<onemcu_tpiu_pidr2::OnemcuTpiuPidr2Spec>;
#[doc = "Peripheral ID2"]
pub mod onemcu_tpiu_pidr2;
#[doc = "ONEMCU_TPIU_PIDR3 (rw) register accessor: Peripheral ID3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_pidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_pidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_pidr3`]
module"]
#[doc(alias = "ONEMCU_TPIU_PIDR3")]
pub type OnemcuTpiuPidr3 = crate::Reg<onemcu_tpiu_pidr3::OnemcuTpiuPidr3Spec>;
#[doc = "Peripheral ID3"]
pub mod onemcu_tpiu_pidr3;
#[doc = "ONEMCU_TPIU_CIDR0 (rw) register accessor: Component ID0\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cidr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cidr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_cidr0`]
module"]
#[doc(alias = "ONEMCU_TPIU_CIDR0")]
pub type OnemcuTpiuCidr0 = crate::Reg<onemcu_tpiu_cidr0::OnemcuTpiuCidr0Spec>;
#[doc = "Component ID0"]
pub mod onemcu_tpiu_cidr0;
#[doc = "ONEMCU_TPIU_CIDR1 (rw) register accessor: Component ID1\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cidr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cidr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_cidr1`]
module"]
#[doc(alias = "ONEMCU_TPIU_CIDR1")]
pub type OnemcuTpiuCidr1 = crate::Reg<onemcu_tpiu_cidr1::OnemcuTpiuCidr1Spec>;
#[doc = "Component ID1"]
pub mod onemcu_tpiu_cidr1;
#[doc = "ONEMCU_TPIU_CIDR2 (rw) register accessor: Component ID2\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cidr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cidr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_cidr2`]
module"]
#[doc(alias = "ONEMCU_TPIU_CIDR2")]
pub type OnemcuTpiuCidr2 = crate::Reg<onemcu_tpiu_cidr2::OnemcuTpiuCidr2Spec>;
#[doc = "Component ID2"]
pub mod onemcu_tpiu_cidr2;
#[doc = "ONEMCU_TPIU_CIDR3 (rw) register accessor: Component ID3\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cidr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cidr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@onemcu_tpiu_cidr3`]
module"]
#[doc(alias = "ONEMCU_TPIU_CIDR3")]
pub type OnemcuTpiuCidr3 = crate::Reg<onemcu_tpiu_cidr3::OnemcuTpiuCidr3Spec>;
#[doc = "Component ID3"]
pub mod onemcu_tpiu_cidr3;
#[doc = "APP_CM4_CTI_CONTROL (rw) register accessor: http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_control`]
module"]
#[doc(alias = "APP_CM4_CTI_CONTROL")]
pub type AppCm4CtiControl = crate::Reg<app_cm4_cti_control::AppCm4CtiControlSpec>;
#[doc = "http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html"]
pub mod app_cm4_cti_control;
#[doc = "APP_CM4_CTI_INTACK (rw) register accessor: APP_CM4_CTI_INTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_intack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_intack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_intack`]
module"]
#[doc(alias = "APP_CM4_CTI_INTACK")]
pub type AppCm4CtiIntack = crate::Reg<app_cm4_cti_intack::AppCm4CtiIntackSpec>;
#[doc = "APP_CM4_CTI_INTACK"]
pub mod app_cm4_cti_intack;
#[doc = "APP_CM4_CTI_APPSET (rw) register accessor: APP_CM4_CTI_APPSET\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_appset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_appset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_appset`]
module"]
#[doc(alias = "APP_CM4_CTI_APPSET")]
pub type AppCm4CtiAppset = crate::Reg<app_cm4_cti_appset::AppCm4CtiAppsetSpec>;
#[doc = "APP_CM4_CTI_APPSET"]
pub mod app_cm4_cti_appset;
#[doc = "APP_CM4_CTI_APPCLEAR (rw) register accessor: APP_CM4_CTI_APPCLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_appclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_appclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_appclear`]
module"]
#[doc(alias = "APP_CM4_CTI_APPCLEAR")]
pub type AppCm4CtiAppclear = crate::Reg<app_cm4_cti_appclear::AppCm4CtiAppclearSpec>;
#[doc = "APP_CM4_CTI_APPCLEAR"]
pub mod app_cm4_cti_appclear;
#[doc = "APP_CM4_CTI_APPPULSE (rw) register accessor: APP_CM4_CTI_APPPULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_apppulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_apppulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_apppulse`]
module"]
#[doc(alias = "APP_CM4_CTI_APPPULSE")]
pub type AppCm4CtiApppulse = crate::Reg<app_cm4_cti_apppulse::AppCm4CtiApppulseSpec>;
#[doc = "APP_CM4_CTI_APPPULSE"]
pub mod app_cm4_cti_apppulse;
#[doc = "APP_CM4_CTI_INEN0 (rw) register accessor: APP_CM4_CTI_INEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen0`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN0")]
pub type AppCm4CtiInen0 = crate::Reg<app_cm4_cti_inen0::AppCm4CtiInen0Spec>;
#[doc = "APP_CM4_CTI_INEN0"]
pub mod app_cm4_cti_inen0;
#[doc = "APP_CM4_CTI_INEN1 (rw) register accessor: APP_CM4_CTI_INEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen1`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN1")]
pub type AppCm4CtiInen1 = crate::Reg<app_cm4_cti_inen1::AppCm4CtiInen1Spec>;
#[doc = "APP_CM4_CTI_INEN1"]
pub mod app_cm4_cti_inen1;
#[doc = "APP_CM4_CTI_INEN2 (rw) register accessor: APP_CM4_CTI_INEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen2`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN2")]
pub type AppCm4CtiInen2 = crate::Reg<app_cm4_cti_inen2::AppCm4CtiInen2Spec>;
#[doc = "APP_CM4_CTI_INEN2"]
pub mod app_cm4_cti_inen2;
#[doc = "APP_CM4_CTI_INEN3 (rw) register accessor: APP_CM4_CTI_INEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen3`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN3")]
pub type AppCm4CtiInen3 = crate::Reg<app_cm4_cti_inen3::AppCm4CtiInen3Spec>;
#[doc = "APP_CM4_CTI_INEN3"]
pub mod app_cm4_cti_inen3;
#[doc = "APP_CM4_CTI_INEN4 (rw) register accessor: APP_CM4_CTI_INEN4\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen4`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN4")]
pub type AppCm4CtiInen4 = crate::Reg<app_cm4_cti_inen4::AppCm4CtiInen4Spec>;
#[doc = "APP_CM4_CTI_INEN4"]
pub mod app_cm4_cti_inen4;
#[doc = "APP_CM4_CTI_INEN5 (rw) register accessor: APP_CM4_CTI_INEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen5`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN5")]
pub type AppCm4CtiInen5 = crate::Reg<app_cm4_cti_inen5::AppCm4CtiInen5Spec>;
#[doc = "APP_CM4_CTI_INEN5"]
pub mod app_cm4_cti_inen5;
#[doc = "APP_CM4_CTI_INEN6 (rw) register accessor: APP_CM4_CTI_INEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen6`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN6")]
pub type AppCm4CtiInen6 = crate::Reg<app_cm4_cti_inen6::AppCm4CtiInen6Spec>;
#[doc = "APP_CM4_CTI_INEN6"]
pub mod app_cm4_cti_inen6;
#[doc = "APP_CM4_CTI_INEN7 (rw) register accessor: APP_CM4_CTI_INEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_inen7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_inen7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_inen7`]
module"]
#[doc(alias = "APP_CM4_CTI_INEN7")]
pub type AppCm4CtiInen7 = crate::Reg<app_cm4_cti_inen7::AppCm4CtiInen7Spec>;
#[doc = "APP_CM4_CTI_INEN7"]
pub mod app_cm4_cti_inen7;
#[doc = "APP_CM4_CTI_OUTEN0 (rw) register accessor: APP_CM4_CTI_OUTEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen0`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN0")]
pub type AppCm4CtiOuten0 = crate::Reg<app_cm4_cti_outen0::AppCm4CtiOuten0Spec>;
#[doc = "APP_CM4_CTI_OUTEN0"]
pub mod app_cm4_cti_outen0;
#[doc = "APP_CM4_CTI_OUTEN1 (rw) register accessor: APP_CM4_CTI_OUTEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen1`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN1")]
pub type AppCm4CtiOuten1 = crate::Reg<app_cm4_cti_outen1::AppCm4CtiOuten1Spec>;
#[doc = "APP_CM4_CTI_OUTEN1"]
pub mod app_cm4_cti_outen1;
#[doc = "APP_CM4_CTI_OUTEN2 (rw) register accessor: APP_CM4_CTI_OUTEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen2`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN2")]
pub type AppCm4CtiOuten2 = crate::Reg<app_cm4_cti_outen2::AppCm4CtiOuten2Spec>;
#[doc = "APP_CM4_CTI_OUTEN2"]
pub mod app_cm4_cti_outen2;
#[doc = "APP_CM4_CTI_OUTEN3 (rw) register accessor: APP_CM4_CTI_OUTEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen3`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN3")]
pub type AppCm4CtiOuten3 = crate::Reg<app_cm4_cti_outen3::AppCm4CtiOuten3Spec>;
#[doc = "APP_CM4_CTI_OUTEN3"]
pub mod app_cm4_cti_outen3;
#[doc = "APP_CM4_CTI_OUTEN4 (rw) register accessor: APP_CM4_CTI_OUTEN4\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen4`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN4")]
pub type AppCm4CtiOuten4 = crate::Reg<app_cm4_cti_outen4::AppCm4CtiOuten4Spec>;
#[doc = "APP_CM4_CTI_OUTEN4"]
pub mod app_cm4_cti_outen4;
#[doc = "APP_CM4_CTI_OUTEN5 (rw) register accessor: APP_CM4_CTI_OUTEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen5`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN5")]
pub type AppCm4CtiOuten5 = crate::Reg<app_cm4_cti_outen5::AppCm4CtiOuten5Spec>;
#[doc = "APP_CM4_CTI_OUTEN5"]
pub mod app_cm4_cti_outen5;
#[doc = "APP_CM4_CTI_OUTEN6 (rw) register accessor: APP_CM4_CTI_OUTEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen6`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN6")]
pub type AppCm4CtiOuten6 = crate::Reg<app_cm4_cti_outen6::AppCm4CtiOuten6Spec>;
#[doc = "APP_CM4_CTI_OUTEN6"]
pub mod app_cm4_cti_outen6;
#[doc = "APP_CM4_CTI_OUTEN7 (rw) register accessor: APP_CM4_CTI_OUTEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_outen7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_outen7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_outen7`]
module"]
#[doc(alias = "APP_CM4_CTI_OUTEN7")]
pub type AppCm4CtiOuten7 = crate::Reg<app_cm4_cti_outen7::AppCm4CtiOuten7Spec>;
#[doc = "APP_CM4_CTI_OUTEN7"]
pub mod app_cm4_cti_outen7;
#[doc = "APP_CM4_CTI_TRIGINSTATUS (rw) register accessor: APP_CM4_CTI_TRIGINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_triginstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_triginstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_triginstatus`]
module"]
#[doc(alias = "APP_CM4_CTI_TRIGINSTATUS")]
pub type AppCm4CtiTriginstatus = crate::Reg<app_cm4_cti_triginstatus::AppCm4CtiTriginstatusSpec>;
#[doc = "APP_CM4_CTI_TRIGINSTATUS"]
pub mod app_cm4_cti_triginstatus;
#[doc = "APP_CM4_CTI_TRIGOUTSTATUS (rw) register accessor: APP_CM4_CTI_TRIGOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_trigoutstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_trigoutstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_trigoutstatus`]
module"]
#[doc(alias = "APP_CM4_CTI_TRIGOUTSTATUS")]
pub type AppCm4CtiTrigoutstatus = crate::Reg<app_cm4_cti_trigoutstatus::AppCm4CtiTrigoutstatusSpec>;
#[doc = "APP_CM4_CTI_TRIGOUTSTATUS"]
pub mod app_cm4_cti_trigoutstatus;
#[doc = "APP_CM4_CTI_CHINSTATUS (rw) register accessor: APP_CM4_CTI_CHINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_chinstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_chinstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_chinstatus`]
module"]
#[doc(alias = "APP_CM4_CTI_CHINSTATUS")]
pub type AppCm4CtiChinstatus = crate::Reg<app_cm4_cti_chinstatus::AppCm4CtiChinstatusSpec>;
#[doc = "APP_CM4_CTI_CHINSTATUS"]
pub mod app_cm4_cti_chinstatus;
#[doc = "APP_CM4_CTI_CHOUTSTATUS (rw) register accessor: APP_CM4_CTI_CHOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_choutstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_choutstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_choutstatus`]
module"]
#[doc(alias = "APP_CM4_CTI_CHOUTSTATUS")]
pub type AppCm4CtiChoutstatus = crate::Reg<app_cm4_cti_choutstatus::AppCm4CtiChoutstatusSpec>;
#[doc = "APP_CM4_CTI_CHOUTSTATUS"]
pub mod app_cm4_cti_choutstatus;
#[doc = "APP_CM4_CTI_GATE (rw) register accessor: APP_CM4_CTI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_gate`]
module"]
#[doc(alias = "APP_CM4_CTI_GATE")]
pub type AppCm4CtiGate = crate::Reg<app_cm4_cti_gate::AppCm4CtiGateSpec>;
#[doc = "APP_CM4_CTI_GATE"]
pub mod app_cm4_cti_gate;
#[doc = "APP_CM4_CTI_ASICCTL (rw) register accessor: APP_CM4_CTI_ASICCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_asicctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_asicctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_asicctl`]
module"]
#[doc(alias = "APP_CM4_CTI_ASICCTL")]
pub type AppCm4CtiAsicctl = crate::Reg<app_cm4_cti_asicctl::AppCm4CtiAsicctlSpec>;
#[doc = "APP_CM4_CTI_ASICCTL"]
pub mod app_cm4_cti_asicctl;
#[doc = "APP_CM4_CTI_ITCHINACK (rw) register accessor: APP_CM4_CTI_ITCHINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itchinack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itchinack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_itchinack`]
module"]
#[doc(alias = "APP_CM4_CTI_ITCHINACK")]
pub type AppCm4CtiItchinack = crate::Reg<app_cm4_cti_itchinack::AppCm4CtiItchinackSpec>;
#[doc = "APP_CM4_CTI_ITCHINACK"]
pub mod app_cm4_cti_itchinack;
#[doc = "APP_CM4_CTI_ITTRIGINACK (rw) register accessor: APP_CM4_CTI_ITTRIGINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittriginack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittriginack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_ittriginack`]
module"]
#[doc(alias = "APP_CM4_CTI_ITTRIGINACK")]
pub type AppCm4CtiIttriginack = crate::Reg<app_cm4_cti_ittriginack::AppCm4CtiIttriginackSpec>;
#[doc = "APP_CM4_CTI_ITTRIGINACK"]
pub mod app_cm4_cti_ittriginack;
#[doc = "APP_CM4_CTI_ITCHOUT (rw) register accessor: APP_CM4_CTI_ITCHOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itchout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itchout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_itchout`]
module"]
#[doc(alias = "APP_CM4_CTI_ITCHOUT")]
pub type AppCm4CtiItchout = crate::Reg<app_cm4_cti_itchout::AppCm4CtiItchoutSpec>;
#[doc = "APP_CM4_CTI_ITCHOUT"]
pub mod app_cm4_cti_itchout;
#[doc = "APP_CM4_CTI_ITTRIGOUT (rw) register accessor: APP_CM4_CTI_ITTRIGOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittrigout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittrigout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_ittrigout`]
module"]
#[doc(alias = "APP_CM4_CTI_ITTRIGOUT")]
pub type AppCm4CtiIttrigout = crate::Reg<app_cm4_cti_ittrigout::AppCm4CtiIttrigoutSpec>;
#[doc = "APP_CM4_CTI_ITTRIGOUT"]
pub mod app_cm4_cti_ittrigout;
#[doc = "APP_CM4_CTI_ITCHOUTACK (rw) register accessor: APP_CM4_CTI_ITCHOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itchoutack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itchoutack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_itchoutack`]
module"]
#[doc(alias = "APP_CM4_CTI_ITCHOUTACK")]
pub type AppCm4CtiItchoutack = crate::Reg<app_cm4_cti_itchoutack::AppCm4CtiItchoutackSpec>;
#[doc = "APP_CM4_CTI_ITCHOUTACK"]
pub mod app_cm4_cti_itchoutack;
#[doc = "APP_CM4_CTI_ITTRIGOUTACK (rw) register accessor: APP_CM4_CTI_ITTRIGOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittrigoutack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittrigoutack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_ittrigoutack`]
module"]
#[doc(alias = "APP_CM4_CTI_ITTRIGOUTACK")]
pub type AppCm4CtiIttrigoutack = crate::Reg<app_cm4_cti_ittrigoutack::AppCm4CtiIttrigoutackSpec>;
#[doc = "APP_CM4_CTI_ITTRIGOUTACK"]
pub mod app_cm4_cti_ittrigoutack;
#[doc = "APP_CM4_CTI_ITCHIN (rw) register accessor: APP_CM4_CTI_ITCHIN\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itchin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itchin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_itchin`]
module"]
#[doc(alias = "APP_CM4_CTI_ITCHIN")]
pub type AppCm4CtiItchin = crate::Reg<app_cm4_cti_itchin::AppCm4CtiItchinSpec>;
#[doc = "APP_CM4_CTI_ITCHIN"]
pub mod app_cm4_cti_itchin;
#[doc = "APP_CM4_CTI_ITTRIGIN (rw) register accessor: APP_CM4_CTI_ITTRIGIN\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittrigin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittrigin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_ittrigin`]
module"]
#[doc(alias = "APP_CM4_CTI_ITTRIGIN")]
pub type AppCm4CtiIttrigin = crate::Reg<app_cm4_cti_ittrigin::AppCm4CtiIttriginSpec>;
#[doc = "APP_CM4_CTI_ITTRIGIN"]
pub mod app_cm4_cti_ittrigin;
#[doc = "APP_CM4_CTI_ITCTRL (rw) register accessor: APP_CM4_CTI_ITCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_itctrl`]
module"]
#[doc(alias = "APP_CM4_CTI_ITCTRL")]
pub type AppCm4CtiItctrl = crate::Reg<app_cm4_cti_itctrl::AppCm4CtiItctrlSpec>;
#[doc = "APP_CM4_CTI_ITCTRL"]
pub mod app_cm4_cti_itctrl;
#[doc = "APP_CM4_CTI_Claim_Tag_Set (rw) register accessor: APP_CM4_CTI_Claim_Tag_Set\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_claim_tag_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_claim_tag_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_claim_tag_set`]
module"]
#[doc(alias = "APP_CM4_CTI_Claim_Tag_Set")]
pub type AppCm4CtiClaimTagSet = crate::Reg<app_cm4_cti_claim_tag_set::AppCm4CtiClaimTagSetSpec>;
#[doc = "APP_CM4_CTI_Claim_Tag_Set"]
pub mod app_cm4_cti_claim_tag_set;
#[doc = "APP_CM4_CTI_Claim_Tag_Clear (rw) register accessor: APP_CM4_CTI_Claim_Tag_Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_claim_tag_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_claim_tag_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_claim_tag_clear`]
module"]
#[doc(alias = "APP_CM4_CTI_Claim_Tag_Clear")]
pub type AppCm4CtiClaimTagClear =
    crate::Reg<app_cm4_cti_claim_tag_clear::AppCm4CtiClaimTagClearSpec>;
#[doc = "APP_CM4_CTI_Claim_Tag_Clear"]
pub mod app_cm4_cti_claim_tag_clear;
#[doc = "APP_CM4_CTI_Lock_Access_Register (rw) register accessor: APP_CM4_CTI_Lock_Access_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_lock_access_register::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_lock_access_register::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_lock_access_register`]
module"]
#[doc(alias = "APP_CM4_CTI_Lock_Access_Register")]
pub type AppCm4CtiLockAccessRegister =
    crate::Reg<app_cm4_cti_lock_access_register::AppCm4CtiLockAccessRegisterSpec>;
#[doc = "APP_CM4_CTI_Lock_Access_Register"]
pub mod app_cm4_cti_lock_access_register;
#[doc = "APP_CM4_CTI_Lock_Status_Register (rw) register accessor: APP_CM4_CTI_Lock_Status_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_lock_status_register::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_lock_status_register::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_lock_status_register`]
module"]
#[doc(alias = "APP_CM4_CTI_Lock_Status_Register")]
pub type AppCm4CtiLockStatusRegister =
    crate::Reg<app_cm4_cti_lock_status_register::AppCm4CtiLockStatusRegisterSpec>;
#[doc = "APP_CM4_CTI_Lock_Status_Register"]
pub mod app_cm4_cti_lock_status_register;
#[doc = "APP_CM4_CTI_Authentication_Status (rw) register accessor: APP_CM4_CTI_Authentication_Status\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_authentication_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_authentication_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_authentication_status`]
module"]
#[doc(alias = "APP_CM4_CTI_Authentication_Status")]
pub type AppCm4CtiAuthenticationStatus =
    crate::Reg<app_cm4_cti_authentication_status::AppCm4CtiAuthenticationStatusSpec>;
#[doc = "APP_CM4_CTI_Authentication_Status"]
pub mod app_cm4_cti_authentication_status;
#[doc = "APP_CM4_CTI_Device_ID (rw) register accessor: APP_CM4_CTI_Device_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_device_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_device_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_device_id`]
module"]
#[doc(alias = "APP_CM4_CTI_Device_ID")]
pub type AppCm4CtiDeviceId = crate::Reg<app_cm4_cti_device_id::AppCm4CtiDeviceIdSpec>;
#[doc = "APP_CM4_CTI_Device_ID"]
pub mod app_cm4_cti_device_id;
#[doc = "APP_CM4_CTI_Device_Type_Identifier (rw) register accessor: APP_CM4_CTI_Device_Type_Identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_device_type_identifier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_device_type_identifier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_device_type_identifier`]
module"]
#[doc(alias = "APP_CM4_CTI_Device_Type_Identifier")]
pub type AppCm4CtiDeviceTypeIdentifier =
    crate::Reg<app_cm4_cti_device_type_identifier::AppCm4CtiDeviceTypeIdentifierSpec>;
#[doc = "APP_CM4_CTI_Device_Type_Identifier"]
pub mod app_cm4_cti_device_type_identifier;
#[doc = "APP_CM4_CTI_PeripheralID4 (rw) register accessor: APP_CM4_CTI_PeripheralID4\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id4`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID4")]
pub type AppCm4CtiPeripheralId4 =
    crate::Reg<app_cm4_cti_peripheral_id4::AppCm4CtiPeripheralId4Spec>;
#[doc = "APP_CM4_CTI_PeripheralID4"]
pub mod app_cm4_cti_peripheral_id4;
#[doc = "APP_CM4_CTI_PeripheralID5 (rw) register accessor: APP_CM4_CTI_PeripheralID5\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id5`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID5")]
pub type AppCm4CtiPeripheralId5 =
    crate::Reg<app_cm4_cti_peripheral_id5::AppCm4CtiPeripheralId5Spec>;
#[doc = "APP_CM4_CTI_PeripheralID5"]
pub mod app_cm4_cti_peripheral_id5;
#[doc = "APP_CM4_CTI_PeripheralID6 (rw) register accessor: APP_CM4_CTI_PeripheralID6\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id6`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID6")]
pub type AppCm4CtiPeripheralId6 =
    crate::Reg<app_cm4_cti_peripheral_id6::AppCm4CtiPeripheralId6Spec>;
#[doc = "APP_CM4_CTI_PeripheralID6"]
pub mod app_cm4_cti_peripheral_id6;
#[doc = "APP_CM4_CTI_PeripheralID7 (rw) register accessor: APP_CM4_CTI_PeripheralID7\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id7`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID7")]
pub type AppCm4CtiPeripheralId7 =
    crate::Reg<app_cm4_cti_peripheral_id7::AppCm4CtiPeripheralId7Spec>;
#[doc = "APP_CM4_CTI_PeripheralID7"]
pub mod app_cm4_cti_peripheral_id7;
#[doc = "APP_CM4_CTI_PeripheralID0 (rw) register accessor: APP_CM4_CTI_PeripheralID0\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id0`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID0")]
pub type AppCm4CtiPeripheralId0 =
    crate::Reg<app_cm4_cti_peripheral_id0::AppCm4CtiPeripheralId0Spec>;
#[doc = "APP_CM4_CTI_PeripheralID0"]
pub mod app_cm4_cti_peripheral_id0;
#[doc = "APP_CM4_CTI_PeripheralID1 (rw) register accessor: APP_CM4_CTI_PeripheralID1\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id1`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID1")]
pub type AppCm4CtiPeripheralId1 =
    crate::Reg<app_cm4_cti_peripheral_id1::AppCm4CtiPeripheralId1Spec>;
#[doc = "APP_CM4_CTI_PeripheralID1"]
pub mod app_cm4_cti_peripheral_id1;
#[doc = "APP_CM4_CTI_PeripheralID2 (rw) register accessor: APP_CM4_CTI_PeripheralID2\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id2`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID2")]
pub type AppCm4CtiPeripheralId2 =
    crate::Reg<app_cm4_cti_peripheral_id2::AppCm4CtiPeripheralId2Spec>;
#[doc = "APP_CM4_CTI_PeripheralID2"]
pub mod app_cm4_cti_peripheral_id2;
#[doc = "APP_CM4_CTI_PeripheralID3 (rw) register accessor: APP_CM4_CTI_PeripheralID3\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_peripheral_id3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_peripheral_id3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_peripheral_id3`]
module"]
#[doc(alias = "APP_CM4_CTI_PeripheralID3")]
pub type AppCm4CtiPeripheralId3 =
    crate::Reg<app_cm4_cti_peripheral_id3::AppCm4CtiPeripheralId3Spec>;
#[doc = "APP_CM4_CTI_PeripheralID3"]
pub mod app_cm4_cti_peripheral_id3;
#[doc = "APP_CM4_CTI_Component_ID0 (rw) register accessor: APP_CM4_CTI_Component_ID0\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_component_id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_component_id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_component_id0`]
module"]
#[doc(alias = "APP_CM4_CTI_Component_ID0")]
pub type AppCm4CtiComponentId0 = crate::Reg<app_cm4_cti_component_id0::AppCm4CtiComponentId0Spec>;
#[doc = "APP_CM4_CTI_Component_ID0"]
pub mod app_cm4_cti_component_id0;
#[doc = "APP_CM4_CTI_Component_ID1 (rw) register accessor: APP_CM4_CTI_Component_ID1\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_component_id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_component_id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_component_id1`]
module"]
#[doc(alias = "APP_CM4_CTI_Component_ID1")]
pub type AppCm4CtiComponentId1 = crate::Reg<app_cm4_cti_component_id1::AppCm4CtiComponentId1Spec>;
#[doc = "APP_CM4_CTI_Component_ID1"]
pub mod app_cm4_cti_component_id1;
#[doc = "APP_CM4_CTI_Component_ID2 (rw) register accessor: APP_CM4_CTI_Component_ID2\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_component_id2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_component_id2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_component_id2`]
module"]
#[doc(alias = "APP_CM4_CTI_Component_ID2")]
pub type AppCm4CtiComponentId2 = crate::Reg<app_cm4_cti_component_id2::AppCm4CtiComponentId2Spec>;
#[doc = "APP_CM4_CTI_Component_ID2"]
pub mod app_cm4_cti_component_id2;
#[doc = "APP_CM4_CTI_Component_ID3 (rw) register accessor: APP_CM4_CTI_Component_ID3\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_component_id3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_component_id3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_cm4_cti_component_id3`]
module"]
#[doc(alias = "APP_CM4_CTI_Component_ID3")]
pub type AppCm4CtiComponentId3 = crate::Reg<app_cm4_cti_component_id3::AppCm4CtiComponentId3Spec>;
#[doc = "APP_CM4_CTI_Component_ID3"]
pub mod app_cm4_cti_component_id3;
#[doc = "FEC_CM3_CTI_CONTROL (rw) register accessor: http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_control`]
module"]
#[doc(alias = "FEC_CM3_CTI_CONTROL")]
pub type FecCm3CtiControl = crate::Reg<fec_cm3_cti_control::FecCm3CtiControlSpec>;
#[doc = "http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html"]
pub mod fec_cm3_cti_control;
#[doc = "FEC_CM3_CTI_INTACK (rw) register accessor: FEC_CM3_CTI_INTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_intack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_intack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_intack`]
module"]
#[doc(alias = "FEC_CM3_CTI_INTACK")]
pub type FecCm3CtiIntack = crate::Reg<fec_cm3_cti_intack::FecCm3CtiIntackSpec>;
#[doc = "FEC_CM3_CTI_INTACK"]
pub mod fec_cm3_cti_intack;
#[doc = "FEC_CM3_CTI_APPSET (rw) register accessor: FEC_CM3_CTI_APPSET\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_appset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_appset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_appset`]
module"]
#[doc(alias = "FEC_CM3_CTI_APPSET")]
pub type FecCm3CtiAppset = crate::Reg<fec_cm3_cti_appset::FecCm3CtiAppsetSpec>;
#[doc = "FEC_CM3_CTI_APPSET"]
pub mod fec_cm3_cti_appset;
#[doc = "FEC_CM3_CTI_APPCLEAR (rw) register accessor: FEC_CM3_CTI_APPCLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_appclear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_appclear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_appclear`]
module"]
#[doc(alias = "FEC_CM3_CTI_APPCLEAR")]
pub type FecCm3CtiAppclear = crate::Reg<fec_cm3_cti_appclear::FecCm3CtiAppclearSpec>;
#[doc = "FEC_CM3_CTI_APPCLEAR"]
pub mod fec_cm3_cti_appclear;
#[doc = "FEC_CM3_CTI_APPPULSE (rw) register accessor: FEC_CM3_CTI_APPPULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_apppulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_apppulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_apppulse`]
module"]
#[doc(alias = "FEC_CM3_CTI_APPPULSE")]
pub type FecCm3CtiApppulse = crate::Reg<fec_cm3_cti_apppulse::FecCm3CtiApppulseSpec>;
#[doc = "FEC_CM3_CTI_APPPULSE"]
pub mod fec_cm3_cti_apppulse;
#[doc = "FEC_CM3_CTI_INEN0 (rw) register accessor: FEC_CM3_CTI_INEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen0`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN0")]
pub type FecCm3CtiInen0 = crate::Reg<fec_cm3_cti_inen0::FecCm3CtiInen0Spec>;
#[doc = "FEC_CM3_CTI_INEN0"]
pub mod fec_cm3_cti_inen0;
#[doc = "FEC_CM3_CTI_INEN1 (rw) register accessor: FEC_CM3_CTI_INEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen1`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN1")]
pub type FecCm3CtiInen1 = crate::Reg<fec_cm3_cti_inen1::FecCm3CtiInen1Spec>;
#[doc = "FEC_CM3_CTI_INEN1"]
pub mod fec_cm3_cti_inen1;
#[doc = "FEC_CM3_CTI_INEN2 (rw) register accessor: FEC_CM3_CTI_INEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen2`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN2")]
pub type FecCm3CtiInen2 = crate::Reg<fec_cm3_cti_inen2::FecCm3CtiInen2Spec>;
#[doc = "FEC_CM3_CTI_INEN2"]
pub mod fec_cm3_cti_inen2;
#[doc = "FEC_CM3_CTI_INEN3 (rw) register accessor: FEC_CM3_CTI_INEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen3`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN3")]
pub type FecCm3CtiInen3 = crate::Reg<fec_cm3_cti_inen3::FecCm3CtiInen3Spec>;
#[doc = "FEC_CM3_CTI_INEN3"]
pub mod fec_cm3_cti_inen3;
#[doc = "FEC_CM3_CTI_INEN4 (rw) register accessor: FEC_CM3_CTI_INEN4\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen4`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN4")]
pub type FecCm3CtiInen4 = crate::Reg<fec_cm3_cti_inen4::FecCm3CtiInen4Spec>;
#[doc = "FEC_CM3_CTI_INEN4"]
pub mod fec_cm3_cti_inen4;
#[doc = "FEC_CM3_CTI_INEN5 (rw) register accessor: FEC_CM3_CTI_INEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen5`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN5")]
pub type FecCm3CtiInen5 = crate::Reg<fec_cm3_cti_inen5::FecCm3CtiInen5Spec>;
#[doc = "FEC_CM3_CTI_INEN5"]
pub mod fec_cm3_cti_inen5;
#[doc = "FEC_CM3_CTI_INEN6 (rw) register accessor: FEC_CM3_CTI_INEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen6`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN6")]
pub type FecCm3CtiInen6 = crate::Reg<fec_cm3_cti_inen6::FecCm3CtiInen6Spec>;
#[doc = "FEC_CM3_CTI_INEN6"]
pub mod fec_cm3_cti_inen6;
#[doc = "FEC_CM3_CTI_INEN7 (rw) register accessor: FEC_CM3_CTI_INEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_inen7`]
module"]
#[doc(alias = "FEC_CM3_CTI_INEN7")]
pub type FecCm3CtiInen7 = crate::Reg<fec_cm3_cti_inen7::FecCm3CtiInen7Spec>;
#[doc = "FEC_CM3_CTI_INEN7"]
pub mod fec_cm3_cti_inen7;
#[doc = "FEC_CM3_CTI_OUTEN0 (rw) register accessor: FEC_CM3_CTI_OUTEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen0`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN0")]
pub type FecCm3CtiOuten0 = crate::Reg<fec_cm3_cti_outen0::FecCm3CtiOuten0Spec>;
#[doc = "FEC_CM3_CTI_OUTEN0"]
pub mod fec_cm3_cti_outen0;
#[doc = "FEC_CM3_CTI_OUTEN1 (rw) register accessor: FEC_CM3_CTI_OUTEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen1`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN1")]
pub type FecCm3CtiOuten1 = crate::Reg<fec_cm3_cti_outen1::FecCm3CtiOuten1Spec>;
#[doc = "FEC_CM3_CTI_OUTEN1"]
pub mod fec_cm3_cti_outen1;
#[doc = "FEC_CM3_CTI_OUTEN2 (rw) register accessor: FEC_CM3_CTI_OUTEN2\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen2`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN2")]
pub type FecCm3CtiOuten2 = crate::Reg<fec_cm3_cti_outen2::FecCm3CtiOuten2Spec>;
#[doc = "FEC_CM3_CTI_OUTEN2"]
pub mod fec_cm3_cti_outen2;
#[doc = "FEC_CM3_CTI_OUTEN3 (rw) register accessor: FEC_CM3_CTI_OUTEN3\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen3`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN3")]
pub type FecCm3CtiOuten3 = crate::Reg<fec_cm3_cti_outen3::FecCm3CtiOuten3Spec>;
#[doc = "FEC_CM3_CTI_OUTEN3"]
pub mod fec_cm3_cti_outen3;
#[doc = "FEC_CM3_CTI_OUTEN4 (rw) register accessor: FEC_CM3_CTI_OUTEN4\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen4`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN4")]
pub type FecCm3CtiOuten4 = crate::Reg<fec_cm3_cti_outen4::FecCm3CtiOuten4Spec>;
#[doc = "FEC_CM3_CTI_OUTEN4"]
pub mod fec_cm3_cti_outen4;
#[doc = "FEC_CM3_CTI_OUTEN5 (rw) register accessor: FEC_CM3_CTI_OUTEN5\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen5`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN5")]
pub type FecCm3CtiOuten5 = crate::Reg<fec_cm3_cti_outen5::FecCm3CtiOuten5Spec>;
#[doc = "FEC_CM3_CTI_OUTEN5"]
pub mod fec_cm3_cti_outen5;
#[doc = "FEC_CM3_CTI_OUTEN6 (rw) register accessor: FEC_CM3_CTI_OUTEN6\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen6`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN6")]
pub type FecCm3CtiOuten6 = crate::Reg<fec_cm3_cti_outen6::FecCm3CtiOuten6Spec>;
#[doc = "FEC_CM3_CTI_OUTEN6"]
pub mod fec_cm3_cti_outen6;
#[doc = "FEC_CM3_CTI_OUTEN7 (rw) register accessor: FEC_CM3_CTI_OUTEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_outen7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_outen7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_outen7`]
module"]
#[doc(alias = "FEC_CM3_CTI_OUTEN7")]
pub type FecCm3CtiOuten7 = crate::Reg<fec_cm3_cti_outen7::FecCm3CtiOuten7Spec>;
#[doc = "FEC_CM3_CTI_OUTEN7"]
pub mod fec_cm3_cti_outen7;
#[doc = "FEC_CM3_CTI_TRIGINSTATUS (rw) register accessor: FEC_CM3_CTI_TRIGINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_triginstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_triginstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_triginstatus`]
module"]
#[doc(alias = "FEC_CM3_CTI_TRIGINSTATUS")]
pub type FecCm3CtiTriginstatus = crate::Reg<fec_cm3_cti_triginstatus::FecCm3CtiTriginstatusSpec>;
#[doc = "FEC_CM3_CTI_TRIGINSTATUS"]
pub mod fec_cm3_cti_triginstatus;
#[doc = "FEC_CM3_CTI_TRIGOUTSTATUS (rw) register accessor: FEC_CM3_CTI_TRIGOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_trigoutstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_trigoutstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_trigoutstatus`]
module"]
#[doc(alias = "FEC_CM3_CTI_TRIGOUTSTATUS")]
pub type FecCm3CtiTrigoutstatus = crate::Reg<fec_cm3_cti_trigoutstatus::FecCm3CtiTrigoutstatusSpec>;
#[doc = "FEC_CM3_CTI_TRIGOUTSTATUS"]
pub mod fec_cm3_cti_trigoutstatus;
#[doc = "FEC_CM3_CTI_CHINSTATUS (rw) register accessor: FEC_CM3_CTI_CHINSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_chinstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_chinstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_chinstatus`]
module"]
#[doc(alias = "FEC_CM3_CTI_CHINSTATUS")]
pub type FecCm3CtiChinstatus = crate::Reg<fec_cm3_cti_chinstatus::FecCm3CtiChinstatusSpec>;
#[doc = "FEC_CM3_CTI_CHINSTATUS"]
pub mod fec_cm3_cti_chinstatus;
#[doc = "FEC_CM3_CTI_CHOUTSTATUS (rw) register accessor: FEC_CM3_CTI_CHOUTSTATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_choutstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_choutstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_choutstatus`]
module"]
#[doc(alias = "FEC_CM3_CTI_CHOUTSTATUS")]
pub type FecCm3CtiChoutstatus = crate::Reg<fec_cm3_cti_choutstatus::FecCm3CtiChoutstatusSpec>;
#[doc = "FEC_CM3_CTI_CHOUTSTATUS"]
pub mod fec_cm3_cti_choutstatus;
#[doc = "FEC_CM3_CTI_GATE (rw) register accessor: FEC_CM3_CTI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_gate`]
module"]
#[doc(alias = "FEC_CM3_CTI_GATE")]
pub type FecCm3CtiGate = crate::Reg<fec_cm3_cti_gate::FecCm3CtiGateSpec>;
#[doc = "FEC_CM3_CTI_GATE"]
pub mod fec_cm3_cti_gate;
#[doc = "FEC_CM3_CTI_ASICCTL (rw) register accessor: FEC_CM3_CTI_ASICCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_asicctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_asicctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_asicctl`]
module"]
#[doc(alias = "FEC_CM3_CTI_ASICCTL")]
pub type FecCm3CtiAsicctl = crate::Reg<fec_cm3_cti_asicctl::FecCm3CtiAsicctlSpec>;
#[doc = "FEC_CM3_CTI_ASICCTL"]
pub mod fec_cm3_cti_asicctl;
#[doc = "FEC_CM3_CTI_ITCHINACK (rw) register accessor: FEC_CM3_CTI_ITCHINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itchinack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itchinack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_itchinack`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITCHINACK")]
pub type FecCm3CtiItchinack = crate::Reg<fec_cm3_cti_itchinack::FecCm3CtiItchinackSpec>;
#[doc = "FEC_CM3_CTI_ITCHINACK"]
pub mod fec_cm3_cti_itchinack;
#[doc = "FEC_CM3_CTI_ITTRIGINACK (rw) register accessor: FEC_CM3_CTI_ITTRIGINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_ittriginack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_ittriginack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_ittriginack`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITTRIGINACK")]
pub type FecCm3CtiIttriginack = crate::Reg<fec_cm3_cti_ittriginack::FecCm3CtiIttriginackSpec>;
#[doc = "FEC_CM3_CTI_ITTRIGINACK"]
pub mod fec_cm3_cti_ittriginack;
#[doc = "FEC_CM3_CTI_ITCHOUT (rw) register accessor: FEC_CM3_CTI_ITCHOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itchout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itchout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_itchout`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITCHOUT")]
pub type FecCm3CtiItchout = crate::Reg<fec_cm3_cti_itchout::FecCm3CtiItchoutSpec>;
#[doc = "FEC_CM3_CTI_ITCHOUT"]
pub mod fec_cm3_cti_itchout;
#[doc = "FEC_CM3_CTI_ITTRIGOUT (rw) register accessor: FEC_CM3_CTI_ITTRIGOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_ittrigout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_ittrigout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_ittrigout`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITTRIGOUT")]
pub type FecCm3CtiIttrigout = crate::Reg<fec_cm3_cti_ittrigout::FecCm3CtiIttrigoutSpec>;
#[doc = "FEC_CM3_CTI_ITTRIGOUT"]
pub mod fec_cm3_cti_ittrigout;
#[doc = "FEC_CM3_CTI_ITCHOUTACK (rw) register accessor: FEC_CM3_CTI_ITCHOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itchoutack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itchoutack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_itchoutack`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITCHOUTACK")]
pub type FecCm3CtiItchoutack = crate::Reg<fec_cm3_cti_itchoutack::FecCm3CtiItchoutackSpec>;
#[doc = "FEC_CM3_CTI_ITCHOUTACK"]
pub mod fec_cm3_cti_itchoutack;
#[doc = "FEC_CM3_CTI_ITTRIGOUTACK (rw) register accessor: FEC_CM3_CTI_ITTRIGOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_ittrigoutack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_ittrigoutack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_ittrigoutack`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITTRIGOUTACK")]
pub type FecCm3CtiIttrigoutack = crate::Reg<fec_cm3_cti_ittrigoutack::FecCm3CtiIttrigoutackSpec>;
#[doc = "FEC_CM3_CTI_ITTRIGOUTACK"]
pub mod fec_cm3_cti_ittrigoutack;
#[doc = "FEC_CM3_CTI_ITCHIN (rw) register accessor: FEC_CM3_CTI_ITCHIN\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itchin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itchin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_itchin`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITCHIN")]
pub type FecCm3CtiItchin = crate::Reg<fec_cm3_cti_itchin::FecCm3CtiItchinSpec>;
#[doc = "FEC_CM3_CTI_ITCHIN"]
pub mod fec_cm3_cti_itchin;
#[doc = "FEC_CM3_CTI_ITTRIGIN (rw) register accessor: FEC_CM3_CTI_ITTRIGIN\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_ittrigin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_ittrigin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_ittrigin`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITTRIGIN")]
pub type FecCm3CtiIttrigin = crate::Reg<fec_cm3_cti_ittrigin::FecCm3CtiIttriginSpec>;
#[doc = "FEC_CM3_CTI_ITTRIGIN"]
pub mod fec_cm3_cti_ittrigin;
#[doc = "FEC_CM3_CTI_ITCTRL (rw) register accessor: FEC_CM3_CTI_ITCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_itctrl`]
module"]
#[doc(alias = "FEC_CM3_CTI_ITCTRL")]
pub type FecCm3CtiItctrl = crate::Reg<fec_cm3_cti_itctrl::FecCm3CtiItctrlSpec>;
#[doc = "FEC_CM3_CTI_ITCTRL"]
pub mod fec_cm3_cti_itctrl;
#[doc = "FEC_CM3_CTI_Claim_Tag_Set (rw) register accessor: FEC_CM3_CTI_Claim_Tag_Set\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_claim_tag_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_claim_tag_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_claim_tag_set`]
module"]
#[doc(alias = "FEC_CM3_CTI_Claim_Tag_Set")]
pub type FecCm3CtiClaimTagSet = crate::Reg<fec_cm3_cti_claim_tag_set::FecCm3CtiClaimTagSetSpec>;
#[doc = "FEC_CM3_CTI_Claim_Tag_Set"]
pub mod fec_cm3_cti_claim_tag_set;
#[doc = "FEC_CM3_CTI_Claim_Tag_Clear (rw) register accessor: FEC_CM3_CTI_Claim_Tag_Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_claim_tag_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_claim_tag_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_claim_tag_clear`]
module"]
#[doc(alias = "FEC_CM3_CTI_Claim_Tag_Clear")]
pub type FecCm3CtiClaimTagClear =
    crate::Reg<fec_cm3_cti_claim_tag_clear::FecCm3CtiClaimTagClearSpec>;
#[doc = "FEC_CM3_CTI_Claim_Tag_Clear"]
pub mod fec_cm3_cti_claim_tag_clear;
#[doc = "FEC_CM3_CTI_Lock_Access_Register (rw) register accessor: FEC_CM3_CTI_Lock_Access_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_lock_access_register::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_lock_access_register::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_lock_access_register`]
module"]
#[doc(alias = "FEC_CM3_CTI_Lock_Access_Register")]
pub type FecCm3CtiLockAccessRegister =
    crate::Reg<fec_cm3_cti_lock_access_register::FecCm3CtiLockAccessRegisterSpec>;
#[doc = "FEC_CM3_CTI_Lock_Access_Register"]
pub mod fec_cm3_cti_lock_access_register;
#[doc = "FEC_CM3_CTI_Lock_Status_Register (rw) register accessor: FEC_CM3_CTI_Lock_Status_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_lock_status_register::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_lock_status_register::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_lock_status_register`]
module"]
#[doc(alias = "FEC_CM3_CTI_Lock_Status_Register")]
pub type FecCm3CtiLockStatusRegister =
    crate::Reg<fec_cm3_cti_lock_status_register::FecCm3CtiLockStatusRegisterSpec>;
#[doc = "FEC_CM3_CTI_Lock_Status_Register"]
pub mod fec_cm3_cti_lock_status_register;
#[doc = "FEC_CM3_CTI_Authentication_Status (rw) register accessor: FEC_CM3_CTI_Authentication_Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_authentication_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_authentication_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_authentication_status`]
module"]
#[doc(alias = "FEC_CM3_CTI_Authentication_Status")]
pub type FecCm3CtiAuthenticationStatus =
    crate::Reg<fec_cm3_cti_authentication_status::FecCm3CtiAuthenticationStatusSpec>;
#[doc = "FEC_CM3_CTI_Authentication_Status"]
pub mod fec_cm3_cti_authentication_status;
#[doc = "FEC_CM3_CTI_Device_ID (rw) register accessor: FEC_CM3_CTI_Device_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_device_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_device_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_device_id`]
module"]
#[doc(alias = "FEC_CM3_CTI_Device_ID")]
pub type FecCm3CtiDeviceId = crate::Reg<fec_cm3_cti_device_id::FecCm3CtiDeviceIdSpec>;
#[doc = "FEC_CM3_CTI_Device_ID"]
pub mod fec_cm3_cti_device_id;
#[doc = "FEC_CM3_CTI_Device_Type_Identifier (rw) register accessor: FEC_CM3_CTI_Device_Type_Identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_device_type_identifier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_device_type_identifier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_device_type_identifier`]
module"]
#[doc(alias = "FEC_CM3_CTI_Device_Type_Identifier")]
pub type FecCm3CtiDeviceTypeIdentifier =
    crate::Reg<fec_cm3_cti_device_type_identifier::FecCm3CtiDeviceTypeIdentifierSpec>;
#[doc = "FEC_CM3_CTI_Device_Type_Identifier"]
pub mod fec_cm3_cti_device_type_identifier;
#[doc = "FEC_CM3_CTI_PeripheralID4 (rw) register accessor: FEC_CM3_CTI_PeripheralID4\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id4`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID4")]
pub type FecCm3CtiPeripheralId4 =
    crate::Reg<fec_cm3_cti_peripheral_id4::FecCm3CtiPeripheralId4Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID4"]
pub mod fec_cm3_cti_peripheral_id4;
#[doc = "FEC_CM3_CTI_PeripheralID5 (rw) register accessor: FEC_CM3_CTI_PeripheralID5\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id5`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID5")]
pub type FecCm3CtiPeripheralId5 =
    crate::Reg<fec_cm3_cti_peripheral_id5::FecCm3CtiPeripheralId5Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID5"]
pub mod fec_cm3_cti_peripheral_id5;
#[doc = "FEC_CM3_CTI_PeripheralID6 (rw) register accessor: FEC_CM3_CTI_PeripheralID6\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id6`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID6")]
pub type FecCm3CtiPeripheralId6 =
    crate::Reg<fec_cm3_cti_peripheral_id6::FecCm3CtiPeripheralId6Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID6"]
pub mod fec_cm3_cti_peripheral_id6;
#[doc = "FEC_CM3_CTI_PeripheralID7 (rw) register accessor: FEC_CM3_CTI_PeripheralID7\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id7`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID7")]
pub type FecCm3CtiPeripheralId7 =
    crate::Reg<fec_cm3_cti_peripheral_id7::FecCm3CtiPeripheralId7Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID7"]
pub mod fec_cm3_cti_peripheral_id7;
#[doc = "FEC_CM3_CTI_PeripheralID0 (rw) register accessor: FEC_CM3_CTI_PeripheralID0\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id0`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID0")]
pub type FecCm3CtiPeripheralId0 =
    crate::Reg<fec_cm3_cti_peripheral_id0::FecCm3CtiPeripheralId0Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID0"]
pub mod fec_cm3_cti_peripheral_id0;
#[doc = "FEC_CM3_CTI_PeripheralID1 (rw) register accessor: FEC_CM3_CTI_PeripheralID1\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id1`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID1")]
pub type FecCm3CtiPeripheralId1 =
    crate::Reg<fec_cm3_cti_peripheral_id1::FecCm3CtiPeripheralId1Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID1"]
pub mod fec_cm3_cti_peripheral_id1;
#[doc = "FEC_CM3_CTI_PeripheralID2 (rw) register accessor: FEC_CM3_CTI_PeripheralID2\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id2`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID2")]
pub type FecCm3CtiPeripheralId2 =
    crate::Reg<fec_cm3_cti_peripheral_id2::FecCm3CtiPeripheralId2Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID2"]
pub mod fec_cm3_cti_peripheral_id2;
#[doc = "FEC_CM3_CTI_PeripheralID3 (rw) register accessor: FEC_CM3_CTI_PeripheralID3\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_peripheral_id3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_peripheral_id3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_peripheral_id3`]
module"]
#[doc(alias = "FEC_CM3_CTI_PeripheralID3")]
pub type FecCm3CtiPeripheralId3 =
    crate::Reg<fec_cm3_cti_peripheral_id3::FecCm3CtiPeripheralId3Spec>;
#[doc = "FEC_CM3_CTI_PeripheralID3"]
pub mod fec_cm3_cti_peripheral_id3;
#[doc = "FEC_CM3_CTI_Component_ID0 (rw) register accessor: FEC_CM3_CTI_Component_ID0\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_component_id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_component_id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_component_id0`]
module"]
#[doc(alias = "FEC_CM3_CTI_Component_ID0")]
pub type FecCm3CtiComponentId0 = crate::Reg<fec_cm3_cti_component_id0::FecCm3CtiComponentId0Spec>;
#[doc = "FEC_CM3_CTI_Component_ID0"]
pub mod fec_cm3_cti_component_id0;
#[doc = "FEC_CM3_CTI_Component_ID1 (rw) register accessor: FEC_CM3_CTI_Component_ID1\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_component_id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_component_id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_component_id1`]
module"]
#[doc(alias = "FEC_CM3_CTI_Component_ID1")]
pub type FecCm3CtiComponentId1 = crate::Reg<fec_cm3_cti_component_id1::FecCm3CtiComponentId1Spec>;
#[doc = "FEC_CM3_CTI_Component_ID1"]
pub mod fec_cm3_cti_component_id1;
#[doc = "FEC_CM3_CTI_Component_ID2 (rw) register accessor: FEC_CM3_CTI_Component_ID2\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_component_id2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_component_id2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_component_id2`]
module"]
#[doc(alias = "FEC_CM3_CTI_Component_ID2")]
pub type FecCm3CtiComponentId2 = crate::Reg<fec_cm3_cti_component_id2::FecCm3CtiComponentId2Spec>;
#[doc = "FEC_CM3_CTI_Component_ID2"]
pub mod fec_cm3_cti_component_id2;
#[doc = "FEC_CM3_CTI_Component_ID3 (rw) register accessor: FEC_CM3_CTI_Component_ID3\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_component_id3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_component_id3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fec_cm3_cti_component_id3`]
module"]
#[doc(alias = "FEC_CM3_CTI_Component_ID3")]
pub type FecCm3CtiComponentId3 = crate::Reg<fec_cm3_cti_component_id3::FecCm3CtiComponentId3Spec>;
#[doc = "FEC_CM3_CTI_Component_ID3"]
pub mod fec_cm3_cti_component_id3;
