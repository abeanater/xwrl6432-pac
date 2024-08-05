#[doc = "Register `GIODOUTD` reader"]
pub type R = crate::R<GiodoutdSpec>;
#[doc = "Register `GIODOUTD` writer"]
pub type W = crate::W<GiodoutdSpec>;
#[doc = "Field `GIODOUTD` reader - 7:0\\]
GIO data output for pins in port D"]
pub type GiodoutdR = crate::FieldReader;
#[doc = "Field `GIODOUTD` writer - 7:0\\]
GIO data output for pins in port D"]
pub type GiodoutdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU20` reader - 31:8\\]
Reserved"]
pub type Nu20R = crate::FieldReader<u32>;
#[doc = "Field `NU20` writer - 31:8\\]
Reserved"]
pub type Nu20W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port D"]
    #[inline(always)]
    pub fn giodoutd(&self) -> GiodoutdR {
        GiodoutdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu20(&self) -> Nu20R {
        Nu20R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port D"]
    #[inline(always)]
    #[must_use]
    pub fn giodoutd(&mut self) -> GiodoutdW<GiodoutdSpec> {
        GiodoutdW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu20(&mut self) -> Nu20W<GiodoutdSpec> {
        Nu20W::new(self, 8)
    }
}
#[doc = "GIO data output for pins in port D\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoutd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoutd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodoutdSpec;
impl crate::RegisterSpec for GiodoutdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodoutd::R`](R) reader structure"]
impl crate::Readable for GiodoutdSpec {}
#[doc = "`write(|w| ..)` method takes [`giodoutd::W`](W) writer structure"]
impl crate::Writable for GiodoutdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODOUTD to value 0"]
impl crate::Resettable for GiodoutdSpec {
    const RESET_VALUE: u32 = 0;
}
