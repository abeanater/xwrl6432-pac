#[doc = "Register `TOP_INTMASK` reader"]
pub type R = crate::R<TopIntmaskSpec>;
#[doc = "Register `TOP_INTMASK` writer"]
pub type W = crate::W<TopIntmaskSpec>;
#[doc = "Field `set` reader - 31:0\\]
Mask Interrupt from frame timer 1 : Interrupt is Masked 0 : Interrupt is Unmasked Bit 0 - Mask Interrupts from Frame Timer Bit 31:0 - Reserved"]
pub type SetR = crate::FieldReader<u32>;
#[doc = "Field `set` writer - 31:0\\]
Mask Interrupt from frame timer 1 : Interrupt is Masked 0 : Interrupt is Unmasked Bit 0 - Mask Interrupts from Frame Timer Bit 31:0 - Reserved"]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Interrupt from frame timer 1 : Interrupt is Masked 0 : Interrupt is Unmasked Bit 0 - Mask Interrupts from Frame Timer Bit 31:0 - Reserved"]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Mask Interrupt from frame timer 1 : Interrupt is Masked 0 : Interrupt is Unmasked Bit 0 - Mask Interrupts from Frame Timer Bit 31:0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<TopIntmaskSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "TOP_INTMASK\n\nYou can [`read`](crate::Reg::read) this register and get [`top_intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopIntmaskSpec;
impl crate::RegisterSpec for TopIntmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top_intmask::R`](R) reader structure"]
impl crate::Readable for TopIntmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`top_intmask::W`](W) writer structure"]
impl crate::Writable for TopIntmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOP_INTMASK to value 0"]
impl crate::Resettable for TopIntmaskSpec {
    const RESET_VALUE: u32 = 0;
}
