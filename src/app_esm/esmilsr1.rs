#[doc = "Register `ESMILSR1` reader"]
pub type R = crate::R<Esmilsr1Spec>;
#[doc = "Register `ESMILSR1` writer"]
pub type W = crate::W<Esmilsr1Spec>;
#[doc = "Field `INTLVLSET` reader - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR1 register."]
pub type IntlvlsetR = crate::FieldReader<u32>;
#[doc = "Field `INTLVLSET` writer - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR1 register."]
pub type IntlvlsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR1 register."]
    #[inline(always)]
    pub fn intlvlset(&self) -> IntlvlsetR {
        IntlvlsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn intlvlset(&mut self) -> IntlvlsetW<Esmilsr1Spec> {
        IntlvlsetW::new(self, 0)
    }
}
#[doc = "Interrupt Level Set/Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmilsr1Spec;
impl crate::RegisterSpec for Esmilsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmilsr1::R`](R) reader structure"]
impl crate::Readable for Esmilsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`esmilsr1::W`](W) writer structure"]
impl crate::Writable for Esmilsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMILSR1 to value 0"]
impl crate::Resettable for Esmilsr1Spec {
    const RESET_VALUE: u32 = 0;
}
