#[doc = "Register `GIOCLRB` reader"]
pub type R = crate::R<GioclrbSpec>;
#[doc = "Register `GIOCLRB` writer"]
pub type W = crate::W<GioclrbSpec>;
#[doc = "Field `GIODCLRB` reader - 7:0\\]
GIO data clear for port B"]
pub type GiodclrbR = crate::FieldReader;
#[doc = "Field `GIODCLRB` writer - 7:0\\]
GIO data clear for port B"]
pub type GiodclrbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU30` reader - 31:8\\]
Reserved"]
pub type Nu30R = crate::FieldReader<u32>;
#[doc = "Field `NU30` writer - 31:8\\]
Reserved"]
pub type Nu30W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port B"]
    #[inline(always)]
    pub fn giodclrb(&self) -> GiodclrbR {
        GiodclrbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu30(&self) -> Nu30R {
        Nu30R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port B"]
    #[inline(always)]
    #[must_use]
    pub fn giodclrb(&mut self) -> GiodclrbW<GioclrbSpec> {
        GiodclrbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu30(&mut self) -> Nu30W<GioclrbSpec> {
        Nu30W::new(self, 8)
    }
}
#[doc = "GIO data clear for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioclrbSpec;
impl crate::RegisterSpec for GioclrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioclrb::R`](R) reader structure"]
impl crate::Readable for GioclrbSpec {}
#[doc = "`write(|w| ..)` method takes [`gioclrb::W`](W) writer structure"]
impl crate::Writable for GioclrbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOCLRB to value 0"]
impl crate::Resettable for GioclrbSpec {
    const RESET_VALUE: u32 = 0;
}
