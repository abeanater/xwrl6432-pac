#[doc = "Register `HWACCREG4` reader"]
pub type R = crate::R<Hwaccreg4Spec>;
#[doc = "Register `HWACCREG4` writer"]
pub type W = crate::W<Hwaccreg4Spec>;
#[doc = "Field `SPARE` reader - 31:0\\]
Spare register"]
pub type SpareR = crate::FieldReader<u32>;
#[doc = "Field `SPARE` writer - 31:0\\]
Spare register"]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Spare register"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Spare register"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<Hwaccreg4Spec> {
        SpareW::new(self, 0)
    }
}
#[doc = "HWACCREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg4Spec;
impl crate::RegisterSpec for Hwaccreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg4::R`](R) reader structure"]
impl crate::Readable for Hwaccreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg4::W`](W) writer structure"]
impl crate::Writable for Hwaccreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG4 to value 0"]
impl crate::Resettable for Hwaccreg4Spec {
    const RESET_VALUE: u32 = 0;
}
