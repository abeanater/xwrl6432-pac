#[doc = "Register `GIOCLRD` reader"]
pub type R = crate::R<GioclrdSpec>;
#[doc = "Register `GIOCLRD` writer"]
pub type W = crate::W<GioclrdSpec>;
#[doc = "Field `GIODCLRD` reader - 7:0\\]
GIO data clear for port D"]
pub type GiodclrdR = crate::FieldReader;
#[doc = "Field `GIODCLRD` writer - 7:0\\]
GIO data clear for port D"]
pub type GiodclrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU32` reader - 31:8\\]
Reserved"]
pub type Nu32R = crate::FieldReader<u32>;
#[doc = "Field `NU32` writer - 31:8\\]
Reserved"]
pub type Nu32W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port D"]
    #[inline(always)]
    pub fn giodclrd(&self) -> GiodclrdR {
        GiodclrdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu32(&self) -> Nu32R {
        Nu32R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port D"]
    #[inline(always)]
    #[must_use]
    pub fn giodclrd(&mut self) -> GiodclrdW<GioclrdSpec> {
        GiodclrdW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu32(&mut self) -> Nu32W<GioclrdSpec> {
        Nu32W::new(self, 8)
    }
}
#[doc = "GIO data clear for port D\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioclrdSpec;
impl crate::RegisterSpec for GioclrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioclrd::R`](R) reader structure"]
impl crate::Readable for GioclrdSpec {}
#[doc = "`write(|w| ..)` method takes [`gioclrd::W`](W) writer structure"]
impl crate::Writable for GioclrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOCLRD to value 0"]
impl crate::Resettable for GioclrdSpec {
    const RESET_VALUE: u32 = 0;
}
