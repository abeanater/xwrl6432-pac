#[doc = "Register `DFT_APPSS_LSTC_VBUSP_CLK_GATE` reader"]
pub type R = crate::R<DftAppssLstcVbuspClkGateSpec>;
#[doc = "Register `DFT_APPSS_LSTC_VBUSP_CLK_GATE` writer"]
pub type W = crate::W<DftAppssLstcVbuspClkGateSpec>;
#[doc = "Field `enable` reader - 2:0\\]
3'b000 : Gate clock to LSTC VBUSP 3'b111 : Ungate Clock to LSTC VBUSP"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
3'b000 : Gate clock to LSTC VBUSP 3'b111 : Ungate Clock to LSTC VBUSP"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Gate clock to LSTC VBUSP 3'b111 : Ungate Clock to LSTC VBUSP"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 : Gate clock to LSTC VBUSP 3'b111 : Ungate Clock to LSTC VBUSP"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<DftAppssLstcVbuspClkGateSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "DFT_APPSS_LSTC_VBUSP_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_appss_lstc_vbusp_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_appss_lstc_vbusp_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftAppssLstcVbuspClkGateSpec;
impl crate::RegisterSpec for DftAppssLstcVbuspClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_appss_lstc_vbusp_clk_gate::R`](R) reader structure"]
impl crate::Readable for DftAppssLstcVbuspClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_appss_lstc_vbusp_clk_gate::W`](W) writer structure"]
impl crate::Writable for DftAppssLstcVbuspClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFT_APPSS_LSTC_VBUSP_CLK_GATE to value 0"]
impl crate::Resettable for DftAppssLstcVbuspClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
