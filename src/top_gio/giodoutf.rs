#[doc = "Register `GIODOUTF` reader"]
pub type R = crate::R<GiodoutfSpec>;
#[doc = "Register `GIODOUTF` writer"]
pub type W = crate::W<GiodoutfSpec>;
#[doc = "Field `GIODOUTF` reader - 7:0\\]
GIO data output for pins in port F"]
pub type GiodoutfR = crate::FieldReader;
#[doc = "Field `GIODOUTF` writer - 7:0\\]
GIO data output for pins in port F"]
pub type GiodoutfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU22` reader - 31:8\\]
Reserved"]
pub type Nu22R = crate::FieldReader<u32>;
#[doc = "Field `NU22` writer - 31:8\\]
Reserved"]
pub type Nu22W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port F"]
    #[inline(always)]
    pub fn giodoutf(&self) -> GiodoutfR {
        GiodoutfR::new((self.bits & 0xff) as u8)
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
GIO data output for pins in port F"]
    #[inline(always)]
    #[must_use]
    pub fn giodoutf(&mut self) -> GiodoutfW<GiodoutfSpec> {
        GiodoutfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu22(&mut self) -> Nu22W<GiodoutfSpec> {
        Nu22W::new(self, 8)
    }
}
#[doc = "GIO data output for pins in Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodoutfSpec;
impl crate::RegisterSpec for GiodoutfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodoutf::R`](R) reader structure"]
impl crate::Readable for GiodoutfSpec {}
#[doc = "`write(|w| ..)` method takes [`giodoutf::W`](W) writer structure"]
impl crate::Writable for GiodoutfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODOUTF to value 0"]
impl crate::Resettable for GiodoutfSpec {
    const RESET_VALUE: u32 = 0;
}
