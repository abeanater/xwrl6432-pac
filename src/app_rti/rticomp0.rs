#[doc = "Register `RTICOMP0` reader"]
pub type R = crate::R<Rticomp0Spec>;
#[doc = "Register `RTICOMP0` writer"]
pub type W = crate::W<Rticomp0Spec>;
#[doc = "Field `COMP0` reader - 31:0\\]
COMP0: Compare 0. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp0R = crate::FieldReader<u32>;
#[doc = "Field `COMP0` writer - 31:0\\]
COMP0: Compare 0. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
pub type Comp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
COMP0: Compare 0. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
COMP0: Compare 0. This registers holds a compare value, which is compared with the counter selected in the compare control logic. If the Free Running Counter matches the compare value, an interrupt is flagged. With this register it is also possible to initiate a DMA request. User and privilege mode (read): current compare value Privilege mode (write): update of the compare register with a new compare value Note: Reset behavior A reset does not generate a compare match, since the compare logic will only be active, when the associated counter block is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> Comp0W<Rticomp0Spec> {
        Comp0W::new(self, 0)
    }
}
#[doc = "Compare 0 compare value to be compared with the counters\n\nYou can [`read`](crate::Reg::read) this register and get [`rticomp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticomp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticomp0Spec;
impl crate::RegisterSpec for Rticomp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticomp0::R`](R) reader structure"]
impl crate::Readable for Rticomp0Spec {}
#[doc = "`write(|w| ..)` method takes [`rticomp0::W`](W) writer structure"]
impl crate::Writable for Rticomp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICOMP0 to value 0"]
impl crate::Resettable for Rticomp0Spec {
    const RESET_VALUE: u32 = 0;
}
