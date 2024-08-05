#[doc = "Register `DCCVALID0` reader"]
pub type R = crate::R<Dccvalid0Spec>;
#[doc = "Register `DCCVALID0` writer"]
pub type W = crate::W<Dccvalid0Spec>;
#[doc = "Field `VALID0` reader - 15:0\\]
This field contains the current value of valid counter 0. - (RO )"]
pub type Valid0R = crate::FieldReader<u16>;
#[doc = "Field `VALID0` writer - 15:0\\]
This field contains the current value of valid counter 0. - (RO )"]
pub type Valid0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU8` reader - 31:16\\]
Reserved"]
pub type Nu8R = crate::FieldReader<u16>;
#[doc = "Field `NU8` writer - 31:16\\]
Reserved"]
pub type Nu8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This field contains the current value of valid counter 0. - (RO )"]
    #[inline(always)]
    pub fn valid0(&self) -> Valid0R {
        Valid0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu8(&self) -> Nu8R {
        Nu8R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This field contains the current value of valid counter 0. - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn valid0(&mut self) -> Valid0W<Dccvalid0Spec> {
        Valid0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu8(&mut self) -> Nu8W<Dccvalid0Spec> {
        Nu8W::new(self, 16)
    }
}
#[doc = "Value of the valid counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dccvalid0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccvalid0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dccvalid0Spec;
impl crate::RegisterSpec for Dccvalid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccvalid0::R`](R) reader structure"]
impl crate::Readable for Dccvalid0Spec {}
#[doc = "`write(|w| ..)` method takes [`dccvalid0::W`](W) writer structure"]
impl crate::Writable for Dccvalid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCVALID0 to value 0"]
impl crate::Resettable for Dccvalid0Spec {
    const RESET_VALUE: u32 = 0;
}
