#[doc = "Register `HWACCREG14` reader"]
pub type R = crate::R<Hwaccreg14Spec>;
#[doc = "Register `HWACCREG14` writer"]
pub type W = crate::W<Hwaccreg14Spec>;
#[doc = "Field `PARAMDONESTAT` reader - 31:0\\]
Parameter-set done status: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 32-bit status register indicate which of the 32 parameter-sets have completed."]
pub type ParamdonestatR = crate::FieldReader<u32>;
#[doc = "Field `PARAMDONESTAT` writer - 31:0\\]
Parameter-set done status: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 32-bit status register indicate which of the 32 parameter-sets have completed."]
pub type ParamdonestatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Parameter-set done status: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 32-bit status register indicate which of the 32 parameter-sets have completed."]
    #[inline(always)]
    pub fn paramdonestat(&self) -> ParamdonestatR {
        ParamdonestatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Parameter-set done status: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 32-bit status register indicate which of the 32 parameter-sets have completed."]
    #[inline(always)]
    #[must_use]
    pub fn paramdonestat(&mut self) -> ParamdonestatW<Hwaccreg14Spec> {
        ParamdonestatW::new(self, 0)
    }
}
#[doc = "HWACCREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg14Spec;
impl crate::RegisterSpec for Hwaccreg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg14::R`](R) reader structure"]
impl crate::Readable for Hwaccreg14Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg14::W`](W) writer structure"]
impl crate::Writable for Hwaccreg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG14 to value 0"]
impl crate::Resettable for Hwaccreg14Spec {
    const RESET_VALUE: u32 = 0;
}
