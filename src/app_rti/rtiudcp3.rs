#[doc = "Register `RTIUDCP3` reader"]
pub type R = crate::R<Rtiudcp3Spec>;
#[doc = "Register `RTIUDCP3` writer"]
pub type W = crate::W<Rtiudcp3Spec>;
#[doc = "Field `UDCP3` reader - 31:0\\]
UDCP3: Update compare 3 Register. This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp3R = crate::FieldReader<u32>;
#[doc = "Field `UDCP3` writer - 31:0\\]
UDCP3: Update compare 3 Register. This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
pub type Udcp3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP3: Update compare 3 Register. This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    pub fn udcp3(&self) -> Udcp3R {
        Udcp3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
UDCP3: Update compare 3 Register. This registers holds a value, which is added to the value in the compare 3 register each time a compare matches. This gives the possibility to generate periodic interrupts without software intervention. User and privilege mode (read): value to be added to the compare 3 register on the next compare match Privilege mode (write): new update value"]
    #[inline(always)]
    #[must_use]
    pub fn udcp3(&mut self) -> Udcp3W<Rtiudcp3Spec> {
        Udcp3W::new(self, 0)
    }
}
#[doc = "Update Compare 3 value to be added to the compare register 3 value on compare match\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiudcp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiudcp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtiudcp3Spec;
impl crate::RegisterSpec for Rtiudcp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiudcp3::R`](R) reader structure"]
impl crate::Readable for Rtiudcp3Spec {}
#[doc = "`write(|w| ..)` method takes [`rtiudcp3::W`](W) writer structure"]
impl crate::Writable for Rtiudcp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIUDCP3 to value 0"]
impl crate::Resettable for Rtiudcp3Spec {
    const RESET_VALUE: u32 = 0;
}
