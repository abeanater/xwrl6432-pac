#[doc = "Register `ESMIOFFLR` reader"]
pub type R = crate::R<EsmiofflrSpec>;
#[doc = "Register `ESMIOFFLR` writer"]
pub type W = crate::W<EsmiofflrSpec>;
#[doc = "Field `INTOFFL` reader - 7:0\\]
Offset Low Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the low level interrupt line. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the low level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will not clear the corresponding flag in the ESMSR1 register. Group2 interrupts are fixed to the high level interrupt line only. User and privileged mode (write): Writes have no effect."]
pub type IntofflR = crate::FieldReader;
#[doc = "Field `INTOFFL` writer - 7:0\\]
Offset Low Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the low level interrupt line. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the low level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will not clear the corresponding flag in the ESMSR1 register. Group2 interrupts are fixed to the high level interrupt line only. User and privileged mode (write): Writes have no effect."]
pub type IntofflW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Offset Low Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the low level interrupt line. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the low level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will not clear the corresponding flag in the ESMSR1 register. Group2 interrupts are fixed to the high level interrupt line only. User and privileged mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn intoffl(&self) -> IntofflR {
        IntofflR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Offset Low Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the low level interrupt line. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the low level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will not clear the corresponding flag in the ESMSR1 register. Group2 interrupts are fixed to the high level interrupt line only. User and privileged mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn intoffl(&mut self) -> IntofflW<EsmiofflrSpec> {
        IntofflW::new(self, 0)
    }
}
#[doc = "ESM Interrupt Offset Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiofflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiofflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsmiofflrSpec;
impl crate::RegisterSpec for EsmiofflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmiofflr::R`](R) reader structure"]
impl crate::Readable for EsmiofflrSpec {}
#[doc = "`write(|w| ..)` method takes [`esmiofflr::W`](W) writer structure"]
impl crate::Writable for EsmiofflrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIOFFLR to value 0"]
impl crate::Resettable for EsmiofflrSpec {
    const RESET_VALUE: u32 = 0;
}
