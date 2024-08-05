#[doc = "Register `RTICPUC1` reader"]
pub type R = crate::R<Rticpuc1Spec>;
#[doc = "Register `RTICPUC1` writer"]
pub type W = crate::W<Rticpuc1Spec>;
#[doc = "Field `CPUC1` reader - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: then, frequency = RTICLK/ (2^32) If CPUC1 Γëá 0: then , frequency = RTICLK/(CPUC1 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
pub type Cpuc1R = crate::FieldReader<u32>;
#[doc = "Field `CPUC1` writer - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: then, frequency = RTICLK/ (2^32) If CPUC1 Γëá 0: then , frequency = RTICLK/(CPUC1 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
pub type Cpuc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: then, frequency = RTICLK/ (2^32) If CPUC1 Γëá 0: then , frequency = RTICLK/(CPUC1 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
    #[inline(always)]
    pub fn cpuc1(&self) -> Cpuc1R {
        Cpuc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: then, frequency = RTICLK/ (2^32) If CPUC1 Γëá 0: then , frequency = RTICLK/(CPUC1 + 1) User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed"]
    #[inline(always)]
    #[must_use]
    pub fn cpuc1(&mut self) -> Cpuc1W<Rticpuc1Spec> {
        Cpuc1W::new(self, 0)
    }
}
#[doc = "Compare Up Counter 1 compare value compared with prescale counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rticpuc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticpuc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticpuc1Spec;
impl crate::RegisterSpec for Rticpuc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticpuc1::R`](R) reader structure"]
impl crate::Readable for Rticpuc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rticpuc1::W`](W) writer structure"]
impl crate::Writable for Rticpuc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICPUC1 to value 0"]
impl crate::Resettable for Rticpuc1Spec {
    const RESET_VALUE: u32 = 0;
}
