#[doc = "Register `GIODOUTH` reader"]
pub type R = crate::R<GiodouthSpec>;
#[doc = "Register `GIODOUTH` writer"]
pub type W = crate::W<GiodouthSpec>;
#[doc = "Field `GIODOUTH` reader - 7:0\\]
GIO data output for pins in port H"]
pub type GiodouthR = crate::FieldReader;
#[doc = "Field `GIODOUTH` writer - 7:0\\]
GIO data output for pins in port H"]
pub type GiodouthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU22` reader - 31:8\\]
Reserved"]
pub type Nu22R = crate::FieldReader<u32>;
#[doc = "Field `NU22` writer - 31:8\\]
Reserved"]
pub type Nu22W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port H"]
    #[inline(always)]
    pub fn giodouth(&self) -> GiodouthR {
        GiodouthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu22(&self) -> Nu22R {
        Nu22R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port H"]
    #[inline(always)]
    #[must_use]
    pub fn giodouth(&mut self) -> GiodouthW<GiodouthSpec> {
        GiodouthW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu22(&mut self) -> Nu22W<GiodouthSpec> {
        Nu22W::new(self, 8)
    }
}
#[doc = "GIO data output for pins in Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giodouth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodouth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodouthSpec;
impl crate::RegisterSpec for GiodouthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodouth::R`](R) reader structure"]
impl crate::Readable for GiodouthSpec {}
#[doc = "`write(|w| ..)` method takes [`giodouth::W`](W) writer structure"]
impl crate::Writable for GiodouthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODOUTH to value 0"]
impl crate::Resettable for GiodouthSpec {
    const RESET_VALUE: u32 = 0;
}
