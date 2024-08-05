#[doc = "Register `DCCCNTSEED0` reader"]
pub type R = crate::R<Dcccntseed0Spec>;
#[doc = "Register `DCCCNTSEED0` writer"]
pub type W = crate::W<Dcccntseed0Spec>;
#[doc = "Field `COUNTSEED0` reader - 19:0\\]
The seed value for Counter 0. The seed value that gets loaded into counter 0 (clock source 0)"]
pub type Countseed0R = crate::FieldReader<u32>;
#[doc = "Field `COUNTSEED0` writer - 19:0\\]
The seed value for Counter 0. The seed value that gets loaded into counter 0 (clock source 0)"]
pub type Countseed0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `NU3` reader - 31:20\\]
Reserved"]
pub type Nu3R = crate::FieldReader<u16>;
#[doc = "Field `NU3` writer - 31:20\\]
Reserved"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
The seed value for Counter 0. The seed value that gets loaded into counter 0 (clock source 0)"]
    #[inline(always)]
    pub fn countseed0(&self) -> Countseed0R {
        Countseed0R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
The seed value for Counter 0. The seed value that gets loaded into counter 0 (clock source 0)"]
    #[inline(always)]
    #[must_use]
    pub fn countseed0(&mut self) -> Countseed0W<Dcccntseed0Spec> {
        Countseed0W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<Dcccntseed0Spec> {
        Nu3W::new(self, 20)
    }
}
#[doc = "Seed value for the counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccntseed0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccntseed0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcccntseed0Spec;
impl crate::RegisterSpec for Dcccntseed0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcccntseed0::R`](R) reader structure"]
impl crate::Readable for Dcccntseed0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcccntseed0::W`](W) writer structure"]
impl crate::Writable for Dcccntseed0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCCNTSEED0 to value 0"]
impl crate::Resettable for Dcccntseed0Spec {
    const RESET_VALUE: u32 = 0;
}
