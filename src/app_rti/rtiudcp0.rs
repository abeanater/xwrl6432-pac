#[doc = "Register `RTIUDCP0` reader"]
pub type R = crate::R<Rtiudcp0Spec>;
#[doc = "Register `RTIUDCP0` writer"]
pub type W = crate::W<Rtiudcp0Spec>;
#[doc = "Field `UDCP0` reader - 31:0\\]
UDCP0: Update Compare 0 Register. This registers holds a value, which is added to the value in the compare 0 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 0 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp0R = crate::FieldReader<u32>;
#[doc = "Field `UDCP0` writer - 31:0\\]
UDCP0: Update Compare 0 Register. This registers holds a value, which is added to the value in the compare 0 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 0 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP0: Update Compare 0 Register. This registers holds a value, which is added to the value in the compare 0 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 0 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    pub fn udcp0(&self) -> Udcp0R {
        Udcp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP0: Update Compare 0 Register. This registers holds a value, which is added to the value in the compare 0 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 0 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    #[must_use]
    pub fn udcp0(&mut self) -> Udcp0W<Rtiudcp0Spec> {
        Udcp0W::new(self, 0)
    }
}
#[doc = "Update Compare 0 value to be added to the compare register 0 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtiudcp0Spec;
impl crate::RegisterSpec for Rtiudcp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiudcp0::R`](R) reader structure"]
impl crate::Readable for Rtiudcp0Spec {}
#[doc = "`write(|w| ..)` method takes [`rtiudcp0::W`](W) writer structure"]
impl crate::Writable for Rtiudcp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIUDCP0 to value 0"]
impl crate::Resettable for Rtiudcp0Spec {
    const RESET_VALUE: u32 = 0;
}
