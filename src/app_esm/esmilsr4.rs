#[doc = "Register `ESMILSR4` reader"]
pub type R = crate::R<Esmilsr4Spec>;
#[doc = "Register `ESMILSR4` writer"]
pub type W = crate::W<Esmilsr4Spec>;
#[doc = "Field `INTLVLSET` reader - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR4 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR4 register."]
pub type IntlvlsetR = crate::FieldReader<u32>;
#[doc = "Field `INTLVLSET` writer - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR4 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR4 register."]
pub type IntlvlsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR4 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR4 register."]
    #[inline(always)]
    pub fn intlvlset(&self) -> IntlvlsetR {
        IntlvlsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set Interrupt Priority Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding clear bit in the ESMILCR4 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to high level interrupt line and sets the corresponding clear bit in the ESMILCR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn intlvlset(&mut self) -> IntlvlsetW<Esmilsr4Spec> {
        IntlvlsetW::new(self, 0)
    }
}
#[doc = "Interrupt Level Set/Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilsr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilsr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmilsr4Spec;
impl crate::RegisterSpec for Esmilsr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmilsr4::R`](R) reader structure"]
impl crate::Readable for Esmilsr4Spec {}
#[doc = "`write(|w| ..)` method takes [`esmilsr4::W`](W) writer structure"]
impl crate::Writable for Esmilsr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMILSR4 to value 0"]
impl crate::Resettable for Esmilsr4Spec {
    const RESET_VALUE: u32 = 0;
}
