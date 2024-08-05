#[doc = "Register `XTALCLKX2_CLK_GATE` reader"]
pub type R = crate::R<Xtalclkx2ClkGateSpec>;
#[doc = "Register `XTALCLKX2_CLK_GATE` writer"]
pub type W = crate::W<Xtalclkx2ClkGateSpec>;
#[doc = "Field `XTALCLKX2_CLK_GATE` reader - 2:0\\]
Writing 3'b111 will gate the XTALX2 clock.Writing 3'b000 will ungate the clock."]
pub type Xtalclkx2ClkGateR = crate::FieldReader;
#[doc = "Field `XTALCLKX2_CLK_GATE` writer - 2:0\\]
Writing 3'b111 will gate the XTALX2 clock.Writing 3'b000 will ungate the clock."]
pub type Xtalclkx2ClkGateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the XTALX2 clock.Writing 3'b000 will ungate the clock."]
    #[inline(always)]
    pub fn xtalclkx2_clk_gate(&self) -> Xtalclkx2ClkGateR {
        Xtalclkx2ClkGateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the XTALX2 clock.Writing 3'b000 will ungate the clock."]
    #[inline(always)]
    #[must_use]
    pub fn xtalclkx2_clk_gate(&mut self) -> Xtalclkx2ClkGateW<Xtalclkx2ClkGateSpec> {
        Xtalclkx2ClkGateW::new(self, 0)
    }
}
#[doc = "XTALCLKX2_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalclkx2_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalclkx2_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtalclkx2ClkGateSpec;
impl crate::RegisterSpec for Xtalclkx2ClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalclkx2_clk_gate::R`](R) reader structure"]
impl crate::Readable for Xtalclkx2ClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalclkx2_clk_gate::W`](W) writer structure"]
impl crate::Writable for Xtalclkx2ClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTALCLKX2_CLK_GATE to value 0"]
impl crate::Resettable for Xtalclkx2ClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
