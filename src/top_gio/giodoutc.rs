#[doc = "Register `GIODOUTC` reader"]
pub type R = crate::R<GiodoutcSpec>;
#[doc = "Register `GIODOUTC` writer"]
pub type W = crate::W<GiodoutcSpec>;
#[doc = "Field `GIODOUTC` reader - 7:0\\]
GIO data output for pins in port C"]
pub type GiodoutcR = crate::FieldReader;
#[doc = "Field `GIODOUTC` writer - 7:0\\]
GIO data output for pins in port C"]
pub type GiodoutcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU19` reader - 31:8\\]
Reserved"]
pub type Nu19R = crate::FieldReader<u32>;
#[doc = "Field `NU19` writer - 31:8\\]
Reserved"]
pub type Nu19W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port C"]
    #[inline(always)]
    pub fn giodoutc(&self) -> GiodoutcR {
        GiodoutcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu19(&self) -> Nu19R {
        Nu19R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port C"]
    #[inline(always)]
    #[must_use]
    pub fn giodoutc(&mut self) -> GiodoutcW<GiodoutcSpec> {
        GiodoutcW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu19(&mut self) -> Nu19W<GiodoutcSpec> {
        Nu19W::new(self, 8)
    }
}
#[doc = "GIO data output for pins in port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodoutcSpec;
impl crate::RegisterSpec for GiodoutcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodoutc::R`](R) reader structure"]
impl crate::Readable for GiodoutcSpec {}
#[doc = "`write(|w| ..)` method takes [`giodoutc::W`](W) writer structure"]
impl crate::Writable for GiodoutcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODOUTC to value 0"]
impl crate::Resettable for GiodoutcSpec {
    const RESET_VALUE: u32 = 0;
}
