#[doc = "Register `DCCCNT1` reader"]
pub type R = crate::R<Dcccnt1Spec>;
#[doc = "Register `DCCCNT1` writer"]
pub type W = crate::W<Dcccnt1Spec>;
#[doc = "Field `COUNT1` reader - 19:0\\]
This field contains the current value of counter 1. - (RO )"]
pub type Count1R = crate::FieldReader<u32>;
#[doc = "Field `COUNT1` writer - 19:0\\]
This field contains the current value of counter 1. - (RO )"]
pub type Count1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `NU9` reader - 31:20\\]
Reserved"]
pub type Nu9R = crate::FieldReader<u16>;
#[doc = "Field `NU9` writer - 31:20\\]
Reserved"]
pub type Nu9W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 1. - (RO )"]
    #[inline(always)]
    pub fn count1(&self) -> Count1R {
        Count1R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu9(&self) -> Nu9R {
        Nu9R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 1. - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn count1(&mut self) -> Count1W<Dcccnt1Spec> {
        Count1W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu9(&mut self) -> Nu9W<Dcccnt1Spec> {
        Nu9W::new(self, 20)
    }
}
#[doc = "Value of the counter attached to clock source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcccnt1Spec;
impl crate::RegisterSpec for Dcccnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcccnt1::R`](R) reader structure"]
impl crate::Readable for Dcccnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcccnt1::W`](W) writer structure"]
impl crate::Writable for Dcccnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCCNT1 to value 0"]
impl crate::Resettable for Dcccnt1Spec {
    const RESET_VALUE: u32 = 0;
}
