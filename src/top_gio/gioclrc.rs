#[doc = "Register `GIOCLRC` reader"]
pub type R = crate::R<GioclrcSpec>;
#[doc = "Register `GIOCLRC` writer"]
pub type W = crate::W<GioclrcSpec>;
#[doc = "Field `GIODCLRC` reader - 7:0\\]
GIO data clear for port C"]
pub type GiodclrcR = crate::FieldReader;
#[doc = "Field `GIODCLRC` writer - 7:0\\]
GIO data clear for port C"]
pub type GiodclrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU31` reader - 31:8\\]
Reserved"]
pub type Nu31R = crate::FieldReader<u32>;
#[doc = "Field `NU31` writer - 31:8\\]
Reserved"]
pub type Nu31W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port C"]
    #[inline(always)]
    pub fn giodclrc(&self) -> GiodclrcR {
        GiodclrcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu31(&self) -> Nu31R {
        Nu31R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port C"]
    #[inline(always)]
    #[must_use]
    pub fn giodclrc(&mut self) -> GiodclrcW<GioclrcSpec> {
        GiodclrcW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu31(&mut self) -> Nu31W<GioclrcSpec> {
        Nu31W::new(self, 8)
    }
}
#[doc = "GIO data clear for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioclrcSpec;
impl crate::RegisterSpec for GioclrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioclrc::R`](R) reader structure"]
impl crate::Readable for GioclrcSpec {}
#[doc = "`write(|w| ..)` method takes [`gioclrc::W`](W) writer structure"]
impl crate::Writable for GioclrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOCLRC to value 0"]
impl crate::Resettable for GioclrcSpec {
    const RESET_VALUE: u32 = 0;
}
