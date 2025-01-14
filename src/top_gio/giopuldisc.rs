#[doc = "Register `GIOPULDISC` reader"]
pub type R = crate::R<GiopuldiscSpec>;
#[doc = "Register `GIOPULDISC` writer"]
pub type W = crate::W<GiopuldiscSpec>;
#[doc = "Field `GIOPULDISC` reader - 7:0\\]
GIO pull disable for port C"]
pub type GiopuldiscR = crate::FieldReader;
#[doc = "Field `GIOPULDISC` writer - 7:0\\]
GIO pull disable for port C"]
pub type GiopuldiscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU37` reader - 31:8\\]
Reserved"]
pub type Nu37R = crate::FieldReader<u32>;
#[doc = "Field `NU37` writer - 31:8\\]
Reserved"]
pub type Nu37W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port C"]
    #[inline(always)]
    pub fn giopuldisc(&self) -> GiopuldiscR {
        GiopuldiscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu37(&self) -> Nu37R {
        Nu37R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port C"]
    #[inline(always)]
    #[must_use]
    pub fn giopuldisc(&mut self) -> GiopuldiscW<GiopuldiscSpec> {
        GiopuldiscW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu37(&mut self) -> Nu37W<GiopuldiscSpec> {
        Nu37W::new(self, 8)
    }
}
#[doc = "GIO pul disable for port C\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopuldiscSpec;
impl crate::RegisterSpec for GiopuldiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopuldisc::R`](R) reader structure"]
impl crate::Readable for GiopuldiscSpec {}
#[doc = "`write(|w| ..)` method takes [`giopuldisc::W`](W) writer structure"]
impl crate::Writable for GiopuldiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPULDISC to value 0"]
impl crate::Resettable for GiopuldiscSpec {
    const RESET_VALUE: u32 = 0;
}
