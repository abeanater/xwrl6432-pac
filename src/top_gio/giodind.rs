#[doc = "Register `GIODIND` reader"]
pub type R = crate::R<GiodindSpec>;
#[doc = "Register `GIODIND` writer"]
pub type W = crate::W<GiodindSpec>;
#[doc = "Field `GIODIND` reader - 7:0\\]
GIO data input for pins in port D"]
pub type GiodindR = crate::FieldReader;
#[doc = "Field `GIODIND` writer - 7:0\\]
GIO data input for pins in port D"]
pub type GiodindW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU14` reader - 31:8\\]
Reserved"]
pub type Nu14R = crate::FieldReader<u32>;
#[doc = "Field `NU14` writer - 31:8\\]
Reserved"]
pub type Nu14W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port D"]
    #[inline(always)]
    pub fn giodind(&self) -> GiodindR {
        GiodindR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu14(&self) -> Nu14R {
        Nu14R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data input for pins in port D"]
    #[inline(always)]
    #[must_use]
    pub fn giodind(&mut self) -> GiodindW<GiodindSpec> {
        GiodindW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu14(&mut self) -> Nu14W<GiodindSpec> {
        Nu14W::new(self, 8)
    }
}
#[doc = "GIO data input for pins in port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giodind::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodind::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodindSpec;
impl crate::RegisterSpec for GiodindSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodind::R`](R) reader structure"]
impl crate::Readable for GiodindSpec {}
#[doc = "`write(|w| ..)` method takes [`giodind::W`](W) writer structure"]
impl crate::Writable for GiodindSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIND to value 0"]
impl crate::Resettable for GiodindSpec {
    const RESET_VALUE: u32 = 0;
}
