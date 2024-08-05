#[doc = "Register `GIOCLRF` reader"]
pub type R = crate::R<GioclrfSpec>;
#[doc = "Register `GIOCLRF` writer"]
pub type W = crate::W<GioclrfSpec>;
#[doc = "Field `GIODCLRF` reader - 7:0\\]
GIO data clear for port F"]
pub type GiodclrfR = crate::FieldReader;
#[doc = "Field `GIODCLRF` writer - 7:0\\]
GIO data clear for port F"]
pub type GiodclrfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU34` reader - 31:8\\]
Reserved"]
pub type Nu34R = crate::FieldReader<u32>;
#[doc = "Field `NU34` writer - 31:8\\]
Reserved"]
pub type Nu34W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port F"]
    #[inline(always)]
    pub fn giodclrf(&self) -> GiodclrfR {
        GiodclrfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu34(&self) -> Nu34R {
        Nu34R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port F"]
    #[inline(always)]
    #[must_use]
    pub fn giodclrf(&mut self) -> GiodclrfW<GioclrfSpec> {
        GiodclrfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu34(&mut self) -> Nu34W<GioclrfSpec> {
        Nu34W::new(self, 8)
    }
}
#[doc = "GIO data clear for Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioclrfSpec;
impl crate::RegisterSpec for GioclrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioclrf::R`](R) reader structure"]
impl crate::Readable for GioclrfSpec {}
#[doc = "`write(|w| ..)` method takes [`gioclrf::W`](W) writer structure"]
impl crate::Writable for GioclrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOCLRF to value 0"]
impl crate::Resettable for GioclrfSpec {
    const RESET_VALUE: u32 = 0;
}
