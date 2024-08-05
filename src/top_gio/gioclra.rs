#[doc = "Register `GIOCLRA` reader"]
pub type R = crate::R<GioclraSpec>;
#[doc = "Register `GIOCLRA` writer"]
pub type W = crate::W<GioclraSpec>;
#[doc = "Field `GIODCLRA` reader - 7:0\\]
GIO data clear for port A"]
pub type GiodclraR = crate::FieldReader;
#[doc = "Field `GIODCLRA` writer - 7:0\\]
GIO data clear for port A"]
pub type GiodclraW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU29` reader - 31:8\\]
Reserved"]
pub type Nu29R = crate::FieldReader<u32>;
#[doc = "Field `NU29` writer - 31:8\\]
Reserved"]
pub type Nu29W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port A"]
    #[inline(always)]
    pub fn giodclra(&self) -> GiodclraR {
        GiodclraR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu29(&self) -> Nu29R {
        Nu29R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port A"]
    #[inline(always)]
    #[must_use]
    pub fn giodclra(&mut self) -> GiodclraW<GioclraSpec> {
        GiodclraW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu29(&mut self) -> Nu29W<GioclraSpec> {
        Nu29W::new(self, 8)
    }
}
#[doc = "GIO data clear for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioclraSpec;
impl crate::RegisterSpec for GioclraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioclra::R`](R) reader structure"]
impl crate::Readable for GioclraSpec {}
#[doc = "`write(|w| ..)` method takes [`gioclra::W`](W) writer structure"]
impl crate::Writable for GioclraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOCLRA to value 0"]
impl crate::Resettable for GioclraSpec {
    const RESET_VALUE: u32 = 0;
}
