#[doc = "Register `RTICOMP1CLR` reader"]
pub type R = crate::R<Rticomp1clrSpec>;
#[doc = "Register `RTICOMP1CLR` writer"]
pub type W = crate::W<Rticomp1clrSpec>;
#[doc = "Field `COMP1CLR` reader - 31:0\\]
COMP1CLR: Compare 1 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 1 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp1clrR = crate::FieldReader<u32>;
#[doc = "Field `COMP1CLR` writer - 31:0\\]
COMP1CLR: Compare 1 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 1 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp1clrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
COMP1CLR: Compare 1 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 1 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    pub fn comp1clr(&self) -> Comp1clrR {
        Comp1clrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
COMP1CLR: Compare 1 Clear. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, the Compare 1 interrupt or DMA request line is cleared. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp1clr(&mut self) -> Comp1clrW<Rticomp1clrSpec> {
        Comp1clrW::new(self, 0)
    }
}
#[doc = "Compare 1 Clear compare value to be compared with the counter to clear the compare1 interrupt line\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp1clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp1clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticomp1clrSpec;
impl crate::RegisterSpec for Rticomp1clrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticomp1clr::R`](R) reader structure"]
impl crate::Readable for Rticomp1clrSpec {}
#[doc = "`write(|w| ..)` method takes [`rticomp1clr::W`](W) writer structure"]
impl crate::Writable for Rticomp1clrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICOMP1CLR to value 0"]
impl crate::Resettable for Rticomp1clrSpec {
    const RESET_VALUE: u32 = 0;
}
