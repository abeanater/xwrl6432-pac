#[doc = "Register `PLLDIG_APLL_SW_DIS_DELAY2` reader"]
pub type R = crate::R<PlldigApllSwDisDelay2Spec>;
#[doc = "Register `PLLDIG_APLL_SW_DIS_DELAY2` writer"]
pub type W = crate::W<PlldigApllSwDisDelay2Spec>;
#[doc = "Field `cfg_plldig_disable_delay` reader - 15:0\\]
Delay between the PLL clock source switching and disabling of the PLL DIG"]
pub type CfgPlldigDisableDelayR = crate::FieldReader<u16>;
#[doc = "Field `cfg_plldig_disable_delay` writer - 15:0\\]
Delay between the PLL clock source switching and disabling of the PLL DIG"]
pub type CfgPlldigDisableDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cfg_apll_disable_delay` reader - 31:16\\]
Delay between the PLL clock source switching and disabling of the APLL"]
pub type CfgApllDisableDelayR = crate::FieldReader<u16>;
#[doc = "Field `cfg_apll_disable_delay` writer - 31:16\\]
Delay between the PLL clock source switching and disabling of the APLL"]
pub type CfgApllDisableDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Delay between the PLL clock source switching and disabling of the PLL DIG"]
    #[inline(always)]
    pub fn cfg_plldig_disable_delay(&self) -> CfgPlldigDisableDelayR {
        CfgPlldigDisableDelayR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Delay between the PLL clock source switching and disabling of the APLL"]
    #[inline(always)]
    pub fn cfg_apll_disable_delay(&self) -> CfgApllDisableDelayR {
        CfgApllDisableDelayR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Delay between the PLL clock source switching and disabling of the PLL DIG"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_disable_delay(
        &mut self,
    ) -> CfgPlldigDisableDelayW<PlldigApllSwDisDelay2Spec> {
        CfgPlldigDisableDelayW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Delay between the PLL clock source switching and disabling of the APLL"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_apll_disable_delay(&mut self) -> CfgApllDisableDelayW<PlldigApllSwDisDelay2Spec> {
        CfgApllDisableDelayW::new(self, 16)
    }
}
#[doc = "PLLDIG_APLL_SW_DIS_DELAY2\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_apll_sw_dis_delay2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_apll_sw_dis_delay2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigApllSwDisDelay2Spec;
impl crate::RegisterSpec for PlldigApllSwDisDelay2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_apll_sw_dis_delay2::R`](R) reader structure"]
impl crate::Readable for PlldigApllSwDisDelay2Spec {}
#[doc = "`write(|w| ..)` method takes [`plldig_apll_sw_dis_delay2::W`](W) writer structure"]
impl crate::Writable for PlldigApllSwDisDelay2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_APLL_SW_DIS_DELAY2 to value 0"]
impl crate::Resettable for PlldigApllSwDisDelay2Spec {
    const RESET_VALUE: u32 = 0;
}
