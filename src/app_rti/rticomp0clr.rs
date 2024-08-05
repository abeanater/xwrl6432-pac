#[doc = "Register `RTICOMP0CLR` reader"]
pub type R = crate::R<Rticomp0clrSpec>;
#[doc = "Register `RTICOMP0CLR` writer"]
pub type W = crate::W<Rticomp0clrSpec>;
#[doc = "Field `COMP0CLR` reader - 31:0\\]
COMP0CLR: Compare 0 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 0 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp0clrR = crate::FieldReader<u32>;
#[doc = "Field `COMP0CLR` writer - 31:0\\]
COMP0CLR: Compare 0 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 0 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp0clrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
COMP0CLR: Compare 0 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 0 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    pub fn comp0clr(&self) -> Comp0clrR {
        Comp0clrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
COMP0CLR: Compare 0 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the compare 0 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp0clr(&mut self) -> Comp0clrW<Rticomp0clrSpec> {
        Comp0clrW::new(self, 0)
    }
}
#[doc = "Compare 0 Clear compare value to be compared with the counter to clear the compare0 interrupt line\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp0clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp0clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticomp0clrSpec;
impl crate::RegisterSpec for Rticomp0clrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticomp0clr::R`](R) reader structure"]
impl crate::Readable for Rticomp0clrSpec {}
#[doc = "`write(|w| ..)` method takes [`rticomp0clr::W`](W) writer structure"]
impl crate::Writable for Rticomp0clrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICOMP0CLR to value 0"]
impl crate::Resettable for Rticomp0clrSpec {
    const RESET_VALUE: u32 = 0;
}
