#[doc = "Register `GIODIRD` reader"]
pub type R = crate::R<GiodirdSpec>;
#[doc = "Register `GIODIRD` writer"]
pub type W = crate::W<GiodirdSpec>;
#[doc = "Field `GIODIRD` reader - 7:0\\]
GIO data direction of pins in Port D"]
pub type GiodirdR = crate::FieldReader;
#[doc = "Field `GIODIRD` writer - 7:0\\]
GIO data direction of pins in Port D"]
pub type GiodirdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU8` reader - 31:8\\]
Reserved"]
pub type Nu8R = crate::FieldReader<u32>;
#[doc = "Field `NU8` writer - 31:8\\]
Reserved"]
pub type Nu8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port D"]
    #[inline(always)]
    pub fn giodird(&self) -> GiodirdR {
        GiodirdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu8(&self) -> Nu8R {
        Nu8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port D"]
    #[inline(always)]
    #[must_use]
    pub fn giodird(&mut self) -> GiodirdW<GiodirdSpec> {
        GiodirdW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu8(&mut self) -> Nu8W<GiodirdSpec> {
        Nu8W::new(self, 8)
    }
}
#[doc = "GIO data direction of pins in Port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giodird::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodird::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodirdSpec;
impl crate::RegisterSpec for GiodirdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodird::R`](R) reader structure"]
impl crate::Readable for GiodirdSpec {}
#[doc = "`write(|w| ..)` method takes [`giodird::W`](W) writer structure"]
impl crate::Writable for GiodirdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIRD to value 0"]
impl crate::Resettable for GiodirdSpec {
    const RESET_VALUE: u32 = 0;
}
