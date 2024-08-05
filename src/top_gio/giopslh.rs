#[doc = "Register `GIOPSLH` reader"]
pub type R = crate::R<GiopslhSpec>;
#[doc = "Register `GIOPSLH` writer"]
pub type W = crate::W<GiopslhSpec>;
#[doc = "Field `GIOPSLH` reader - 7:0\\]
GIO pull select for port H"]
pub type GiopslhR = crate::FieldReader;
#[doc = "Field `GIOPSLH` writer - 7:0\\]
GIO pull select for port H"]
pub type GiopslhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU40` reader - 31:8\\]
Reserved"]
pub type Nu40R = crate::FieldReader<u32>;
#[doc = "Field `NU40` writer - 31:8\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port H"]
    #[inline(always)]
    pub fn giopslh(&self) -> GiopslhR {
        GiopslhR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu40(&self) -> Nu40R {
        Nu40R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port H"]
    #[inline(always)]
    #[must_use]
    pub fn giopslh(&mut self) -> GiopslhW<GiopslhSpec> {
        GiopslhW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<GiopslhSpec> {
        Nu40W::new(self, 8)
    }
}
#[doc = "GIO pul select for port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopslhSpec;
impl crate::RegisterSpec for GiopslhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopslh::R`](R) reader structure"]
impl crate::Readable for GiopslhSpec {}
#[doc = "`write(|w| ..)` method takes [`giopslh::W`](W) writer structure"]
impl crate::Writable for GiopslhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPSLH to value 0"]
impl crate::Resettable for GiopslhSpec {
    const RESET_VALUE: u32 = 0;
}
