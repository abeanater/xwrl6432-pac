#[doc = "Register `APLLDIV2_CLK_GATE` reader"]
pub type R = crate::R<Aplldiv2ClkGateSpec>;
#[doc = "Register `APLLDIV2_CLK_GATE` writer"]
pub type W = crate::W<Aplldiv2ClkGateSpec>;
#[doc = "Field `APLLDIV2_CLK_GATE` reader - 2:0\\]
Writing 3'b111 will gate the APLL/2clock to EDCC.Writing 3'b000 will ungate the clock."]
pub type Aplldiv2ClkGateR = crate::FieldReader;
#[doc = "Field `APLLDIV2_CLK_GATE` writer - 2:0\\]
Writing 3'b111 will gate the APLL/2clock to EDCC.Writing 3'b000 will ungate the clock."]
pub type Aplldiv2ClkGateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the APLL/2clock to EDCC.Writing 3'b000 will ungate the clock."]
    #[inline(always)]
    pub fn aplldiv2_clk_gate(&self) -> Aplldiv2ClkGateR {
        Aplldiv2ClkGateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Writing 3'b111 will gate the APLL/2clock to EDCC.Writing 3'b000 will ungate the clock."]
    #[inline(always)]
    #[must_use]
    pub fn aplldiv2_clk_gate(&mut self) -> Aplldiv2ClkGateW<Aplldiv2ClkGateSpec> {
        Aplldiv2ClkGateW::new(self, 0)
    }
}
#[doc = "APLLDIV2_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`aplldiv2_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aplldiv2_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aplldiv2ClkGateSpec;
impl crate::RegisterSpec for Aplldiv2ClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aplldiv2_clk_gate::R`](R) reader structure"]
impl crate::Readable for Aplldiv2ClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`aplldiv2_clk_gate::W`](W) writer structure"]
impl crate::Writable for Aplldiv2ClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APLLDIV2_CLK_GATE to value 0"]
impl crate::Resettable for Aplldiv2ClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
