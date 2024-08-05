#[doc = "Register `GIODOUTA` reader"]
pub type R = crate::R<GiodoutaSpec>;
#[doc = "Register `GIODOUTA` writer"]
pub type W = crate::W<GiodoutaSpec>;
#[doc = "Field `GIODOUTA` reader - 7:0\\]
GIO data output for pins in port A"]
pub type GiodoutaR = crate::FieldReader;
#[doc = "Field `GIODOUTA` writer - 7:0\\]
GIO data output for pins in port A"]
pub type GiodoutaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU17` reader - 31:8\\]
Reserved"]
pub type Nu17R = crate::FieldReader<u32>;
#[doc = "Field `NU17` writer - 31:8\\]
Reserved"]
pub type Nu17W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port A"]
    #[inline(always)]
    pub fn giodouta(&self) -> GiodoutaR {
        GiodoutaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu17(&self) -> Nu17R {
        Nu17R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port A"]
    #[inline(always)]
    #[must_use]
    pub fn giodouta(&mut self) -> GiodoutaW<GiodoutaSpec> {
        GiodoutaW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu17(&mut self) -> Nu17W<GiodoutaSpec> {
        Nu17W::new(self, 8)
    }
}
#[doc = "GIO data output for pins in port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giodouta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodouta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodoutaSpec;
impl crate::RegisterSpec for GiodoutaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodouta::R`](R) reader structure"]
impl crate::Readable for GiodoutaSpec {}
#[doc = "`write(|w| ..)` method takes [`giodouta::W`](W) writer structure"]
impl crate::Writable for GiodoutaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODOUTA to value 0"]
impl crate::Resettable for GiodoutaSpec {
    const RESET_VALUE: u32 = 0;
}
