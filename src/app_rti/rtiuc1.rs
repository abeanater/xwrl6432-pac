#[doc = "Register `RTIUC1` reader"]
pub type R = crate::R<Rtiuc1Spec>;
#[doc = "Register `RTIUC1` writer"]
pub type W = crate::W<Rtiuc1Spec>;
#[doc = "Field `UC1` reader - 31:0\\]
UC1: Up Counter 1. This registers holds the current value of the Up Counter 1 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 1. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 1 and Free Running Counter 1. User and privilege mode (read): value of the counter when the Free Running Counter 1 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC1 then it can take a long time until a compare matches, since RTIUC1 has to count up until it overflows."]
pub type Uc1R = crate::FieldReader<u32>;
#[doc = "Field `UC1` writer - 31:0\\]
UC1: Up Counter 1. This registers holds the current value of the Up Counter 1 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 1. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 1 and Free Running Counter 1. User and privilege mode (read): value of the counter when the Free Running Counter 1 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC1 then it can take a long time until a compare matches, since RTIUC1 has to count up until it overflows."]
pub type Uc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
UC1: Up Counter 1. This registers holds the current value of the Up Counter 1 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 1. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 1 and Free Running Counter 1. User and privilege mode (read): value of the counter when the Free Running Counter 1 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC1 then it can take a long time until a compare matches, since RTIUC1 has to count up until it overflows."]
    #[inline(always)]
    pub fn uc1(&self) -> Uc1R {
        Uc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
UC1: Up Counter 1. This registers holds the current value of the Up Counter 1 and prescales the RTI clock. It will be only updated by a previous read of Free Running Counter 1. This gives effectively a 64 bit read of both counters, without having the problem of a counter being updated between two consecutive reads on Up Counter 1 and Free Running Counter 1. User and privilege mode (read): value of the counter when the Free Running Counter 1 was read Privilege mode (write): the counter can be preset by writing to this register. The counter increments then from this written value upwards. Note: Presetting counters If counters have to be preset, they have to be stopped from counting in the RTIGCTRL register in order to ensure consistency between RTIUC1 and RTIFRC1. Note: Preset value concern If the preset value is bigger than the compare value stored in register RTICPUC1 then it can take a long time until a compare matches, since RTIUC1 has to count up until it overflows."]
    #[inline(always)]
    #[must_use]
    pub fn uc1(&mut self) -> Uc1W<Rtiuc1Spec> {
        Uc1W::new(self, 0)
    }
}
#[doc = "Up Counter 1 current value of prescale counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiuc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiuc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtiuc1Spec;
impl crate::RegisterSpec for Rtiuc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiuc1::R`](R) reader structure"]
impl crate::Readable for Rtiuc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtiuc1::W`](W) writer structure"]
impl crate::Writable for Rtiuc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIUC1 to value 0"]
impl crate::Resettable for Rtiuc1Spec {
    const RESET_VALUE: u32 = 0;
}
