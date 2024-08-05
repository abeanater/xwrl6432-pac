#[doc = "Register `ESMIOFFHR` reader"]
pub type R = crate::R<EsmioffhrSpec>;
#[doc = "Register `ESMIOFFHR` writer"]
pub type W = crate::W<EsmioffhrSpec>;
#[doc = "Field `INTOFFH` reader - 8:0\\]
Offset High Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the high level interrupt line. Interrupts of error Group2 have higher priority than interrupts of error Group1. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the high level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 0, error Group2. ... 40h Interrupt pending for channel 31, error Group2. 41h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will clear the corresponding flag in the ESMSR2 register; will not clear ESMSR1 and ESMSSR2 and the offset register gets updated. User and privileged mode (write): Writes have no effect."]
pub type IntoffhR = crate::FieldReader<u16>;
#[doc = "Field `INTOFFH` writer - 8:0\\]
Offset High Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the high level interrupt line. Interrupts of error Group2 have higher priority than interrupts of error Group1. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the high level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 0, error Group2. ... 40h Interrupt pending for channel 31, error Group2. 41h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will clear the corresponding flag in the ESMSR2 register; will not clear ESMSR1 and ESMSSR2 and the offset register gets updated. User and privileged mode (write): Writes have no effect."]
pub type IntoffhW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Offset High Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the high level interrupt line. Interrupts of error Group2 have higher priority than interrupts of error Group1. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the high level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 0, error Group2. ... 40h Interrupt pending for channel 31, error Group2. 41h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will clear the corresponding flag in the ESMSR2 register; will not clear ESMSR1 and ESMSSR2 and the offset register gets updated. User and privileged mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn intoffh(&self) -> IntoffhR {
        IntoffhR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Offset High Level Interrupt. This vector gives the channel number of the highest pending interrupt request for the high level interrupt line. Interrupts of error Group2 have higher priority than interrupts of error Group1. Inside a group, channel 0 has highest priority and channel 31 has lowest priority. User and privileged mode (read): Returns number of pending interrupt with the highest priority for the high level interrupt line. 0 No pending interrupt. 1h Interrupt pending for channel 0, error Group1. ... 20h Interrupt pending for channel 31, error Group1. 21h Interrupt pending for channel 0, error Group2. ... 40h Interrupt pending for channel 31, error Group2. 41h Interrupt pending for channel 32, error Group1. ... 60h Interrupt pending for channel 63, error Group1. Note: Reading the interrupt vector will clear the corresponding flag in the ESMSR2 register; will not clear ESMSR1 and ESMSSR2 and the offset register gets updated. User and privileged mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn intoffh(&mut self) -> IntoffhW<EsmioffhrSpec> {
        IntoffhW::new(self, 0)
    }
}
#[doc = "ESM Interrupt Offset High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmioffhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmioffhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsmioffhrSpec;
impl crate::RegisterSpec for EsmioffhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmioffhr::R`](R) reader structure"]
impl crate::Readable for EsmioffhrSpec {}
#[doc = "`write(|w| ..)` method takes [`esmioffhr::W`](W) writer structure"]
impl crate::Writable for EsmioffhrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIOFFHR to value 0"]
impl crate::Resettable for EsmioffhrSpec {
    const RESET_VALUE: u32 = 0;
}
