#[doc = "Register `RTIUDCP2` reader"]
pub type R = crate::R<Rtiudcp2Spec>;
#[doc = "Register `RTIUDCP2` writer"]
pub type W = crate::W<Rtiudcp2Spec>;
#[doc = "Field `UDCP2` reader - 31:0\\]
UDCP2: Update compare 2 Register. This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp2R = crate::FieldReader<u32>;
#[doc = "Field `UDCP2` writer - 31:0\\]
UDCP2: Update compare 2 Register. This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP2: Update compare 2 Register. This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    pub fn udcp2(&self) -> Udcp2R {
        Udcp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP2: Update compare 2 Register. This registers holds a value, which is added to the value in the compare 2 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 2 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    #[must_use]
    pub fn udcp2(&mut self) -> Udcp2W<Rtiudcp2Spec> {
        Udcp2W::new(self, 0)
    }
}
#[doc = "Update Compare 2 value to be added to the compare register 2 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtiudcp2Spec;
impl crate::RegisterSpec for Rtiudcp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiudcp2::R`](R) reader structure"]
impl crate::Readable for Rtiudcp2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtiudcp2::W`](W) writer structure"]
impl crate::Writable for Rtiudcp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIUDCP2 to value 0"]
impl crate::Resettable for Rtiudcp2Spec {
    const RESET_VALUE: u32 = 0;
}
