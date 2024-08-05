#[doc = "Register `HWACCREG15` reader"]
pub type R = crate::R<Hwaccreg15Spec>;
#[doc = "Register `HWACCREG15` writer"]
pub type W = crate::W<Hwaccreg15Spec>;
#[doc = "Field `PARAMDONECLR` reader - 31:0\\]
Status bits in PARAMDONESTAT are not automatically cleared, but they can be individually cleared by writing to 32-bit register PARAMDONECLR."]
pub type ParamdoneclrR = crate::FieldReader<u32>;
#[doc = "Field `PARAMDONECLR` writer - 31:0\\]
Status bits in PARAMDONESTAT are not automatically cleared, but they can be individually cleared by writing to 32-bit register PARAMDONECLR."]
pub type ParamdoneclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Status bits in PARAMDONESTAT are not automatically cleared, but they can be individually cleared by writing to 32-bit register PARAMDONECLR."]
    #[inline(always)]
    pub fn paramdoneclr(&self) -> ParamdoneclrR {
        ParamdoneclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Status bits in PARAMDONESTAT are not automatically cleared, but they can be individually cleared by writing to 32-bit register PARAMDONECLR."]
    #[inline(always)]
    #[must_use]
    pub fn paramdoneclr(&mut self) -> ParamdoneclrW<Hwaccreg15Spec> {
        ParamdoneclrW::new(self, 0)
    }
}
#[doc = "HWACCREG15\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg15Spec;
impl crate::RegisterSpec for Hwaccreg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg15::R`](R) reader structure"]
impl crate::Readable for Hwaccreg15Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg15::W`](W) writer structure"]
impl crate::Writable for Hwaccreg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG15 to value 0"]
impl crate::Resettable for Hwaccreg15Spec {
    const RESET_VALUE: u32 = 0;
}
