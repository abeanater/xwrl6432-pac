#[doc = "Register `PLLDIG_EN` reader"]
pub type R = crate::R<PlldigEnSpec>;
#[doc = "Register `PLLDIG_EN` writer"]
pub type W = crate::W<PlldigEnSpec>;
#[doc = "Field `cfg_plldig_en` reader - 2:0\\]
PLL DIG enable 0x0 = PLL DIG disable 0x7 = PLL DIG enable"]
pub type CfgPlldigEnR = crate::FieldReader;
#[doc = "Field `cfg_plldig_en` writer - 2:0\\]
PLL DIG enable 0x0 = PLL DIG disable 0x7 = PLL DIG enable"]
pub type CfgPlldigEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cfg_pll_auto_switch_enable` reader - 10:8\\]
PLL DIG and APLL auto switch enable 0x0 = PLL DIG wont be auto turn off when APLL is enable 0x7 = PLL DIG will be auto turn off when APLL is enable"]
pub type CfgPllAutoSwitchEnableR = crate::FieldReader;
#[doc = "Field `cfg_pll_auto_switch_enable` writer - 10:8\\]
PLL DIG and APLL auto switch enable 0x0 = PLL DIG wont be auto turn off when APLL is enable 0x7 = PLL DIG will be auto turn off when APLL is enable"]
pub type CfgPllAutoSwitchEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cfg_plldig_lockmon_enable` reader - 18:16\\]
PLL DIG lockmon enable 0x0 = PLL DIG lockmon disbale 0x7 = PLL DIG lockmon enable"]
pub type CfgPlldigLockmonEnableR = crate::FieldReader;
#[doc = "Field `cfg_plldig_lockmon_enable` writer - 18:16\\]
PLL DIG lockmon enable 0x0 = PLL DIG lockmon disbale 0x7 = PLL DIG lockmon enable"]
pub type CfgPlldigLockmonEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
PLL DIG enable 0x0 = PLL DIG disable 0x7 = PLL DIG enable"]
    #[inline(always)]
    pub fn cfg_plldig_en(&self) -> CfgPlldigEnR {
        CfgPlldigEnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
PLL DIG and APLL auto switch enable 0x0 = PLL DIG wont be auto turn off when APLL is enable 0x7 = PLL DIG will be auto turn off when APLL is enable"]
    #[inline(always)]
    pub fn cfg_pll_auto_switch_enable(&self) -> CfgPllAutoSwitchEnableR {
        CfgPllAutoSwitchEnableR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
PLL DIG lockmon enable 0x0 = PLL DIG lockmon disbale 0x7 = PLL DIG lockmon enable"]
    #[inline(always)]
    pub fn cfg_plldig_lockmon_enable(&self) -> CfgPlldigLockmonEnableR {
        CfgPlldigLockmonEnableR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
PLL DIG enable 0x0 = PLL DIG disable 0x7 = PLL DIG enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_en(&mut self) -> CfgPlldigEnW<PlldigEnSpec> {
        CfgPlldigEnW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
PLL DIG and APLL auto switch enable 0x0 = PLL DIG wont be auto turn off when APLL is enable 0x7 = PLL DIG will be auto turn off when APLL is enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_pll_auto_switch_enable(&mut self) -> CfgPllAutoSwitchEnableW<PlldigEnSpec> {
        CfgPllAutoSwitchEnableW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
PLL DIG lockmon enable 0x0 = PLL DIG lockmon disbale 0x7 = PLL DIG lockmon enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_lockmon_enable(&mut self) -> CfgPlldigLockmonEnableW<PlldigEnSpec> {
        CfgPlldigLockmonEnableW::new(self, 16)
    }
}
#[doc = "PLLDIG_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigEnSpec;
impl crate::RegisterSpec for PlldigEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_en::R`](R) reader structure"]
impl crate::Readable for PlldigEnSpec {}
#[doc = "`write(|w| ..)` method takes [`plldig_en::W`](W) writer structure"]
impl crate::Writable for PlldigEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_EN to value 0"]
impl crate::Resettable for PlldigEnSpec {
    const RESET_VALUE: u32 = 0;
}
