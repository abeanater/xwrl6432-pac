#[doc = "Register `PLLDIG_OVERRIDE` reader"]
pub type R = crate::R<PlldigOverrideSpec>;
#[doc = "Register `PLLDIG_OVERRIDE` writer"]
pub type W = crate::W<PlldigOverrideSpec>;
#[doc = "Field `cfg_sel_ov_final_plldig_apll_mux_sel` reader - 2:0\\]
Mux select control to select the override value of the fast clock src mux select 0x0 = functional value selected for the fast clock src mux select 0x7 = Override value selected for the fast clock src mux select"]
pub type CfgSelOvFinalPlldigApllMuxSelR = crate::FieldReader;
#[doc = "Field `cfg_sel_ov_final_plldig_apll_mux_sel` writer - 2:0\\]
Mux select control to select the override value of the fast clock src mux select 0x0 = functional value selected for the fast clock src mux select 0x7 = Override value selected for the fast clock src mux select"]
pub type CfgSelOvFinalPlldigApllMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cfg_ov_final_plldig_apll_mux_sel` reader - 5:3\\]
Override control for the fast clock src mux select 0x0 = PLL DIG selected as fast clock 0x7 = APLL selected as fast clock"]
pub type CfgOvFinalPlldigApllMuxSelR = crate::FieldReader;
#[doc = "Field `cfg_ov_final_plldig_apll_mux_sel` writer - 5:3\\]
Override control for the fast clock src mux select 0x0 = PLL DIG selected as fast clock 0x7 = APLL selected as fast clock"]
pub type CfgOvFinalPlldigApllMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Mux select control to select the override value of the fast clock src mux select 0x0 = functional value selected for the fast clock src mux select 0x7 = Override value selected for the fast clock src mux select"]
    #[inline(always)]
    pub fn cfg_sel_ov_final_plldig_apll_mux_sel(&self) -> CfgSelOvFinalPlldigApllMuxSelR {
        CfgSelOvFinalPlldigApllMuxSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Override control for the fast clock src mux select 0x0 = PLL DIG selected as fast clock 0x7 = APLL selected as fast clock"]
    #[inline(always)]
    pub fn cfg_ov_final_plldig_apll_mux_sel(&self) -> CfgOvFinalPlldigApllMuxSelR {
        CfgOvFinalPlldigApllMuxSelR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Mux select control to select the override value of the fast clock src mux select 0x0 = functional value selected for the fast clock src mux select 0x7 = Override value selected for the fast clock src mux select"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_sel_ov_final_plldig_apll_mux_sel(
        &mut self,
    ) -> CfgSelOvFinalPlldigApllMuxSelW<PlldigOverrideSpec> {
        CfgSelOvFinalPlldigApllMuxSelW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Override control for the fast clock src mux select 0x0 = PLL DIG selected as fast clock 0x7 = APLL selected as fast clock"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_ov_final_plldig_apll_mux_sel(
        &mut self,
    ) -> CfgOvFinalPlldigApllMuxSelW<PlldigOverrideSpec> {
        CfgOvFinalPlldigApllMuxSelW::new(self, 3)
    }
}
#[doc = "PLLDIG_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigOverrideSpec;
impl crate::RegisterSpec for PlldigOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_override::R`](R) reader structure"]
impl crate::Readable for PlldigOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`plldig_override::W`](W) writer structure"]
impl crate::Writable for PlldigOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_OVERRIDE to value 0"]
impl crate::Resettable for PlldigOverrideSpec {
    const RESET_VALUE: u32 = 0;
}
