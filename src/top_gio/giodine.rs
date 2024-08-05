#[doc = "Register `GIODINE` reader"]
pub type R = crate::R<GiodineSpec>;
#[doc = "Register `GIODINE` writer"]
pub type W = crate::W<GiodineSpec>;
#[doc = "Field `GIODINE` reader - 7:0\\]
GIO data input for pins in port E"]
pub type GiodineR = crate::FieldReader;
#[doc = "Field `GIODINE` writer - 7:0\\]
GIO data input for pins in port E"]
pub type GiodineW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU15` reader - 31:8\\]
Reserved"]
pub type Nu15R = crate::FieldReader<u32>;
#[doc = "Field `NU15` writer - 31:8\\]
Reserved"]
pub type Nu15W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port E"]
    #[inline(always)]
    pub fn giodine(&self) -> GiodineR {
        GiodineR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu15(&self) -> Nu15R {
        Nu15R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port E"]
    #[inline(always)]
    #[must_use]
    pub fn giodine(&mut self) -> GiodineW<GiodineSpec> {
        GiodineW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu15(&mut self) -> Nu15W<GiodineSpec> {
        Nu15W::new(self, 8)
    }
}
#[doc = "GIO data input for pins in port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giodine::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodine::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodineSpec;
impl crate::RegisterSpec for GiodineSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodine::R`](R) reader structure"]
impl crate::Readable for GiodineSpec {}
#[doc = "`write(|w| ..)` method takes [`giodine::W`](W) writer structure"]
impl crate::Writable for GiodineSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODINE to value 0"]
impl crate::Resettable for GiodineSpec {
    const RESET_VALUE: u32 = 0;
}
