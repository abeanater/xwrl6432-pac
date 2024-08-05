#[doc = "Register `GIOPULDISH` reader"]
pub type R = crate::R<GiopuldishSpec>;
#[doc = "Register `GIOPULDISH` writer"]
pub type W = crate::W<GiopuldishSpec>;
#[doc = "Field `GIOPULDISH` reader - 7:0\\]
GIO pull disable for port H"]
pub type GiopuldishR = crate::FieldReader;
#[doc = "Field `GIOPULDISH` writer - 7:0\\]
GIO pull disable for port H"]
pub type GiopuldishW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU40` reader - 31:8\\]
Reserved"]
pub type Nu40R = crate::FieldReader<u32>;
#[doc = "Field `NU40` writer - 31:8\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port H"]
    #[inline(always)]
    pub fn giopuldish(&self) -> GiopuldishR {
        GiopuldishR::new((self.bits & 0xff) as u8)
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
GIO pull disable for port H"]
    #[inline(always)]
    #[must_use]
    pub fn giopuldish(&mut self) -> GiopuldishW<GiopuldishSpec> {
        GiopuldishW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<GiopuldishSpec> {
        Nu40W::new(self, 8)
    }
}
#[doc = "GIO pul disable for port H\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldish::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldish::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopuldishSpec;
impl crate::RegisterSpec for GiopuldishSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopuldish::R`](R) reader structure"]
impl crate::Readable for GiopuldishSpec {}
#[doc = "`write(|w| ..)` method takes [`giopuldish::W`](W) writer structure"]
impl crate::Writable for GiopuldishSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPULDISH to value 0"]
impl crate::Resettable for GiopuldishSpec {
    const RESET_VALUE: u32 = 0;
}
