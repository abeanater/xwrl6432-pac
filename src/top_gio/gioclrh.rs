#[doc = "Register `GIOCLRH` reader"]
pub type R = crate::R<GioclrhSpec>;
#[doc = "Register `GIOCLRH` writer"]
pub type W = crate::W<GioclrhSpec>;
#[doc = "Field `GIODCLRH` reader - 7:0\\]
GIO data clear for port H"]
pub type GiodclrhR = crate::FieldReader;
#[doc = "Field `GIODCLRH` writer - 7:0\\]
GIO data clear for port H"]
pub type GiodclrhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU34` reader - 31:8\\]
Reserved"]
pub type Nu34R = crate::FieldReader<u32>;
#[doc = "Field `NU34` writer - 31:8\\]
Reserved"]
pub type Nu34W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port H"]
    #[inline(always)]
    pub fn giodclrh(&self) -> GiodclrhR {
        GiodclrhR::new((self.bits & 0xff) as u8)
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
GIO data clear for port H"]
    #[inline(always)]
    #[must_use]
    pub fn giodclrh(&mut self) -> GiodclrhW<GioclrhSpec> {
        GiodclrhW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu34(&mut self) -> Nu34W<GioclrhSpec> {
        Nu34W::new(self, 8)
    }
}
#[doc = "GIO data clear for Port H\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioclrhSpec;
impl crate::RegisterSpec for GioclrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioclrh::R`](R) reader structure"]
impl crate::Readable for GioclrhSpec {}
#[doc = "`write(|w| ..)` method takes [`gioclrh::W`](W) writer structure"]
impl crate::Writable for GioclrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOCLRH to value 0"]
impl crate::Resettable for GioclrhSpec {
    const RESET_VALUE: u32 = 0;
}
