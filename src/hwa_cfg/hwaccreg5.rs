#[doc = "Register `HWACCREG5` reader"]
pub type R = crate::R<Hwaccreg5Spec>;
#[doc = "Register `HWACCREG5` writer"]
pub type W = crate::W<Hwaccreg5Spec>;
#[doc = "Field `BPMPATTERNMSB` reader - 31:0\\]
BPM pattern MSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmpatternmsbR = crate::FieldReader<u32>;
#[doc = "Field `BPMPATTERNMSB` writer - 31:0\\]
BPM pattern MSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmpatternmsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern MSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    pub fn bpmpatternmsb(&self) -> BpmpatternmsbR {
        BpmpatternmsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern MSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bpmpatternmsb(&mut self) -> BpmpatternmsbW<Hwaccreg5Spec> {
        BpmpatternmsbW::new(self, 0)
    }
}
#[doc = "HWACCREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg5Spec;
impl crate::RegisterSpec for Hwaccreg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg5::R`](R) reader structure"]
impl crate::Readable for Hwaccreg5Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg5::W`](W) writer structure"]
impl crate::Writable for Hwaccreg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG5 to value 0"]
impl crate::Resettable for Hwaccreg5Spec {
    const RESET_VALUE: u32 = 0;
}
