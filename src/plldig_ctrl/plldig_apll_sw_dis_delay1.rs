#[doc = "Register `PLLDIG_APLL_SW_DIS_DELAY1` reader"]
pub type R = crate::R<PlldigApllSwDisDelay1Spec>;
#[doc = "Register `PLLDIG_APLL_SW_DIS_DELAY1` writer"]
pub type W = crate::W<PlldigApllSwDisDelay1Spec>;
#[doc = "Field `cfg_apll_auto_switch_delay` reader - 15:0\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
pub type CfgApllAutoSwitchDelayR = crate::FieldReader<u16>;
#[doc = "Field `cfg_apll_auto_switch_delay` writer - 15:0\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
pub type CfgApllAutoSwitchDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cfg_plldig_auto_switch_delay` reader - 31:16\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
pub type CfgPlldigAutoSwitchDelayR = crate::FieldReader<u16>;
#[doc = "Field `cfg_plldig_auto_switch_delay` writer - 31:16\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
pub type CfgPlldigAutoSwitchDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
    #[inline(always)]
    pub fn cfg_apll_auto_switch_delay(&self) -> CfgApllAutoSwitchDelayR {
        CfgApllAutoSwitchDelayR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
    #[inline(always)]
    pub fn cfg_plldig_auto_switch_delay(&self) -> CfgPlldigAutoSwitchDelayR {
        CfgPlldigAutoSwitchDelayR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_apll_auto_switch_delay(
        &mut self,
    ) -> CfgApllAutoSwitchDelayW<PlldigApllSwDisDelay1Spec> {
        CfgApllAutoSwitchDelayW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Delay to switch the PLL clock source when the auto PLL switch mode is enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_auto_switch_delay(
        &mut self,
    ) -> CfgPlldigAutoSwitchDelayW<PlldigApllSwDisDelay1Spec> {
        CfgPlldigAutoSwitchDelayW::new(self, 16)
    }
}
#[doc = "PLLDIG_APLL_SW_DIS_DELAY1\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_apll_sw_dis_delay1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_apll_sw_dis_delay1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigApllSwDisDelay1Spec;
impl crate::RegisterSpec for PlldigApllSwDisDelay1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_apll_sw_dis_delay1::R`](R) reader structure"]
impl crate::Readable for PlldigApllSwDisDelay1Spec {}
#[doc = "`write(|w| ..)` method takes [`plldig_apll_sw_dis_delay1::W`](W) writer structure"]
impl crate::Writable for PlldigApllSwDisDelay1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_APLL_SW_DIS_DELAY1 to value 0"]
impl crate::Resettable for PlldigApllSwDisDelay1Spec {
    const RESET_VALUE: u32 = 0;
}
