#[doc = "Register `GIOPULDISB` reader"]
pub type R = crate::R<GiopuldisbSpec>;
#[doc = "Register `GIOPULDISB` writer"]
pub type W = crate::W<GiopuldisbSpec>;
#[doc = "Field `GIOPULDISB` reader - 7:0\\]
GIO pull disable for port B"]
pub type GiopuldisbR = crate::FieldReader;
#[doc = "Field `GIOPULDISB` writer - 7:0\\]
GIO pull disable for port B"]
pub type GiopuldisbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU36` reader - 31:8\\]
Reserved"]
pub type Nu36R = crate::FieldReader<u32>;
#[doc = "Field `NU36` writer - 31:8\\]
Reserved"]
pub type Nu36W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port B"]
    #[inline(always)]
    pub fn giopuldisb(&self) -> GiopuldisbR {
        GiopuldisbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu36(&self) -> Nu36R {
        Nu36R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port B"]
    #[inline(always)]
    #[must_use]
    pub fn giopuldisb(&mut self) -> GiopuldisbW<GiopuldisbSpec> {
        GiopuldisbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu36(&mut self) -> Nu36W<GiopuldisbSpec> {
        Nu36W::new(self, 8)
    }
}
#[doc = "GIO pul disable for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopuldisbSpec;
impl crate::RegisterSpec for GiopuldisbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopuldisb::R`](R) reader structure"]
impl crate::Readable for GiopuldisbSpec {}
#[doc = "`write(|w| ..)` method takes [`giopuldisb::W`](W) writer structure"]
impl crate::Writable for GiopuldisbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPULDISB to value 0"]
impl crate::Resettable for GiopuldisbSpec {
    const RESET_VALUE: u32 = 0;
}
