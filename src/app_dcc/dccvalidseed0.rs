#[doc = "Register `DCCVALIDSEED0` reader"]
pub type R = crate::R<Dccvalidseed0Spec>;
#[doc = "Register `DCCVALIDSEED0` writer"]
pub type W = crate::W<Dccvalidseed0Spec>;
#[doc = "Field `VALIDSEED0` reader - 15:0\\]
The seed value for Valid Duration Counter 0.The seed value that gets loaded into the valid duration counter for clock source 0"]
pub type Validseed0R = crate::FieldReader<u16>;
#[doc = "Field `VALIDSEED0` writer - 15:0\\]
The seed value for Valid Duration Counter 0.The seed value that gets loaded into the valid duration counter for clock source 0"]
pub type Validseed0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU4` reader - 31:16\\]
Reserved"]
pub type Nu4R = crate::FieldReader<u16>;
#[doc = "Field `NU4` writer - 31:16\\]
Reserved"]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The seed value for Valid Duration Counter 0.The seed value that gets loaded into the valid duration counter for clock source 0"]
    #[inline(always)]
    pub fn validseed0(&self) -> Validseed0R {
        Validseed0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The seed value for Valid Duration Counter 0.The seed value that gets loaded into the valid duration counter for clock source 0"]
    #[inline(always)]
    #[must_use]
    pub fn validseed0(&mut self) -> Validseed0W<Dccvalidseed0Spec> {
        Validseed0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<Dccvalidseed0Spec> {
        Nu4W::new(self, 16)
    }
}
#[doc = "Seed value for the timeout counter attached to clock source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dccvalidseed0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccvalidseed0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dccvalidseed0Spec;
impl crate::RegisterSpec for Dccvalidseed0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccvalidseed0::R`](R) reader structure"]
impl crate::Readable for Dccvalidseed0Spec {}
#[doc = "`write(|w| ..)` method takes [`dccvalidseed0::W`](W) writer structure"]
impl crate::Writable for Dccvalidseed0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCVALIDSEED0 to value 0"]
impl crate::Resettable for Dccvalidseed0Spec {
    const RESET_VALUE: u32 = 0;
}
