#[doc = "Register `GIOPSLF` reader"]
pub type R = crate::R<GiopslfSpec>;
#[doc = "Register `GIOPSLF` writer"]
pub type W = crate::W<GiopslfSpec>;
#[doc = "Field `GIOPSLF` reader - 7:0\\]
GIO pull select for port F"]
pub type GiopslfR = crate::FieldReader;
#[doc = "Field `GIOPSLF` writer - 7:0\\]
GIO pull select for port F"]
pub type GiopslfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU40` reader - 31:8\\]
Reserved"]
pub type Nu40R = crate::FieldReader<u32>;
#[doc = "Field `NU40` writer - 31:8\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port F"]
    #[inline(always)]
    pub fn giopslf(&self) -> GiopslfR {
        GiopslfR::new((self.bits & 0xff) as u8)
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
GIO pull select for port F"]
    #[inline(always)]
    #[must_use]
    pub fn giopslf(&mut self) -> GiopslfW<GiopslfSpec> {
        GiopslfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<GiopslfSpec> {
        Nu40W::new(self, 8)
    }
}
#[doc = "GIO pul select for port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopslfSpec;
impl crate::RegisterSpec for GiopslfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopslf::R`](R) reader structure"]
impl crate::Readable for GiopslfSpec {}
#[doc = "`write(|w| ..)` method takes [`giopslf::W`](W) writer structure"]
impl crate::Writable for GiopslfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPSLF to value 0"]
impl crate::Resettable for GiopslfSpec {
    const RESET_VALUE: u32 = 0;
}
