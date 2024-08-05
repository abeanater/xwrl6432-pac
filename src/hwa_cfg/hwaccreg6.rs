#[doc = "Register `HWACCREG6` reader"]
pub type R = crate::R<Hwaccreg6Spec>;
#[doc = "Register `HWACCREG6` writer"]
pub type W = crate::W<Hwaccreg6Spec>;
#[doc = "Field `BPMPATTERNLSB` reader - 31:0\\]
BPM pattern LSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmpatternlsbR = crate::FieldReader<u32>;
#[doc = "Field `BPMPATTERNLSB` writer - 31:0\\]
BPM pattern LSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
pub type BpmpatternlsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern LSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    pub fn bpmpatternlsb(&self) -> BpmpatternlsbR {
        BpmpatternlsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
BPM pattern LSB: Specifies the BPM pattern to be used to multiply the input samples if BPM removal is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bpmpatternlsb(&mut self) -> BpmpatternlsbW<Hwaccreg6Spec> {
        BpmpatternlsbW::new(self, 0)
    }
}
#[doc = "HWACCREG6\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg6Spec;
impl crate::RegisterSpec for Hwaccreg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg6::R`](R) reader structure"]
impl crate::Readable for Hwaccreg6Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg6::W`](W) writer structure"]
impl crate::Writable for Hwaccreg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG6 to value 0"]
impl crate::Resettable for Hwaccreg6Spec {
    const RESET_VALUE: u32 = 0;
}
