#[doc = "Register `GIODIRA` reader"]
pub type R = crate::R<GiodiraSpec>;
#[doc = "Register `GIODIRA` writer"]
pub type W = crate::W<GiodiraSpec>;
#[doc = "Field `GIODIRA` reader - 7:0\\]
GIO data direction of pins in Port A"]
pub type GiodiraR = crate::FieldReader;
#[doc = "Field `GIODIRA` writer - 7:0\\]
GIO data direction of pins in Port A"]
pub type GiodiraW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU5` reader - 31:8\\]
Reserved"]
pub type Nu5R = crate::FieldReader<u32>;
#[doc = "Field `NU5` writer - 31:8\\]
Reserved"]
pub type Nu5W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port A"]
    #[inline(always)]
    pub fn giodira(&self) -> GiodiraR {
        GiodiraR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port A"]
    #[inline(always)]
    #[must_use]
    pub fn giodira(&mut self) -> GiodiraW<GiodiraSpec> {
        GiodiraW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<GiodiraSpec> {
        Nu5W::new(self, 8)
    }
}
#[doc = "GIO data direction of pins in Port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giodira::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodira::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodiraSpec;
impl crate::RegisterSpec for GiodiraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodira::R`](R) reader structure"]
impl crate::Readable for GiodiraSpec {}
#[doc = "`write(|w| ..)` method takes [`giodira::W`](W) writer structure"]
impl crate::Writable for GiodiraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIRA to value 0"]
impl crate::Resettable for GiodiraSpec {
    const RESET_VALUE: u32 = 0;
}
