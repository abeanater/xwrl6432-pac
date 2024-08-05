#[doc = "Register `RTICPUC0` reader"]
pub type R = crate::R<Rticpuc0Spec>;
#[doc = "Register `RTICPUC0` writer"]
pub type W = crate::W<Rticpuc0Spec>;
#[doc = "Field `CPUC0` reader - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 0. When the compare matches, Free Running counter 0 is incremented. The Up Counter is set to zero when the counter value matches the CPUC0 value. The value set in this prescales the RTI clock. If CPUC0 = 0: then, frequency = RTICLK/ (2^32) If CPUC0 Γëá 0: then , frequency = RTICLK/(CPUC0 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
pub type Cpuc0R = crate::FieldReader<u32>;
#[doc = "Field `CPUC0` writer - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 0. When the compare matches, Free Running counter 0 is incremented. The Up Counter is set to zero when the counter value matches the CPUC0 value. The value set in this prescales the RTI clock. If CPUC0 = 0: then, frequency = RTICLK/ (2^32) If CPUC0 Γëá 0: then , frequency = RTICLK/(CPUC0 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
pub type Cpuc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 0. When the compare matches, Free Running counter 0 is incremented. The Up Counter is set to zero when the counter value matches the CPUC0 value. The value set in this prescales the RTI clock. If CPUC0 = 0: then, frequency = RTICLK/ (2^32) If CPUC0 Γëá 0: then , frequency = RTICLK/(CPUC0 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
    #[inline(always)]
    pub fn cpuc0(&self) -> Cpuc0R {
        Cpuc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 0. When the compare matches, Free Running counter 0 is incremented. The Up Counter is set to zero when the counter value matches the CPUC0 value. The value set in this prescales the RTI clock. If CPUC0 = 0: then, frequency = RTICLK/ (2^32) If CPUC0 Γëá 0: then , frequency = RTICLK/(CPUC0 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
    #[inline(always)]
    #[must_use]
    pub fn cpuc0(&mut self) -> Cpuc0W<Rticpuc0Spec> {
        Cpuc0W::new(self, 0)
    }
}
#[doc = "Compare Up Counter 0 compare value compared with prescale counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rticpuc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticpuc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticpuc0Spec;
impl crate::RegisterSpec for Rticpuc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticpuc0::R`](R) reader structure"]
impl crate::Readable for Rticpuc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rticpuc0::W`](W) writer structure"]
impl crate::Writable for Rticpuc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICPUC0 to value 0"]
impl crate::Resettable for Rticpuc0Spec {
    const RESET_VALUE: u32 = 0;
}
