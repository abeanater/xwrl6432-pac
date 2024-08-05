#[doc = "Register `RTIFRC1` reader"]
pub type R = crate::R<Rtifrc1Spec>;
#[doc = "Register `RTIFRC1` writer"]
pub type W = crate::W<Rtifrc1Spec>;
#[doc = "Field `FRC1` reader - 31:0\\]
FRC1: Free Running Counter 1. This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1."]
pub type Frc1R = crate::FieldReader<u32>;
#[doc = "Field `FRC1` writer - 31:0\\]
FRC1: Free Running Counter 1. This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1."]
pub type Frc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
FRC1: Free Running Counter 1. This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1."]
    #[inline(always)]
    pub fn frc1(&self) -> Frc1R {
        Frc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
FRC1: Free Running Counter 1. This registers holds the current value of the Free Running Counter 1 and will be updated continuously. User and privilege mode (read): current value of the counter Privilege mode (write): The counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1."]
    #[inline(always)]
    #[must_use]
    pub fn frc1(&mut self) -> Frc1W<Rtifrc1Spec> {
        Frc1W::new(self, 0)
    }
}
#[doc = "Free Running Counter 1 current value of free running counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtifrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtifrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtifrc1Spec;
impl crate::RegisterSpec for Rtifrc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtifrc1::R`](R) reader structure"]
impl crate::Readable for Rtifrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtifrc1::W`](W) writer structure"]
impl crate::Writable for Rtifrc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIFRC1 to value 0"]
impl crate::Resettable for Rtifrc1Spec {
    const RESET_VALUE: u32 = 0;
}
