#[doc = "Register `CM4_FORCE_HCLK_GATE` reader"]
pub type R = crate::R<Cm4ForceHclkGateSpec>;
#[doc = "Register `CM4_FORCE_HCLK_GATE` writer"]
pub type W = crate::W<Cm4ForceHclkGateSpec>;
#[doc = "Field `enable` reader - 2:0\\]
3'b000 - CM4 HCLK is ungated 3'b111 - CM4 HCLK is gated"]
pub type EnableR = crate::FieldReader;
#[doc = "Field `enable` writer - 2:0\\]
3'b000 - CM4 HCLK is ungated 3'b111 - CM4 HCLK is gated"]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 - CM4 HCLK is ungated 3'b111 - CM4 HCLK is gated"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
3'b000 - CM4 HCLK is ungated 3'b111 - CM4 HCLK is gated"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Cm4ForceHclkGateSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "CM4_FORCE_HCLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_force_hclk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_force_hclk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4ForceHclkGateSpec;
impl crate::RegisterSpec for Cm4ForceHclkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_force_hclk_gate::R`](R) reader structure"]
impl crate::Readable for Cm4ForceHclkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`cm4_force_hclk_gate::W`](W) writer structure"]
impl crate::Writable for Cm4ForceHclkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_FORCE_HCLK_GATE to value 0"]
impl crate::Resettable for Cm4ForceHclkGateSpec {
    const RESET_VALUE: u32 = 0;
}
