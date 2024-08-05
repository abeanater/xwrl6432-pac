#[doc = "Register `DCCCNTSEED1` reader"]
pub type R = crate::R<Dcccntseed1Spec>;
#[doc = "Register `DCCCNTSEED1` writer"]
pub type W = crate::W<Dcccntseed1Spec>;
#[doc = "Field `COUNTSEED1` reader - 19:0\\]
The seed value for Counter 1. The seed value that gets loaded into counter 1 (clock source 1"]
pub type Countseed1R = crate::FieldReader<u32>;
#[doc = "Field `COUNTSEED1` writer - 19:0\\]
The seed value for Counter 1. The seed value that gets loaded into counter 1 (clock source 1"]
pub type Countseed1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `NU5` reader - 31:20\\]
Reserved"]
pub type Nu5R = crate::FieldReader<u16>;
#[doc = "Field `NU5` writer - 31:20\\]
Reserved"]
pub type Nu5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
The seed value for Counter 1. The seed value that gets loaded into counter 1 (clock source 1"]
    #[inline(always)]
    pub fn countseed1(&self) -> Countseed1R {
        Countseed1R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
The seed value for Counter 1. The seed value that gets loaded into counter 1 (clock source 1"]
    #[inline(always)]
    #[must_use]
    pub fn countseed1(&mut self) -> Countseed1W<Dcccntseed1Spec> {
        Countseed1W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<Dcccntseed1Spec> {
        Nu5W::new(self, 20)
    }
}
#[doc = "Seed value for the counter attached to clock source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccntseed1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccntseed1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcccntseed1Spec;
impl crate::RegisterSpec for Dcccntseed1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcccntseed1::R`](R) reader structure"]
impl crate::Readable for Dcccntseed1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcccntseed1::W`](W) writer structure"]
impl crate::Writable for Dcccntseed1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCCNTSEED1 to value 0"]
impl crate::Resettable for Dcccntseed1Spec {
    const RESET_VALUE: u32 = 0;
}
