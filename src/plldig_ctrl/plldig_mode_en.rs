#[doc = "Register `PLLDIG_MODE_EN` reader"]
pub type R = crate::R<PlldigModeEnSpec>;
#[doc = "Register `PLLDIG_MODE_EN` writer"]
pub type W = crate::W<PlldigModeEnSpec>;
#[doc = "Field `cfg_plldig_highfreq_mode_en` reader - 0:0\\]
PLL DIG high frequency mode operation"]
pub type CfgPlldigHighfreqModeEnR = crate::BitReader;
#[doc = "Field `cfg_plldig_highfreq_mode_en` writer - 0:0\\]
PLL DIG high frequency mode operation"]
pub type CfgPlldigHighfreqModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_plldig_lowfreq_mode_en` reader - 16:16\\]
PLL DIG high frequency mode operation, Divide by 2 the PLL clock out"]
pub type CfgPlldigLowfreqModeEnR = crate::BitReader;
#[doc = "Field `cfg_plldig_lowfreq_mode_en` writer - 16:16\\]
PLL DIG high frequency mode operation, Divide by 2 the PLL clock out"]
pub type CfgPlldigLowfreqModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PLL DIG high frequency mode operation"]
    #[inline(always)]
    pub fn cfg_plldig_highfreq_mode_en(&self) -> CfgPlldigHighfreqModeEnR {
        CfgPlldigHighfreqModeEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
PLL DIG high frequency mode operation, Divide by 2 the PLL clock out"]
    #[inline(always)]
    pub fn cfg_plldig_lowfreq_mode_en(&self) -> CfgPlldigLowfreqModeEnR {
        CfgPlldigLowfreqModeEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PLL DIG high frequency mode operation"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_highfreq_mode_en(&mut self) -> CfgPlldigHighfreqModeEnW<PlldigModeEnSpec> {
        CfgPlldigHighfreqModeEnW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
PLL DIG high frequency mode operation, Divide by 2 the PLL clock out"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_lowfreq_mode_en(&mut self) -> CfgPlldigLowfreqModeEnW<PlldigModeEnSpec> {
        CfgPlldigLowfreqModeEnW::new(self, 16)
    }
}
#[doc = "PLLDIG_MODE_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_mode_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_mode_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigModeEnSpec;
impl crate::RegisterSpec for PlldigModeEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_mode_en::R`](R) reader structure"]
impl crate::Readable for PlldigModeEnSpec {}
#[doc = "`write(|w| ..)` method takes [`plldig_mode_en::W`](W) writer structure"]
impl crate::Writable for PlldigModeEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_MODE_EN to value 0"]
impl crate::Resettable for PlldigModeEnSpec {
    const RESET_VALUE: u32 = 0;
}
