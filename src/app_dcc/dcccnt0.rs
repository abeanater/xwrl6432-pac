#[doc = "Register `DCCCNT0` reader"]
pub type R = crate::R<Dcccnt0Spec>;
#[doc = "Register `DCCCNT0` writer"]
pub type W = crate::W<Dcccnt0Spec>;
#[doc = "Field `COUNT0` reader - 19:0\\]
This field contains the current value of counter 0. - (RO )"]
pub type Count0R = crate::FieldReader<u32>;
#[doc = "Field `COUNT0` writer - 19:0\\]
This field contains the current value of counter 0. - (RO )"]
pub type Count0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `NU7` reader - 31:20\\]
Reserved"]
pub type Nu7R = crate::FieldReader<u16>;
#[doc = "Field `NU7` writer - 31:20\\]
Reserved"]
pub type Nu7W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 0. - (RO )"]
    #[inline(always)]
    pub fn count0(&self) -> Count0R {
        Count0R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu7(&self) -> Nu7R {
        Nu7R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 0. - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn count0(&mut self) -> Count0W<Dcccnt0Spec> {
        Count0W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu7(&mut self) -> Nu7W<Dcccnt0Spec> {
        Nu7W::new(self, 20)
    }
}
#[doc = "Value of the counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcccnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcccnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcccnt0Spec;
impl crate::RegisterSpec for Dcccnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcccnt0::R`](R) reader structure"]
impl crate::Readable for Dcccnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`dcccnt0::W`](W) writer structure"]
impl crate::Writable for Dcccnt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCCNT0 to value 0"]
impl crate::Resettable for Dcccnt0Spec {
    const RESET_VALUE: u32 = 0;
}