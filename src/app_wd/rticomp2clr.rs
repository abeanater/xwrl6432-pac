#[doc = "Register `RTICOMP2CLR` reader"]
pub type R = crate::R<Rticomp2clrSpec>;
#[doc = "Register `RTICOMP2CLR` writer"]
pub type W = crate::W<Rticomp2clrSpec>;
#[doc = "Field `COMP2CLR` reader - 31:0\\]
COMP2CLR: Compare 2 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 2 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp2clrR = crate::FieldReader<u32>;
#[doc = "Field `COMP2CLR` writer - 31:0\\]
COMP2CLR: Compare 2 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 2 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp2clrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
COMP2CLR: Compare 2 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 2 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    pub fn comp2clr(&self) -> Comp2clrR {
        Comp2clrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
COMP2CLR: Compare 2 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 2 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp2clr(&mut self) -> Comp2clrW<Rticomp2clrSpec> {
        Comp2clrW::new(self, 0)
    }
}
#[doc = "Compare 2 Clear compare value to be compared with the counter to clear the compare2 interrupt line\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp2clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp2clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticomp2clrSpec;
impl crate::RegisterSpec for Rticomp2clrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticomp2clr::R`](R) reader structure"]
impl crate::Readable for Rticomp2clrSpec {}
#[doc = "`write(|w| ..)` method takes [`rticomp2clr::W`](W) writer structure"]
impl crate::Writable for Rticomp2clrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICOMP2CLR to value 0"]
impl crate::Resettable for Rticomp2clrSpec {
    const RESET_VALUE: u32 = 0;
}
