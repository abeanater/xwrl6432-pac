#[doc = "Register `DEBUG_STATUS_AON_1` reader"]
pub type R = crate::R<DebugStatusAon1Spec>;
#[doc = "Register `DEBUG_STATUS_AON_1` writer"]
pub type W = crate::W<DebugStatusAon1Spec>;
#[doc = "Field `sram_ldo_state` reader - 4:0\\]
status reg for sram_ldo_state"]
pub type SramLdoStateR = crate::FieldReader;
#[doc = "Field `sram_ldo_state` writer - 4:0\\]
status reg for sram_ldo_state"]
pub type SramLdoStateW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sram_ldo_en` reader - 5:5\\]
status reg for sram_ldo_en"]
pub type SramLdoEnR = crate::BitReader;
#[doc = "Field `sram_ldo_en` writer - 5:5\\]
status reg for sram_ldo_en"]
pub type SramLdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram_ka_ldo_en` reader - 6:6\\]
status reg for sram_ka_ldo_en"]
pub type SramKaLdoEnR = crate::BitReader;
#[doc = "Field `sram_ka_ldo_en` writer - 6:6\\]
status reg for sram_ka_ldo_en"]
pub type SramKaLdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_ldo_state` reader - 10:7\\]
status reg for dig_ldo_state"]
pub type DigLdoStateR = crate::FieldReader;
#[doc = "Field `dig_ldo_state` writer - 10:7\\]
status reg for dig_ldo_state"]
pub type DigLdoStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dig_ldo_en` reader - 11:11\\]
status reg for dig_ldo_en"]
pub type DigLdoEnR = crate::BitReader;
#[doc = "Field `dig_ldo_en` writer - 11:11\\]
status reg for dig_ldo_en"]
pub type DigLdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_ka_ldo_en` reader - 12:12\\]
status reg for dig_ka_ldo_en"]
pub type DigKaLdoEnR = crate::BitReader;
#[doc = "Field `dig_ka_ldo_en` writer - 12:12\\]
status reg for dig_ka_ldo_en"]
pub type DigKaLdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bgap_state` reader - 16:13\\]
status reg for bgap_state"]
pub type BgapStateR = crate::FieldReader;
#[doc = "Field `bgap_state` writer - 16:13\\]
status reg for bgap_state"]
pub type BgapStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bgap_en` reader - 17:17\\]
status reg for bgap_en"]
pub type BgapEnR = crate::BitReader;
#[doc = "Field `bgap_en` writer - 17:17\\]
status reg for bgap_en"]
pub type BgapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bgap_cap_sw_enz` reader - 18:18\\]
status reg for bgap_cap_sw_enz"]
pub type BgapCapSwEnzR = crate::BitReader;
#[doc = "Field `bgap_cap_sw_enz` writer - 18:18\\]
status reg for bgap_cap_sw_enz"]
pub type BgapCapSwEnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bgap_cap_charge_en` reader - 19:19\\]
status reg for bgap_cap_charge_en"]
pub type BgapCapChargeEnR = crate::BitReader;
#[doc = "Field `bgap_cap_charge_en` writer - 19:19\\]
status reg for bgap_cap_charge_en"]
pub type BgapCapChargeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bgap_hib_cap_sw_en` reader - 20:20\\]
status reg for bgap_hib_cap_sw_en"]
pub type BgapHibCapSwEnR = crate::BitReader;
#[doc = "Field `bgap_hib_cap_sw_en` writer - 20:20\\]
status reg for bgap_hib_cap_sw_en"]
pub type BgapHibCapSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bgap_hib_ref_cap_charge_en` reader - 21:21\\]
status reg for bgap_hib_ref_cap_charge_en"]
pub type BgapHibRefCapChargeEnR = crate::BitReader;
#[doc = "Field `bgap_hib_ref_cap_charge_en` writer - 21:21\\]
status reg for bgap_hib_ref_cap_charge_en"]
pub type BgapHibRefCapChargeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
status reg for sram_ldo_state"]
    #[inline(always)]
    pub fn sram_ldo_state(&self) -> SramLdoStateR {
        SramLdoStateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for sram_ldo_en"]
    #[inline(always)]
    pub fn sram_ldo_en(&self) -> SramLdoEnR {
        SramLdoEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for sram_ka_ldo_en"]
    #[inline(always)]
    pub fn sram_ka_ldo_en(&self) -> SramKaLdoEnR {
        SramKaLdoEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - 10:7\\]
status reg for dig_ldo_state"]
    #[inline(always)]
    pub fn dig_ldo_state(&self) -> DigLdoStateR {
        DigLdoStateR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for dig_ldo_en"]
    #[inline(always)]
    pub fn dig_ldo_en(&self) -> DigLdoEnR {
        DigLdoEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for dig_ka_ldo_en"]
    #[inline(always)]
    pub fn dig_ka_ldo_en(&self) -> DigKaLdoEnR {
        DigKaLdoEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - 16:13\\]
status reg for bgap_state"]
    #[inline(always)]
    pub fn bgap_state(&self) -> BgapStateR {
        BgapStateR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
status reg for bgap_en"]
    #[inline(always)]
    pub fn bgap_en(&self) -> BgapEnR {
        BgapEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
status reg for bgap_cap_sw_enz"]
    #[inline(always)]
    pub fn bgap_cap_sw_enz(&self) -> BgapCapSwEnzR {
        BgapCapSwEnzR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
status reg for bgap_cap_charge_en"]
    #[inline(always)]
    pub fn bgap_cap_charge_en(&self) -> BgapCapChargeEnR {
        BgapCapChargeEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
status reg for bgap_hib_cap_sw_en"]
    #[inline(always)]
    pub fn bgap_hib_cap_sw_en(&self) -> BgapHibCapSwEnR {
        BgapHibCapSwEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
status reg for bgap_hib_ref_cap_charge_en"]
    #[inline(always)]
    pub fn bgap_hib_ref_cap_charge_en(&self) -> BgapHibRefCapChargeEnR {
        BgapHibRefCapChargeEnR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
status reg for sram_ldo_state"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ldo_state(&mut self) -> SramLdoStateW<DebugStatusAon1Spec> {
        SramLdoStateW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for sram_ldo_en"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ldo_en(&mut self) -> SramLdoEnW<DebugStatusAon1Spec> {
        SramLdoEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for sram_ka_ldo_en"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ka_ldo_en(&mut self) -> SramKaLdoEnW<DebugStatusAon1Spec> {
        SramKaLdoEnW::new(self, 6)
    }
    #[doc = "Bits 7:10 - 10:7\\]
status reg for dig_ldo_state"]
    #[inline(always)]
    #[must_use]
    pub fn dig_ldo_state(&mut self) -> DigLdoStateW<DebugStatusAon1Spec> {
        DigLdoStateW::new(self, 7)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for dig_ldo_en"]
    #[inline(always)]
    #[must_use]
    pub fn dig_ldo_en(&mut self) -> DigLdoEnW<DebugStatusAon1Spec> {
        DigLdoEnW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for dig_ka_ldo_en"]
    #[inline(always)]
    #[must_use]
    pub fn dig_ka_ldo_en(&mut self) -> DigKaLdoEnW<DebugStatusAon1Spec> {
        DigKaLdoEnW::new(self, 12)
    }
    #[doc = "Bits 13:16 - 16:13\\]
status reg for bgap_state"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_state(&mut self) -> BgapStateW<DebugStatusAon1Spec> {
        BgapStateW::new(self, 13)
    }
    #[doc = "Bit 17 - 17:17\\]
status reg for bgap_en"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_en(&mut self) -> BgapEnW<DebugStatusAon1Spec> {
        BgapEnW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
status reg for bgap_cap_sw_enz"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_cap_sw_enz(&mut self) -> BgapCapSwEnzW<DebugStatusAon1Spec> {
        BgapCapSwEnzW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
status reg for bgap_cap_charge_en"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_cap_charge_en(&mut self) -> BgapCapChargeEnW<DebugStatusAon1Spec> {
        BgapCapChargeEnW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
status reg for bgap_hib_cap_sw_en"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_hib_cap_sw_en(&mut self) -> BgapHibCapSwEnW<DebugStatusAon1Spec> {
        BgapHibCapSwEnW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
status reg for bgap_hib_ref_cap_charge_en"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_hib_ref_cap_charge_en(&mut self) -> BgapHibRefCapChargeEnW<DebugStatusAon1Spec> {
        BgapHibRefCapChargeEnW::new(self, 21)
    }
}
#[doc = "DEBUG_STATUS_AON_1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon1Spec;
impl crate::RegisterSpec for DebugStatusAon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_1::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon1Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_1::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_1 to value 0"]
impl crate::Resettable for DebugStatusAon1Spec {
    const RESET_VALUE: u32 = 0;
}
