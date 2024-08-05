#[doc = "Register `TZFRC_ETSEL` reader"]
pub type R = crate::R<TzfrcEtselSpec>;
#[doc = "Register `TZFRC_ETSEL` writer"]
pub type W = crate::W<TzfrcEtselSpec>;
#[doc = "Field `Reserved2` reader - 0:0\\]
Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - 0:0\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFRC_CBC` reader - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a cycle-by-cycle trip event and sets the TZFLG\\[CBC\\]
bit."]
pub type TzfrcCbcR = crate::BitReader;
#[doc = "Field `TZFRC_CBC` writer - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a cycle-by-cycle trip event and sets the TZFLG\\[CBC\\]
bit."]
pub type TzfrcCbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFRC_OST` reader - 2:2\\]
Force a One-Shot Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a one-shot trip event and sets the TZFLG\\[OST\\]
bit"]
pub type TzfrcOstR = crate::BitReader;
#[doc = "Field `TZFRC_OST` writer - 2:2\\]
Force a One-Shot Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a one-shot trip event and sets the TZFLG\\[OST\\]
bit"]
pub type TzfrcOstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFRC_DCAEVT1` reader - 3:3\\]
Force Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0 1 Writing 1 forces the DCAEVT1 event trip condition and sets the TZFLG\\[DCAEVT1\\]
bit"]
pub type TzfrcDcaevt1R = crate::BitReader;
#[doc = "Field `TZFRC_DCAEVT1` writer - 3:3\\]
Force Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0 1 Writing 1 forces the DCAEVT1 event trip condition and sets the TZFLG\\[DCAEVT1\\]
bit"]
pub type TzfrcDcaevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFRC_DCAEVT2` reader - 4:4\\]
Force Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCAEVT2 event trip condition and sets the TZFLG\\[DCAEVT2\\]
bit"]
pub type TzfrcDcaevt2R = crate::BitReader;
#[doc = "Field `TZFRC_DCAEVT2` writer - 4:4\\]
Force Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCAEVT2 event trip condition and sets the TZFLG\\[DCAEVT2\\]
bit"]
pub type TzfrcDcaevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFRC_DCBEVT1` reader - 5:5\\]
Force Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT1 event trip condition and sets the TZFLG\\[DCBEVT1\\]
bit"]
pub type TzfrcDcbevt1R = crate::BitReader;
#[doc = "Field `TZFRC_DCBEVT1` writer - 5:5\\]
Force Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT1 event trip condition and sets the TZFLG\\[DCBEVT1\\]
bit"]
pub type TzfrcDcbevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZFRC_DCBEVT2` reader - 6:6\\]
Force Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT2 event trip condition and sets the TZFLG\\[DCBEVT2\\]
bit."]
pub type TzfrcDcbevt2R = crate::BitReader;
#[doc = "Field `TZFRC_DCBEVT2` writer - 6:6\\]
Force Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT2 event trip condition and sets the TZFLG\\[DCBEVT2\\]
bit."]
pub type TzfrcDcbevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 15:7\\]
Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `Reserved3` writer - 15:7\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ETSEL_INTSEL` reader - 18:16\\]
ePWM Interrupt (EPWMx_INT) Selection Options 0 Reserved 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing."]
pub type EtselIntselR = crate::FieldReader;
#[doc = "Field `ETSEL_INTSEL` writer - 18:16\\]
ePWM Interrupt (EPWMx_INT) Selection Options 0 Reserved 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing."]
pub type EtselIntselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETSEL_INTEN` reader - 19:19\\]
Enable ePWM Interrupt (EPWMx_INT) Generation 0 Disable EPWMx_INT generation 1 Enable EPWMx_INT generation"]
pub type EtselIntenR = crate::BitReader;
#[doc = "Field `ETSEL_INTEN` writer - 19:19\\]
Enable ePWM Interrupt (EPWMx_INT) Generation 0 Disable EPWMx_INT generation 1 Enable EPWMx_INT generation"]
pub type EtselIntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 23:20\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 23:20\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETSEL_SOCASEL` reader - 26:24\\]
EPWMxSOCA Selection Options These bits determine when a EPWMxSOCA pulse will be generated. 0 Enable DCAEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
pub type EtselSocaselR = crate::FieldReader;
#[doc = "Field `ETSEL_SOCASEL` writer - 26:24\\]
EPWMxSOCA Selection Options These bits determine when a EPWMxSOCA pulse will be generated. 0 Enable DCAEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
pub type EtselSocaselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETSEL_SOCAEN` reader - 27:27\\]
Enable the ADC Start of Conversion A (EPWMxSOCA) Pulse 0 Disable EPWMxSOCA. 1 Enable EPWMxSOCA pulse."]
pub type EtselSocaenR = crate::BitReader;
#[doc = "Field `ETSEL_SOCAEN` writer - 27:27\\]
Enable the ADC Start of Conversion A (EPWMxSOCA) Pulse 0 Disable EPWMxSOCA. 1 Enable EPWMxSOCA pulse."]
pub type EtselSocaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSEL_SOCBSEL` reader - 30:28\\]
EPWMxSOCB Selection Options These bits determine when a EPWMxSOCB pulse will be generated. 0 Enable DCBEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
pub type EtselSocbselR = crate::FieldReader;
#[doc = "Field `ETSEL_SOCBSEL` writer - 30:28\\]
EPWMxSOCB Selection Options These bits determine when a EPWMxSOCB pulse will be generated. 0 Enable DCBEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
pub type EtselSocbselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETSEL_SOCBEN` reader - 31:31\\]
Enable the ADC Start of Conversion B (EPWMxSOCB) Pulse 0 Disable EPWMxSOCB. 1 Enable EPWMxSOCB pulse"]
pub type EtselSocbenR = crate::BitReader;
#[doc = "Field `ETSEL_SOCBEN` writer - 31:31\\]
Enable the ADC Start of Conversion B (EPWMxSOCB) Pulse 0 Disable EPWMxSOCB. 1 Enable EPWMxSOCB pulse"]
pub type EtselSocbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a cycle-by-cycle trip event and sets the TZFLG\\[CBC\\]
bit."]
    #[inline(always)]
    pub fn tzfrc_cbc(&self) -> TzfrcCbcR {
        TzfrcCbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Force a One-Shot Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a one-shot trip event and sets the TZFLG\\[OST\\]
bit"]
    #[inline(always)]
    pub fn tzfrc_ost(&self) -> TzfrcOstR {
        TzfrcOstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0 1 Writing 1 forces the DCAEVT1 event trip condition and sets the TZFLG\\[DCAEVT1\\]
bit"]
    #[inline(always)]
    pub fn tzfrc_dcaevt1(&self) -> TzfrcDcaevt1R {
        TzfrcDcaevt1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCAEVT2 event trip condition and sets the TZFLG\\[DCAEVT2\\]
bit"]
    #[inline(always)]
    pub fn tzfrc_dcaevt2(&self) -> TzfrcDcaevt2R {
        TzfrcDcaevt2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT1 event trip condition and sets the TZFLG\\[DCBEVT1\\]
bit"]
    #[inline(always)]
    pub fn tzfrc_dcbevt1(&self) -> TzfrcDcbevt1R {
        TzfrcDcbevt1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT2 event trip condition and sets the TZFLG\\[DCBEVT2\\]
bit."]
    #[inline(always)]
    pub fn tzfrc_dcbevt2(&self) -> TzfrcDcbevt2R {
        TzfrcDcbevt2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
ePWM Interrupt (EPWMx_INT) Selection Options 0 Reserved 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing."]
    #[inline(always)]
    pub fn etsel_intsel(&self) -> EtselIntselR {
        EtselIntselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable ePWM Interrupt (EPWMx_INT) Generation 0 Disable EPWMx_INT generation 1 Enable EPWMx_INT generation"]
    #[inline(always)]
    pub fn etsel_inten(&self) -> EtselIntenR {
        EtselIntenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
EPWMxSOCA Selection Options These bits determine when a EPWMxSOCA pulse will be generated. 0 Enable DCAEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
    #[inline(always)]
    pub fn etsel_socasel(&self) -> EtselSocaselR {
        EtselSocaselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Enable the ADC Start of Conversion A (EPWMxSOCA) Pulse 0 Disable EPWMxSOCA. 1 Enable EPWMxSOCA pulse."]
    #[inline(always)]
    pub fn etsel_socaen(&self) -> EtselSocaenR {
        EtselSocaenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
EPWMxSOCB Selection Options These bits determine when a EPWMxSOCB pulse will be generated. 0 Enable DCBEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
    #[inline(always)]
    pub fn etsel_socbsel(&self) -> EtselSocbselR {
        EtselSocbselR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable the ADC Start of Conversion B (EPWMxSOCB) Pulse 0 Disable EPWMxSOCB. 1 Enable EPWMxSOCB pulse"]
    #[inline(always)]
    pub fn etsel_socben(&self) -> EtselSocbenR {
        EtselSocbenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<TzfrcEtselSpec> {
        Reserved2W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force a Cycle-by-Cycle Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a cycle-by-cycle trip event and sets the TZFLG\\[CBC\\]
bit."]
    #[inline(always)]
    #[must_use]
    pub fn tzfrc_cbc(&mut self) -> TzfrcCbcW<TzfrcEtselSpec> {
        TzfrcCbcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Force a One-Shot Trip Event via Software 0 Writing of 0 is ignored. Always reads back a 0. 1 Forces a one-shot trip event and sets the TZFLG\\[OST\\]
bit"]
    #[inline(always)]
    #[must_use]
    pub fn tzfrc_ost(&mut self) -> TzfrcOstW<TzfrcEtselSpec> {
        TzfrcOstW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Flag for Digital Compare Output A Event 1 0 Writing 0 has no effect. This bit always reads back 0 1 Writing 1 forces the DCAEVT1 event trip condition and sets the TZFLG\\[DCAEVT1\\]
bit"]
    #[inline(always)]
    #[must_use]
    pub fn tzfrc_dcaevt1(&mut self) -> TzfrcDcaevt1W<TzfrcEtselSpec> {
        TzfrcDcaevt1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Flag for Digital Compare Output A Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCAEVT2 event trip condition and sets the TZFLG\\[DCAEVT2\\]
bit"]
    #[inline(always)]
    #[must_use]
    pub fn tzfrc_dcaevt2(&mut self) -> TzfrcDcaevt2W<TzfrcEtselSpec> {
        TzfrcDcaevt2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Flag for Digital Compare Output B Event 1 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT1 event trip condition and sets the TZFLG\\[DCBEVT1\\]
bit"]
    #[inline(always)]
    #[must_use]
    pub fn tzfrc_dcbevt1(&mut self) -> TzfrcDcbevt1W<TzfrcEtselSpec> {
        TzfrcDcbevt1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Flag for Digital Compare Output B Event 2 0 Writing 0 has no effect. This bit always reads back 0. 1 Writing 1 forces the DCBEVT2 event trip condition and sets the TZFLG\\[DCBEVT2\\]
bit."]
    #[inline(always)]
    #[must_use]
    pub fn tzfrc_dcbevt2(&mut self) -> TzfrcDcbevt2W<TzfrcEtselSpec> {
        TzfrcDcbevt2W::new(self, 6)
    }
    #[doc = "Bits 7:15 - 15:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<TzfrcEtselSpec> {
        Reserved3W::new(self, 7)
    }
    #[doc = "Bits 16:18 - 18:16\\]
ePWM Interrupt (EPWMx_INT) Selection Options 0 Reserved 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing."]
    #[inline(always)]
    #[must_use]
    pub fn etsel_intsel(&mut self) -> EtselIntselW<TzfrcEtselSpec> {
        EtselIntselW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
Enable ePWM Interrupt (EPWMx_INT) Generation 0 Disable EPWMx_INT generation 1 Enable EPWMx_INT generation"]
    #[inline(always)]
    #[must_use]
    pub fn etsel_inten(&mut self) -> EtselIntenW<TzfrcEtselSpec> {
        EtselIntenW::new(self, 19)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<TzfrcEtselSpec> {
        Reserved1W::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
EPWMxSOCA Selection Options These bits determine when a EPWMxSOCA pulse will be generated. 0 Enable DCAEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
    #[inline(always)]
    #[must_use]
    pub fn etsel_socasel(&mut self) -> EtselSocaselW<TzfrcEtselSpec> {
        EtselSocaselW::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
Enable the ADC Start of Conversion A (EPWMxSOCA) Pulse 0 Disable EPWMxSOCA. 1 Enable EPWMxSOCA pulse."]
    #[inline(always)]
    #[must_use]
    pub fn etsel_socaen(&mut self) -> EtselSocaenW<TzfrcEtselSpec> {
        EtselSocaenW::new(self, 27)
    }
    #[doc = "Bits 28:30 - 30:28\\]
EPWMxSOCB Selection Options These bits determine when a EPWMxSOCB pulse will be generated. 0 Enable DCBEVT1.soc event 1h Enable event time-base counter equal to zero. (TBCTR = 0x0000) 2h Enable event time-base counter equal to period (TBCTR = TBPRD) 3h Enable event time-base counter equal to zero or period (TBCTR = 0x0000 or TBCTR = TBPRD). This mode is useful in up-down count mode. 4h Enable event time-base counter equal to CMPA when the timer is incrementing. 5h Enable event time-base counter equal to CMPA when the timer is decrementing. 6h Enable event: time-base counter equal to CMPB when the timer is incrementing. 7h Enable event: time-base counter equal to CMPB when the timer is decrementing"]
    #[inline(always)]
    #[must_use]
    pub fn etsel_socbsel(&mut self) -> EtselSocbselW<TzfrcEtselSpec> {
        EtselSocbselW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable the ADC Start of Conversion B (EPWMxSOCB) Pulse 0 Disable EPWMxSOCB. 1 Enable EPWMxSOCB pulse"]
    #[inline(always)]
    #[must_use]
    pub fn etsel_socben(&mut self) -> EtselSocbenW<TzfrcEtselSpec> {
        EtselSocbenW::new(self, 31)
    }
}
#[doc = "Trip-Zone Force Register / Event-Trigger Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzfrc_etsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfrc_etsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzfrcEtselSpec;
impl crate::RegisterSpec for TzfrcEtselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzfrc_etsel::R`](R) reader structure"]
impl crate::Readable for TzfrcEtselSpec {}
#[doc = "`write(|w| ..)` method takes [`tzfrc_etsel::W`](W) writer structure"]
impl crate::Writable for TzfrcEtselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZFRC_ETSEL to value 0"]
impl crate::Resettable for TzfrcEtselSpec {
    const RESET_VALUE: u32 = 0;
}
