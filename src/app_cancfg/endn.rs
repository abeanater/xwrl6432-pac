#[doc = "Register `ENDN` reader"]
pub type R = crate::R<EndnSpec>;
#[doc = "Register `ENDN` writer"]
pub type W = crate::W<EndnSpec>;
#[doc = "Field `ETV` reader - 31:0\\]
Endianess test value"]
pub type EtvR = crate::FieldReader<u32>;
#[doc = "Field `ETV` writer - 31:0\\]
Endianess test value"]
pub type EtvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Endianess test value"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Endianess test value"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> EtvW<EndnSpec> {
        EtvW::new(self, 0)
    }
}
#[doc = "ENDN\n\nYou can [`read`](crate::Reg::read) this register and get [`endn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndnSpec;
impl crate::RegisterSpec for EndnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endn::R`](R) reader structure"]
impl crate::Readable for EndnSpec {}
#[doc = "`write(|w| ..)` method takes [`endn::W`](W) writer structure"]
impl crate::Writable for EndnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDN to value 0"]
impl crate::Resettable for EndnSpec {
    const RESET_VALUE: u32 = 0;
}
