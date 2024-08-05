#[doc = "Register `RTIUDCP1` reader"]
pub type R = crate::R<Rtiudcp1Spec>;
#[doc = "Register `RTIUDCP1` writer"]
pub type W = crate::W<Rtiudcp1Spec>;
#[doc = "Field `UDCP1` reader - 31:0\\]
UDCP1: Update compare1 Register. This registers holds a value, which is added to the value in the compare1 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare1 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp1R = crate::FieldReader<u32>;
#[doc = "Field `UDCP1` writer - 31:0\\]
UDCP1: Update compare1 Register. This registers holds a value, which is added to the value in the compare1 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare1 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP1: Update compare1 Register. This registers holds a value, which is added to the value in the compare1 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare1 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    pub fn udcp1(&self) -> Udcp1R {
        Udcp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP1: Update compare1 Register. This registers holds a value, which is added to the value in the compare1 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare1 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    #[must_use]
    pub fn udcp1(&mut self) -> Udcp1W<Rtiudcp1Spec> {
        Udcp1W::new(self, 0)
    }
}
#[doc = "Update Compare 1 value to be added to the compare register 1 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtiudcp1Spec;
impl crate::RegisterSpec for Rtiudcp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiudcp1::R`](R) reader structure"]
impl crate::Readable for Rtiudcp1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtiudcp1::W`](W) writer structure"]
impl crate::Writable for Rtiudcp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIUDCP1 to value 0"]
impl crate::Resettable for Rtiudcp1Spec {
    const RESET_VALUE: u32 = 0;
}
