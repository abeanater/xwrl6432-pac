#[doc = "Register `GIOPULDISE` reader"]
pub type R = crate::R<GiopuldiseSpec>;
#[doc = "Register `GIOPULDISE` writer"]
pub type W = crate::W<GiopuldiseSpec>;
#[doc = "Field `GIOPULDISE` reader - 7:0\\]
GIO pull disable for port E"]
pub type GiopuldiseR = crate::FieldReader;
#[doc = "Field `GIOPULDISE` writer - 7:0\\]
GIO pull disable for port E"]
pub type GiopuldiseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU39` reader - 31:8\\]
Reserved"]
pub type Nu39R = crate::FieldReader<u32>;
#[doc = "Field `NU39` writer - 31:8\\]
Reserved"]
pub type Nu39W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port E"]
    #[inline(always)]
    pub fn giopuldise(&self) -> GiopuldiseR {
        GiopuldiseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu39(&self) -> Nu39R {
        Nu39R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port E"]
    #[inline(always)]
    #[must_use]
    pub fn giopuldise(&mut self) -> GiopuldiseW<GiopuldiseSpec> {
        GiopuldiseW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu39(&mut self) -> Nu39W<GiopuldiseSpec> {
        Nu39W::new(self, 8)
    }
}
#[doc = "GIO pul disable for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopuldiseSpec;
impl crate::RegisterSpec for GiopuldiseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopuldise::R`](R) reader structure"]
impl crate::Readable for GiopuldiseSpec {}
#[doc = "`write(|w| ..)` method takes [`giopuldise::W`](W) writer structure"]
impl crate::Writable for GiopuldiseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPULDISE to value 0"]
impl crate::Resettable for GiopuldiseSpec {
    const RESET_VALUE: u32 = 0;
}
