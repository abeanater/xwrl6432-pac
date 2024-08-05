#[doc = "Register `GIOPULDISA` reader"]
pub type R = crate::R<GiopuldisaSpec>;
#[doc = "Register `GIOPULDISA` writer"]
pub type W = crate::W<GiopuldisaSpec>;
#[doc = "Field `GIOPULDISA` reader - 7:0\\]
GIO pull disable for port A"]
pub type GiopuldisaR = crate::FieldReader;
#[doc = "Field `GIOPULDISA` writer - 7:0\\]
GIO pull disable for port A"]
pub type GiopuldisaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - 31:8\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:8\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port A"]
    #[inline(always)]
    pub fn giopuldisa(&self) -> GiopuldisaR {
        GiopuldisaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port A"]
    #[inline(always)]
    #[must_use]
    pub fn giopuldisa(&mut self) -> GiopuldisaW<GiopuldisaSpec> {
        GiopuldisaW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<GiopuldisaSpec> {
        NuW::new(self, 8)
    }
}
#[doc = "GIO pul disable for port A\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopuldisaSpec;
impl crate::RegisterSpec for GiopuldisaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopuldisa::R`](R) reader structure"]
impl crate::Readable for GiopuldisaSpec {}
#[doc = "`write(|w| ..)` method takes [`giopuldisa::W`](W) writer structure"]
impl crate::Writable for GiopuldisaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPULDISA to value 0"]
impl crate::Resettable for GiopuldisaSpec {
    const RESET_VALUE: u32 = 0;
}
